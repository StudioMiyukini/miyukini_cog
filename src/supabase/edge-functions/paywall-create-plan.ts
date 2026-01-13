// @ts-nocheck
// Edge Function (Deno) - Paywall Create Plan
// Objectif : créer un plan PayPal depuis la configuration SuperAdmin
//
// Responsabilités:
// - Vérifier auth (super_admin uniquement)
// - Charger plan depuis saas_paywall_plans
// - Charger config PayPal provider
// - Obtenir access token PayPal (OAuth2)
// - Créer plan PayPal via API (POST /v1/billing/plans)
// - Mettre à jour plan (paypal_plan_id, status = 'active')
//
// Sécurité:
// - verify_jwt est activé au déploiement (Authorization: Bearer <JWT> requis)

import "jsr:@supabase/functions-js/edge-runtime.d.ts";

import { createClient } from "https://esm.sh/@supabase/supabase-js@2.49.1";

const GLOBAL_WORKSPACE_ID = "00000000-0000-0000-0000-000000000000";

function json(data: unknown, status = 200) {
  return new Response(JSON.stringify(data), {
    status,
    headers: {
      "Content-Type": "application/json",
      Connection: "keep-alive",
      "Access-Control-Allow-Origin": "*",
      "Access-Control-Allow-Methods": "GET, POST, OPTIONS",
      "Access-Control-Allow-Headers": "Content-Type, Authorization",
    },
  });
}

function cors() {
  return new Response(null, {
    status: 204,
    headers: {
      "Access-Control-Allow-Origin": "*",
      "Access-Control-Allow-Methods": "GET, POST, OPTIONS",
      "Access-Control-Allow-Headers": "Content-Type, Authorization",
    },
  });
}

function asNonEmptyString(v: unknown) {
  return typeof v === "string" && v.trim().length ? v.trim() : null;
}

function base64(s: string) {
  return btoa(s);
}

async function getPayPalAccessToken(
  baseUrl: string,
  clientId: string,
  clientSecret: string
): Promise<string> {
  const tokenUrl = `${baseUrl}/v1/oauth2/token`;
  const auth = base64(`${clientId}:${clientSecret}`);

  console.log(`[PayPal] Requesting access token from: ${tokenUrl}`);

  const res = await fetch(tokenUrl, {
    method: "POST",
    headers: {
      Authorization: `Basic ${auth}`,
      "Content-Type": "application/x-www-form-urlencoded",
      Accept: "application/json",
    },
    body: new URLSearchParams({ grant_type: "client_credentials" }),
  });

  if (!res.ok) {
    const text = await res.text();
    console.error(`[PayPal] OAuth failed: ${res.status} ${text}`);
    throw new Error(`PayPal OAuth failed: ${res.status} ${text}`);
  }

  const data = await res.json();
  console.log(`[PayPal] Access token obtained successfully`);
  return data.access_token;
}

