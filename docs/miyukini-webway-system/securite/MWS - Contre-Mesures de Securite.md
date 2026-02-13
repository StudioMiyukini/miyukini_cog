# MWS — Contre-Mesures de Sécurité (référentiel normatif)

## Contexte

Ce document est le **référentiel normatif** des contre-mesures de sécurité du MWS. Il recense les mesures issues de l'[audit de sécurité](./MWS%20-%20Audit%20de%20Securite%20Complet.md) et indique où chacune est spécifiée ou implémentée dans la documentation MWS.

**Référence fondatrice :** [MWS - Document Fondateur](../MWS%20-%20Document%20Fondateur.md)  
**Source :** [MWS - Audit de Sécurité Complet](./MWS%20-%20Audit%20de%20Securite%20Complet.md)

---

## 1. Vue d'ensemble

| Priorité | Nombre | Documents dédiés |
|----------|--------|-------------------|
| Critique | 2 | Haute Disponibilité Origin, Procédure Failover, Protection DDoS |
| Élevée | 4 | Chiffrement et TLS, Protocole Relay, Passeport/Permis, Registre de Services |
| Moyenne | 5 | Chiffrement et TLS, Quarantaine, Protocole, Déploiement |
| Faible | 5 | Lobbys, Déploiement, Ops |

---

## 2. Contre-mesures par identifiant (R-001 à R-015)

### Priorité critique

| ID | Contre-mesure | Document(s) MWS | Section / contenu |
|----|---------------|-----------------|-------------------|
| **R-001** | Haute disponibilité Origin | [MWS - Haute Disponibilité Origin](./MWS%20-%20Haute%20Disponibilite%20Origin.md) | Architecture actif-passif, RTO/RPO, réplication |
| **R-001** | Procédure de failover | [MWS - Procédure de Failover](./MWS%20-%20Procedure%20de%20Failover.md) | Déclenchement, actif-passif, relay promu |
| **R-002** | Protection DDoS Origin | [MWS - Protection DDoS](./MWS%20-%20Protection%20DDoS.md) | Rate limiting, PoW, anti-DDoS frontal |

### Priorité élevée

| ID | Contre-mesure | Document(s) MWS | Section / contenu |
|----|---------------|----------------|-------------------|
| **R-003** | Signature des paquets DATA (MAC) | [MWS - Chiffrement et TLS](./MWS%20-%20Chiffrement%20et%20TLS.md), [Protocole Relay](../protocole/MWS%20-%20Protocole%20Relay.md) | § MAC sur canal données ; format DATA + mac 32 octets |
| **R-004** | Protection Eclipse (liste trackers signée) | [MWS - Passeport et Visa](../verification/MWS%20-%20Passeport%20et%20Visa.md), [Protocole Relay](../protocole/MWS%20-%20Protocole%20Relay.md) | tracker_signature dans REGISTER_OK ; vérification côté COG |
| **R-004bis** | Adresse Origin non falsifiable | [MWS - Manifeste Origin et Adresse Canonique](./MWS%20-%20Manifeste%20Origin%20et%20Adresse%20Canonique.md) | Manifeste Origin signé ; clé racine dans le client ; certificate pinning |
| **R-005** | Signature des binaires (supply chain) | [MWS - Registre de Services et Isolation](./MWS%20-%20Registre%20de%20Services%20et%20Isolation.md) | Champs signature, signing_key ; vérification avant installation |
| **R-007** | Certificate pinning Origin | [MWS - Chiffrement et TLS](./MWS%20-%20Chiffrement%20et%20TLS.md), [Contre-Mesures Prioritaires](./MWS%20-%20Contre-Mesures%20Prioritaires.md) | Exigence certificate pinning pour clients Origin |

### Priorité moyenne

| ID | Contre-mesure | Document(s) MWS | Section / contenu |
|----|---------------|----------------|-------------------|
| **R-006** | Fenêtre timestamp ±10 s | [MWS - Chiffrement et TLS](./MWS%20-%20Chiffrement%20et%20TLS.md) | Replay protection : fenêtre ±10 s ; NTP recommandé |
| **R-007** | Rotation automatique des tokens | [MWS - Chiffrement et TLS](./MWS%20-%20Chiffrement%20et%20TLS.md) | Rotation 7 jours ; révocation ; notification |
| **R-008** | Durée max exemption temps réel | [MWS - Chiffrement et TLS](./MWS%20-%20Chiffrement%20et%20TLS.md) | Exemption temps réel : durée max 4 h ; renouvellement |
| **R-009** | Révocation de Permis en temps réel | [MWS - Quarantaine et Blacklist](./MWS%20-%20Quarantaine%20et%20Blacklist.md) | PERMIT_REVOKE ; propagation ; cache de révocation |
| **R-010** | Validation schéma JSON | [MWS - Protocole Relay](../protocole/MWS%20-%20Protocole%20Relay.md) | Schémas JSON pour manifest et payloads ; profondeur max 5 |

