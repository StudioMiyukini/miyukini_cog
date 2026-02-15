# MWS — Origin

## Source de Vérité du Réseau Webway

L’**Origin** est l’acteur MWS qui fait office de **source de vérité** pour le réseau. Un COG de type ORIGIN héberge typiquement un Relay et un Tracker, et sert de point d’authentification et de référence.

## Rôle

| Fonction | Description |
|----------|-------------|
| **Source de vérité** | Identité racine, manifeste, adresse canonique |
| **Relay + Tracker** | Hébergement des services Relay et Tracker |
| **Authentification** | Point d’attestation pour les COGs (optionnel) |

## Manifeste Origin

Le manifeste Origin décrit :
- L’identité et l’adresse canonique de l’Origin
- Les capacités offertes (Relay, Tracker)
- Les versions supportées
- La politique de confiance (LSI, VID, WID)

## Déploiement

- **Oracle Cloud** : Guide d’implémentation Origin
- **Hostinger** : Guide d’implémentation Origin
- **Haute disponibilité** : Procédure et contre-mesures documentées

## Sécurité

- **Protection DDoS** : Mesures documentées
- **Quarantaine et blacklist** : Gestion des COGs malveillants
- **Failover** : Procedure de bascule
- **Contre-mesures** : Liste prioritaire et audit de sécurité

Documentation détaillée : `docs/miyukini-webway-system/`.
