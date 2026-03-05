# JayFestival â€” Ã‰tat de la documentation des services interfacÃ©s

## Contexte

Ce document fait le point sur la **documentation** de tous les **services interfacÃ©s avec JayFestival** (services Jay, outils Miyu*, Cores), en vue dâ€™une **implÃ©mentation complÃ¨te incluant lâ€™UI**. Pour chaque service, il indique ce qui existe, ce qui manque, et les **ambiguÃ¯tÃ©s ou choix humains** Ã  trancher.

**RÃ©fÃ©rence** : [JayFestival - Interpolarite Services Jay](./JayFestival%20-%20Interpolarite%20Services%20Jay.md) pour les couplages cÃ´tÃ© JayFestival.

## PortÃ©e / Scope

- **PÃ©rimÃ¨tre** : Services Jay (JayKoa, JayXpose, JayFaim, JayKonta), outils Miyu* (Miyauth, Miyuprofile, Miyunotify, Miyuinvoice, Miyubooking, Miyucms, Miyumedia, Miyufeeds, MiyuClock), Cores (StrongFather, KindMother, Master Butler, WorrySentinel), backend alpha (Supabase).
- **CritÃ¨re Â« prÃªt pour implÃ©mentation complÃ¨te UI incluse Â»** : Document fondateur ou Ã©quivalent ; OpÃ©rateurs / Kits ou contrats dâ€™intÃ©gration ; parcours ou Ã©crans dÃ©crits ; points dâ€™entrÃ©e UI ou patterns dâ€™intÃ©gration documentÃ©s lorsque lâ€™UI est exposÃ©e ou intÃ©grÃ©e dans JayFestival.
- **Hors pÃ©rimÃ¨tre** : ImplÃ©mentation effective du code (ce document ne fait que lâ€™audit de la doc).

---

## 1. SynthÃ¨se par service

| Service | Doc existante | PrÃªt implÃ©mentation complÃ¨te (UI incluse) | Manques principaux |
|---------|----------------|-------------------------------------------|---------------------|
| **JayKoa** | Oui (fondateur, OpÃ©rateurs, Parcours, Ecrans et UI, Integration Consommateurs, Bornage, Maquettes) | **Oui** | â€” |
| **JayKonta** | Oui (fondateur, publics Account/Purse, Integration Services, Points dâ€™entrÃ©e) | **Partiel** | Pas de doc Â« Ecrans et UI Â» dÃ©diÃ©e ; UI Ã  dÃ©duire des Parcours. |
| **JayXpose** | Document fondateur uniquement | **Non** | OpÃ©rateurs/Kits, Parcours, Ecrans/UI, Contrat dâ€™intÃ©gration JayFestival. |
| **JayFaim** | Document fondateur uniquement | **Non** | OpÃ©rateurs/Kits, Parcours, Ecrans/UI, Contrat dâ€™intÃ©gration JayFestival. |
| **Miyauth** | Doc fondatrice, Reference Outils, contrats, Implementation Guidelines, Audit | **Oui** (cÃ´tÃ© consommation) | Pas dâ€™Ã©crans propres ; JayFestival consomme auth â†’ Ã©crans Connexion/Inscription dans JayFestival. |
| **Miyuprofile** | _index + rÃ©fÃ©rences Ã©parses | **Oui** (alpha) | **P1 tranchÃ©** : Supabase uniquement pour le moment (source de vÃ©ritÃ© = tables Supabase). |
| **Miyunotify** | Doc fondatrice, Reference Outils, contrats, Implementation Guidelines | **Oui** (cÃ´tÃ© consommation) | Pas dâ€™Ã©cran propre ; JayFestival dÃ©clenche envois â†’ pas dâ€™UI dÃ©diÃ©e cÃ´tÃ© JayFestival. |
| **Miyuinvoice** | Doc fondatrice, Reference Outils, contrats, Implementation Guidelines | **Oui** (cÃ´tÃ© consommation) | UI devis/factures dans JayFestival selon spec UI Catakana ; couplage JayKonta Ã  clarifier. |
| **Miyubooking** | Doc fondatrice, Reference Outils, contrats, Implementation Guidelines | **Partiel** | Parcours/Ã©crans rÃ©servation dans JayFestival Ã  aligner avec Miyubooking. |
| **Miyucms / Miyumedia** | Docs fondatrices, Reference Outils, contrats | **Partiel** | Contrat dâ€™intÃ©gration Â« documents dâ€™Ã©dition Â» depuis JayFestival Ã  formaliser. |
| **Miyufeeds** | Doc fondatrice | **Partiel** | Usage Â« ActualitÃ©s Â» public (phase 2) Ã  borner. |
| **MiyuClock** | Doc fondatrice, Reference Outils, contrats (KindMother, Security, etc.) | **Oui** (cÃ´tÃ© consommation) | **P1 tranchÃ©** : MiyuClock **atteste l'horaire et la date IRL** ; JayKoa organise les donnÃ©es et fait l'interface utilisateur. |
| **KindMother / StrongFather / Master Butler / WorrySentinel** | Contrats COG (core) | **Oui** (cÃ´tÃ© gouvernance) | Pas dâ€™UI propre ; intÃ©gration dans JayFestival via Mandats et persistance. |
| **Supabase** | RÃ©fÃ©rence Base de Donnees et Migration | **Oui** (alpha) | Exception prÃ©-COG ; pas dâ€™Ã©cran Â« Supabase Â» â€” backend uniquement. |

