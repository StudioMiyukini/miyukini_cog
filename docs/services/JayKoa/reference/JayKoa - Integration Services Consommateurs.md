# JayKoa â€” IntÃ©gration avec les services consommateurs

## Contexte

Ce document dÃ©crit les **schÃ©mas dâ€™intÃ©gration** entre **JayKoa** et les **services consommateurs** (JayRDV, JayFestival, et futurs services). **JayKoa intÃ¨gre tout ce qui manipule des dates** (JayRDV, JayFestival, futurs services). Il prÃ©cise qui publie quelles entrÃ©es agenda, qui interroge quoi, et comment la gouvernance (Mandats, niveaux de sÃ©curitÃ©) sâ€™applique.

## PortÃ©e / Scope

- **PÃ©rimÃ¨tre** : ModÃ¨le dâ€™intÃ©gration JayKoa â†” JayRDV, JayFestival, futurs services ; types dâ€™entrÃ©es ; flux et responsabilitÃ©s.
- **Hors pÃ©rimÃ¨tre** : SpÃ©cifications API dÃ©taillÃ©es (rÃ©fÃ©rencÃ©es dans les contrats dâ€™OpÃ©rateurs et Kits).

---

## 1. Principe dâ€™intÃ©gration

- **JayKoa** expose des **OpÃ©rateurs** et **Kits dâ€™outils** (entrÃ©es, conflits, vue, export).
- Chaque **service consommateur** :
  - **dÃ©tient** les donnÃ©es mÃ©tier (qui a quel RDV, quelle candidature, etc.) ;
  - **publie** vers JayKoa les **entrÃ©es agenda** nÃ©cessaires (plage, type, **nature** â€” ex. prÃ©sence physique â€”, identifiant opaque, niveau de sÃ©curitÃ©) ;
  - **interroge** JayKoa pour conflits, vues agrÃ©gÃ©es, export, selon Mandat et permissions.
