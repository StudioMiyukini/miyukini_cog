// @ts-nocheck
// Source local (référence) de l’Edge Function Supabase: paypal-test
// Déployée via MCP Supabase.

import "jsr:@supabase/functions-js/edge-runtime.d.ts";

import { createClient } from "https://esm.sh/@supabase/supabase-js@2.49.1";

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

function base64(s: string) {
  return btoa(s);
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

    const { data: cfg, error: cfgError } = await serviceClient
      .from("paypal_provider_configs")
      .select("id,is_active,environment,client_id,client_secret")
      .eq("workspace_id", GLOBAL_WORKSPACE_ID)
      .maybeSingle();
    if (cfgError) return json({ error: "config_read_failed", details: cfgError.message }, 500);
    if (!cfg) return json({ error: "paypal_not_configured" }, 400);

    const env = cfg.environment === "live" ? "live" : "sandbox";
    const clientId = asNonEmptyString(cfg.client_id);
    const clientSecret = asNonEmptyString(cfg.client_secret);

    if (!clientId || !clientSecret) return json({ error: "paypal_config_incomplete" }, 400);

    const baseUrl = env === "live" ? "https://api-m.paypal.com" : "https://api-m.sandbox.paypal.com";

    try {
      const tokenUrl = `${baseUrl}/v1/oauth2/token`;
      const auth = base64(`${clientId}:${clientSecret}`);

      const res = await fetch(tokenUrl, {
        method: "POST",
        headers: {
          Authorization: `Basic ${auth}`,
          "Content-Type": "application/x-www-form-urlencoded",
          Accept: "application/json",
        },
        body: new URLSearchParams({ grant_type: "client_credentials" }),
      });

      const text = await res.text();
      let parsed: any = null;
      try {
        parsed = JSON.parse(text);
      } catch {
        parsed = { raw: text };
      }

      if (!res.ok) {
        const msg = `paypal_token_failed:${res.status}:${res.statusText}`;
        await serviceClient
          .from("paypal_provider_configs")
          .update({
            last_test_at: startedAt,
            last_test_status: "error",
            last_test_error: `${msg} ${(parsed?.error_description ?? parsed?.error ?? "")}`.slice(0, 1000),
            updated_by: user.id,
          })
          .eq("id", cfg.id);
        return json({ error: msg, details: parsed }, 500);
      }

      await serviceClient
        .from("paypal_provider_configs")
        .update({
          last_test_at: startedAt,
          last_test_status: "ok",
          last_test_error: null,
          updated_by: user.id,
        })
        .eq("id", cfg.id);

      return json({ ok: true, environment: env, scope: parsed?.scope ?? null });
    } catch (e) {
      const msg = e instanceof Error ? e.message : String(e);
      await serviceClient
        .from("paypal_provider_configs")
        .update({
          last_test_at: startedAt,
          last_test_status: "error",
          last_test_error: msg.slice(0, 1000),
          updated_by: user.id,
        })
        .eq("id", cfg.id);
      return json({ error: "paypal_test_failed", details: msg }, 500);
    }
  } catch (e) {
    const msg = e instanceof Error ? e.message : String(e);
    return json({ error: "unexpected", details: msg }, 500);
  }
});