---

## 2. DÃ©tail par service

### 2.1 JayKoa (agenda)

| Ã‰lÃ©ment | Existant | Commentaire |
|---------|----------|-------------|
| Document fondateur | Oui | [JayKoa - Document Fondateur](../../JayKoa/JayKoa%20-%20Document%20Fondateur.md) |
| OpÃ©rateurs et Toolkits | Oui | [JayKoa - Operateurs et Toolkits](../../JayKoa/JayKoa%20-%20Operateurs%20et%20Toolkits.md) |
| Parcours utilisateurs | Oui | [JayKoa - Parcours Utilisateurs](../../JayKoa/JayKoa%20-%20Parcours%20Utilisateurs.md) |
| Ecrans et UI | Oui | [JayKoa - Ecrans et UI](../../JayKoa/JayKoa%20-%20Ecrans%20et%20UI.md) â€” composants (calendrier, alerte conflit, export) et intÃ©gration dans UIs consommatrices |
| Integration Services Consommateurs | Oui | [JayKoa - Integration Services Consommateurs](../../JayKoa/reference/JayKoa%20-%20Integration%20Services%20Consommateurs.md) â€” types dâ€™entrÃ©es JayFestival, responsabilitÃ©s |
| Maquettes UI | Oui | [JayKoa - Maquettes UI Type Google Agenda](../../JayKoa/reference/JayKoa%20-%20Maquettes%20UI%20Type%20Google%20Agenda.md) |
| Bornage | Oui | [JayKoa - Bornage Implementation](../../JayKoa/JayKoa%20-%20Bornage%20Implementation.md) |

**PrÃªt pour implÃ©mentation complÃ¨te UI incluse** : Oui. JayFestival doit intÃ©grer les composants/patterns dÃ©crits dans Â« Ecrans et UI Â» (vue calendrier, alerte conflit, export) dans ses Ã©crans Exposant/Visiteur/Organisateur. **DÃ©cision P1 (tranchÃ©e)** : **JayKoa organise les donnÃ©es et fait lâ€™interface avec lâ€™utilisateur** ; **MiyuClock atteste lâ€™horaire et la date IRL** (rÃ©fÃ©rentiel temps rÃ©el). Voir [InterpolaritÃ© Services Jay](./JayFestival%20-%20Interpolarite%20Services%20Jay.md) et rÃ©fÃ©rence MiyuClock.

---

### 2.2 JayKonta (budget, devis, facturation)

| Ã‰lÃ©ment | Existant | Commentaire |
|---------|----------|-------------|
| Document fondateur | Oui | [JayKonta - Document Fondateur](../../JayKonta/JayKonta%20-%20Document%20Fondateur.md) |
| Publics (Account, Purse) | Oui | Analyse des besoins, OpÃ©rateurs et Toolkits, Parcours par public |
| Integration Services | Oui | [JayKonta - Integration Services](../../JayKonta/reference/JayKonta%20-%20Integration%20Services.md) â€” flux JayFestival (devis, factures, budget) |
| Points dâ€™entrÃ©e | Oui | [JayKonta - Points Entree JayBudget et JayKonta](../../JayKonta/reference/JayKonta%20-%20Points%20Entree%20JayBudget%20et%20JayKonta.md) |
| Ecrans et UI | Non | Aucun document Â« Ecrans et UI Â» dÃ©diÃ© ; lâ€™UI est dÃ©crite dans les Parcours (Account, Purse). |

