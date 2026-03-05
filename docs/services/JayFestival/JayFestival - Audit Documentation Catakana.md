# JayFestival â€” Audit documentation vs projet Catakana

## Contexte

Ce document constitue lâ€™**audit de la documentation JayFestival** par rapport au projet **Catakana** (.Catakana) : qualitÃ© de la transcription/traduction des fonctionnalitÃ©s Catakana vers JayFestival, couverture des toolkits, opÃ©rateurs, services imbriquÃ©s, et niveau de dÃ©tail conceptuel, dÃ©taillÃ© et de guidage/bornage pour lâ€™implÃ©mentation.

**RÃ©fÃ©rences** : [Liste des fonctionnalitÃ©s Catakana](..//..//..//README.md), [APPLICATION_COMPLETE_DOCUMENTATION Catakana](..//..//..//README.md), documentation JayFestival (Document fondateur, publics Organisateurs / Exposants / Visiteurs / Utilisateur non connectÃ©, rÃ©fÃ©rence InterpolaritÃ©).

## PortÃ©e / Scope

- **PÃ©rimÃ¨tre** : Comparaison fonctionnalitÃ©s Catakana â†” documentation JayFestival ; toolkits, opÃ©rateurs, services imbriquÃ©s ; qualitÃ© conceptuelle, dÃ©taillÃ©e et bornage implÃ©mentation.
- **Hors pÃ©rimÃ¨tre** : Code source Catakana, implÃ©mentation effective des crates Miyukini.

---

## 1. SynthÃ¨se des mÃ©triques (%)

| CritÃ¨re | Score | Commentaire |
|--------|-------|-------------|
| **Transcription/traduction des fonctionnalitÃ©s Catakana â†’ JayFestival** | **88 %** | Couverture trÃ¨s bonne ; manques : journal des modifications (programme), commentaires internes/notes privÃ©es exposants, import Google Sheet explicite, modules complÃ©mentaires (matÃ©riel, reporting, urgences). |
| **Documentation des toolkits nÃ©cessaires** | **92 %** | Toolkits identifiÃ©s et dÃ©crits par public (Organisateurs 9, Exposants 6, Visiteurs 5+) ; outils agrÃ©gÃ©s en exemples ; pas de contrat formel par outil. |
| **Documentation des opÃ©rateurs** | **90 %** | OpÃ©rateurs clairement nommÃ©s (JayFestival Organisateur, JayFestival Ã‰dition, MFS Exposant, JayFestival Candidatures, JayFestival Visiteur) ; lien opÃ©rateur â†” livrables prÃ©sent ; MFS non dÃ©fini dans le glossaire. |
| **Services imbriquÃ©s et interpolaritÃ©** | **95 %** | RÃ©fÃ©rence InterpolaritÃ© complÃ¨te (JayXpose, JayFaim, JayKoa, JayKonta) ; Document fondateur et besoins citent Miyauth, Miyuprofile, Miyunotify, Miyuinvoice, MiyuClock, Miyubooking, Miyucms, Miyumedia, Miyucptaledger, Miyuexpense, Miyucomptareports. |
| **Documentation conceptuelle (niveau fondateur)** | **95 %** | Vision, B2B2C, Store, comptes cross-Ã©vÃ©nements, hÃ©ritage Catakana, macro, distribution, politique rÃ©sidence donnÃ©es ; alignement Glossaire Miyukini. |
| **Documentation dans le dÃ©tail (besoins, Ã©crans, parcours)** | **93 %** | Besoins numÃ©rotÃ©s (ORG/EXP/VIS/UNC), critÃ¨res dâ€™acceptation, user stories, NFR, cas limites, MoSCoW ; Ã©crans et cycle par public ; quelques besoins Catakana non dÃ©taillÃ©s (voir Â§ 2). |
| **Guidage / bornage de lâ€™implÃ©mentation** | **55 %** | Pas de document Â« Bornage Implementation Â» dÃ©diÃ© (contrairement Ã  JayKoa) ; besoins et critÃ¨res dâ€™acceptation servent de bornes ; pas de phases MVP/phase 2, pas de hors scope explicite par phase, pas de critÃ¨res de livraison formalisÃ©s. |

**Score global pondÃ©rÃ© (qualitÃ© doc vs Catakana + implÃ©mentation)** : **â‰ˆ 85 %**.

---

## 2. Transcription/traduction des fonctionnalitÃ©s Catakana â†’ JayFestival

### 2.1 FonctionnalitÃ©s de la Â« Liste des fonctionnalitÃ©s Â» Catakana

| Bloc Catakana | Ã‰lÃ©ment | JayFestival | Statut |
|---------------|---------|-------------|--------|
| **Authentification Orga** | Comptes organisateurs, rÃ´les (bÃ©nÃ©vole, exposant, manager, admin) | Compte organisateur, rÃ´les Admin / Manager / BÃ©nÃ©vole ; Exposant = compte distinct | âœ… Transcrit (terminologie COG) |
| | Connexion email/mdp ou lien magique | ORG-01, Miyauth, lien magique citÃ© | âœ… |
| | Protection des routes (middleware) | Mandat, Master Butler, NFR-ORG-07 | âœ… |
| | Interface gestion des rÃ´les (admin) | ORG-46, ORG-47, ORG-48, Ã‰quipe & Permissions | âœ… |
| **Exposants** | Liste filtrable | ORG-12, EXP-05 Ã  EXP-08 | âœ… |
| | Fiche exposant dÃ©taillÃ©e (contact, statut, historique) | ORG-15, EXP-17, fiche par Ã©dition | âœ… |
| | Ajout manuel ou import Google Sheet / CSV | ORG-18 import CSV/tableur | âš ï¸ Google Sheet non explicite |
| | Changement de statut (en attente, validÃ©, refusÃ©) | ORG-13, ORG-14, EXP-12 | âœ… |
| | Commentaires internes ou notes privÃ©es | â€” | âŒ Non documentÃ© |
| | TÃ©lÃ©versement de documents par exposant | EXP-21 (documents signÃ©s/complÃ©tÃ©s) | âœ… |
| **Devis & Factures** | GÃ©nÃ©ration devis, conversion en facture, PDF, historique, envoi email, marquage payÃ©/en attente | ORG-16, ORG-17, EXP-22 Ã  EXP-24, Miyuinvoice / JayKonta | âœ… |
| **Plan & Emplacement** | Plan interactif, attribution (drag & drop ou formulaire), zones, tailles, lÃ©gende, export, zones techniques | ORG-19 Ã  ORG-22, Kit Plan de salle | âœ… |
| **Programme** | Animations, association scÃ¨ne/salle/lieu, horaires, chevauchement bloquÃ© | ORG-23 Ã  ORG-26, Kit Programme | âœ… |
| | Vue chronologique ou par salle, filtres | ORG-25 | âœ… |
| | Ã‰dition rapide du programme en live | â€” | âš ï¸ Non explicite |
| | Journal des modifications | â€” | âŒ Non documentÃ© |
| **Documents & LÃ©gal** | Contrats types, envoi Ã  signer, historique, accÃ¨s par rÃ´le | ORG-32 Ã  ORG-34, Kit Documents & LÃ©gal | âœ… |
| **Notifications** | Annonces globales, notifications ciblÃ©es, journal messages, rÃ©daction/planification alerte | ORG-35 Ã  ORG-37, Miyunotify | âœ… |
| **Modules complÃ©mentaires** | Gestion matÃ©riel prÃªtÃ©/placÃ© | â€” | âŒ Non documentÃ© |
| | Gestion bÃ©nÃ©voles (planning, Ã©quipes) | ORG-47, ORG-48 (zones, crÃ©neaux) | âš ï¸ Partiel |
| | Outils de reporting (participation, paiements, retards) | ORG-30 balance, exports ; pas de reporting dÃ©diÃ© | âš ï¸ Partiel |
| | Suivi interventions techniques ou urgences | â€” | âŒ Non documentÃ© |

### 2.2 Ã‰lÃ©ments Catakana (APPLICATION_COMPLETE) non transcrits ou hors scope

| Ã‰lÃ©ment Catakana | PrÃ©sence JayFestival | Remarque |
|------------------|----------------------|----------|
| Module ActualitÃ©s (News) | Annonces (ORG-35, ORG-36) ; pas de flux Â« ActualitÃ©s Â» Ã©ditorial public | Peut relever de Miyucms / Miyufeeds ; non formalisÃ© pour JayFestival |
| InvitÃ©s (Guests), prestations, rÃ©servations crÃ©neaux | Non documentÃ© comme bloc dÃ©diÃ© | Peut Ãªtre couvert par Â« Services visiteur Â» (ateliers, crÃ©neaux) ; Ã  prÃ©ciser |
| RPG (stats, inventaire) | Non documentÃ© | Hors scope ou Ã  traiter dans un autre service |
| Galeries par Ã©dition | Non documentÃ© | Peut relever de Miyumedia ; non formalisÃ© |
| RÃ©servation de stands avec tarifs types (stand intÃ©rieur/extÃ©rieur/restauration, options) | Devis/facture (Miyuinvoice) ; plan de salle avec attribution | Tarifs types et options (chaises, tables, etc.) dans le dÃ©tail des devis, pas en tant que catalogue de stands tarifÃ©s |
| Liste des emplacements (statut libre/rÃ©servÃ©/occupÃ©) | Plan de salle, stands, attribution | Couvert conceptuellement ; dÃ©tail Â« statut temps rÃ©el Â» Ã  confirmer |

---

## 3. Toolkits, opÃ©rateurs, services imbriquÃ©s

### 3.1 Toolkits documentÃ©s

- **Organisateurs** : Ã‰ditions, Exposants (cÃ´tÃ© organisateur), Plan de salle, Programme, Budget, Documents & LÃ©gal, Services visiteur, Publication catalogue, Ã‰quipe & Permissions â€” **9 toolkits**, avec exemples dâ€™outils agrÃ©gÃ©s.
- **Exposants** : Candidatures Exposant, Participations & Ã‰ditions, Agenda cross-Ã©vÃ©nements, Documents Exposant, Facturation Exposant (Miyuinvoice), RÃ©pertoire Exposants â€” **6 toolkits**.
- **Visiteurs** : Agenda Visiteur, Billets & RÃ©servations, Pass VIP, Suivi dâ€™activitÃ©s, etc. â€” **5+ toolkits** (dÃ©crits dans Visiteurs - Operateurs et Toolkits).

**Score** : Couverture trÃ¨s bonne ; manque une liste consolidÃ©e Â« Tous les toolkits JayFestival Â» et des contrats dâ€™outils formels (nom, paramÃ¨tres, garanties).

### 3.2 OpÃ©rateurs documentÃ©s

- **Organisateurs** : JayFestival Organisateur, JayFestival Ã‰dition.
- **Exposants** : MFS Exposant, JayFestival Candidatures (Â« MFS Â» non dÃ©fini dans le glossaire).
- **Visiteurs** : JayFestival Visiteur (ou Ã©quivalent).
- **Catalogue / UNC** : FaÃ§ade publique gouvernÃ©e, pas dâ€™opÃ©rateur nommÃ© spÃ©cifiquement.

### 3.3 Services imbriquÃ©s et interpolaritÃ©

- **Document [JayFestival - Interpolarite Services Jay](reference/JayFestival%20-%20Interpolarite%20Services%20Jay.md)** : JayXpose (fiche/rÃ©pertoire exposants), JayFaim (restauration Ã©vÃ©nement), JayKoa (agenda, conflits de dates), JayKonta (budget, devis/factures).
- **Document fondateur et besoins** : Miyauth, Miyuprofile, Miyunotify, Miyuinvoice, MiyuClock, Miyubooking, Miyucms, Miyumedia, Miyucptaledger, Miyuexpense, Miyucomptareports, StrongFather, Master Butler, KindMother, WorrySentinel.

**Score** : TrÃ¨s bon ; rÃ©fÃ©rence centralisÃ©e et cohÃ©rente.

---

## 4. QualitÃ© conceptuelle, dÃ©taillÃ©e et bornage implÃ©mentation

### 4.1 Niveau conceptuel

- Document fondateur : raison dâ€™Ãªtre, vision, B2B2C, Store, comptes cross-Ã©vÃ©nements, hÃ©ritage Catakana, macro, distribution (organisateurs, exposants, visiteurs), politique de rÃ©sidence des donnÃ©es sensibles.
- Par public : Parcours, capacitÃ©s et livrables ; Analyse des besoins ; Ã‰crans et cycle ; OpÃ©rateurs et Toolkits.
- Terminologie alignÃ©e sur le Glossaire Miyukini (OpÃ©rateur, Mandat, Kit dâ€™outils, etc.).

**Score** : 95 %.

### 4.2 Niveau dÃ©taillÃ©

- Besoins numÃ©rotÃ©s avec critÃ¨res dâ€™acceptation (ORG-xx, EXP-xx, VIS-xx, UNC-xx).
- User stories, pain points, opportunitÃ©s, MoSCoW, NFR, cas limites, mÃ©triques de succÃ¨s.
- Ã‰crans listÃ©s avec phase, objectif, organisation, besoins, navigation.

**Score** : 93 %.

### 4.3 Guidage / bornage implÃ©mentation

- **PrÃ©sent** : Besoins et critÃ¨res dâ€™acceptation utilisables comme bornes ; prioritÃ©s MoSCoW ; dÃ©pendances entre publics et avec les services Jay.
- **Absent** : Document dÃ©diÃ© Â« Bornage Implementation Â» (in scope / hors scope par phase, critÃ¨res de livraison, dÃ©pendances techniques formalisÃ©es) comme pour JayKoa ; pas de phases MVP / phase 2 explicites ; pas de liste Â« hors scope v1 Â» consolidÃ©e.

**Score** : 55 %.

---

## 5. Recommandations

1. ~~**CrÃ©er un document Â« JayFestival - Bornage Implementation Â»**~~ **Fait** : [JayFestival - Bornage Implementation](./JayFestival%20-%20Bornage%20Implementation.md) ; pÃ©rimÃ¨tre MVP / phase 2, hors scope, dÃ©pendances, critÃ¨res de livraison.
2. **ComplÃ©ter les manques de transcription** : commentaires internes / notes privÃ©es sur exposants (besoin ORG ou EXP) ; journal des modifications du programme (optionnel) ; mention explicite de lâ€™import depuis Google Sheet (ou dÃ©cision de ne pas supporter).
3. **PrÃ©ciser le sigle Â« MFS Â»** (ex. Miyukini Festival Service ou Ã©quivalent) dans le glossaire ou dans le document fondateur JayFestival.
4. **Documenter ou dÃ©cider** : module ActualitÃ©s (News) public (Miyufeeds / Miyucms vs Annonces) ; gestion matÃ©riel, reporting avancÃ©, interventions techniques/urgences (hors scope v1 ou backlog).
5. **Ajouter une rÃ©fÃ©rence Â« Niveaux SÃ©curitÃ© et Protection DonnÃ©es Â»** pour JayFestival (sur le modÃ¨le JayKonta / JayKoa) si nÃ©cessaire pour les audits et la conformitÃ©.
6. **RÃ©fÃ©rence UI et implÃ©mentation** : [JayFestival - Reference UI Transcription Catakana](./JayFestival%20-%20Reference%20UI%20Transcription%20Catakana.md) retranscrit lâ€™UI complÃ¨te Catakana (Atomic, thÃ¨me, ui-kit, Ã©crans) dans la stack actuelle (Dioxus) pour prÃ©parer lâ€™implÃ©mentation.

---

## 6. RÃ©fÃ©rences

- [JayFestival - Document Fondateur](./JayFestival%20-%20Document%20Fondateur.md)
- [JayFestival - Bornage Implementation](./JayFestival%20-%20Bornage%20Implementation.md)
- [JayFestival - Reference UI Transcription Catakana](./JayFestival%20-%20Reference%20UI%20Transcription%20Catakana.md)
- [JayFestival - Interpolarite Services Jay](./reference/JayFestival%20-%20Interpolarite%20Services%20Jay.md)
- [JayKoa - Bornage Implementation](../JayKoa/JayKoa%20-%20Bornage%20Implementation.md)
- [Liste des fonctionnalitÃ©s Catakana](..//..//..//README.md)
- Documentation publics : [Organisateurs](./publics/Organisateurs/_index.md), [Exposants](./publics/Exposants/_index.md), [Visiteurs](./publics/Visiteurs/_index.md), [Utilisateur non connectÃ©](./publics/UtilisateurNonConnecte/_index.md)

---

**Document** : JayFestival â€” Audit documentation vs projet Catakana  
**Version** : 1.0  
**Date** : 2026-02-03  
**Statut** : Rapport dâ€™audit â€” mÃ©triques et recommandations.

