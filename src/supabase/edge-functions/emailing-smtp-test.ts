// @ts-nocheck
// Source local (référence) de l’Edge Function Supabase: emailing-smtp-test
// Déployée via MCP Supabase.

import "jsr:@supabase/functions-js/edge-runtime.d.ts";

import { createClient } from "https://esm.sh/@supabase/supabase-js@2.49.1";
import { SMTPClient } from "https://deno.land/x/denomailer@1.6.0/mod.ts";

const GLOBAL_WORKSPACE_ID = "00000000-0000-0000-0000-000000000000";

function json(data: unknown, status = 200) {
  return new Response(JSON.stringify(data), {
    status,
    headers: {
      "Content-Type": "application/json",
      Connection: "keep-alive",
    },
  });
}

function asNonEmptyString(v: unknown) {
  return typeof v === "string" && v.trim().length ? v.trim() : null;
}

Deno.serve(async (req: Request) => {
  const startedAt = new Date().toISOString();
  try {
    if (req.method !== "POST") return json({ error: "method_not_allowed" }, 405);

    const supabaseUrl = Deno.env.get("SUPABASE_URL");
    const anonKey = Deno.env.get("SUPABASE_ANON_KEY");
    const serviceRoleKey = Deno.env.get("SUPABASE_SERVICE_ROLE_KEY");
    if (!supabaseUrl || !anonKey || !serviceRoleKey) {
      return json({ error: "missing_env" }, 500);
    }

    const authHeader = req.headers.get("Authorization") ?? "";
    const userClient = createClient(supabaseUrl, anonKey, {
      auth: { persistSession: false, autoRefreshToken: false },
      global: { headers: { Authorization: authHeader } },
    });
    const serviceClient = createClient(supabaseUrl, serviceRoleKey, {
      auth: { persistSession: false, autoRefreshToken: false },
    });

    const { data: userData, error: userError } = await userClient.auth.getUser();
    const user = userData?.user ?? null;
    if (userError || !user) return json({ error: "unauthorized" }, 401);

    const { data: profile, error: profileError } = await serviceClient
      .from("profiles")
      .select("id,role")
      .eq("id", user.id)
      .maybeSingle();
    if (profileError) return json({ error: "profile_lookup_failed" }, 500);
    if (!profile || profile.role !== "super_admin") return json({ error: "forbidden" }, 403);

    const body = await req.json().catch(() => ({}));
    const to = asNonEmptyString(body?.to);
    const subject = asNonEmptyString(body?.subject) ?? "Test SMTP (Miyukini)";
    const text =
      asNonEmptyString(body?.text) ?? `Email de test envoyé depuis Miyukini (${startedAt}).`;

    if (!to) return json({ error: "invalid_to" }, 400);

    const { data: cfg, error: cfgError } = await serviceClient
      .from("email_provider_configs")
      .select(
        "id,provider,is_active,smtp_host,smtp_port,smtp_username,smtp_password,smtp_secure,from_email,from_name,reply_to",
      )
      .eq("workspace_id", GLOBAL_WORKSPACE_ID)
      .eq("provider", "smtp")
      .eq("is_active", true)
      .maybeSingle();
    if (cfgError) return json({ error: "config_read_failed", details: cfgError.message }, 500);
    if (!cfg) return json({ error: "smtp_not_configured" }, 400);

    const smtpHost = asNonEmptyString(cfg.smtp_host);
    const smtpPort = Number(cfg.smtp_port ?? 0);
    const smtpUser = asNonEmptyString(cfg.smtp_username);
    const smtpPass = asNonEmptyString(cfg.smtp_password);
    const fromEmail = asNonEmptyString(cfg.from_email);
    const fromName = asNonEmptyString(cfg.from_name);
    const replyTo = asNonEmptyString(cfg.reply_to);
    const secure = Boolean(cfg.smtp_secure);

    if (!smtpHost || !fromEmail || !smtpPort) return json({ error: "smtp_config_incomplete" }, 400);

    const from = fromName ? `${fromName} <${fromEmail}>` : fromEmail;

    try {
      const client = new SMTPClient({
        connection: {
          hostname: smtpHost,
          port: smtpPort,
          tls: secure,
          auth: smtpUser && smtpPass ? { username: smtpUser, password: smtpPass } : undefined,
        },
      });

      await client.send({
        from,
        to,
        subject,
        content: text,
        ...(replyTo ? { replyTo } : {}),
      });

      await client.close();

      await serviceClient
        .from("email_provider_configs")
        .update({
          last_test_at: startedAt,
          last_test_status: "ok",
          last_test_error: null,
          updated_by: user.id,
        })
        .eq("id", cfg.id);

      return json({ ok: true, sent_at: startedAt });
    } catch (e) {
      const msg = e instanceof Error ? e.message : String(e);
      await serviceClient
        .from("email_provider_configs")
        .update({
          last_test_at: startedAt,
          last_test_status: "error",
          last_test_error: msg.slice(0, 1000),
          updated_by: user.id,
        })
        .eq("id", cfg.id);
      return json({ error: "smtp_send_failed", details: msg }, 500);
    }
  } catch (e) {
    const msg = e instanceof Error ? e.message : String(e);
    return json({ error: "unexpected", details: msg }, 500);
  }
});