**PrÃªt pour implÃ©mentation complÃ¨te UI incluse** : Partiel. Les flux et opÃ©rateurs sont documentÃ©s ; lâ€™UI budget/devis/factures dans JayFestival est couverte par la [Specification UI Conforme Catakana](../JayFestival%20-%20Specification%20UI%20Conforme%20Catakana.md) (Ã©crans ORG-E12, ORG-E13, EXP-E13, etc.). Pour une UI **JayKonta native** (hors JayFestival), il manque une spec Ã©crans dÃ©diÃ©e.

**DÃ©cision P0 (tranchÃ©e)** : **Miyuinvoice + JayKonta** â€” facturation exposants = Miyuinvoice en faÃ§ade avec JayKonta en backend (devis, factures, encaissements).

---

### 2.3 JayXpose (profil exposant, vitrine)

| Ã‰lÃ©ment | Existant | Commentaire |
|---------|----------|-------------|
| Document fondateur | Oui | [JayXpose - Document Fondateur](../../JayXpose/JayXpose%20-%20Document%20Fondateur.md) |
| OpÃ©rateurs et Toolkits | Non | Non documentÃ©. |
| Parcours / publics | Non | Non documentÃ©. |
| Ecrans et UI | Non | Non documentÃ©. |
| Contrat dâ€™intÃ©gration JayFestival | Non | Lâ€™interpolaritÃ© est dÃ©crite (fiche exposant, rÃ©pertoire) mais pas le contrat (API, donnÃ©es exposÃ©es, champs requis). |

**PrÃªt pour implÃ©mentation complÃ¨te UI incluse** : Non. Impossible dâ€™implÃ©menter lâ€™intÃ©gration Â« fiche exposant / rÃ©pertoire Â» cÃ´tÃ© JayFestival sans : (1) OpÃ©rateurs/Kits JayXpose, (2) Contrat dâ€™intÃ©gration (quels champs JayFestival lit, comment lier exposant â†” vitrine), (3) Optionnel : Ã©crans/UI vitrine si JayFestival affiche un bloc Â« Vitrine Â» ou lien vers vitrine.

**Choix humain** : En alpha JayFestival, la fiche exposant peut rester **locale** (donnÃ©es Catakana/JayFestival uniquement) sans JayXpose ; documenter explicitement Â« JayXpose = phase 2 ou optionnel alpha Â».

---

### 2.4 JayFaim (restauration sur Ã©vÃ©nement)

| Ã‰lÃ©ment | Existant | Commentaire |
|---------|----------|-------------|
| Document fondateur | Oui | [JayFaim - Document Fondateur](../../JayFaim/JayFaim%20-%20Document%20Fondateur.md) |
| OpÃ©rateurs et Toolkits | Non | Non documentÃ©. |
| Parcours / Ecrans et UI | Non | Non documentÃ©. |
| Contrat dâ€™intÃ©gration JayFestival | Non | Orchestration dÃ©crite (crÃ©neaux, commandes, paiement) mais pas le contrat (qui appelle qui, donnÃ©es partagÃ©es). |

**PrÃªt pour implÃ©mentation complÃ¨te UI incluse** : Non. JayFaim est marquÃ© **phase 2 ou optionnel** dans le Bornage JayFestival ; pour une implÃ©mentation complÃ¨te ultÃ©rieure, il faudra : OpÃ©rateurs/Kits JayFaim, Parcours, Ecrans/UI, Contrat dâ€™intÃ©gration avec JayFestival (et JayKonta).

**Choix humain** : Confirmer que JayFaim est **hors scope alpha** ; documenter la roadmap (phase 2) pour ne pas bloquer lâ€™alpha.

---

### 2.5 Miyauth (authentification)

