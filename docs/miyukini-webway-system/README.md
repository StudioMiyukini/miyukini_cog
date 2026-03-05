# Miyukini Webway System (MWS) â€” Racine Documentaire

## Contexte

Le **Miyukini Webway System (MWS)** est trop vaste pour Ãªtre contenu dans une seule strate Miyukini. En raison de son **caractÃ¨re exceptionnel**, sa **racine documentaire** est Ã©tablie ici : `docs/miyukini-webway-system`.

Le MWS est considÃ©rÃ© comme un **systÃ¨me complet** dans l'environnement Miyukini, **uniquement subordonnÃ© aux Cores**. Il est **consommÃ© par toutes les strates**, Ã  diffÃ©rents endroits et selon diffÃ©rents usages.

---

## EntrÃ©e principale

**Document fondateur :** [MWS - Document Fondateur](./MWS%20-%20Document%20Fondateur.md)

---

## Structure complÃ¨te de la documentation MWS

### Racine

| Document | Description |
|----------|-------------|
| [README.md](./README.md) | Ce fichier â€” point d'entrÃ©e |
| [MWS - Document Fondateur](./MWS%20-%20Document%20Fondateur.md) | DÃ©finition, positionnement, principes cardinaux |
| [MWS - Audit Documentation](./MWS%20-%20Audit%20Documentation.md) | Audit de la doc MWS, terminologie (Permis de circulation, trackers officiels), corrections appliquÃ©es |

### Architecture (`architecture/`)

| Document | Description |
|----------|-------------|
| [MWS - Architecture et Subordination aux Cores](./architecture/MWS%20-%20Architecture%20et%20Subordination%20aux%20Cores.md) | Position du MWS vs pyramide des strates, subordination exclusive aux Cores |

### Acteurs (`acteurs/`)

| Document | Description |
|----------|-------------|
| [MWS - Origin](./acteurs/MWS%20-%20Origin.md) | Origin : point d'origine, source de vÃ©ritÃ©, Registre de Services, Passeports spÃ©ciaux |
| [MWS - Relays](./acteurs/MWS%20-%20Relays.md) | Relays : duplications d'Origin, vÃ©rification, distribution des versions |
| [MWS - Trackers](./acteurs/MWS%20-%20Trackers.md) | Trackers : douaniers, pools ; catalogue web = services WEB publics ; Lobbys visibles depuis les services |

### VÃ©rification (`verification/`)

| Document | Description |
|----------|-------------|
| [MWS - Passeport et Permis](./verification/MWS%20-%20Passeport%20et%20Visa.md) | Structure du Passeport COG, types (Standard/SpÃ©cial), Permis de circulation (accord relay), accord d'hÃ´te |
| [MWS - Flux de VÃ©rification](./verification/MWS%20-%20Flux%20de%20Verification.md) | Les 3 phases : clÃ© Cores, blocs de code Services, santÃ© environnement |

### SÃ©curitÃ© (`securite/`)

| Document | Description |
|----------|-------------|
| [MWS - Quarantaine et Blacklist](./securite/MWS%20-%20Quarantaine%20et%20Blacklist.md) | Escalade progressive, auto-destruction, alerte rÃ©seau, confinement |
| [MWS - Chiffrement et TLS](./securite/MWS%20-%20Chiffrement%20et%20TLS.md) | TLS obligatoire, exemption temps rÃ©el, gestion des secrets |
| [MWS - Registre de Services et Isolation](./securite/MWS%20-%20Registre%20de%20Services%20et%20Isolation.md) | Registre de Services, services tiers, isolation des COGs non conformes |

### Lobbys et Connexions (`lobbys/`)

| Document | Description |
|----------|-------------|
| [MWS - Lobbys, Favoris et Amis](./lobbys/MWS%20-%20Lobbys%20Favoris%20et%20Amis.md) | Lobbys publics/privÃ©s, surfaces de connexion, favoris, relation amis |

### Protocole (`protocole/`)

| Document | Description |
|----------|-------------|
| [MWS - Protocole Relay](./protocole/MWS%20-%20Protocole%20Relay.md) | Format binaire, types de messages, sÃ©quences d'Ã©change |

### DÃ©ploiement (`deploiement/`)

| Document | Description |
|----------|-------------|
| [MWS - Guide de DÃ©ploiement](./deploiement/MWS%20-%20Guide%20de%20Deploiement.md) | Installation relay/tracker, TLS, systemd, monitoring |

