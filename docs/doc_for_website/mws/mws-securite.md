# MWS — Sécurité

## Chiffrement et TLS

- **TLS** : Toutes les communications MWS utilisent TLS (1.3 recommandé).
- **Chiffrement bout à bout** : Les tunnels étendus entre COGs chiffrent les données.
- **Signatures** : Manifeste Origin, Permis de circulation et messages critiques sont signés.

## Contre-Mesures

| Mesure | Description |
|--------|-------------|
| **Protection DDoS** | Limitation de débit, blacklist, absorption de pic |
| **Quarantaine** | Isolation des COGs suspects avant décision |
| **Blacklist** | Exclusion des COGs malveillants (Origin, Relays, Trackers) |
| **Failover** | Procédure de bascule pour Origin/Relays |
| **Haute disponibilité Origin** | Redondance et reprise après incident |

## Registre de Services et Isolation

- Les services exposés sur le Webway sont enregistrés et scopés.
- Isolation des contextes : un COG ne peut pas accéder aux données d’un autre sans accord explicite et contrôle BorderGuard.

## Audit de Sécurité

Un audit de sécurité complet du MWS est documenté ; les contre-mesures prioritaires et le manifeste Origin complètent le dispositif.

Documentation détaillée : `docs/miyukini-webway-system/securite/`.