| Ã‰lÃ©ment | Existant | Commentaire |
|---------|----------|-------------|
| Documentation fondatrice | Oui | [MiyuAuth - Documentation Fondatrice](../../../tools/MiyuAuth/MiyuAuth%20-%20Documentation%20Fondatrice.md) |
| Reference Outils | Oui | [MiyuAuth - Reference Outils](../../../tools/MiyuAuth/MiyuAuth%20-%20Reference%20Outils.md) |
| Contrats (governance, security, KindMother, etc.) | Oui | Plusieurs contrats sous `tools/MiyuAuth/contracts/` |
| Implementation Guidelines | Oui | [MiyuAuth - Reference Implementation Guidelines](../../../tools/MiyuAuth/implementation/MiyuAuth%20-%20Reference%20Implementation%20Guidelines.md) |
| Ecrans Connexion/Inscription | â€” | PortÃ©s par **JayFestival** (UNC-E12, UNC-E13, ORG-E02, ORG-E03, etc.) ; Miyauth fournit lâ€™auth, pas lâ€™UI. |

**PrÃªt pour implÃ©mentation complÃ¨te UI incluse** : Oui pour la **consommation** depuis JayFestival. Lâ€™UI (formulaires Connexion, Inscription) est dans le pÃ©rimÃ¨tre JayFestival (Specification UI Conforme Catakana) ; Miyauth est appelÃ© en backend. JayFestival a une Auth Ã  lui, dÃ©rivÃ©e de lâ€™Auth Catakana qui utilise Supabase Auth ; en alpha, cette Auth JayFestival sâ€™appuie sur Supabase Auth (exception prÃ©-COG).

**AmbiguÃ¯tÃ©** : En alpha, Supabase Auth est utilisÃ© ; la **bascule vers Miyauth** (COG-native) nâ€™est pas datÃ©e â€” Ã  documenter dans la roadmap post-alpha.

---

### 2.6 Miyuprofile (profil utilisateur)

| Ã‰lÃ©ment | Existant | Commentaire |
|---------|----------|-------------|
| Documentation | _index + rÃ©fÃ©rences Ã©parses (Document fondateur JayFestival, publics) | Pas de dossier dÃ©diÃ© Â« Miyuprofile Â» avec Doc fondatrice au mÃªme niveau que MiyuAuth. |
| Profil organisateur / exposant / visiteur | CitÃ© dans JayFestival (Bornage, Document fondateur) | Fiche organisateur, fiche exposant, profil visiteur â€” partie dans JayFestival, partie potentiellement Miyuprofile. |

**PrÃªt pour implÃ©mentation complÃ¨te UI incluse** : Partiel. Les Ã©crans Â« Mon compte Â», Â« Fiche entreprise Â» sont dÃ©crits dans les Ã‰crans et cycle JayFestival ; la frontiÃ¨re **donnÃ©es profil Miyuprofile vs donnÃ©es locales JayFestival** nâ€™est pas formalisÃ©e.

**Choix humain** : (1) OÃ¹ se trouve la source de vÃ©ritÃ© du profil (Miyuprofile vs tables JayFestival/Supabase) ? (2) CrÃ©er ou non un document fondateur / OpÃ©rateurs Miyuprofile et un contrat dâ€™intÃ©gration JayFestival â†” Miyuprofile.

---

### 2.7 Miyunotify (annonces, notifications)

| Ã‰lÃ©ment | Existant | Commentaire |
|---------|----------|-------------|
| Documentation fondatrice | Oui | [MiyuNotify - Documentation Fondatrice](../../../tools/MiyuNotify/MiyuNotify%20-%20Documentation%20Fondatrice.md) |
| Reference Outils | Oui | [MiyuNotify - Reference Outils](../../../tools/MiyuNotify/MiyuNotify%20-%20Reference%20Outils.md) |
| Contrats, Implementation Guidelines | Oui | Sous `tools/MiyuNotify/` |
| UI dans JayFestival | â€” | JayFestival **dÃ©clenche** les envois (annonces, notifications candidature, etc.) ; pas dâ€™Ã©cran Â« Miyunotify Â» propre dans JayFestival â€” les Ã©crans sont Â« Annonces et notifications Â» (ORG-E23), Â« Notifications et prÃ©fÃ©rences Â» (EXP-E19), etc., avec appels Ã  Miyunotify. |