- **Types dâ€™Ã©vÃ©nements** : JayKoa distingue notamment les Ã©vÃ©nements de type **prÃ©sence physique** (ne doivent pas se chevaucher ; si lâ€™utilisateur force, pas de blocage mais notification et indicateurs UI â€” alertes, rouge clignotant â€” jusquâ€™Ã  rÃ©solution). Les services consommateurs **dÃ©clarent la nature** des entrÃ©es (prÃ©sence physique ou autre) lors de la publication pour que JayKoa applique la rÃ¨gle de conflit adÃ©quate.
- **KindMother** : la rÃ©sidence des donnÃ©es sensibles reste dÃ©finie par le **contrat de chaque service** (JayRDV, JayFestival) et la [Politique de rÃ©sidence des donnÃ©es sensibles](..//..//..//miyukini-webway-system//reference//_index.md). JayKoa peut dÃ©tenir des **rÃ©fÃ©rences** ou des **synthÃ¨ses** sans Ãªtre la seule copie des donnÃ©es personnelles ou mÃ©tier.

---

## 2. JayRDV

### 2.1 Usage de JayKoa

| CapacitÃ© | Description |
|----------|-------------|
| **CrÃ©neaux et RDV** | Publication des plages (RDV, crÃ©neaux rÃ©servÃ©s, exceptions) ; type dâ€™entrÃ©e Â« RDV Â» ou Â« crÃ©neau Â». |
| **Conflits** | VÃ©rification de conflit (double rÃ©servation, chevauchement) avant validation dâ€™un RDV. |
| **Vue calendrier** | Vue pro (RDV du jour, semaine, mois) ; vue client (Â« Mes RDV Â») ; agrÃ©gation possible avec dâ€™autres sources si Mandat. |
| **Export** | Export iCal/PDF pour le professionnel ou le client ; pas dâ€™exposition des donnÃ©es dâ€™autres clients. |

### 2.2 Types dâ€™entrÃ©es agenda (JayRDV)

| Type dâ€™entrÃ©e | DonnÃ©es publiÃ©es vers JayKoa | Niveau WorrySentinel typique |
|---------------|----------------------------------------|------------------------------|
| **RDV** | Plage (dÃ©but, fin), fuseau, id opaque, rÃ©fÃ©rence utilisateur (pro, client), **nature** (ex. prÃ©sence physique), pas de nom ni dÃ©tail en clair dans JayKoa | 1â€“2 selon contexte |
| **CrÃ©neau rÃ©servÃ©** | Plage, id opaque, rÃ©fÃ©rence ressource (pro, praticien) | 0â€“1 |
| **Exception** | Plage (congÃ©s, absence), id opaque, rÃ©fÃ©rence pro | 1 |

### 2.3 ResponsabilitÃ©s

| Acteur | ResponsabilitÃ© |
|--------|----------------|
| **JayRDV** | DÃ©tient les donnÃ©es mÃ©tier (qui, quoi, oÃ¹) ; publie les entrÃ©es agenda (plage, type, id) ; interroge JayKoa pour conflits et vues. |
| **JayKoa** | Stocke les rÃ©fÃ©rences et synthÃ¨ses ; calcule les conflits ; fournit vues agrÃ©gÃ©es et export selon Mandat et niveau de sÃ©curitÃ©. |

---

## 3. JayFestival (JayFestival)

### 3.1 Usage de JayKoa

| CapacitÃ© | Description |
|----------|-------------|
| **Agenda cross-Ã©vÃ©nements** | Publication des plages (Ã©ditions, participations, candidatures, ateliers rÃ©servÃ©s) ; types dâ€™entrÃ©e Â« Ã©dition Â», Â« participation Â», Â« atelier Â». |
| **Conflits de dates** | VÃ©rification avant dÃ©pÃ´t de candidature ou inscription (exposant ou visiteur) : pas de chevauchement avec une autre Ã©dition ou crÃ©neau dÃ©jÃ  inscrit. |
| **Vue calendrier** | Vue exposant (Ã©ditions candidat/inscrit) ; vue visiteur (Ã©vÃ©nements, ateliers, rÃ©servations) ; agrÃ©gation possible avec RDV (JayRDV) si Mandat. |
| **Export** | Export iCal/PDF pour exposant ou visiteur ; pas dâ€™exposition des donnÃ©es dâ€™autres utilisateurs. |

### 3.2 Types dâ€™entrÃ©es agenda (JayFestival)

| Type dâ€™entrÃ©e | DonnÃ©es publiÃ©es vers JayKoa | Niveau WorrySentinel typique |
|---------------|----------------------------------------|------------------------------|
| **Ã‰dition (Ã©vÃ©nement)** | Plage (dates de lâ€™Ã©dition), id opaque, rÃ©fÃ©rence organisateur, pas de dÃ©tail mÃ©tier en clair | 0â€“1 |
| **Participation** | Plage (dates de lâ€™Ã©dition), id opaque, rÃ©fÃ©rence exposant/visiteur, statut (candidat, inscrit), **nature** (ex. prÃ©sence physique pour une Ã©dition festival) | 1â€“2 |
| **Atelier rÃ©servÃ©** | Plage (crÃ©neau atelier), id opaque, rÃ©fÃ©rence visiteur | 1â€“2 |

### 3.3 ResponsabilitÃ©s

| Acteur | ResponsabilitÃ© |
|--------|----------------|
| **JayFestival** | DÃ©tient les donnÃ©es mÃ©tier (Ã©ditions, candidatures, participations, ateliers) ; publie les entrÃ©es agenda (plage, type, id) ; interroge JayKoa pour conflits et vues. |
| **JayKoa** | Stocke les rÃ©fÃ©rences et synthÃ¨ses ; calcule les conflits (ex. deux Ã©ditions Ã  la mÃªme date pour un exposant) ; fournit vues agrÃ©gÃ©es et export selon Mandat et niveau de sÃ©curitÃ©. |

---

## 4. AgrÃ©gation multi-services

Lorsquâ€™un **mÃªme utilisateur** (ex. exposant) a des entrÃ©es agenda issues de **plusieurs services** (JayRDV + JayFestival) :

- **JayKoa** peut fournir une **vue agrÃ©gÃ©e** (calendrier unifiÃ©) et une **dÃ©tection de conflits cross-service** (ex. RDV le mÃªme jour quâ€™une Ã©dition festival) **si et seulement si** :
  - les deux services ont publiÃ© les entrÃ©es concernÃ©es vers JayKoa ;
  - le **Mandat de Permission** ou le **Mandat public dâ€™accÃ¨s** autorise lâ€™agrÃ©gation pour cet utilisateur ;
  - le **niveau de sÃ©curitÃ©** du contexte (WorrySentinel) est respectÃ© pour chaque entrÃ©e.

Les **services consommateurs** restent responsables du **niveau** des donnÃ©es quâ€™ils publient ; JayKoa applique les **rÃ¨gles de visibilitÃ©** (pas dâ€™affichage ni dâ€™export au-delÃ  du niveau autorisÃ©).

---

## 5. Futurs services

Tout **nouveau service** qui gÃ¨re des plages temporelles, des rÃ©servations ou des Ã©vÃ©nements (formations, interventions, maintenance, etc.) peut **sâ€™intÃ©grer** Ã  JayKoa en :

- **DÃ©clarant** les types dâ€™entrÃ©es quâ€™il publie (plage, type, niveau WorrySentinel).
- **Publient** les entrÃ©es agenda selon le contrat JayKoa.
- **Interrogeant** JayKoa pour conflits, vues agrÃ©gÃ©es, export, selon Mandat.

La liste des types dâ€™entrÃ©es et des niveaux est **extensible** ; les rÃ¨gles de conflit et de visibilitÃ© restent gÃ©rÃ©es par JayKoa de maniÃ¨re uniforme.

---

## 6. SÃ©quence dâ€™intÃ©gration (qui appelle quoi, dans quel ordre)

Les flux ci-dessous dÃ©crivent **Ã©tape par Ã©tape** comment les services consommateurs interagissent avec JayKoa.

### 6.1 Flux publication dâ€™entrÃ©es

| Ã‰tape | Acteur | Action |
|-------|--------|--------|
| 1 | Utilisateur | Valide une action mÃ©tier (crÃ©ation RDV, candidature, inscription atelier) dans lâ€™UI du service consommateur (JayRDV, JayFestival). |
| 2 | Service consommateur | Enregistre les donnÃ©es mÃ©tier (qui, quoi, oÃ¹) ; **appelle** JayKoa (OpÃ©rateur EntrÃ©es / Kit EntrÃ©es) : **publier** lâ€™entrÃ©e agenda (plage, type, nature, id opaque, rÃ©fÃ©rence utilisateur, source). |
| 3 | JayKoa | Enregistre la **rÃ©fÃ©rence** ; met Ã  jour les index (conflits, vues) ; rend lâ€™entrÃ©e disponible pour les requÃªtes selon Mandat. |
| 4 | Service consommateur | Affiche la confirmation Ã  lâ€™utilisateur ; lâ€™entrÃ©e apparaÃ®tra dans Â« Mon agenda Â» lors des prochaines interrogations vue. |

### 6.2 Flux vÃ©rification conflit (avant ou lors de la validation)

| Ã‰tape | Acteur | Action |
|-------|--------|--------|
| 1 | Utilisateur | Remplit un formulaire (candidature, rÃ©servation, inscription) avec une date ou une plage. |
| 2 | Service consommateur | **Appelle** JayKoa (OpÃ©rateur Conflits / Kit Conflits) : **vÃ©rifier conflit** pour utilisateur U, plage P, type T. |
| 3 | JayKoa | Compare la plage avec les entrÃ©es existantes de lâ€™utilisateur ; retourne conflit oui/non + liste des entrÃ©es en conflit (plage, type, libellÃ© court). |
| 4 | Service consommateur | Affiche lâ€™alerte (AGD-UI-02) si conflit ; lâ€™utilisateur modifie, annule ou confirme. Si confirmation : enregistrement mÃ©tier puis **flux publication** (6.1). |
| 5 | JayKoa | Si publication : enregistre lâ€™entrÃ©e ; pour prÃ©sence physique en conflit, maintient le statut Â« conflit non rÃ©solu Â» et fournit les donnÃ©es pour AGD-UI-06. |

### 6.3 Flux vue calendrier / liste / export

| Ã‰tape | Acteur | Action |
|-------|--------|--------|
| 1 | Utilisateur | Ouvre la page Â« Mon agenda Â», calendrier, liste ou export. |
| 2 | Service consommateur | **Appelle** JayKoa (OpÃ©rateur Vue & Export / Kit Vue & Export) : **entrÃ©es** pour utilisateur U, **pÃ©riode** P, **filtres** F (source, type, statut, visibilitÃ© â€” voir Â§ 7). |
| 3 | JayKoa | Retourne les entrÃ©es Ã©ligibles (Mandat, niveau de sÃ©curitÃ©, pÃ©riode, filtres). Pour export : gÃ©nÃ¨re le fichier iCal/PDF (AGD-SEC-3). |
| 4 | Service consommateur | Affiche la vue calendrier (AGD-UI-01), liste (AGD-UI-07) ou propose le tÃ©lÃ©chargement du fichier. |

### 6.4 Flux Ã©vÃ©nements publics et ajout Ã  mon agenda

| Ã‰tape | Acteur | Action |
|-------|--------|--------|
| 1 | Utilisateur | Ouvre la page Â« DÃ©couvrir Â», Â« Ã‰vÃ©nements publics Â» ou une zone Â« Ajouter Ã  mon agenda Â» dans lâ€™UI du service consommateur (JayFestival, JayRDV). |
| 2 | Service consommateur | **Appelle** JayKoa (OpÃ©rateur Ã‰vÃ©nements publics / Kit Ã‰vÃ©nements publics) : **liste des Ã©vÃ©nements publics** pour pÃ©riode P, filtres F (source, type). |
| 3 | JayKoa | Retourne la liste des Ã©vÃ©nements publics Ã©ligibles (plage, type, source, libellÃ© court, id opaque) â€” sans donnÃ©es personnelles. |
| 4 | Service consommateur | Affiche la vue Ã©vÃ©nements publics (AGD-UI-09). |
| 5 | Utilisateur | **SÃ©lectionne** un ou plusieurs Ã©vÃ©nements et clique Â« Ajouter Ã  mon agenda Â» (AGD-UI-10). |
| 6 | Service consommateur | DÃ©clenche le **flux mÃ©tier** (inscription, rÃ©servation) ; **appelle** JayKoa pour **vÃ©rification conflit** (6.2) ; affiche alerte si conflit. |
| 7 | Utilisateur | Confirme ou modifie ; le service consommateur enregistre lâ€™inscription/rÃ©servation puis **publie** lâ€™entrÃ©e vers JayKoa (flux 6.1). |
| 8 | JayKoa | Enregistre lâ€™entrÃ©e ; lâ€™Ã©vÃ©nement apparaÃ®t dans Â« Mon agenda Â». |

---

## 7. Contrat conceptuel : paramÃ¨tres et filtres

Les **paramÃ¨tres** passÃ©s par les services consommateurs Ã  JayKoa et les **filtres** supportÃ©s sont formalisÃ©s ci-dessous (contrat conceptuel, pas spÃ©cification API).

### 7.1 ParamÃ¨tres dâ€™interrogation (vue, export, Ã©vÃ©nements publics)

| ParamÃ¨tre | Description | Obligatoire / optionnel |
|-----------|-------------|-------------------------|
| **Utilisateur** | RÃ©fÃ©rence utilisateur (id opaque ou contexte Mandat) pour Â« mes entrÃ©es Â». | Obligatoire pour vue/export Â« Mon agenda Â». |
| **PÃ©riode** | Date dÃ©but, date fin (ou plage) pour restreindre les entrÃ©es. | Obligatoire pour vue/export. |
| **Filtres** | Voir Â§ 7.2. | Optionnel ; si absent, toutes les entrÃ©es Ã©ligibles (Mandat, niveau de sÃ©curitÃ©) sont retournÃ©es. |

### 7.2 Filtres supportÃ©s par JayKoa

| Filtre | Description | UtilisÃ© dans |
|--------|-------------|--------------|
| **Source** | Service dâ€™origine (JayRDV, JayFestival, etc.). | Vue agrÃ©gÃ©e, filtre AGD-UI-04. |
| **Type** | Type dâ€™entrÃ©e (RDV, Ã©dition, atelier, participation, etc.). | Vue calendrier, liste, AGD-UI-04. |
| **Statut** | Statut mÃ©tier si exposÃ© par le consommateur (ex. candidat, inscrit, confirmÃ©). | Filtres dÃ©taillÃ©s, vue liste. |
| **VisibilitÃ©** | Public vs privÃ© : nâ€™afficher que les entrÃ©es Â« public Â» (pour catalogue Ã©vÃ©nements publics) ou Â« mes entrÃ©es Â» (privÃ©). | Vue Ã©vÃ©nements publics (AGD-UI-09), vue Â« Mon agenda Â». |
| **Nature** | Nature de lâ€™Ã©vÃ©nement (ex. prÃ©sence physique) pour rÃ¨gles de conflit ou affichage. | Conflits, vues. |

Les services consommateurs **passent** ces filtres lors des appels Ã  JayKoa ; JayKoa **applique** les rÃ¨gles de visibilitÃ© (Mandat, WorrySentinel) et retourne uniquement les entrÃ©es Ã©ligibles.

### 7.3 Format de sortie (conceptuel)

- **Liste dâ€™entrÃ©es** : pour chaque entrÃ©e : plage (dÃ©but, fin), type, libellÃ© court, source, id opaque, statut/nature si exposÃ©.
- **Conflit** : conflit oui/non ; liste des entrÃ©es en conflit (plage, type, libellÃ© court).
- **Export** : fichier iCal ou PDF (entrÃ©es Ã©ligibles, pas de donnÃ©es au-delÃ  du niveau autorisÃ©).

---

## 8. RÃ©fÃ©rences

| Document | RÃ´le |
|----------|------|
| [JayKoa - Document Fondateur](../JayKoa%20-%20Document%20Fondateur.md) | Contexte, positionnement, intÃ©gration synthÃ©tique. |
| [JayKoa - Operateurs et Toolkits](../JayKoa%20-%20Operateurs%20et%20Toolkits.md) | OpÃ©rateurs, Kits, filtres supportÃ©s. |
| [JayRDV - Document Fondateur](../../JayRDV/JayRDV%20-%20Document%20Fondateur.md) | Service consommateur (RDV, crÃ©neaux). |
| [JayFestival - Document Fondateur](../../JayFestival/JayFestival%20-%20Document%20Fondateur.md) | Service consommateur (agenda cross-Ã©vÃ©nements). |

---

**Document** : JayKoa â€” IntÃ©gration avec les services consommateurs  
**Version** : 1.1  
**Date** : 2026-01-31  
**Statut** : Document de rÃ©fÃ©rence (intÃ©gration, sÃ©quence, contrat, filtres)