### Priorité faible

| ID | Contre-mesure | Document(s) MWS | Section / contenu |
|----|---------------|----------------|-------------------|
| **R-011** | Limite essais Lobby | [MWS - Lobbys, Favoris et Amis](../lobbys/MWS%20-%20Lobbys%20Favoris%20et%20Amis.md) | 3 essais + délai exponentiel ; recommandations mot de passe |
| **R-012** | Badge Lobby vérifié | [MWS - Lobbys, Favoris et Amis](../lobbys/MWS%20-%20Lobbys%20Favoris%20et%20Amis.md) | Lobbys vérifiés ; affichage cog_id hôte |
| **R-013** | DNSSEC | [MWS - Guide de Déploiement](../deploiement/MWS%20-%20Guide%20de%20Deploiement.md), [Protection DDoS](./MWS%20-%20Protection%20DDoS.md) | Recommandation DNSSEC sur domaines MWS |
| **R-014** | Certificate pinning | [MWS - Chiffrement et TLS](./MWS%20-%20Chiffrement%20et%20TLS.md) | Exigence pour clients se connectant à Origin |
| **R-015** | Fuzzing | [MWS - Audit de Sécurité Complet](./MWS%20-%20Audit%20de%20Securite%20Complet.md), [Contre-Mesures Prioritaires](./MWS%20-%20Contre-Mesures%20Prioritaires.md) | Campagne de fuzzing sur parser binaire ; plan de remédiation |

---

## 3. Contre-mesures par document MWS

| Document | Contre-mesures intégrées |
|----------|---------------------------|
| [MWS - Haute Disponibilité Origin](./MWS%20-%20Haute%20Disponibilite%20Origin.md) | R-001 |
| [MWS - Procédure de Failover](./MWS%20-%20Procedure%20de%20Failover.md) | R-001 |
| [MWS - Protection DDoS](./MWS%20-%20Protection%20DDoS.md) | R-002 |
| [MWS - Chiffrement et TLS](./MWS%20-%20Chiffrement%20et%20TLS.md) | R-003, R-006, R-007, R-008, R-014 |
| [MWS - Registre de Services et Isolation](./MWS%20-%20Registre%20de%20Services%20et%20Isolation.md) | R-005 |
| [MWS - Quarantaine et Blacklist](./MWS%20-%20Quarantaine%20et%20Blacklist.md) | R-009 |
| [MWS - Passeport et Visa](../verification/MWS%20-%20Passeport%20et%20Visa.md) | R-004 (Permis + trackers signés) |
| [MWS - Manifeste Origin et Adresse Canonique](./MWS%20-%20Manifeste%20Origin%20et%20Adresse%20Canonique.md) | Adresse Origin non falsifiable (manifeste signé) |
| [MWS - Protocole Relay](../protocole/MWS%20-%20Protocole%20Relay.md) | R-003 (MAC DATA), R-004 (REGISTER_OK), R-010 (JSON) |
| [MWS - Guide de Déploiement](../deploiement/MWS%20-%20Guide%20de%20Deploiement.md) | R-002 (rate limiting), R-006 (NTP), R-013 |
| [MWS - Lobbys, Favoris et Amis](../lobbys/MWS%20-%20Lobbys%20Favoris%20et%20Amis.md) | R-011, R-012 |

---

## 4. Plan de remédiation (rappel)

Le plan détaillé (phases 1 à 3, jalons, responsabilités) figure dans l'[Audit de Sécurité Complet](./MWS%20-%20Audit%20de%20Securite%20Complet.md) — section 8. Les implémentations techniques détaillées sont dans [MWS - Contre-Mesures Prioritaires](./MWS%20-%20Contre-Mesures%20Prioritaires.md).

---

## Références

- [MWS - Audit de Sécurité Complet](./MWS%20-%20Audit%20de%20Securite%20Complet.md)
- [MWS - Contre-Mesures Prioritaires](./MWS%20-%20Contre-Mesures%20Prioritaires.md)
- [MWS - Document Fondateur](../MWS%20-%20Document%20Fondateur.md)

---

**Version :** 1.0  
**Classification :** Documentation MWS — Sécurité