**PrÃªt pour implÃ©mentation complÃ¨te UI incluse** : Oui pour la consommation. Les Ã©crans cÃ´tÃ© JayFestival sont documentÃ©s (Specification UI, Ã‰crans et cycle) ; Miyunotify est un outil appelÃ© en backend. Contrat dâ€™intÃ©gration (payload, Ã©vÃ©nements) Ã  vÃ©rifier dans Miyunotify Reference Outils / contrats.

---

### 2.8 Miyuinvoice (devis, factures)

| Ã‰lÃ©ment | Existant | Commentaire |
|---------|----------|-------------|
| Documentation fondatrice | Oui | [MiyuInvoice - Documentation Fondatrice](../../../tools/MiyuInvoice/MiyuInvoice%20-%20Documentation%20Fondatrice.md) |
| Reference Outils, contrats, Implementation Guidelines | Oui | Sous `tools/MiyuInvoice/` |
| Couplage JayKonta | CitÃ© (Bornage : Â« Miyuinvoice / JayKonta Â») | RÃ´le respectif non tranchÃ© : JayKonta = service COG budget/facturation ; Miyuinvoice = outil ? faÃ§ade ? |

**PrÃªt pour implÃ©mentation complÃ¨te UI incluse** : Oui pour la consommation, sous rÃ©serve de clarifier Miyuinvoice vs JayKonta. Lâ€™UI devis/factures dans JayFestival (ORG-E12, ORG-E13, EXP-E13) est dans la Specification UI Conforme Catakana.

**Choix humain** : DÃ©cision : facturation exposants JayFestival = **JayKonta uniquement**, ou **Miyuinvoice** (qui sâ€™appuie sur JayKonta) ? Si Miyuinvoice est la faÃ§ade, documenter le flux JayFestival â†’ Miyuinvoice â†’ JayKonta.

---

### 2.9 Miyubooking (rÃ©servations, crÃ©neaux, billets, pass)

| Ã‰lÃ©ment | Existant | Commentaire |
|---------|----------|-------------|
| Documentation fondatrice | Oui | [MiyuBooking - Documentation Fondatrice](../../../tools/MiyuBooking/MiyuBooking%20-%20Documentation%20Fondatrice.md) |
| Reference Outils, contrats, Implementation Guidelines | Oui | Sous `tools/MiyuBooking/` |
| Ã‰crans JayFestival (rÃ©servations, billets, pass) | Oui (Ã‰crans et cycle Visiteurs, Organisateurs) | VIS-E06 Ã  VIS-E09, ORG-E24 (services visiteur) â€” parcours dÃ©crits. |

**PrÃªt pour implÃ©mentation complÃ¨te UI incluse** : Partiel. Les parcours et Ã©crans sont dans JayFestival ; le **contrat dâ€™intÃ©gration** (quels Kits Miyubooking sont appelÃ©s, pour quelles rÃ©servations/billets/pass) nâ€™est pas formalisÃ© dans un document unique Â« JayFestival â†” Miyubooking Â». Ã€ prÃ©ciser : crÃ©ation de crÃ©neaux, rÃ©servation atelier, Ã©mission billet, pass VIP.

**Recommandation** : RÃ©diger une section Â« JayFestival â†” Miyubooking Â» dans un doc dâ€™intÃ©gration (ou dans InterpolaritÃ©) : capacitÃ©s consommÃ©es, donnÃ©es Ã©changÃ©es, Ã©crans concernÃ©s.

---

### 2.10 Miyucms / Miyumedia (documents, mÃ©dias, actualitÃ©s)

| Ã‰lÃ©ment | Existant | Commentaire |
|---------|----------|-------------|
| Documentation | MiyuMedia Doc fondatrice, Reference Outils ; MiyuCMS idem | Sous `tools/MiyuMedia/`, `tools/MiyuCMS/` |
| Usage JayFestival | Documents dâ€™Ã©dition (contrats, rÃ¨glements), galeries (phase 2), actualitÃ©s (Miyufeeds/Miyucms) | CitÃ© dans Bornage, Document fondateur. |

**PrÃªt pour implÃ©mentation complÃ¨te UI incluse** : Partiel. Pas de **contrat dâ€™intÃ©gration** explicite Â« JayFestival â†” Miyucms/Miyumedia Â» (upload, stockage, lien document â†” Ã©dition, affichage dans Ã©crans Documents ORG-E22, EXP-E12). Pour phase 2 (galeries, actualitÃ©s), borner le pÃ©rimÃ¨tre.

