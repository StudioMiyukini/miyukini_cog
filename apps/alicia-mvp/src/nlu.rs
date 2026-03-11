//! Moteur NLU : LLM (via API OpenAI-compatible) ou regex fallback.
//! Boucle agentique avec tool calling.

use std::path::PathBuf;
use std::sync::Arc;

use miyualicia::intent::Intent;
use miyualicia::nlu_fallback::FallbackNluParser;
use tokio::sync::RwLock;

use crate::tools;

/// System prompt par defaut pour les reponses conversationnelles.
pub const DEFAULT_RESPONSE_PROMPT: &str = "Tu es Alicia, une assistante domotique et personnelle francaise. \
Tu reponds de facon tres courte et naturelle (1-2 phrases max) pour confirmer \
l'action ou indiquer ce que tu ne peux pas faire. \
Tu as acces a des outils : fichiers, base de donnees, recherche web. \
Si l'utilisateur pose une question factuelle, utilise search_web. \
Si l'utilisateur demande de noter/memoriser quelque chose, utilise la base de donnees ou les fichiers. \
Sois chaleureuse mais concise. Pas d'emoji.";

/// Nombre max d'iterations de la boucle agentique.
const MAX_TOOL_ROUNDS: usize = 5;

/// Moteur NLU : soit LLM (via API OpenAI-compatible), soit regex fallback.
pub struct NluEngine {
    pub llm_url: Option<String>,
    pub llm_key: Option<String>,
    pub client: reqwest::Client,
    pub fallback: FallbackNluParser,
    pub tool_ctx: Option<Arc<tools::ToolContext>>,
}

impl NluEngine {
    pub fn new(
        llm_url: Option<String>,
        llm_key: Option<String>,
        search_url: Option<String>,
        workspace: Option<PathBuf>,
    ) -> Self {
        let tool_ctx = workspace.map(|ws| {
            let system_prompt = Arc::new(RwLock::new(DEFAULT_RESPONSE_PROMPT.to_string()));
            let ctx = tools::ToolContext::new(ws, system_prompt, search_url);
            if let Err(e) = ctx.init_db() {
                println!("  [Tools] Erreur init DB: {e}");
            } else {
                println!("  [Tools] DB memoire initialisee: {}", ctx.db_path.display());
            }
            Arc::new(ctx)
        });

        Self {
            llm_url,
            llm_key,
            client: reqwest::Client::new(),
            fallback: FallbackNluParser,
            tool_ctx,
        }
    }

    pub fn label(&self) -> &str {
        if self.llm_url.is_some() {
            "LLM"
        } else {
            "Regex"
        }
    }

    pub async fn parse(&self, transcript: &str) -> (Intent, String) {
        if let Some(url) = &self.llm_url {
            match self.parse_via_llm(url, transcript).await {
                Ok((intent, raw)) => (intent, raw),
                Err(e) => {
                    println!("  [NLU] LLM erreur ({e}), fallback regex");
                    let intent = self.fallback.parse(transcript);
                    let raw = format!("{intent:?}");
                    (intent, raw)
                }
            }
        } else {
            let intent = self.fallback.parse(transcript);
            let raw = format!("{intent:?}");
            (intent, raw)
        }
    }