### Strates (`strates/`)

| Document | Description |
|----------|-------------|
| [MWS - Consommation par les Strates](./strates/MWS%20-%20Consommation%20par%20les%20Strates.md) | Comment chaque strate (Cores, Outils, OpÃ©rateurs) consomme le MWS |

### Index des rÃ©fÃ©rences (`reference/`)

| Document | Description |
|----------|-------------|
| [_index.md](./reference/_index.md) | Index centralisÃ© vers les documents de rÃ©fÃ©rence dÃ©taillÃ©s |

---

## Carte de navigation

```
docs/miyukini-webway-system/
â”‚
â”œâ”€â”€ README.md                           â† Vous Ãªtes ici
â”œâ”€â”€ MWS - Document Fondateur.md         â† Point d'entrÃ©e conceptuel
â”‚
â”œâ”€â”€ architecture/
â”‚   â””â”€â”€ MWS - Architecture et Subordination aux Cores.md
â”‚
â”œâ”€â”€ acteurs/
â”‚   â”œâ”€â”€ MWS - Origin.md
â”‚   â”œâ”€â”€ MWS - Relays.md
â”‚   â””â”€â”€ MWS - Trackers.md
â”‚
â”œâ”€â”€ verification/
â”‚   â”œâ”€â”€ MWS - Passeport et Permis.md (Passeport et Visa)
â”‚   â””â”€â”€ MWS - Flux de Verification.md
â”‚
â”œâ”€â”€ securite/
â”‚   â”œâ”€â”€ MWS - Quarantaine et Blacklist.md
â”‚   â”œâ”€â”€ MWS - Chiffrement et TLS.md
â”‚   â””â”€â”€ MWS - Registre de Services et Isolation.md
â”‚
â”œâ”€â”€ lobbys/
â”‚   â””â”€â”€ MWS - Lobbys Favoris et Amis.md
â”‚
â”œâ”€â”€ protocole/
â”‚   â””â”€â”€ MWS - Protocole Relay.md
â”‚
â”œâ”€â”€ deploiement/
â”‚   â””â”€â”€ MWS - Guide de Deploiement.md
â”‚
â”œâ”€â”€ strates/
â”‚   â””â”€â”€ MWS - Consommation par les Strates.md
â”‚
â””â”€â”€ reference/
    â””â”€â”€ _index.md                       â† Liens vers docs/reference/, docs/tools/, docs/setup/
```

---

## RÃ©fÃ©rences externes (hors racine MWS)

Les documents dÃ©taillÃ©s (spÃ©cifications, protocoles, contrats, outils) restent dans l'arborescence existante ; cette racine **pointe** vers eux et en donne la logique :

### RÃ©fÃ©rences conceptuelles (`docs/reference/`)

- [Miyukini Webway System](reference//_index.md)
- [Miyukini Webway System Complet](reference//_index.md)
- [Miyukini Webway System Normes et Standards](reference//_index.md)
- [Miyukini Webway Relay](reference//_index.md)
- [Miyukini Webway Relay Protocol](reference//_index.md)
- [Glossaire](reference//_index.md)

### Outils MWS (`docs/tools/`)

- [MiyuWebwayTracker](../tools/MiyuWebwayTracker/)
- [MiyuWebwayParticipant](../tools/MiyuWebwayParticipant/)

### Setup et dÃ©ploiement (`docs/setup/`)

- [Webway Relay Deployment Guide](setup//Miyukini%20-%20Webway%20Relay%20Deployment%20Guide.md)
- [Hostinger VPS Origin Webway](setup//Miyukini%20-%20Hostinger%20VPS%20Origin%20Webway.md)

---

## Principes de cette documentation

1. **ExhaustivitÃ©** : Chaque aspect du MWS est documentÃ© dans un fichier dÃ©diÃ©.
2. **CohÃ©rence** : Les termes et concepts sont alignÃ©s avec le [Glossaire](reference//_index.md).
3. **RÃ©fÃ©rences croisÃ©es** : Chaque document pointe vers les documents liÃ©s.
4. **Pas de duplication** : Les spÃ©cifications dÃ©taillÃ©es restent dans `docs/reference/` ; cette racine en donne la vision d'ensemble.

---

**Version :** 2.0  
**Statut :** Racine documentaire officielle du MWS  
**DerniÃ¨re mise Ã  jour :** 2026-02-13


