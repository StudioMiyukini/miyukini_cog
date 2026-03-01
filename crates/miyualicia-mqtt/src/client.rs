//! Client MQTT haut niveau pour Alicia.
//!
//! Encapsule `rumqttc::AsyncClient` et son event loop.
//! Reconnexion automatique transparente en arriere-plan.

// @id: toolkit.alicia.mqtt.client
// @role: mqtt_async_client
// @layer: 6
// @human: Client MQTT async pour Alicia ; wrapping rumqttc, reconnexion, broadcast.
// @do: provide_mqtt_client_with_reconnection

use std::sync::Arc;
use std::time::Duration;

use tokio::sync::{broadcast, Mutex};
use tracing;

use crate::config::MqttConfig;
use crate::errors::MqttError;
use crate::message::{MqttIncoming, MqttMessage, QosLevel};

/// Etat de la connexion MQTT.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionStatus {
    /// Non connecte au broker.
    Disconnected,
    /// Connexion en cours.
    Connecting,
    /// Connecte au broker.
    Connected,
    /// Reconnexion en cours apres deconnexion.
    Reconnecting,
}

/// Client MQTT haut niveau pour Alicia.
///
/// `MqttClient` est `Clone` (Arc interne). Plusieurs composants peuvent
/// partager le meme client. Chaque clone partage la meme connexion.
#[derive(Debug, Clone)]
pub struct MqttClient {
    inner: Arc<MqttClientInner>,
}

#[derive(Debug)]
struct MqttClientInner {
    config: MqttConfig,
    client: Mutex<Option<rumqttc::AsyncClient>>,
    status: Mutex<ConnectionStatus>,
    /// Canal broadcast pour les messages entrants.
    tx: broadcast::Sender<MqttIncoming>,
}

impl MqttClient {
    /// Cree un nouveau client MQTT a partir de la configuration.
    /// Ne se connecte pas encore au broker.
    pub fn new(config: MqttConfig) -> Self {
        let (tx, _) = broadcast::channel(config.channel_capacity);
        Self {
            inner: Arc::new(MqttClientInner {
                config,
                client: Mutex::new(None),
                status: Mutex::new(ConnectionStatus::Disconnected),
                tx,
            }),
        }
    }

    /// Demarre la connexion au broker et lance la boucle de traitement.
    ///
    /// Cette methode est non-bloquante : elle lance un `tokio::spawn`.
    /// La connexion effective est asynchrone.
    pub async fn connect(&self) -> Result<(), MqttError> {
        let mut status = self.inner.status.lock().await;
        if *status == ConnectionStatus::Connected {
            return Ok(());
        }
        *status = ConnectionStatus::Connecting;
        drop(status);

        let cfg = &self.inner.config;
        let mut mqtt_options = rumqttc::MqttOptions::new(
            &cfg.client_id,
            &cfg.broker_host,
            cfg.broker_port,
        );
        mqtt_options.set_keep_alive(Duration::from_secs(cfg.keepalive_secs));

        let (client, mut eventloop) =
            rumqttc::AsyncClient::new(mqtt_options, cfg.channel_capacity);

        *self.inner.client.lock().await = Some(client);

        let inner = Arc::clone(&self.inner);
        tokio::spawn(async move {
            loop {
                match eventloop.poll().await {
                    Ok(rumqttc::Event::Incoming(rumqttc::Packet::ConnAck(_))) => {
                        *inner.status.lock().await = ConnectionStatus::Connected;
                        tracing::info!("MQTT connecte a {}:{}", inner.config.broker_host, inner.config.broker_port);
                    }
                    Ok(rumqttc::Event::Incoming(rumqttc::Packet::Publish(publish))) => {
                        let qos = match publish.qos {
                            rumqttc::QoS::AtMostOnce => QosLevel::AtMostOnce,
                            rumqttc::QoS::AtLeastOnce | rumqttc::QoS::ExactlyOnce => {
                                QosLevel::AtLeastOnce
                            }
                        };
                        let incoming = MqttIncoming {
                            topic: publish.topic.clone(),
                            payload: publish.payload.clone(),
                            qos,
                        };
                        // Ignorer l'erreur si personne n'ecoute
                        let _ = inner.tx.send(incoming);
                    }
                    Ok(_) => {}
                    Err(e) => {
                        tracing::warn!("Erreur event loop MQTT : {e}");
                        *inner.status.lock().await = ConnectionStatus::Reconnecting;
                        tokio::time::sleep(Duration::from_secs(
                            inner.config.reconnect_delay_secs,
                        ))
                        .await;
                    }
                }
            }
        });

        Ok(())
    }

