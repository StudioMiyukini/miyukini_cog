# MWS — Document Fondateur

## Miyukini Webway System

Le **Miyukini Webway System** (MWS) est le système de présence, de découverte et de transport des COGs sur le réseau. Il permet aux environnements de se trouver, de communiquer et de se fédérer de manière **optionnelle** et **sécurisée**.

## Position dans l'Architecture

> Le MWS est un **système complet**, subordonné aux Cores et **consommé** par toutes les strates.

Le MWS n'est **pas** une strate de la pyramide. C'est un système transversal qui :
- Est **uniquement subordonné aux Cores**
- Est **consommé** par Cores, Outils, Opérateurs et Services
- Respecte les Lois d'Autonomie (LOI-2 : optionnel, LOI-6 : fédération)

## Acteurs du Webway

| Acteur | Rôle |
|--------|------|
| **Origin** | Source de vérité ; héberge relay + tracker ; point d’authentification |
| **Relays** | Duplication, vérification, délivrance de Permis de circulation |
| **Trackers** | Découverte des COGs ; pools par version ; contrôle (douaniers) |
| **COG Participant** | COG qui rejoint le réseau (MiyuWebwayParticipant) |
| **COG Tracker** | COG qui fait office de Tracker (MiyuWebwayTracker) |

## Concepts Clés

### Permis de Circulation

Document qui atteste qu’un COG est autorisé à circuler sur le Webway (identité, version, tracker, signature).

### Accord d’Hôte

Contrat entre un COG et un Relay/Origin pour l’hébergement ou la relève de services.

### Tunnel Étendu

Canal sécurisé entre COGs pour échanger des données (chiffrement, intégrité).

## Principes

- **Optionnel** : Un COG peut fonctionner sans jamais rejoindre le Webway (LOI-2).
- **Fédération** : Rejoindre le réseau ne crée pas de dépendance critique (LOI-6).
- **Subordination** : Toute décision MWS est validée par les Cores (BorderGuard, TAMR).
- **Souveraineté** : Les données restent sous le contrôle du COG ; le MWS ne les stocke pas.

## Principes cardinaux

- **Le maillage ne fait pas confiance** — il transporte et expose des informations de présence et des chemins ; il ne gouverne pas les accès métier.
- **Passeport et Permis de circulation** restent souverains : le Webway permet de savoir *où* et *comment* initier une visite gouvernée, pas d'accéder aux données.
- **Aucun Core partagé** : la présence ne donne aucun accès aux Cores ; elle indique où aller pour initier une visite.
- **Une seule gouvernance active par ressource** : le COG Hébergeur décide (accord d'hôte, refus, révocation) ; Origin/Relays décident du Permis de circulation.
- **Trackers officiels uniquement** : le Permis est valable sur tout le réseau ; le Relay remet avec le Permis les adresses des Trackers connus d'Origin.

## Consommation par les strates

| Strate / Niveau | Consommation du MWS |
|-----------------|----------------------|
| **Cores** | Attestation d'environnement, conformité, politique de présence ; dialogue avec Origin/Relays pour Permis de circulation. |
| **Outils (Strate 6)** | MiyuWebwayTracker, MiyuWebwayParticipant : annonces, découverte, Lobbys, catalogue. |
| **Opérateurs / Services (Strate 7+)** | Annonces de services, exposition de Lobbys, consommation des Lobbys d'autres COGs, favoris, amis. |
| **BondingBrother (Strate 5)** | Pont vers le réseau ; exposition des intentions de visite et réception des réponses de découverte. |

## Racine Documentaire

Documentation détaillée : `docs/miyukini-webway-system/` — index des références : `reference/`.
