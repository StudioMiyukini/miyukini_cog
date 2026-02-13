# Miyukini Webway System (MWS) — Racine Documentaire

## Contexte

Le **Miyukini Webway System (MWS)** est trop vaste pour être contenu dans une seule strate Miyukini. En raison de son **caractère exceptionnel**, sa **racine documentaire** est établie ici : `docs/miyukini-webway-system`.

Le MWS est considéré comme un **système complet** dans l'environnement Miyukini, **uniquement subordonné aux Cores**. Il est **consommé par toutes les strates**, à différents endroits et selon différents usages.

---

## Entrée principale

**Document fondateur :** [MWS - Document Fondateur](./MWS%20-%20Document%20Fondateur.md)

---

## Structure complète de la documentation MWS

### Racine

| Document | Description |
|----------|-------------|
| [README.md](./README.md) | Ce fichier — point d'entrée |
| [MWS - Document Fondateur](./MWS%20-%20Document%20Fondateur.md) | Définition, positionnement, principes cardinaux |

### Architecture (`architecture/`)

| Document | Description |
|----------|-------------|
| [MWS - Architecture et Subordination aux Cores](./architecture/MWS%20-%20Architecture%20et%20Subordination%20aux%20Cores.md) | Position du MWS vs pyramide des strates, subordination exclusive aux Cores |

### Acteurs (`acteurs/`)

| Document | Description |
|----------|-------------|
| [MWS - Origin](./acteurs/MWS%20-%20Origin.md) | Origin : point d'origine, source de vérité, Registre de Services, Passeports spéciaux |
| [MWS - Relays](./acteurs/MWS%20-%20Relays.md) | Relays : duplications d'Origin, vérification, distribution des versions |
| [MWS - Trackers](./acteurs/MWS%20-%20Trackers.md) | Trackers : douaniers, pools ; catalogue web = services WEB publics ; Lobbys visibles depuis les services |

### Vérification (`verification/`)

| Document | Description |
|----------|-------------|
| [MWS - Passeport et Permis](./verification/MWS%20-%20Passeport%20et%20Visa.md) | Structure du Passeport COG, types (Standard/Spécial), Permis de circulation (accord relay), accord d'hôte |
| [MWS - Flux de Vérification](./verification/MWS%20-%20Flux%20de%20Verification.md) | Les 3 phases : clé Cores, blocs de code Services, santé environnement |

### Sécurité (`securite/`)

| Document | Description |
|----------|-------------|
| [MWS - Quarantaine et Blacklist](./securite/MWS%20-%20Quarantaine%20et%20Blacklist.md) | Escalade progressive, auto-destruction, alerte réseau, confinement |
| [MWS - Chiffrement et TLS](./securite/MWS%20-%20Chiffrement%20et%20TLS.md) | TLS obligatoire, exemption temps réel, gestion des secrets |
| [MWS - Registre de Services et Isolation](./securite/MWS%20-%20Registre%20de%20Services%20et%20Isolation.md) | Registre de Services, services tiers, isolation des COGs non conformes |

### Lobbys et Connexions (`lobbys/`)

| Document | Description |
|----------|-------------|
| [MWS - Lobbys, Favoris et Amis](./lobbys/MWS%20-%20Lobbys%20Favoris%20et%20Amis.md) | Lobbys publics/privés, surfaces de connexion, favoris, relation amis |

### Protocole (`protocole/`)

| Document | Description |
|----------|-------------|
| [MWS - Protocole Relay](./protocole/MWS%20-%20Protocole%20Relay.md) | Format binaire, types de messages, séquences d'échange |

### Déploiement (`deploiement/`)

| Document | Description |
|----------|-------------|
| [MWS - Guide de Déploiement](./deploiement/MWS%20-%20Guide%20de%20Deploiement.md) | Installation relay/tracker, TLS, systemd, monitoring |

### Strates (`strates/`)

| Document | Description |
|----------|-------------|
| [MWS - Consommation par les Strates](./strates/MWS%20-%20Consommation%20par%20les%20Strates.md) | Comment chaque strate (Cores, Outils, Opérateurs) consomme le MWS |

### Index des références (`reference/`)

| Document | Description |
|----------|-------------|
| [_index.md](./reference/_index.md) | Index centralisé vers les documents de référence détaillés |

---

## Carte de navigation

```
docs/miyukini-webway-system/
│
├── README.md                           ← Vous êtes ici
├── MWS - Document Fondateur.md         ← Point d'entrée conceptuel
│
├── architecture/
│   └── MWS - Architecture et Subordination aux Cores.md
│
├── acteurs/
│   ├── MWS - Origin.md
│   ├── MWS - Relays.md
│   └── MWS - Trackers.md
│
├── verification/
│   ├── MWS - Passeport et Permis.md (Passeport et Visa)
│   └── MWS - Flux de Verification.md
│
├── securite/
│   ├── MWS - Quarantaine et Blacklist.md
│   ├── MWS - Chiffrement et TLS.md
│   └── MWS - Registre de Services et Isolation.md
│
├── lobbys/
│   └── MWS - Lobbys Favoris et Amis.md
│
├── protocole/
│   └── MWS - Protocole Relay.md
│
├── deploiement/
│   └── MWS - Guide de Deploiement.md
│
├── strates/
│   └── MWS - Consommation par les Strates.md
│
└── reference/
    └── _index.md                       ← Liens vers docs/reference/, docs/tools/, docs/setup/
```

---

## Références externes (hors racine MWS)

Les documents détaillés (spécifications, protocoles, contrats, outils) restent dans l'arborescence existante ; cette racine **pointe** vers eux et en donne la logique :

### Références conceptuelles (`docs/reference/`)

- [Miyukini Webway System](../reference/Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System.md)
- [Miyukini Webway System Complet](../reference/Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System%20Complet.md)
- [Miyukini Webway System Normes et Standards](../reference/Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System%20Normes%20et%20Standards.md)
- [Miyukini Webway Relay](../reference/Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20Relay.md)
- [Miyukini Webway Relay Protocol](../reference/Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20Relay%20Protocol.md)
- [Glossaire](../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

### Outils MWS (`docs/tools/`)

- [MiyuWebwayTracker](../tools/MiyuWebwayTracker/)
- [MiyuWebwayParticipant](../tools/MiyuWebwayParticipant/)

### Setup et déploiement (`docs/setup/`)

- [Webway Relay Deployment Guide](../setup/Miyukini%20-%20Webway%20Relay%20Deployment%20Guide.md)
- [Oracle Cloud Instance Webway Relay](../setup/Miyukini%20-%20Oracle%20Cloud%20Instance%20Webway%20Relay.md)

---

## Principes de cette documentation

1. **Exhaustivité** : Chaque aspect du MWS est documenté dans un fichier dédié.
2. **Cohérence** : Les termes et concepts sont alignés avec le [Glossaire](../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md).
3. **Références croisées** : Chaque document pointe vers les documents liés.
4. **Pas de duplication** : Les spécifications détaillées restent dans `docs/reference/` ; cette racine en donne la vision d'ensemble.

---

**Version :** 2.0  
**Statut :** Racine documentaire officielle du MWS  
**Dernière mise à jour :** 2026-02-13
