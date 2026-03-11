//! Client MCP (Model Context Protocol) minimal pour Alicia.
//!
//! Connecte a des serveurs MCP via stdio, decouvre leurs outils,
//! et les rend disponibles dans la boucle agentique.

use std::collections::HashMap;
use std::path::Path;

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};
use tokio::process::{Child, ChildStdin, ChildStdout};

/// Definition d'un outil MCP.
#[derive(Debug, Clone)]
pub struct McpToolDef {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
}

/// Client MCP connecte a un serveur via stdio.
pub struct McpClient {
    pub name: String,
    child: Child,
    stdin: BufWriter<ChildStdin>,
    stdout: BufReader<ChildStdout>,
    pub tools: Vec<McpToolDef>,
    next_id: u64,
}

/// Configuration d'un serveur MCP depuis mcp.json.
#[derive(Debug, serde::Deserialize)]
pub struct McpServerConfig {
    pub name: String,
    pub command: String,
    #[serde(default)]
    pub args: Vec<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct McpConfig {
    pub servers: Vec<McpServerConfig>,
}

/// Gestionnaire de tous les clients MCP.
pub struct McpManager {
    pub clients: HashMap<String, McpClient>,
}

impl McpManager {
    /// Charge la config depuis `{workspace}/mcp.json` et connecte les serveurs.
    pub async fn from_config(config_path: &Path) -> anyhow::Result<Self> {
        let content = tokio::fs::read_to_string(config_path).await?;
        let config: McpConfig = serde_json::from_str(&content)?;

        let mut clients = HashMap::new();

        for server in config.servers {
            println!("  [MCP] Connexion a '{}' ({} {})...",
                server.name, server.command, server.args.join(" "));

            match McpClient::connect(&server).await {
                Ok(client) => {
                    println!("  [MCP] '{}' connecte, {} outil(s)", client.name, client.tools.len());
                    for tool in &client.tools {
                        println!("    - {}: {}", tool.name, tool.description.chars().take(60).collect::<String>());
                    }
                    clients.insert(client.name.clone(), client);
                }
                Err(e) => {
                    println!("  [MCP] Erreur connexion '{}': {e}", server.name);
                }
            }
        }

        Ok(Self { clients })
    }

    /// Retourne toutes les definitions d'outils MCP au format OpenAI function calling.
    /// Les noms sont prefixes : `mcp_{server}_{tool}`.
    pub fn all_tool_definitions(&self) -> Vec<serde_json::Value> {
        let mut defs = Vec::new();

        for (server_name, client) in &self.clients {
            for tool in &client.tools {
                let prefixed_name = format!("mcp_{}_{}", server_name, tool.name);
                defs.push(serde_json::json!({
                    "type": "function",
                    "function": {
                        "name": prefixed_name,
                        "description": tool.description,
                        "parameters": tool.input_schema
                    }
                }));
            }
        }

        defs
    }

    /// Appelle un outil MCP. Le nom doit etre prefixe : `mcp_{server}_{tool}`.
    pub async fn call_tool(
        &mut self,
        prefixed_name: &str,
        args: serde_json::Value,
    ) -> anyhow::Result<String> {
        // Parser le nom : mcp_{server}_{tool}
        let without_prefix = prefixed_name
            .strip_prefix("mcp_")
            .ok_or_else(|| anyhow::anyhow!("Nom invalide: {prefixed_name}"))?;

        // Trouver le premier _ apres le server name
        let (server_name, tool_name) = without_prefix
            .split_once('_')
            .ok_or_else(|| anyhow::anyhow!("Format invalide: {prefixed_name}"))?;

        let client = self
            .clients
            .get_mut(server_name)
            .ok_or_else(|| anyhow::anyhow!("Serveur MCP '{server_name}' non connecte"))?;

        client.call_tool(tool_name, args).await
    }

