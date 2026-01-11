// @ts-nocheck
// Edge Function (Deno) - Commerce devis reminders worker
// Déclenchement: cron (Supabase Scheduled Functions) ou webhook interne.
//
// Responsabilités:
// - Lire `public.commerce_quote_reminders` (status/run_at/next_retry_at)
// - (Optionnel) annuler si le devis n’est plus en statut relançable
// - Alimenter `public.commerce_outbox` avec event_type = `commerce.quote.reminder.due`
// - Mettre à jour `commerce_quote_reminders` en `sent` ou `error` avec retry/backoff
//
// Sécurité:
// - verify_jwt est activé au déploiement (Authorization: Bearer <JWT> requis)
// - Optionnel: vérifier un secret via header `x-cron-secret` si CRON_SECRET est défini

import "jsr:@supabase/functions-js/edge-runtime.d.ts";

import { createClient } from "https://esm.sh/@supabase/supabase-js@2.49.1";

type ReminderStatus = "scheduled" | "sent" | "cancelled" | "error";

type ReminderRow = {
  id: string;
  quote_id: string;
  recipient_email: string;
  sequence: number;
  run_at: string;
  status: ReminderStatus;
  sent_at: string | null;
  attempt_count: number;
  next_retry_at: string | null;
  last_error: string | null;
  payload: Record<string, unknown>;
  dedupe_key: string;
};

type QuoteRow = {
  id: string;
  status:
    | "draft"
    | "sent"
    | "viewed"
    | "accepted"
    | "rejected"
    | "expired"
    | "cancelled";
  quote_number: string;
  requester_id: string | null;
};

function json(data: unknown, status = 200) {
  return new Response(JSON.stringify(data), {
    status,
    headers: {
      "Content-Type": "application/json",
      Connection: "keep-alive",
    },
  });
}

function computeBackoffSeconds(attemptCount: number) {
  // attempt_count = nb d'échecs déjà enregistrés
  // On calcule le prochain délai après l'échec courant.
  // 1 => 60s, 2 => 300s, 3 => 1800s, puis cap à 6h.
  const schedule = [60, 300, 1800, 7200, 21600];
  const idx = Math.min(Math.max(attemptCount, 0), schedule.length - 1);
  return schedule[idx];
}

Deno.serve(async (req: Request) => {
  try {
    if (req.method !== "POST") {
      return json({ error: "method_not_allowed" }, 405);
    }

    const cronSecret = Deno.env.get("CRON_SECRET");
    if (cronSecret) {
      const provided = req.headers.get("x-cron-secret");
      if (!provided || provided !== cronSecret) {
        return json({ error: "forbidden" }, 403);
      }
    }

    const supabaseUrl = Deno.env.get("SUPABASE_URL");
    const serviceRoleKey = Deno.env.get("SUPABASE_SERVICE_ROLE_KEY");
    if (!supabaseUrl || !serviceRoleKey) {
      return json({ error: "missing_env" }, 500);
    }

    const supabase = createClient(supabaseUrl, serviceRoleKey, {
      auth: { persistSession: false, autoRefreshToken: false },
    });

    const nowIso = new Date().toISOString();

    // Sélection des reminders à traiter
    // - scheduled dont run_at <= now
    // - error dont next_retry_at <= now
    const { data: reminders, error: remindersError } = await supabase
      .from("commerce_quote_reminders")
      .select(
        "id,quote_id,recipient_email,sequence,run_at,status,sent_at,attempt_count,next_retry_at,last_error,payload,dedupe_key",
      )
      .or(`and(status.eq.scheduled,run_at.lte.${nowIso}),and(status.eq.error,next_retry_at.lte.${nowIso})`)
      .order("run_at", { ascending: true })
      .limit(50);

    if (remindersError) {
      return json({ error: "read_failed", details: remindersError.message }, 500);
    }

    const results = {
      scanned: reminders?.length ?? 0,
      sent: 0,
      cancelled: 0,
      errored: 0,
      outbox_already: 0,
    };

    for (const r of (reminders ?? []) as ReminderRow[]) {
      // Vérifier statut du devis (si non relançable => cancel)
      const { data: quote, error: quoteError } = await supabase
        .from("commerce_quotes")
        .select("id,status,quote_number,requester_id")
        .eq("id", r.quote_id)
        .maybeSingle();

      if (quoteError) {
        const msg = `quote_lookup_failed:${quoteError.message}`;
        results.errored++;
        const attempt = (r.attempt_count ?? 0) + 1;
        const backoffSec = computeBackoffSeconds(attempt - 1);
        const nextRetryAt = new Date(Date.now() + backoffSec * 1000).toISOString();
        await supabase
          .from("commerce_quote_reminders")
          .update({
            status: "error",
            attempt_count: attempt,
            last_error: msg,
            next_retry_at: nextRetryAt,
          })
          .eq("id", r.id);
        continue;
      }

      const q = quote as QuoteRow | null;
      if (!q) {
        // Devis supprimé => annuler
        results.cancelled++;
        await supabase
          .from("commerce_quote_reminders")
          .update({ status: "cancelled", last_error: "quote_not_found" })
          .eq("id", r.id);
        continue;
      }

      if (!(q.status === "sent" || q.status === "viewed")) {
        results.cancelled++;
        await supabase
          .from("commerce_quote_reminders")
          .update({
            status: "cancelled",
            last_error: `quote_not_relaunchable:${q.status}`,
          })
          .eq("id", r.id);
        continue;
      }

      // Alimenter l’outbox (idempotence via dedupe_key UNIQUE côté commerce_outbox)
      const payload = {
        quote_id: r.quote_id,
        quote_number: q.quote_number,
        requester_id: q.requester_id,
        recipient_email: r.recipient_email,
        sequence: r.sequence,
        scheduled_for: r.run_at,
        ...((r.payload ?? {}) as Record<string, unknown>),
      };

      const { error: outboxError } = await supabase
        .from("commerce_outbox")
        .insert({
          event_type: "commerce.quote.reminder.due",
          dedupe_key: r.dedupe_key,
          payload,
          status: "pending",
        });

      if (outboxError) {
        // 23505 = unique_violation => déjà en outbox => considéré "sent" côté reminder
        const pgCode = (outboxError as unknown as { code?: string }).code;
        if (pgCode === "23505") {
          results.outbox_already++;
          results.sent++;
          await supabase
            .from("commerce_quote_reminders")
            .update({
              status: "sent",
              sent_at: nowIso,
              last_error: null,
              next_retry_at: null,
            })
            .eq("id", r.id);
          continue;
        }

        results.errored++;
        const attempt = (r.attempt_count ?? 0) + 1;
        const backoffSec = computeBackoffSeconds(attempt - 1);
        const nextRetryAt = new Date(Date.now() + backoffSec * 1000).toISOString();
        await supabase
          .from("commerce_quote_reminders")
          .update({
            status: "error",
            attempt_count: attempt,
            last_error: `outbox_insert_failed:${outboxError.message}`,
            next_retry_at: nextRetryAt,
          })
          .eq("id", r.id);
        continue;
      }

      results.sent++;
      await supabase
        .from("commerce_quote_reminders")
        .update({
          status: "sent",
          sent_at: nowIso,
          last_error: null,
          next_retry_at: null,
        })
        .eq("id", r.id);
    }

    return json({ ok: true, now: nowIso, results });
  } catch (e) {
    const msg = e instanceof Error ? e.message : String(e);
    return json({ error: "unexpected", details: msg }, 500);
  }
});

