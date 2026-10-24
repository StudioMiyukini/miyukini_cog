//! @id     miyukini_cog_bridge_whitelist
//! @do     connect
//! @role   orchestration
//! @layer  core
//! @human  miyukini-cog-bridge — module « whitelist ».

//! Mode LAN — connexion directe HTTP à un COG Host qui a appairé l'appareil.
//!
//! L'appareil, une fois appairé (clé épinglée), obtient un passeport puis un
//! visa et prouve la possession de sa clé à chaque requête
//! ([`crate::identite`]). Le `device_id` n'est plus un secret au porteur : il
//! ne sert qu'aux journaux.

use crate::identite::ClientAppareil;
use crate::BridgeResult;

/// Établit une connexion directe au COG Host en LAN : passeport, visa, puis
/// `POST /api/bridge/validate` (visa + preuve de possession).
pub async fn connect(host: &str, port: u16, device_id: &str, client: &ClientAppareil) -> BridgeResult<()> {
    tracing::info!("LAN : connexion à {host}:{port} (appareil {device_id})");
    client.ouvrir_session().await?;
    let r = client.valider().await?;
    tracing::info!("LAN : appareil authentifié ({})", r.role);
    Ok(())
}
