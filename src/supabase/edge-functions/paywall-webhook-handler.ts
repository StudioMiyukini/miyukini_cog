// @ts-nocheck
// Edge Function (Deno) - Paywall Webhook Handler
// Objectif : traiter les webhooks PayPal spécifiques au paywall
//
// Responsabilités:
// - Vérifier signature webhook (via PayPal API)
// - Parser l'événement
// - Enqueue dans payment_outbox avec dedupe_key unique
// - Retourner 200 OK immédiatement
// - Worker asynchrone traite l'événement
//
// Événements traités:
// - BILLING.SUBSCRIPTION.ACTIVATED → mettre à jour subscription (status = ACTIVE), mettre à jour user_tier, émettre paywall.subscription.activated
// - BILLING.SUBSCRIPTION.CANCELLED → mettre à jour subscription (status = CANCELLED), downgrade user_tier, émettre paywall.subscription.cancelled
// - BILLING.SUBSCRIPTION.PAYMENT.COMPLETED → enregistrer transaction, mettre à jour next_billing_date, émettre paywall.subscription.payment.completed
// - BILLING.SUBSCRIPTION.PAYMENT.FAILED → enregistrer transaction (status = DECLINED), suspendre subscription, émettre paywall.subscription.payment.failed
// - BILLING.SUBSCRIPTION.SUSPENDED → mettre à jour subscription (status = SUSPENDED), émettre paywall.subscription.suspended
// - BILLING.SUBSCRIPTION.UPDATED → mettre à jour subscription, émettre paywall.subscription.updated
//
// Sécurité:
// - verify_jwt peut être désactivé pour les webhooks PayPal (vérification signature à la place)

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

async function getPayPalAccessToken(
  baseUrl: string,
  clientId: string,
  clientSecret: string
): Promise<string> {
  const tokenUrl = `${baseUrl}/v1/oauth2/token`;
  const auth = btoa(`${clientId}:${clientSecret}`);

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
    throw new Error(`PayPal OAuth failed: ${res.status} ${text}`);
  }

  const data = await res.json();
  return data.access_token;
}

async function verifyWebhookSignature(
  baseUrl: string,
  accessToken: string,
  webhookId: string,
  headers: Headers,
  body: string
): Promise<boolean> {
  const verifyUrl = `${baseUrl}/v1/notifications/verify-webhook-signature`;
  
  const authAlgo = headers.get("PAYPAL-AUTH-ALGO");
  const certUrl = headers.get("PAYPAL-CERT-URL");
  const transmissionId = headers.get("PAYPAL-TRANSMISSION-ID");
  const transmissionSig = headers.get("PAYPAL-TRANSMISSION-SIG");
  const transmissionTime = headers.get("PAYPAL-TRANSMISSION-TIME");

  if (!authAlgo || !certUrl || !transmissionId || !transmissionSig || !transmissionTime) {
    console.error(`[PayPal] Missing webhook signature headers`);
    return false;
  }
  
  console.log(`[PayPal] Verifying webhook signature with transmission_id: ${transmissionId}`);

  const verifyPayload = {
    auth_algo: authAlgo,
    cert_url: certUrl,
    transmission_id: transmissionId,
    transmission_sig: transmissionSig,
    transmission_time: transmissionTime,
    webhook_id: webhookId,
    webhook_event: JSON.parse(body),
  };

  const res = await fetch(verifyUrl, {
    method: "POST",
    headers: {
      Authorization: `Bearer ${accessToken}`,
      "Content-Type": "application/json",
      Accept: "application/json",
    },
    body: JSON.stringify(verifyPayload),
  });

    if (!res.ok) {
      const errorText = await res.text();
      console.error(`[PayPal] Signature verification failed: ${res.status} ${errorText}`);
      return false;
    }
    const result = await res.json();
    const isValid = result.verification_status === "SUCCESS";
    console.log(`[PayPal] Signature verification result: ${result.verification_status}`);
    return isValid;
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
    const serviceRoleKey = Deno.env.get("SUPABASE_SERVICE_ROLE_KEY");
    if (!supabaseUrl || !serviceRoleKey) {
      return json({ error: "missing_env" }, 500);
    }

    const serviceClient = createClient(supabaseUrl, serviceRoleKey, {
      auth: { persistSession: false, autoRefreshToken: false },
    });

    // Charger config PayPal
    const { data: cfg, error: cfgError } = await serviceClient
      .from("paypal_provider_configs")
      .select("id,is_active,environment,client_id,client_secret,webhook_id")
      .eq("workspace_id", GLOBAL_WORKSPACE_ID)
      .maybeSingle();
    if (cfgError || !cfg || !cfg.is_active) {
      return json({ error: "paypal_not_configured" }, 400);
    }

    const env = cfg.environment === "live" ? "live" : "sandbox";
    const baseUrl = env === "live" ? "https://api-m.paypal.com" : "https://api-m.sandbox.paypal.com";

    const body = await req.text();
    console.log(`[${requestId}] Webhook body received (${body.length} bytes)`);
    const event = JSON.parse(body);
    const eventType = event.event_type;
    console.log(`[${requestId}] Event type: ${eventType}`);

    // Vérifier signature webhook (si webhook_id configuré)
    if (cfg.webhook_id) {
      console.log(`[${requestId}] Verifying webhook signature with webhook_id: ${cfg.webhook_id}`);
      const accessToken = await getPayPalAccessToken(
        baseUrl,
        cfg.client_id,
        cfg.client_secret
      );
      const isValid = await verifyWebhookSignature(
        baseUrl,
        accessToken,
        cfg.webhook_id,
        req.headers,
        body
      );
      if (!isValid) {
        console.error(`[${requestId}] Webhook signature verification failed`);
        return json({ error: "invalid_signature" }, 401);
      }
      console.log(`[${requestId}] Webhook signature verified successfully`);
    } else {
      console.log(`[${requestId}] Webhook signature verification skipped (no webhook_id configured)`);
    }

    // Créer dedupe_key unique
    const dedupeKey = `paywall_webhook:${event.id || event.resource?.id || Date.now()}`;
    console.log(`[${requestId}] Dedupe key: ${dedupeKey}`);

    // Enqueue dans payment_outbox
    console.log(`[${requestId}] Enqueueing event in payment_outbox...`);
    const { error: outboxError } = await serviceClient.from("payment_outbox").insert({
      provider: "paypal",
      event_type: `paywall.${eventType.toLowerCase()}`,
      dedupe_key: dedupeKey,
      module_id: "saas-paywall",
      payload: event,
      status: "pending",
    });

    if (outboxError) {
      // Si doublon (dedupe_key existe), c'est OK (idempotence)
      if (outboxError.code === "23505") {
        console.log(`[${requestId}] Duplicate event detected (idempotence)`);
        return json({ ok: true, message: "duplicate_event" });
      }
      console.error(`[${requestId}] Outbox insert failed:`, outboxError.message);
      return json({ error: "outbox_insert_failed", details: outboxError.message }, 500);
    }
    console.log(`[${requestId}] Event enqueued successfully`);

    // Retourner 200 OK immédiatement (traitement asynchrone)
    const duration = Date.now() - new Date(startedAt).getTime();
    console.log(`[${requestId}] Webhook processed in ${duration}ms`);
    return json({ ok: true, event_type: eventType });
  } catch (e) {
    const msg = e instanceof Error ? e.message : String(e);
    console.error(`[${requestId}] Unexpected error:`, msg, e);
    return json({ error: "unexpected", details: msg }, 500);
  }
});