    /// Deconnecte proprement le client.
    pub async fn disconnect(&self) -> Result<(), MqttError> {
        let mut guard = self.inner.client.lock().await;
        if let Some(client) = guard.take() {
            client
                .disconnect()
                .await
                .map_err(|e| MqttError::ConnectionFailed {
                    host: self.inner.config.broker_host.clone(),
                    port: self.inner.config.broker_port,
                    source: e,
                })?;
        }
        *self.inner.status.lock().await = ConnectionStatus::Disconnected;
        Ok(())
    }

    /// S'abonne a un topic MQTT avec le QoS specifie.
    pub async fn subscribe(&self, topic: &str, qos: QosLevel) -> Result<(), MqttError> {
        let guard = self.inner.client.lock().await;
        let client = guard.as_ref().ok_or(MqttError::NotConnected)?;
        client
            .subscribe(topic, qos.into())
            .await
            .map_err(|e| MqttError::SubscribeFailed {
                topic: topic.to_string(),
                source: e,
            })
    }

    /// Se desabonne d'un topic.
    pub async fn unsubscribe(&self, topic: &str) -> Result<(), MqttError> {
        let guard = self.inner.client.lock().await;
        let client = guard.as_ref().ok_or(MqttError::NotConnected)?;
        client
            .unsubscribe(topic)
            .await
            .map_err(|e| MqttError::SubscribeFailed {
                topic: topic.to_string(),
                source: e,
            })
    }

    /// Publie un message vers le broker.
    ///
    /// Retourne `MqttError::NotConnected` si le broker est inaccessible.
    pub async fn publish(&self, message: MqttMessage) -> Result<(), MqttError> {
        let guard = self.inner.client.lock().await;
        let client = guard.as_ref().ok_or(MqttError::NotConnected)?;
        client
            .publish(
                &message.topic,
                message.qos.into(),
                message.retain,
                message.payload,
            )
            .await
            .map_err(|e| MqttError::PublishFailed {
                topic: message.topic,
                source: e,
            })
    }

    /// Publie un payload JSON vers un topic.
    /// QoS par defaut : `AtLeastOnce` (garanti pour les commandes).
    pub async fn publish_json(
        &self,
        topic: &str,
        payload: &serde_json::Value,
    ) -> Result<(), MqttError> {
        let msg = MqttMessage::json(topic, payload, QosLevel::AtLeastOnce)?;
        self.publish(msg).await
    }

    /// Retourne un `broadcast::Receiver` pour recevoir les messages entrants.
    pub fn subscribe_incoming(&self) -> broadcast::Receiver<MqttIncoming> {
        self.inner.tx.subscribe()
    }

    /// Retourne l'etat courant de la connexion.
    pub async fn status(&self) -> ConnectionStatus {
        *self.inner.status.lock().await
    }

    /// Retourne `true` si le client est actuellement connecte au broker.
    pub async fn is_connected(&self) -> bool {
        *self.inner.status.lock().await == ConnectionStatus::Connected
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_new_disconnected() {
        let config = MqttConfig::default();
        let client = MqttClient::new(config);
        // On ne peut pas tester async ici mais on verifie la construction
        assert!(Arc::strong_count(&client.inner) == 1);
    }

    #[tokio::test]
    async fn test_client_initial_status_disconnected() {
        let config = MqttConfig::default();
        let client = MqttClient::new(config);
        assert_eq!(client.status().await, ConnectionStatus::Disconnected);
        assert!(!client.is_connected().await);
    }

    #[tokio::test]
    async fn test_client_publish_without_connect_fails() {
        let config = MqttConfig::default();
        let client = MqttClient::new(config);
        let msg = MqttMessage {
            topic: "test".to_string(),
            payload: vec![],
            qos: QosLevel::AtMostOnce,
            retain: false,
        };
        let err = client.publish(msg).await.unwrap_err();
        assert!(matches!(err, MqttError::NotConnected));
    }

    #[tokio::test]
    async fn test_client_subscribe_without_connect_fails() {
        let config = MqttConfig::default();
        let client = MqttClient::new(config);
        let err = client
            .subscribe("test/topic", QosLevel::AtMostOnce)
            .await
            .unwrap_err();
        assert!(matches!(err, MqttError::NotConnected));
    }

    #[test]
    fn test_client_clone_shares_inner() {
        let config = MqttConfig::default();
        let client1 = MqttClient::new(config);
        let client2 = client1.clone();
        assert!(Arc::strong_count(&client1.inner) == 2);
        assert!(Arc::ptr_eq(&client1.inner, &client2.inner));
    }
}