Deno.serve(async (req: Request) => {
  const requestId = crypto.randomUUID();
  const startedAt = new Date().toISOString();
  
  console.log(`[${requestId}] ${req.method} ${req.url} - Started at ${startedAt}`);
  
  try {
    // Gérer les requêtes OPTIONS (preflight CORS)
    if (req.method === "OPTIONS") {
      console.log(`[${requestId}] OPTIONS request - returning CORS headers`);
      return cors();
    }
    
    if (req.method !== "POST") {
      console.log(`[${requestId}] Method not allowed: ${req.method}`);
      return json({ error: "method_not_allowed" }, 405);
    }

    const supabaseUrl = Deno.env.get("SUPABASE_URL");
    const anonKey = Deno.env.get("SUPABASE_ANON_KEY");
    const serviceRoleKey = Deno.env.get("SUPABASE_SERVICE_ROLE_KEY");
    if (!supabaseUrl || !anonKey || !serviceRoleKey) {
      console.error(`[${requestId}] Missing environment variables`);
      return json({ error: "missing_env" }, 500);
    }
    
    console.log(`[${requestId}] Environment check OK`);

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
    if (userError || !user) {
      console.error(`[${requestId}] Auth failed:`, userError?.message || "no user");
      return json({ error: "unauthorized" }, 401);
    }
    console.log(`[${requestId}] User authenticated: ${user.id}`);

    const { data: profile, error: profileError } = await serviceClient
      .from("profiles")
      .select("id,role")
      .eq("id", user.id)
      .maybeSingle();
    if (profileError) return json({ error: "profile_lookup_failed" }, 500);
    if (!profile || profile.role !== "super_admin") return json({ error: "forbidden" }, 403);

    const body = await req.json();
    console.log(`[${requestId}] Request body:`, JSON.stringify(body));
    const planId = asNonEmptyString(body.plan_id);
    if (!planId) {
      console.error(`[${requestId}] Missing plan_id`);
      return json({ error: "plan_id_required" }, 400);
    }
    console.log(`[${requestId}] Processing plan_id: ${planId}`);

    // Charger le plan
    const { data: plan, error: planError } = await serviceClient
      .from("saas_paywall_plans")
      .select("*")
      .eq("id", planId)
      .maybeSingle();
    if (planError) {
      console.error(`[${requestId}] Plan load error:`, planError.message);
      return json({ error: "plan_load_failed", details: planError.message }, 500);
    }
    if (!plan) {
      console.error(`[${requestId}] Plan not found: ${planId}`);
      return json({ error: "plan_not_found" }, 404);
    }
    if (plan.paypal_plan_id) {
      console.log(`[${requestId}] Plan already has PayPal ID: ${plan.paypal_plan_id}`);
      return json({ error: "plan_already_created" }, 400);
    }
    console.log(`[${requestId}] Plan loaded: ${plan.name}`);

    // Charger config PayPal
    const { data: cfg, error: cfgError } = await serviceClient
      .from("paypal_provider_configs")
      .select("id,is_active,environment,client_id,client_secret")
      .eq("workspace_id", GLOBAL_WORKSPACE_ID)
      .maybeSingle();
    if (cfgError) return json({ error: "config_read_failed", details: cfgError.message }, 500);
    if (!cfg || !cfg.is_active) return json({ error: "paypal_not_configured" }, 400);

    const env = cfg.environment === "live" ? "live" : "sandbox";
    const clientId = asNonEmptyString(cfg.client_id);
    const clientSecret = asNonEmptyString(cfg.client_secret);
    if (!clientId || !clientSecret) return json({ error: "paypal_config_incomplete" }, 400);

    const baseUrl = env === "live" ? "https://api-m.paypal.com" : "https://api-m.sandbox.paypal.com";

    // Obtenir access token
    console.log(`[${requestId}] Getting PayPal access token...`);
    const accessToken = await getPayPalAccessToken(baseUrl, clientId, clientSecret);
    console.log(`[${requestId}] PayPal access token obtained`);

    // Créer le plan PayPal
    const planUrl = `${baseUrl}/v1/billing/plans`;
    const returnUrl = cfg.return_url || `${supabaseUrl.replace("/rest/v1", "")}/pricing/success`;
    const cancelUrl = cfg.cancel_url || `${supabaseUrl.replace("/rest/v1", "")}/pricing/cancel`;

    const paypalPlanPayload = {
      product_id: null, // À créer séparément si nécessaire
      name: plan.name,
      description: plan.description || plan.name,
      status: "ACTIVE",
      billing_cycles: [
        {
          frequency: {
            interval_unit: plan.billing_cycle === "yearly" ? "YEAR" : "MONTH",
            interval_count: 1,
          },
          tenure_type: "REGULAR",
          sequence: 1,
          total_cycles: 0, // 0 = infini
          pricing_scheme: {
            fixed_price: {
              value: String(plan.price_monthly),
              currency_code: plan.currency,
            },
          },
        },
      ],
      payment_preferences: {
        auto_bill_outstanding: true,
        setup_fee: {
          value: "0",
          currency_code: plan.currency,
        },
        setup_fee_failure_action: "CONTINUE",
        payment_failure_threshold: 3,
      },
      taxes: {
        percentage: "0",
        inclusive: false,
      },
    };

    console.log(`[${requestId}] Creating PayPal plan...`);
    const planRes = await fetch(planUrl, {
      method: "POST",
      headers: {
        Authorization: `Bearer ${accessToken}`,
        "Content-Type": "application/json",
        Accept: "application/json",
      },
      body: JSON.stringify(paypalPlanPayload),
    });

    if (!planRes.ok) {
      const errorText = await planRes.text();
      console.error(`[${requestId}] PayPal plan creation failed:`, errorText);
      return json(
        { error: "paypal_plan_creation_failed", details: errorText },
        planRes.status
      );
    }

    const paypalPlan = await planRes.json();
    const paypalPlanId = paypalPlan.id;
    console.log(`[${requestId}] PayPal plan created: ${paypalPlanId}`);

    // Mettre à jour le plan dans la DB
    console.log(`[${requestId}] Updating plan in database...`);
    const { error: updateError } = await serviceClient
      .from("saas_paywall_plans")
      .update({
        paypal_plan_id: paypalPlanId,
        status: "active",
      })
      .eq("id", planId);

    if (updateError) {
      console.error(`[${requestId}] Plan update failed:`, updateError.message);
      return json({ error: "plan_update_failed", details: updateError.message }, 500);
    }

    console.log(`[${requestId}] Plan updated successfully`);
    const duration = Date.now() - new Date(startedAt).getTime();
    console.log(`[${requestId}] Request completed in ${duration}ms`);

    return json({
      paypal_plan_id: paypalPlanId,
      status: "active",
    });
  } catch (e) {
    const msg = e instanceof Error ? e.message : String(e);
    console.error(`[${requestId}] Unexpected error:`, msg, e);
    return json({ error: "unexpected", details: msg }, 500);
  }
});