**Choix humain** : En alpha, les documents dâ€™Ã©dition peuvent rester en **stockage local** (Supabase Storage ou tables) ; migration vers Miyucms/Miyumedia en phase 2 Ã  documenter.

---

### 2.11 Miyufeeds (flux actualitÃ©s)

| Ã‰lÃ©ment | Existant | Commentaire |
|---------|----------|-------------|
| Documentation fondatrice | Oui | [MiyuFeeds - Documentation Fondatrice](../../../tools/MiyuFeeds/MiyuFeeds%20-%20Documentation%20Fondatrice.md) |
| Usage JayFestival | Phase 2 â€” module ActualitÃ©s (News) public | Bornage : Â« Miyucms/Miyufeeds Â» pour actualitÃ©s ; annonces organisateur = Miyunotify en alpha. |

**PrÃªt pour implÃ©mentation complÃ¨te UI incluse** : Partiel (phase 2). DÃ©cision : actualitÃ©s Ã©ditoriales public = Miyufeeds en phase 2 ; pas bloquant pour alpha.

---

### 2.12 MiyuClock (horloge, agenda)

| Ã‰lÃ©ment | Existant | Commentaire |
|---------|----------|-------------|
| Documentation fondatrice | Oui | [MiyuClock - Documentation Fondatrice](../../../tools/MiyuClock/MiyuClock%20-%20Documentation%20Fondatrice.md) |
| Reference Outils, contrats (KindMother, Security, etc.) | Oui | Sous `tools/MiyuClock/` |
| RÃ´le vs JayKoa | CitÃ© (Document fondateur JayFestival : Â« MiyuClock, Miyubooking, donnÃ©es dâ€™Ã©dition Â» pour agenda cross-Ã©vÃ©nements) | JayKoa = intÃ©grateur des **dates** (entrÃ©es agenda, conflits, vues). MiyuClock = outil **horloge** / temps ? |

**PrÃªt pour implÃ©mentation complÃ¨te UI incluse** : Oui pour la consommation, si MiyuClock est clairement positionnÃ© (fuseaux, rÃ©fÃ©rentiel temps). **AmbiguÃ¯tÃ©** : MiyuClock vs JayKoa â€” partage des rÃ´les (qui gÃ¨re quoi) Ã  clarifier dans un document (ex. Â« Agenda cross-Ã©vÃ©nements : JayKoa + MiyuClock Â»). Les Ã©crans agenda dans JayFestival sâ€™appuient sur JayKoa (Ecrans et UI) ; MiyuClock peut Ãªtre utilisÃ© pour fuseaux / affichage temps.

---

### 2.13 Cores (StrongFather, KindMother, Master Butler, WorrySentinel)

| Ã‰lÃ©ment | Existant | Commentaire |
|---------|----------|-------------|
| Contrats COG | Oui | Sous `docs/core/` pour chaque Core. |
| UI | N/A | Pas dâ€™Ã©cran Â« Core Â» ; intÃ©gration dans JayFestival via Mandats, persistance, permissions, niveaux de sÃ©curitÃ©. |

**PrÃªt pour implÃ©mentation complÃ¨te UI incluse** : Oui (cÃ´tÃ© gouvernance). Lâ€™UI JayFestival respecte les dÃ©cisions des Cores ; pas de doc UI spÃ©cifique Â« Core Â» nÃ©cessaire pour JayFestival.

---

### 2.14 Supabase (backend alpha)

| Ã‰lÃ©ment | Existant | Commentaire |
|---------|----------|-------------|
| Reference Base de Donnees et Migration | Oui | [JayFestival - Reference Base de Donnees et Migration](./JayFestival%20-%20Reference%20Base%20de%20Donnees%20et%20Migration%20Supabase%20vers%20SQLite.md) â€” tables, RLS, mapping services. |
| UI | N/A | Backend uniquement ; pas dâ€™Ã©cran Supabase. |

**PrÃªt pour implÃ©mentation complÃ¨te UI incluse** : Oui pour lâ€™alpha (backend documentÃ©). Migration post-alpha vers SQLite + KindMother documentÃ©e dans le mÃªme doc.

---

