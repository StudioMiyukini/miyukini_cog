# Exposants â€” Parcours, capacitÃ©s et dashboard

## Contexte

Ce document dÃ©taille le **parcours**, les **capacitÃ©s** et le **dashboard** du public cible **Exposants** dans le cadre du service Miyukini Festival Service. Il complÃ¨te le [document fondateur](..//..//_index.md).

## PortÃ©e / Scope

- **Public** : Exposants (professionnels ou structures participant Ã  des Ã©vÃ©nements en tant quâ€™exposants).
- **PÃ©rimÃ¨tre** : onboarding, dashboard dÃ©diÃ©, candidatures, participations, agenda, conflits de dates, limites.
- **Hors pÃ©rimÃ¨tre** : spÃ©cifications techniques dâ€™implÃ©mentation (OpÃ©rateurs, Kits, API).

---

## 1. Profil du public

| CritÃ¨re | Description |
|---------|-------------|
| **Qui** | Professionnels, entreprises, associations participant Ã  des festivals/Ã©vÃ©nements avec un stand ou une prÃ©sence exposant. |
| **Compte** | Cross-Ã©vÃ©nements : un mÃªme exposant peut **participer Ã  plusieurs festivals**. |
| **AccÃ¨s** | Authentification (Miyauth), permissions (Master Butler), rÃ´le exposant. |
| **Espace** | **Dashboard exposant dÃ©diÃ©** : vue unifiÃ©e sur toutes ses candidatures, participations, documents, factures et **agenda**. |

---

## 2. Parcours utilisateur

### 2.1 Onboarding

1. **CrÃ©ation de compte** : inscription en tant quâ€™exposant (Miyauth, Miyuprofile, fiche entreprise/contact).
2. **Validation** : selon politique plateforme ou selon validation par lâ€™organisateur pour une Ã©dition donnÃ©e.
3. **Attribution des permissions** : rÃ´le exposant (Master Butler).
4. **PremiÃ¨re candidature** : dÃ©pÃ´t dâ€™une candidature pour une Ã©dition (festival) ; lâ€™exposant peut ensuite en dÃ©poser dâ€™autres pour dâ€™autres Ã©ditions.

Le compte est **cross-Ã©vÃ©nements** dÃ¨s lâ€™origine : lâ€™exposant peut candidater et participer Ã  autant de festivals que souhaitÃ©, sous rÃ©serve des rÃ¨gles dâ€™agenda (conflits de dates).

### 2.2 Parcours type (cycle de vie)

| Ã‰tape | Action | RÃ©sultat |
|-------|--------|----------|
| **Connexion** | Connexion avec identifiants exposant. | AccÃ¨s au **dashboard exposant**. |
| **Vue dâ€™ensemble** | Consultation du dashboard : candidatures, participations, agenda, documents, factures. | Vue unifiÃ©e sur **tous les festivals** concernÃ©s. |
| **DÃ©couverte** | Consultation de lâ€™annuaire des Ã©vÃ©nements (catalogue). | Liste des festivals ouverts aux candidatures. |
| **Candidature** | DÃ©pÃ´t dâ€™une candidature pour une Ã©dition ; saisie des informations demandÃ©es par lâ€™organisateur. | Candidature en attente ; **vÃ©rification agenda** (conflit de dates ?). |
| **Suivi** | Consultation du statut (en attente, validÃ©e, refusÃ©e) ; rÃ©ception des documents, devis, factures. | Suivi par Ã©dition. |
| **Participation** | Une fois validÃ© : accÃ¨s aux documents de lâ€™Ã©dition, emplacement, programme, facturation (Miyuinvoice). | Participation active Ã  lâ€™Ã©dition. |
| **ClÃ´ture** | Fin de lâ€™Ã©dition : archivage des documents et factures dans le dashboard. | Historique conservÃ© ; possibilitÃ© de candidater Ã  dâ€™autres Ã©ditions. |

### 2.3 Gestion dâ€™agenda et conflits de dates

- **ProblÃ©matique** : un exposant ne doit pas sâ€™inscrire Ã  **deux Ã©vÃ©nements Ã  la mÃªme date** (besoin dÃ©jÃ  rencontrÃ© en pratique â€” Â« dÃ©jÃ  vu Â»).
- **Solution** : **gestion dâ€™agenda** (calendrier cross-Ã©vÃ©nements) :
  - Visualisation des dates des Ã©vÃ©nements auxquels lâ€™exposant est inscrit ou candidat.
  - **Alerte ou blocage** en cas de chevauchement de dates avant validation dâ€™une nouvelle candidature.
  - Lâ€™exposant peut organiser son planning sur plusieurs festivals sans double engagement.

Cette capacitÃ© relÃ¨ve de lâ€™OpÃ©rateur ou Kit **Agenda cross-Ã©vÃ©nements** (MiyuClock, Miyubooking, donnÃ©es dâ€™Ã©dition).

### 2.4 Points de sortie / passerelles

- **Vers organisateurs** : les candidatures et participations sont gÃ©rÃ©es par les organisateurs de chaque Ã©dition ; lâ€™exposant ne modifie pas les paramÃ¨tres des Ã©ditions.
- **Vers catalogue** : la fiche exposant peut apparaÃ®tre dans le **rÃ©pertoire des exposants** (selon politique plateforme), visible par [utilisateur non connectÃ©](../UtilisateurNonConnecte/_index.md) et tous les publics.
- **Vers visiteurs** : un exposant peut aussi Ãªtre visiteur sur dâ€™autres Ã©vÃ©nements (compte distinct ou mÃªme personne avec deux rÃ´les selon modÃ¨le plateforme).

---

## 3. Dashboard exposant : capacitÃ©s et livrables

### 3.1 Vue dâ€™ensemble

| Bloc | Contenu |
|------|---------|
| **Candidatures** | Liste des candidatures (en attente, validÃ©es, refusÃ©es) par Ã©dition ; accÃ¨s au dÃ©tail et aux piÃ¨ces jointes. |
| **Participations** | Liste des Ã©ditions auxquelles lâ€™exposant participe (validÃ©) ; accÃ¨s aux documents, emplacement, programme de lâ€™Ã©dition. |
| **Agenda** | Calendrier cross-Ã©vÃ©nements : dates des Ã©vÃ©nements (candidat ou inscrit) ; alerte conflits de dates. |
| **Documents** | Documents reÃ§us ou Ã  renvoyer par Ã©dition (contrats, rÃ¨glements, conventions). |
| **Factures** | Devis et factures (Miyuinvoice) par Ã©dition ; statut de paiement, tÃ©lÃ©chargement. |

### 3.2 Candidatures

- **DÃ©pÃ´t** : formulaire de candidature par Ã©dition (champs dÃ©finis par lâ€™organisateur).
- **PiÃ¨ces jointes** : upload de documents (fiche entreprise, logo, etc.).
- **Statuts** : en attente, validÃ©e, refusÃ©e ; notification (Miyunotify) selon paramÃ©trage organisateur.
- **VÃ©rification agenda** : avant validation cÃ´tÃ© organisateur ou Ã  la soumission, la plateforme peut signaler un conflit de dates avec une autre Ã©dition Ã  laquelle lâ€™exposant est dÃ©jÃ  inscrit ou candidat.

### 3.3 Participations (Ã©ditions validÃ©es)

- **Fiche par Ã©dition** : rÃ©sumÃ© (dates, lieu, statut), lien vers les documents et la facturation.
- **Emplacement** : stand ou zone attribuÃ© (lien vers plan de salle si exposÃ© par lâ€™organisateur).
- **Programme** : accÃ¨s au programme public de lâ€™Ã©dition si mis Ã  disposition.

### 3.4 Documents et facturation

- **Documents** : consultation et tÃ©lÃ©chargement des contrats, rÃ¨glements, conventions ; envoi des documents signÃ©s ou complÃ©tÃ©s selon workflow organisateur.
- **Devis et factures** (Miyuinvoice) : consultation, tÃ©lÃ©chargement PDF, suivi du statut de paiement (payÃ© / en attente).

### 3.5 RÃ©pertoire des exposants

- **VisibilitÃ©** : la fiche exposant (entreprise, contact, Ã©ditions participÃ©es, etc.) peut Ãªtre publiÃ©e dans le **rÃ©pertoire des exposants** du catalogue (annuaire), selon la politique plateforme et les choix de lâ€™organisateur.
- **BÃ©nÃ©fice** : mise en visibilitÃ© pour les visiteurs et les autres organisateurs.

---

## 4. Limites et gouvernance

| Aspect | RÃ¨gle |
|--------|--------|
| **Candidatures** | Lâ€™exposant ne peut pas modifier les paramÃ¨tres des Ã©ditions ; il dÃ©pose une candidature et attend la dÃ©cision de lâ€™organisateur (StrongFather, validation). |
| **Agenda** | La plateforme signale ou bloque les conflits de dates ; lâ€™exposant reste responsable de la cohÃ©rence de son planning. |
| **DonnÃ©es** | Les donnÃ©es exposant (fiche, candidatures, factures) sont souveraines et protÃ©gÃ©es ; accÃ¨s restreint selon Mandat et rÃ´le (Master Butler). |
| **Facturation** | Ã‰mise par lâ€™organisateur via Miyuinvoice ; lâ€™exposant consulte et paie selon les modalitÃ©s de lâ€™Ã©dition. |

---

## 5. RÃ©fÃ©rences

- [Document fondateur Miyukini Festival Service](..//..//_index.md) â€” Â§ 5 Distribution exposants
- [Public Organisateurs](../Organisateurs/_index.md) | [Public Visiteurs](../Visiteurs/_index.md) | [Utilisateur non connectÃ©](../UtilisateurNonConnecte/_index.md)

