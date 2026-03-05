# MiyukiniWatch â€” Architecture et Positionnement

## Contexte

**MiyukiniWatch** est un **Service interne COG (Type 1)** qui mesure les habitudes et les interactions de l'utilisateur avec son environnement Miyukini. Ce document dÃ©crit son positionnement dans la Pyramide Miyukini, ses relations avec les Cores, les Outils et les autres Services, ainsi que le flux d'exÃ©cution standard.

## PortÃ©e / Scope

- **Applicable Ã  :** Architecture, positionnement pyramidal, dÃ©pendances inter-strates, flux de gouvernance.
- **Audience :** Architectes, dÃ©veloppeurs, Ã©quipes sÃ©curitÃ©.
- **Statut :** Document normatif â€” rÃ©fÃ©rence architecturale du Service MiyukiniWatch.

---

## 1. Positionnement dans la Pyramide Miyukini

MiyukiniWatch s'inscrit dans la Pyramide des strates comme un **Service** visible par l'utilisateur, portÃ© par des **OpÃ©rateurs** gouvernÃ©s.

| Strate | Ã‰lÃ©ment | RÃ´le vis-Ã -vis de MiyukiniWatch |
|--------|---------|---------------------------------|
| **7** | OpÃ©rateurs MiyukiniWatch | ExÃ©cutent la collecte des mÃ©triques, l'agrÃ©gation, la prÃ©sentation des donnÃ©es et l'effacement sur demande. |
| **6** | Outils & Kits d'Outils | CapacitÃ©s exÃ©cutables utilisÃ©es par les OpÃ©rateurs : Ã©criture/lecture locale (MiyuStorage), horodatage (MiyuTime), agrÃ©gation statistique. |
| **5** | BondingBrother | MÃ©diation entre les OpÃ©rateurs MiyukiniWatch et les Cores. Traduit les intentions de collecte et d'effacement vers les autoritÃ©s. |
| **4** | Cores | Gouvernent le comportement : KindMother (persistance), StrongFather (dÃ©cision), Caring Nanny (observation d'Ã©tat), WorrySentinel (sÃ©curitÃ©), Master Butler (permissions). |
| **3** | Invariants & Contrats | Principes non nÃ©gociables : pas de lecture de contenus, donnÃ©es locales uniquement, transparence totale. |
| **K** | Kernel | Substrat technique neutre : scheduling, I/O, identifiants. |

### 1.1 Flux d'exÃ©cution standard

```
Utilisateur â†’ MiyukiniWatch (Service) â†’ OpÃ©rateurs MiyukiniWatch â†’ BondingBrother â†’ Cores â†’ Outils â†’ ExÃ©cution
```

Le flux se dÃ©compose en deux voies :

**Voie passive (collecte silencieuse) :**
```
Ã‰vÃ©nement utilisateur (clic, ouverture service, connexion)
  â†’ OpÃ©rateur Collecteur MiyukiniWatch
    â†’ BondingBrother
      â†’ StrongFather : "Cette mÃ©trique peut-elle Ãªtre enregistrÃ©e ?"
      â†’ KindMother : WriteIntent (enregistrement de la mÃ©trique)
    â†’ Outils d'Ã©criture locale
      â†’ MÃ©trique persistÃ©e dans le stockage COG
```

**Voie active (consultation par l'utilisateur) :**
```
Utilisateur ouvre MiyukiniWatch depuis Central
  â†’ OpÃ©rateur Consultation MiyukiniWatch
    â†’ BondingBrother
      â†’ KindMother : ReadIntent (lecture des mÃ©triques)
      â†’ Master Butler : "L'utilisateur a-t-il accÃ¨s Ã  ces donnÃ©es ?"
    â†’ Outils de lecture locale
      â†’ DonnÃ©es prÃ©sentÃ©es Ã  l'utilisateur
```

---

## 2. Type de Service et espaces

| Attribut | Valeur |
|----------|--------|
| **Type** | Service interne COG (Type 1) |
| **Espace** | Miyukini Central uniquement |
| **Surface externe** | Aucune â€” pas de surface web, pas de protocole Inter-COG |
| **VisibilitÃ©** | ApparaÃ®t dans la liste des services de Central (Salon / BibliothÃ¨que) |

**RÃ¨gle canonique :** MiyukiniWatch est un service purement interne. Il ne communique jamais avec l'extÃ©rieur du COG. Aucune donnÃ©e ne transite vers un autre COG, un serveur tiers ou le MWS.

---

## 3. OpÃ©rateurs MiyukiniWatch

Les OpÃ©rateurs sont les entitÃ©s fonctionnelles gouvernÃ©es qui exÃ©cutent les capacitÃ©s du service. MiyukiniWatch dÃ©ploie trois OpÃ©rateurs principaux :

| OpÃ©rateur | ResponsabilitÃ© | Mode |
|-----------|---------------|------|
| **MiyukiniWatchCollector** | Collecte des mÃ©triques en temps rÃ©el (sessions, services, amis, clics). Ã‰coute les Ã©vÃ©nements du COG et produit des WriteIntent vers KindMother. | Passif â€” tourne en arriÃ¨re-plan. |
| **MiyukiniWatchAggregator** | AgrÃ¨ge les mÃ©triques brutes en rÃ©sumÃ©s exploitables (par jour, semaine, mois). Produit les agrÃ©gats consommÃ©s par Miou. | PÃ©riodique â€” dÃ©clenchÃ© Ã  intervalles ou Ã  la demande. |
| **MiyukiniWatchPresenter** | PrÃ©sente les donnÃ©es Ã  l'utilisateur lorsqu'il ouvre le service. GÃ¨re l'effacement et la dÃ©sactivation de la collecte. | Actif â€” dÃ©clenchÃ© par l'utilisateur. |

### 3.1 RÃ¨gle fondamentale des OpÃ©rateurs

> Â« Les OpÃ©rateurs sont gouvernÃ©s, jamais autonomes. Â»

Les OpÃ©rateurs MiyukiniWatch ne prennent aucune dÃ©cision seuls. Chaque action de collecte, d'agrÃ©gation ou d'effacement est soumise aux Cores via BondingBrother. Le Collector ne peut enregistrer une mÃ©trique que si StrongFather l'autorise et que KindMother accepte l'Ã©criture.

---

## 4. Interactions avec les Cores

### 4.1 Matrice Core Ã— ResponsabilitÃ©

| Core | RÃ´le vis-Ã -vis de MiyukiniWatch |
|------|---------------------------------|
| **StrongFather** | DÃ©cide si la collecte d'une mÃ©trique est autorisÃ©e. Valide les opÃ©rations d'effacement. Ã‰met les Mandats de Permission pour les OpÃ©rateurs. |
| **KindMother** | AutoritÃ© exclusive de persistance locale. ReÃ§oit et traite les WriteIntent (enregistrement de mÃ©triques) et ReadIntent (consultation). GÃ¨re la rÃ©sidence des donnÃ©es et la purge par rÃ©tention. |
| **Caring Nanny** | Observe l'Ã©tat du systÃ¨me. Peut restreindre la collecte si le COG est en Ã©tat dÃ©gradÃ© (T2, T3, T4) pour prÃ©server les ressources. |
| **Master Butler** | Registre des capacitÃ©s et permissions : qui peut consulter les mÃ©triques, qui peut effacer, qui peut dÃ©sactiver la collecte. |
| **WorrySentinel** | Gouverne le niveau de sÃ©curitÃ© des donnÃ©es MiyukiniWatch. DÃ©finit les politiques de chiffrement au repos et de classification des mÃ©triques. |
| **Border Guard** | Garantit que les donnÃ©es MiyukiniWatch ne franchissent jamais les frontiÃ¨res du COG. Toute tentative d'export est bloquÃ©e par dÃ©faut. |
| **Ever Buddy** | GÃ¨re l'Ã©volution du Service (nouvelles mÃ©triques, changement de format, dÃ©prÃ©ciation). Assure la compatibilitÃ© des agrÃ©gats lors des mises Ã  jour. |
| **TAMR** | Point d'intervention humaine : l'utilisateur peut demander un effacement total, une dÃ©sactivation, ou une consultation des rÃ¨gles en cours. |

### 4.2 Flux de gouvernance dÃ©taillÃ© â€” Collecte d'une mÃ©trique

```
1. Ã‰vÃ©nement dÃ©tectÃ© (ouverture d'un service, par exemple)
2. MiyukiniWatchCollector prÃ©pare un WriteIntent
3. BondingBrother transmet l'intention Ã  StrongFather
   â†’ StrongFather vÃ©rifie :
     - La collecte est-elle activÃ©e ? (TAMR / prÃ©fÃ©rence utilisateur)
     - L'Ã©tat du COG permet-il la collecte ? (Caring Nanny)
     - Le type de mÃ©trique est-il autorisÃ© ? (Master Butler)
   â†’ Si refusÃ© : WriteIntent rejetÃ©, la mÃ©trique n'est pas enregistrÃ©e
4. StrongFather Ã©met un Mandat de Permission
5. BondingBrother transmet le WriteIntent validÃ© Ã  KindMother
   â†’ KindMother persiste la mÃ©trique dans le stockage local
6. Confirmation remontÃ©e Ã  l'OpÃ©rateur
```

---

## 5. DÃ©pendances architecturales

### 5.1 DÃ©pendances amont (MiyukiniWatch consomme)

| DÃ©pendance | Nature | Description |
|------------|--------|-------------|
| **Miyukini Central** | Service hÃ´te | MiyukiniWatch est un service dans Central ; il dÃ©pend de Central pour le cycle de vie (dÃ©marrage, arrÃªt). |
| **KindMother** | Core (Strate 4) | Persistance locale des mÃ©triques. MiyukiniWatch ne peut pas fonctionner sans KindMother. |
| **Ã‰vÃ©nements COG** | Bus d'Ã©vÃ©nements | MiyukiniWatch Ã©coute les Ã©vÃ©nements du COG (ouverture de service, connexion, etc.) pour dÃ©clencher la collecte. |

### 5.2 DÃ©pendances aval (d'autres consomment MiyukiniWatch)

| Consommateur | Nature | Description |
|--------------|--------|-------------|
| **Miou** | Sous-service de Central | Consomme les **agrÃ©gats** produits par MiyukiniWatch pour adapter ses bulles, rappels et suggestions. Voir [MiyukiniWatch - IntÃ©gration Miou et AgrÃ©gats](./MiyukiniWatch%20-%20Integration%20Miou%20et%20Agregats.md). |
| **Salon (Miyukini Central)** | Vue de Central | Les suggestions affichÃ©es sur le Salon peuvent Ãªtre dÃ©rivÃ©es des mÃ©triques MiyukiniWatch. |

### 5.3 IndÃ©pendance vis-Ã -vis du MWS

MiyukiniWatch **n'a aucune dÃ©pendance sur le MWS** (Miyukini Webway System). ConformÃ©ment Ã  LOI-1 et LOI-2 :

- Aucune donnÃ©e MiyukiniWatch ne transite sur le rÃ©seau.
- Le service fonctionne identiquement que le COG soit connectÃ© au Webway ou totalement isolÃ©.
- Un COG de type LONE (structurellement isolÃ©) bÃ©nÃ©ficie de MiyukiniWatch sans aucune restriction.

---

## 6. ConformitÃ© aux Lois d'Autonomie

| Loi | ConformitÃ© MiyukiniWatch |
|-----|--------------------------|
| **LOI-1** | Aucune dÃ©pendance externe critique. MiyukiniWatch ne requiert que KindMother (locale) et les Ã©vÃ©nements internes du COG. |
| **LOI-2** | L'isolement est un Ã©tat normal. MiyukiniWatch fonctionne sans rÃ©seau. |
| **LOI-3** | L'Ã©tat local est souverain. Les mÃ©triques appartiennent au COG et Ã  son utilisateur. |
| **LOI-4** | Pas de temps global requis. Les horodatages sont locaux au COG. |
| **LOI-5** | CoÃ»t proportionnel au hardware. La collecte et l'agrÃ©gation sont lÃ©gÃ¨res et ne requiÃ¨rent pas de ressources disproportionnÃ©es. |
| **LOI-6** | L'autonomie n'empÃªche pas la fÃ©dÃ©ration. MiyukiniWatch ne participe pas Ã  la fÃ©dÃ©ration mais n'empÃªche pas les services qui consomment ses agrÃ©gats (ex. Miou) de fonctionner dans un contexte fÃ©dÃ©rÃ©. |
| **LOI-7** | La strate Cores est immuable. MiyukiniWatch Ã©volue par environnement ; ses mÃ©triques sont versionnÃ©es. |
| **LOI-8** | Migration = diplomatie. Lors d'une migration de COG, les donnÃ©es MiyukiniWatch suivent la procÃ©dure formelle (pas de copie brute). |

---

## 7. Diagramme de contexte

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚                   COG (souverain)                â”‚
â”‚                                                   â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”    â”‚
â”‚  â”‚          Miyukini Central                 â”‚    â”‚
â”‚  â”‚                                            â”‚    â”‚
â”‚  â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”   â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â” â”‚    â”‚
â”‚  â”‚  â”‚ MiyukiniWatchâ”‚â”€â”€â–¶â”‚      Miou        â”‚ â”‚    â”‚
â”‚  â”‚  â”‚  (Service)   â”‚   â”‚ (Sous-service)   â”‚ â”‚    â”‚
â”‚  â”‚  â””â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”˜   â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜ â”‚    â”‚
â”‚  â”‚         â”‚                                  â”‚    â”‚
â”‚  â”‚         â–¼                                  â”‚    â”‚
â”‚  â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”                         â”‚    â”‚
â”‚  â”‚  â”‚  OpÃ©rateurs  â”‚                         â”‚    â”‚
â”‚  â”‚  â”‚  MW Collect.  â”‚                         â”‚    â”‚
â”‚  â”‚  â”‚  MW Aggreg.   â”‚                         â”‚    â”‚
â”‚  â”‚  â”‚  MW Present.  â”‚                         â”‚    â”‚
â”‚  â”‚  â””â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”˜                         â”‚    â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜    â”‚
â”‚            â”‚                                      â”‚
â”‚            â–¼                                      â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”                            â”‚
â”‚  â”‚  BondingBrother   â”‚                            â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜                            â”‚
â”‚           â”‚                                       â”‚
â”‚           â–¼                                       â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”          â”‚
â”‚  â”‚           Cores (Strate 4)         â”‚          â”‚
â”‚  â”‚  StrongFather â”‚ KindMother â”‚ ...   â”‚          â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜          â”‚
â”‚                                                   â”‚
â”‚  â•³ Aucune sortie rÃ©seau (Border Guard)           â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

---

## 8. RÃ©fÃ©rences

| Document | RÃ´le |
|----------|------|
| [MiyukiniWatch â€” Document Fondateur](./MiyukiniWatch%20-%20Document%20Fondateur.md) | Vision, principes fondateurs, mÃ©triques, non-lecture des contenus. |
| [MiyukiniWatch â€” Contraintes et Invariants](./MiyukiniWatch%20-%20Contraintes%20et%20Invariants.md) | RÃ¨gles non nÃ©gociables. |
| [MiyukiniWatch â€” IntÃ©gration Miou et AgrÃ©gats](./MiyukiniWatch%20-%20Integration%20Miou%20et%20Agregats.md) | Lien avec Miou, format des agrÃ©gats. |
| [Miyukini Central â€” Miou, avatar, bulles et rÃ´le](..//..//_index.md) | RÃ´le de Miou et consommation des agrÃ©gats. |
| [Types de Services et Espaces](..//..//miyukini-webway-system//reference//_index.md) | DÃ©finition du Type 1 (Service interne COG). |
| Architecture Miyukini (skill miyukini-architecture) | Pyramide, Cores, Lois d'Autonomie. |

---

**Document** : MiyukiniWatch â€” Architecture et Positionnement  
**Version** : 1.0  
**Date** : 2026-02-14  
**Statut** : Document normatif â€” rÃ©fÃ©rence architecturale du Service MiyukiniWatch