## 3. AmbiguÃ¯tÃ©s et choix humains Ã  trancher

Les points suivants **nÃ©cessitent une dÃ©cision ou un arbitrage humain** pour finaliser la documentation et permettre une implÃ©mentation sans ambiguÃ¯tÃ©.

### 3.1 Facturation exposants : JayKonta vs Miyuinvoice

- **Constat** : Le Bornage et les docs citent Â« Miyuinvoice / JayKonta Â» pour devis et factures exposants.
- **Question** : La facturation des exposants dans JayFestival passe-t-elle par **JayKonta uniquement** (opÃ©rateurs `quote.create`, `invoice.emit`), ou par **Miyuinvoice** qui lui-mÃªme sâ€™appuie sur JayKonta ?
- **Impact** : Documentation des flux, contrats dâ€™intÃ©gration, implÃ©mentation des Ã©crans ORG-E12, ORG-E13, EXP-E13.
- **Recommandation** : Tracer explicitement dans un document (InterpolaritÃ© ou Reference Base de Donnees) : Â« Facturation exposants JayFestival : [JayKonta seul | Miyuinvoice â†’ JayKonta]. Â»

### 3.2 JayXpose : scope alpha

- **Constat** : JayXpose nâ€™a que le Document fondateur ; pas dâ€™OpÃ©rateurs, Parcours, UI, ni contrat dâ€™intÃ©gration.
- **Question** : En **alpha**, la fiche exposant et le rÃ©pertoire JayFestival utilisent-ils **uniquement les donnÃ©es locales** (Supabase/tables JayFestival), ou doit-on prÃ©voir lâ€™appel Ã  JayXpose dÃ¨s lâ€™alpha ?
- **Impact** : Si JayXpose hors alpha, documenter Â« Fiche exposant alpha = donnÃ©es locales ; JayXpose = phase 2 ou optionnel Â» dans le Bornage et lâ€™InterpolaritÃ©. Si JayXpose en alpha, il faut au minimum un contrat dâ€™intÃ©gration (champs, API ou faÃ§ade).
- **Recommandation** : Conserver JayXpose **optionnel / phase 2** pour lâ€™alpha et le formaliser dans le Bornage.

### 3.3 JayFaim : scope et roadmap

- **Constat** : JayFaim nâ€™a que le Document fondateur ; restauration sur Ã©vÃ©nement = phase 2 ou optionnel.
- **Question** : Confirmer que JayFaim est **hors scope alpha** et documenter la **roadmap** (phase 2) pour ne pas bloquer lâ€™alpha.
- **Recommandation** : Ajouter dans le Bornage une ligne explicite Â« JayFaim : phase 2 ; pas de doc OpÃ©rateurs/UI requise pour alpha. Â»

### 3.4 Miyuprofile : frontiÃ¨re avec JayFestival

- **Constat** : Pas de document fondateur Miyuprofile dÃ©diÃ© au mÃªme niveau que MiyuAuth ; profils (organisateur, exposant, visiteur) sont dÃ©crits dans les Ã©crans JayFestival.
- **Question** : OÃ¹ est la **source de vÃ©ritÃ©** du profil (email, nom, structure, etc.) : Miyuprofile ou tables JayFestival/Supabase ? Faut-il un document Â« JayFestival â†” Miyuprofile Â» (contrat dâ€™intÃ©gration) ?
- **Impact** : ImplÃ©mentation des Ã©crans Â« Mon compte Â», Â« Fiche entreprise Â», synchronisation des donnÃ©es.
- **Recommandation** : DÃ©cider si lâ€™alpha utilise des **profiles Supabase** uniquement, ou si Miyuprofile est consommÃ© ; dans les deux cas, documenter la rÃ¨gle dans Reference Base de Donnees ou Bornage.

### 3.5 Agenda : JayKoa vs MiyuClock