    async fn parse_via_llm(
        &self,
        base_url: &str,
        transcript: &str,
    ) -> anyhow::Result<(Intent, String)> {
        let url = format!("{base_url}/v1/chat/completions");

        let system_prompt = r#"Tu es le module NLU d'Alicia, une assistante domotique. Tu recois une commande vocale transcrite et tu dois extraire l'intention au format JSON strict.

Pieces disponibles : salon, chambre-parentale, chambre-theresa, chambre-eleanore
Types de devices : light, shutter, thermostat, outlet, lock, sensor
Actions possibles : on, off, open, close, lock, unlock, set_brightness, set_temperature, set_position
Routines connues : bonne nuit, je pars, mode cinema, matin
Applications lancables : chrome, firefox, notepad, explorer, spotify, discord, steam, vscode

Reponds UNIQUEMENT avec un JSON valide, sans texte avant/apres. Format :
{"intent": "<type>", "slots": {<slots>}}

Types d'intent :
- control_device : slots = device_type, room_id (optionnel), action, value (optionnel)
- activate_routine : slots = routine_name
- launch_app : slots = app_name
- identify_face : slots = {} (quand l'utilisateur demande "qui est la", "reconnais-moi", "qui suis-je")
- register_face : slots = name (quand l'utilisateur veut enregistrer un visage, ex: "enregistre mon visage sous le nom Theresa")
- query_state : slots = target, property (optionnel)
- query_weather : slots = location (optionnel), horizon (optionnel)
- help : slots = topic (optionnel)
- unknown : slots = {}"#;

        let body = serde_json::json!({
            "model": "qwen/qwen3-vl-4b",
            "messages": [
                { "role": "system", "content": system_prompt },
                { "role": "user", "content": transcript }
            ],
            "temperature": 0.1,
            "max_tokens": 200
        });

        let mut req = self
            .client
            .post(&url)
            .json(&body)
            .timeout(std::time::Duration::from_secs(15));
        if let Some(key) = &self.llm_key {
            req = req.bearer_auth(key);
        }
        let resp = req.send().await?;

        if !resp.status().is_success() {
            anyhow::bail!("LLM HTTP {}", resp.status());
        }

        let json: serde_json::Value = resp.json().await?;
        let content = json["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("{}");

        let raw = content.to_string();

        let cleaned = extract_json_from_response(content);
        let parsed: serde_json::Value = serde_json::from_str(&cleaned)
            .unwrap_or_else(|_| serde_json::json!({"intent": "unknown", "slots": {}}));

        let intent_type = parsed["intent"].as_str().unwrap_or("unknown");
        let slots = &parsed["slots"];

        let intent = match intent_type {
            "control_device" => Intent::ControlDevice {
                device_type: slots["device_type"]
                    .as_str()
                    .unwrap_or("unknown")
                    .to_string(),
                room_id: slots["room_id"].as_str().map(String::from),
                action: slots["action"].as_str().unwrap_or("on").to_string(),
                value: slots.get("value").and_then(|v| {
                    if v.is_null() {
                        None
                    } else {
                        Some(v.clone())
                    }
                }),
            },
            "activate_routine" => Intent::ActivateRoutine {
                routine_name: slots["routine_name"]
                    .as_str()
                    .unwrap_or("")
                    .to_string(),
            },
            "query_state" => Intent::QueryState {
                target: slots["target"]
                    .as_str()
                    .unwrap_or("home")
                    .to_string(),
                property: slots["property"].as_str().map(String::from),
            },
            "query_weather" => Intent::QueryWeather {
                location: slots["location"].as_str().map(String::from),
                horizon: slots["horizon"].as_str().map(String::from),
            },
            "launch_app" => {
                let app_name = slots["app_name"]
                    .as_str()
                    .unwrap_or("")
                    .to_string();
                Intent::Unknown {
                    transcript: format!("__launch_app:{app_name}"),
                }
            }
            "identify_face" => Intent::Unknown {
                transcript: "__identify_face".to_string(),
            },
            "register_face" => {
                let name = slots["name"].as_str().unwrap_or("").to_string();
                Intent::Unknown {
                    transcript: format!("__register_face:{name}"),
                }
            }
            "help" => Intent::Help {
                topic: slots["topic"].as_str().map(String::from),
            },
            _ => Intent::Unknown {
                transcript: transcript.to_string(),
            },
        };

        Ok((intent, raw))
    }

    /// Boucle agentique : execute les tool calls du LLM puis renvoie les resultats.
    async fn agentic_loop(
        &self,
        base_url: &str,
        mut messages: Vec<serde_json::Value>,
        tools_json: &serde_json::Value,
    ) -> anyhow::Result<String> {
        let url = format!("{base_url}/v1/chat/completions");

        for round in 0..MAX_TOOL_ROUNDS {
            let mut body = serde_json::json!({
                "model": "qwen/qwen3-vl-4b",
                "messages": messages,
                "temperature": 0.7,
                "max_tokens": 300
            });

            if !tools_json.as_array().map(|a| a.is_empty()).unwrap_or(true) {
                body["tools"] = tools_json.clone();
            }

            let mut req = self.client.post(&url).json(&body)
                .timeout(std::time::Duration::from_secs(30));
            if let Some(key) = &self.llm_key {
                req = req.bearer_auth(key);
            }
            let resp = req.send().await?;
            if !resp.status().is_success() {
                anyhow::bail!("LLM HTTP {} (round {})", resp.status(), round);
            }
            let json: serde_json::Value = resp.json().await?;
            let choice = &json["choices"][0];
            let finish = choice["finish_reason"].as_str().unwrap_or("");

            if finish != "tool_calls" {
                let content = choice["message"]["content"]
                    .as_str()
                    .unwrap_or("OK, c'est fait.")
                    .trim()
                    .to_string();
                if round > 0 {
                    println!("  [Agent] Reponse finale apres {round} round(s) d'outils");
                }
                return Ok(content);
            }

            let tool_calls = choice["message"]["tool_calls"]
                .as_array()
                .cloned()
                .unwrap_or_default();

            if tool_calls.is_empty() {
                let content = choice["message"]["content"]
                    .as_str()
                    .unwrap_or("OK.")
                    .trim()
                    .to_string();
                return Ok(content);
            }

            println!("  [Agent] Round {} : {} outil(s) appele(s)", round + 1, tool_calls.len());

            messages.push(serde_json::json!({
                "role": "assistant",
                "tool_calls": tool_calls
            }));

            for tc in &tool_calls {
                let fn_name = tc["function"]["name"].as_str().unwrap_or("");
                let fn_args = tc["function"]["arguments"].as_str().unwrap_or("{}");
                let tc_id = tc["id"].as_str().unwrap_or("call_0");

                println!("  [Agent]   -> {fn_name}({})...", fn_args.chars().take(60).collect::<String>());

                let result = if let Some(ref ctx) = self.tool_ctx {
                    tools::execute_tool(ctx, fn_name, fn_args).await
                } else {
                    "Outils non configures (pas de --workspace)".to_string()
                };

                let preview = result.chars().take(100).collect::<String>();
                println!("  [Agent]   <- {preview}...");

                messages.push(serde_json::json!({
                    "role": "tool",
                    "tool_call_id": tc_id,
                    "content": result
                }));
            }
        }

        anyhow::bail!("Boucle agentique: limite de {MAX_TOOL_ROUNDS} rounds atteinte")
    }

    /// Genere une reponse naturelle via LLM (avec outils), ou utilise le template statique.
    pub async fn generate_response(&self, intent: &Intent, transcript: &str) -> String {
        if let Some(url) = &self.llm_url {
            match self.llm_response(url, intent, transcript).await {
                Ok(response) => response,
                Err(_) => crate::demo::generate_response_text(intent, transcript),
            }
        } else {
            crate::demo::generate_response_text(intent, transcript)
        }
    }

    async fn llm_response(
        &self,
        base_url: &str,
        intent: &Intent,
        transcript: &str,
    ) -> anyhow::Result<String> {
        let intent_json = serde_json::to_string(intent)?;

        let system_prompt = if let Some(ref ctx) = self.tool_ctx {
            ctx.system_prompt.read().await.clone()
        } else {
            DEFAULT_RESPONSE_PROMPT.to_string()
        };

        let user_msg = format!(
            "L'utilisateur a dit : \"{transcript}\"\nIntent detectee : {intent_json}\nReponds en une phrase courte."
        );

        let messages = vec![
            serde_json::json!({"role": "system", "content": system_prompt}),
            serde_json::json!({"role": "user", "content": user_msg}),
        ];

        let tools_json = if let Some(ref ctx) = self.tool_ctx {
            tools::tool_definitions(ctx)
        } else {
            serde_json::json!([])
        };

        self.agentic_loop(base_url, messages, &tools_json).await
    }
}

// ---------------------------------------------------------------------------
// JSON extraction helper
// ---------------------------------------------------------------------------

/// Extrait le JSON d'une reponse LLM qui peut contenir du markdown ou du texte.
pub fn extract_json_from_response(content: &str) -> String {
    let trimmed = content.trim();

    if trimmed.starts_with('{') {
        return trimmed.to_string();
    }

    if let Some(start) = trimmed.find("```json") {
        let after = &trimmed[start + 7..];
        if let Some(end) = after.find("```") {
            return after[..end].trim().to_string();
        }
    }

    if let Some(start) = trimmed.find("```") {
        let after = &trimmed[start + 3..];
        if let Some(end) = after.find("```") {
            let block = after[..end].trim();
            if block.starts_with('{') {
                return block.to_string();
            }
        }
    }

    if let Some(start) = trimmed.find('{') {
        if let Some(end) = trimmed.rfind('}') {
            if end > start {
                return trimmed[start..=end].to_string();
            }
        }
    }

    trimmed.to_string()
}
