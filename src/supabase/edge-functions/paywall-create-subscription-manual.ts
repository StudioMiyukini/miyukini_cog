// Edge Function: paywall-create-subscription-manual
// Créer un abonnement avec paiement manuel (virement bancaire, etc.)

import { createClient } from "https://esm.sh/@supabase/supabase-js@2.49.1";

const GLOBAL_WORKSPACE_ID = "00000000-0000-0000-0000-000000000000";

function asNonEmptyString(v: unknown): string {
  if (typeof v !== "string" || v.trim().length === 0) {
    throw new Error("Invalid non-empty string");
  }
  return v.trim();
}

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

    const body = await req.json();
    console.log(`[${requestId}] Request body:`, JSON.stringify(body));

    const planId = asNonEmptyString(body.plan_id);
    const paymentMethod = asNonEmptyString(body.payment_method || "bank_transfer");

    // Valider la méthode de paiement
    const validMethods = ["bank_transfer", "cash", "check", "card_remote", "other"];
    if (!validMethods.includes(paymentMethod)) {
      return json({ error: "invalid_payment_method" }, 400);
    }

    console.log(`[${requestId}] Processing plan_id: ${planId}, payment_method: ${paymentMethod}`);

    // Vérifier qu'aucun abonnement actif n'existe
    const { data: existingSub, error: existingError } = await serviceClient
      .from("saas_paywall_subscriptions")
      .select("id,status")
      .eq("user_id", user.id)
      .in("status", ["ACTIVE", "APPROVED", "APPROVAL_PENDING"])
      .maybeSingle();

    if (existingError) {
      return json({ error: "subscription_check_failed", details: existingError.message }, 500);
    }
    if (existingSub) {
      return json({ error: "active_subscription_exists" }, 400);
    }

    // Charger le plan
    const { data: plan, error: planError } = await serviceClient
      .from("saas_paywall_plans")
      .select("*")
      .eq("id", planId)
      .eq("status", "active")
      .maybeSingle();
    if (planError) {
      return json({ error: "plan_load_failed", details: planError.message }, 500);
    }
    if (!plan) {
      return json({ error: "plan_not_found" }, 404);
    }

    console.log(`[${requestId}] Plan loaded: ${plan.name}`);

    // Créer l'abonnement avec statut APPROVAL_PENDING
    const subscriptionId = crypto.randomUUID();
    const { error: subInsertError } = await serviceClient.from("saas_paywall_subscriptions").insert({
      id: subscriptionId,
      user_id: user.id,
      plan_id: plan.id,
      paypal_subscription_id: null, // Pas de PayPal pour paiement manuel
      payment_method: paymentMethod,
      status: "APPROVAL_PENDING",
      start_date: null,
      end_date: null,
      current_period_start: null,
      current_period_end: null,
      next_billing_date: null,
      cancel_at_period_end: false,
      cancelled_at: null,
      suspended_at: null,
      suspension_reason: null,
      metadata: {
        created_via: "manual_payment",
        payment_method: paymentMethod,
      },
    });

    if (subInsertError) {
      console.error(`[${requestId}] Subscription insert error:`, subInsertError);
      return json({ error: "subscription_creation_failed", details: subInsertError.message }, 500);
    }

    console.log(`[${requestId}] Subscription created: ${subscriptionId}`);

    // Calculer la date d'échéance (aujourd'hui + 1 mois pour monthly, + 1 an pour yearly)
    const dueDate = new Date();
    if (plan.billing_cycle === "yearly") {
      dueDate.setFullYear(dueDate.getFullYear() + 1);
    } else {
      dueDate.setMonth(dueDate.getMonth() + 1);
    }

    // Créer le paiement manuel en attente
    const { data: paymentData, error: paymentError } = await serviceClient
      .from("saas_paywall_manual_payments")
      .insert({
        subscription_id: subscriptionId,
        payment_method: paymentMethod,
        amount: plan.price_monthly,
        currency: plan.currency || "EUR",
        reference: null,
        status: "pending",
        due_date: dueDate.toISOString().split("T")[0],
        paid_at: null,
        confirmed_at: null,
        confirmed_by: null,
        rejected_at: null,
        rejected_by: null,
        rejection_reason: null,
        proof_document_ref: null,
        notes: null,
        billing_cycle: 1,
      })
      .select("id")
      .single();

    if (paymentError) {
      console.error(`[${requestId}] Manual payment insert error:`, paymentError);
      // Nettoyer l'abonnement créé
      await serviceClient.from("saas_paywall_subscriptions").delete().eq("id", subscriptionId);
      return json({ error: "payment_creation_failed", details: paymentError.message }, 500);
    }

    console.log(`[${requestId}] Manual payment created: ${paymentData.id}`);

    // Historique
    await serviceClient.from("saas_paywall_subscription_history").insert({
      subscription_id: subscriptionId,
      event_type: "created",
      old_tier: null,
      new_tier: plan.tier,
      old_plan_id: null,
      new_plan_id: plan.id,
      actor_id: user.id,
      metadata: {
        payment_method: paymentMethod,
        manual_payment_id: paymentData.id,
      },
    });

    // Event outbox pour notifications
    await serviceClient.from("commerce_outbox").insert({
      event_type: "paywall.subscription.created_manual",
      dedupe_key: `paywall.subscription.created_manual:${subscriptionId}`,
      payload: {
        subscription_id: subscriptionId,
        user_id: user.id,
        plan_id: plan.id,
        payment_method: paymentMethod,
        manual_payment_id: paymentData.id,
        due_date: dueDate.toISOString(),
      },
    });

    console.log(`[${requestId}] Subscription created successfully with manual payment`);

    return json({
      success: true,
      subscription_id: subscriptionId,
      payment_id: paymentData.id,
      status: "APPROVAL_PENDING",
      payment_method: paymentMethod,
      message: "Votre demande d'abonnement a été créée. Un administrateur confirmera votre paiement.",
      due_date: dueDate.toISOString().split("T")[0],
    });
  } catch (error) {
    console.error(`[${requestId}] Unexpected error:`, error);
    return json(
      {
        error: "internal_error",
        details: error instanceof Error ? error.message : String(error),
      },
      500
    );
  }
});
