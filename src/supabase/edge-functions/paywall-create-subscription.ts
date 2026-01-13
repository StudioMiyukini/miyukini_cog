// @ts-nocheck
// Edge Function (Deno) - Paywall Create Subscription
// Objectif : créer une subscription PayPal pour un utilisateur
//
// Responsabilités:
// - Vérifier auth (user authentifié)
// - Vérifier qu'aucun abonnement actif n'existe pour cet utilisateur
// - Charger plan depuis saas_paywall_plans
// - Vérifier que le plan est active et a un paypal_plan_id
// - Charger config PayPal provider
// - Obtenir access token PayPal
// - Créer subscription PayPal via API (POST /v1/billing/subscriptions)
// - Enregistrer dans saas_paywall_subscriptions (status = APPROVAL_PENDING)
// - Log dans saas_paywall_subscription_history
// - Retourner approve_url pour redirection
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
    if (userError || !user) {
      console.error(`[${requestId}] Auth failed:`, userError?.message || "no user");
      return json({ error: "unauthorized" }, 401);
    }
    console.log(`[${requestId}] User authenticated: ${user.id}`);

    const body = await req.json();
    console.log(`[${requestId}] Request body:`, JSON.stringify(body));
    const planId = asNonEmptyString(body.plan_id);
    if (!planId) {
      console.error(`[${requestId}] Missing plan_id`);
      return json({ error: "plan_id_required" }, 400);
    }
    console.log(`[${requestId}] Processing plan_id: ${planId}`);

    const returnUrl = asNonEmptyString(body.return_url) || `${supabaseUrl.replace("/rest/v1", "")}/pricing/success`;
    const cancelUrl = asNonEmptyString(body.cancel_url) || `${supabaseUrl.replace("/rest/v1", "")}/pricing/cancel`;

    // Vérifier qu'aucun abonnement actif n'existe
    const { data: existingSub, error: existingError } = await serviceClient
      .from("saas_paywall_subscriptions")
      .select("id,status")
      .eq("user_id", user.id)
      .in("status", ["ACTIVE", "APPROVED", "APPROVAL_PENDING"])
      .maybeSingle();

    if (existingError) return json({ error: "subscription_check_failed", details: existingError.message }, 500);
    if (existingSub) return json({ error: "active_subscription_exists" }, 400);

    // Charger le plan
    const { data: plan, error: planError } = await serviceClient
      .from("saas_paywall_plans")
      .select("*")
      .eq("id", planId)
      .eq("status", "active")
      .maybeSingle();
    if (planError) return json({ error: "plan_load_failed", details: planError.message }, 500);
    if (!plan) return json({ error: "plan_not_found" }, 404);
    if (!plan.paypal_plan_id) return json({ error: "plan_not_created_on_paypal" }, 400);

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
    const accessToken = await getPayPalAccessToken(baseUrl, clientId, clientSecret);

    // Charger le profil utilisateur pour obtenir l'email
    const { data: profile, error: profileError } = await serviceClient
      .from("profiles")
      .select("email,first_name,last_name")
      .eq("id", user.id)
      .maybeSingle();
    if (profileError) return json({ error: "profile_load_failed", details: profileError.message }, 500);

    // Créer la subscription PayPal
    const subscriptionUrl = `${baseUrl}/v1/billing/subscriptions`;
    const paypalSubscriptionPayload = {
      plan_id: plan.paypal_plan_id,
      start_time: new Date(Date.now() + 60000).toISOString(), // 1 minute dans le futur
      subscriber: {
        name: {
          given_name: profile?.first_name || "User",
          surname: profile?.last_name || "",
        },
        email_address: profile?.email || user.email || "",
      },
      application_context: {
        brand_name: cfg.brand_name || "Miyukini Framework",
        locale: "fr-FR",
        shipping_preference: "NO_SHIPPING",
        user_action: "SUBSCRIBE_NOW",
        payment_method: {
          payer_selected: "PAYPAL",
          payee_preferred: "IMMEDIATE_PAYMENT_REQUIRED",
        },
        return_url: returnUrl,
        cancel_url: cancelUrl,
      },
    };

    const subscriptionRes = await fetch(subscriptionUrl, {
      method: "POST",
      headers: {
        Authorization: `Bearer ${accessToken}`,
        "Content-Type": "application/json",
        Accept: "application/json",
      },
      body: JSON.stringify(paypalSubscriptionPayload),
    });

    if (!subscriptionRes.ok) {
      const errorText = await subscriptionRes.text();
      return json(
        { error: "paypal_subscription_creation_failed", details: errorText },
        subscriptionRes.status
      );
    }

    const paypalSubscription = await subscriptionRes.json();
    const paypalSubscriptionId = paypalSubscription.id;
    const approveUrl = paypalSubscription.links?.find((link: any) => link.rel === "approve")?.href;

    if (!approveUrl) {
      return json({ error: "approve_url_not_found" }, 500);
    }

    // Enregistrer dans saas_paywall_subscriptions
    const { data: subscription, error: insertError } = await serviceClient
      .from("saas_paywall_subscriptions")
      .insert({
        user_id: user.id,
        plan_id: planId,
        paypal_subscription_id: paypalSubscriptionId,
        status: "APPROVAL_PENDING",
        metadata: { paypal_subscription: paypalSubscription },
      })
      .select()
      .single();

    if (insertError) {
      return json({ error: "subscription_insert_failed", details: insertError.message }, 500);
    }

    // Log dans l'historique
    await serviceClient.from("saas_paywall_subscription_history").insert({
      subscription_id: subscription.id,
      event_type: "created",
      metadata: { paypal_subscription_id: paypalSubscriptionId },
    });

    const duration = Date.now() - new Date(startedAt).getTime();
    console.log(`[${requestId}] Request completed in ${duration}ms`);
    
    return json({
      subscription_id: paypalSubscriptionId,
      approve_url: approveUrl,
      status: "APPROVAL_PENDING",
    });
  } catch (e) {
    const msg = e instanceof Error ? e.message : String(e);
    console.error(`[${requestId}] Unexpected error:`, msg, e);
    return json({ error: "unexpected", details: msg }, 500);
  }
});