- **Constat** : Document fondateur JayFestival cite Â« MiyuClock, Miyubooking Â» pour lâ€™agenda cross-Ã©vÃ©nements ; JayKoa est lâ€™intÃ©grateur des dates (Integration Services Consommateurs).
- **Question** : **Partage des rÃ´les** : JayKoa = entrÃ©es agenda, conflits, vues agrÃ©gÃ©es ; MiyuClock = quoi exactement (fuseaux, horloge, rÃ©fÃ©rentiel temps) ? Ã‰viter doublon ou flou pour lâ€™implÃ©mentation.
- **Recommandation** : RÃ©diger un paragraphe court dans Â« JayFestival - Interpolarite Services Jay Â» ou dans Â« JayKoa - Integration Services Consommateurs Â» : Â« Pour lâ€™agenda cross-Ã©vÃ©nements, JayFestival publie vers JayKoa ; MiyuClock est utilisÃ© pour [fuseaux / affichage temps / â€¦]. Â»

### 3.6 JayKonta : document Â« Ecrans et UI Â»

- **Constat** : JayKonta nâ€™a pas de document Â« Ecrans et UI Â» ; lâ€™UI est dÃ©crite dans les Parcours (Account, Purse).
- **Question** : Souhaite-t-on un document **JayKonta - Ecrans et UI** (patterns, zones, composants) pour une UI JayKonta native (hors JayFestival) ? Pour JayFestival seul, la Specification UI Conforme Catakana suffit pour les Ã©crans budget/devis/factures.
- **Recommandation** : Pour une **implÃ©mentation complÃ¨te UI incluse** **dans JayFestival** : suffisant. Pour une **future app JayKonta standalone** : crÃ©er un doc Ecrans et UI JayKonta.

---

## 4. Actions recommandÃ©es (documentation)

| PrioritÃ© | Action | Responsable suggÃ©rÃ© |
|----------|--------|----------------------|
| **P0** | TranchÃ© : Miyuinvoice + JayKonta. JayXpose dans l'alpha (parcours demande stands, annuaire exposants). JayFaim hors scope alpha. | Product / Tech |
| **P0** | JayXpose = dans l'alpha (Bornage mis Ã  jour). JayFaim = hors scope alpha. | Product |
| **P1** | RÃ©diger Â« JayFestival â†” Miyubooking Â» : capacitÃ©s consommÃ©es, donnÃ©es, Ã©crans (section dans InterpolaritÃ© ou doc dÃ©diÃ©). | Tech / Doc |
| **P1** | TranchÃ© : Miyuprofile = Supabase uniquement pour le moment (source de vÃ©ritÃ© = tables Supabase). | Product / Tech |
| **P1** | TranchÃ© : JayKoa organise donnÃ©es + interface ; MiyuClock atteste horaire/date IRL (InterpolaritÃ© et rÃ©fÃ©rence MiyuClock mises Ã  jour). | Tech / Doc |
| **P2** | Si roadmap phase 2 : crÃ©er pour JayFaim les documents OpÃ©rateurs/Kits, Parcours, Contrat dâ€™intÃ©gration JayFestival. | Product / Doc |
| **P2** | Si besoin UI JayKonta standalone : crÃ©er Â« JayKonta - Ecrans et UI Â». | Product |

---

## 5. RÃ©fÃ©rences

| Document | RÃ´le |
|----------|------|
| [JayFestival - Interpolarite Services Jay](./JayFestival%20-%20Interpolarite%20Services%20Jay.md) | Couplages JayFestival avec services Jay. |
| [JayFestival - Bornage Implementation](../JayFestival%20-%20Bornage%20Implementation.md) | PÃ©rimÃ¨tre alpha, phase 2, dÃ©pendances. |
| [JayFestival - Specification UI Conforme Catakana](../JayFestival%20-%20Specification%20UI%20Conforme%20Catakana.md) | Ã‰crans et composants UI JayFestival. |
| [JayKoa - Integration Services Consommateurs](../../JayKoa/reference/JayKoa%20-%20Integration%20Services%20Consommateurs.md) | Types dâ€™entrÃ©es JayFestival, responsabilitÃ©s. |
| [JayKonta - Integration Services](../../JayKonta/reference/JayKonta%20-%20Integration%20Services.md) | Flux budget, devis, factures JayFestival. |
| [Miyukini Conceptual References - Interpolarite Services Jay](..//..//..//miyukini-webway-system//reference//_index.md) | Principe global interpolaritÃ©. |

---

**Document** : JayFestival â€” Ã‰tat de la documentation des services interfacÃ©s  
**Version** : 1.0  
**Date** : 2026-02-03  
**Statut** : Document de rÃ©fÃ©rence â€” audit et dÃ©cisions Ã  trancher