    /// Arrete tous les serveurs MCP.
    pub async fn shutdown(&mut self) {
        for (name, mut client) in self.clients.drain() {
            println!("  [MCP] Arret de '{name}'...");
            let _ = client.child.kill().await;
        }
    }
}

/// S-05: Commandes MCP autorisees (whitelist).
const ALLOWED_MCP_COMMANDS: &[&str] = &[
    "npx", "node", "python", "python3", "uvx", "uv",
];

/// S-05: Metacaracteres interdits dans les arguments MCP.
const MCP_BLOCKED_CHARS: &[char] = &['|', ';', '&', '`'];

impl McpClient {
    /// Connecte a un serveur MCP via stdio.
    async fn connect(config: &McpServerConfig) -> anyhow::Result<Self> {
        // S-05: Valider la commande contre la whitelist
        let cmd_base = std::path::Path::new(&config.command)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("");

        if !ALLOWED_MCP_COMMANDS.iter().any(|c| cmd_base.eq_ignore_ascii_case(c)) {
            anyhow::bail!(
                "Commande MCP '{}' non autorisee. Permises: {}",
                config.command,
                ALLOWED_MCP_COMMANDS.join(", ")
            );
        }

        // S-05: Valider les arguments
        for arg in &config.args {
            if arg.chars().any(|c| MCP_BLOCKED_CHARS.contains(&c)) {
                anyhow::bail!("Argument MCP suspect: '{arg}' (caracteres interdits: |;&`)");
            }
        }

        let mut child = tokio::process::Command::new(&config.command)
            .args(&config.args)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()?;

        let stdin = BufWriter::new(
            child
                .stdin
                .take()
                .ok_or_else(|| anyhow::anyhow!("stdin non disponible"))?,
        );
        let stdout = BufReader::new(
            child
                .stdout
                .take()
                .ok_or_else(|| anyhow::anyhow!("stdout non disponible"))?,
        );

        let mut client = Self {
            name: config.name.clone(),
            child,
            stdin,
            stdout,
            tools: Vec::new(),
            next_id: 1,
        };

        // Phase 1 : Initialize
        let init_resp = client
            .send_request(
                "initialize",
                serde_json::json!({
                    "protocolVersion": "2024-11-05",
                    "capabilities": {},
                    "clientInfo": {
                        "name": "alicia-mvp",
                        "version": "0.1.0"
                    }
                }),
            )
            .await?;

        // Envoyer initialized notification
        client.send_notification("notifications/initialized", serde_json::json!({})).await?;

        // Phase 2 : List tools
        let tools_resp = client
            .send_request("tools/list", serde_json::json!({}))
            .await?;

        if let Some(tools) = tools_resp["tools"].as_array() {
            for tool in tools {
                client.tools.push(McpToolDef {
                    name: tool["name"].as_str().unwrap_or("").to_string(),
                    description: tool["description"].as_str().unwrap_or("").to_string(),
                    input_schema: tool["inputSchema"].clone(),
                });
            }
        }

        let _ = init_resp; // utilise pour debug si besoin
        Ok(client)
    }

    /// Appelle un outil sur ce serveur MCP.
    async fn call_tool(
        &mut self,
        tool_name: &str,
        arguments: serde_json::Value,
    ) -> anyhow::Result<String> {
        let resp = self
            .send_request(
                "tools/call",
                serde_json::json!({
                    "name": tool_name,
                    "arguments": arguments
                }),
            )
            .await?;

        // Extraire le contenu de la reponse
        if let Some(content) = resp["content"].as_array() {
            let mut text = String::new();
            for item in content {
                if let Some(t) = item["text"].as_str() {
                    text.push_str(t);
                    text.push('\n');
                }
            }
            Ok(text.trim().to_string())
        } else {
            Ok(serde_json::to_string_pretty(&resp)?)
        }
    }

    /// Envoie une requete JSON-RPC et attend la reponse.
    async fn send_request(
        &mut self,
        method: &str,
        params: serde_json::Value,
    ) -> anyhow::Result<serde_json::Value> {
        let id = self.next_id;
        self.next_id += 1;

        let request = serde_json::json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": method,
            "params": params
        });

        let msg = serde_json::to_string(&request)?;
        self.stdin.write_all(msg.as_bytes()).await?;
        self.stdin.write_all(b"\n").await?;
        self.stdin.flush().await?;

        // Lire la reponse (une ligne JSON)
        let resp = tokio::time::timeout(
            std::time::Duration::from_secs(15),
            self.read_response(),
        )
        .await
        .map_err(|_| anyhow::anyhow!("MCP timeout (15s)"))??;

        if let Some(error) = resp.get("error") {
            anyhow::bail!("MCP error: {}", serde_json::to_string(error)?);
        }

        Ok(resp["result"].clone())
    }

    /// Envoie une notification JSON-RPC (pas de reponse attendue).
    async fn send_notification(
        &mut self,
        method: &str,
        params: serde_json::Value,
    ) -> anyhow::Result<()> {
        let notification = serde_json::json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params
        });

        let msg = serde_json::to_string(&notification)?;
        self.stdin.write_all(msg.as_bytes()).await?;
        self.stdin.write_all(b"\n").await?;
        self.stdin.flush().await?;
        Ok(())
    }

    /// Lit une reponse JSON-RPC depuis stdout.
    async fn read_response(&mut self) -> anyhow::Result<serde_json::Value> {
        let mut line = String::new();
        loop {
            line.clear();
            let n = self.stdout.read_line(&mut line).await?;
            if n == 0 {
                anyhow::bail!("MCP server closed stdout");
            }
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            // Ignorer les notifications (pas de "id")
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(trimmed) {
                if json.get("id").is_some() {
                    return Ok(json);
                }
                // C'est une notification, on continue a lire
            }
        }
    }
}
