# JayFestival — Plan du Service

## Contexte

Ce document constitue le **plan du service** (sitemap) de JayFestival. Il cartographie l'ensemble des ecrans, leur hierarchie, la navigation inter-ecrans et les flux par type d'utilisateur. Des mocks ASCII illustrent les ecrans cles pour guider l'implementation UI.

**Sources** : [Parcours Utilisateurs Schema Flux](./reference/JayFestival%20-%20Parcours%20Utilisateurs%20Schema%20Flux.md), ecrans par public (`publics/`), [Analyse Approfondie Catakana Orga](./JayFestival%20-%20Analyse%20Approfondie%20Catakana%20Orga.md), [Specification UI Conforme Catakana](./JayFestival%20-%20Specification%20UI%20Conforme%20Catakana.md).

## Portee / Scope

- **Perimetre** : Sitemap complet, hierarchie d'ecrans, mocks de navigation, wireframes ASCII des ecrans cles pour les 4 publics (Utilisateur non connecte, Organisateur, Exposant, Visiteur).
- **Hors perimetre** : Maquettes haute fidelite, specifications API, specifications de composants (voir Specification UI Conforme).
- **Audience** : Equipes produit, UX, developpement.

---

## 1. Sitemap global

```
JayFestival
│
├─── FACADE PUBLIQUE (Utilisateur non connecte)
│    ├── UNC-E01  Landing / Accueil catalogue
│    ├── UNC-E02  Liste des evenements
│    ├── UNC-E03  Fiche evenement (detail public)
│    ├── UNC-E06  Liste des organisateurs
│    ├── UNC-E07  Fiche organisateur
│    ├── UNC-E08  Liste des exposants
│    ├── UNC-E09  Fiche exposant
│    ├── UNC-E10  Recherche (resultats et affinage)
│    ├── UNC-E11  CTA contextuels (modal non connecte)
│    ├── UNC-E12  Connexion
│    ├── UNC-E13  Inscription (choix type)
│    └── UNC-E14  Mentions legales / CGU / Confidentialite / Accessibilite
│
├─── ESPACE ORGANISATEUR
│    ├── ORG-E01  Landing (passerelle depuis catalogue)
│    ├── ORG-E02  Connexion
│    ├── ORG-E03  Inscription organisateur
│    ├── ORG-E04  Tableau de bord organisateur
│    ├── ORG-E05  Liste des editions
│    ├── ORG-E06  Creation d'une edition
│    ├── ORG-E07  Dashboard edition ──────────────────── (hub central)
│    │    ├── ORG-E08  Parametrage edition
│    │    ├── ORG-E09  Liste exposants (annuaire local)
│    │    │    ├── ORG-E10  Candidatures
│    │    │    ├── ORG-E11  Fiche exposant
│    │    │    │    ├── ORG-E12  Generation devis
│    │    │    │    └── ORG-E13  Factures
│    │    │    └── ORG-E18  Import exposants (CSV)
│    │    ├── ORG-E14  Plan de salle (definition)
│    │    │    ├── ORG-E15  Attribution emplacements
│    │    │    └── ORG-E16  Visualisation plan
│    │    ├── ORG-E17a Programme (vues chrono/salle)
│    │    │    └── ORG-E17b Creation/edition animation
│    │    ├── ORG-E19  Budget (saisie, ventilation, balance)
│    │    ├── ORG-E22  Documents et legal
│    │    ├── ORG-E23  Annonces et notifications
│    │    ├── ORG-E24  Services visiteur (activation)
│    │    └── ORG-E25  Publication et cloture
│    ├── ORG-E20  Mon compte
│    └── ORG-E21  Equipe et invitations
│
├─── ESPACE EXPOSANT
│    ├── EXP-E01  Landing (passerelle depuis catalogue)
│    ├── EXP-E02  Connexion
│    ├── EXP-E03  Inscription exposant
│    ├── EXP-E04  Dashboard exposant ─────────────────── (hub central)
│    │    ├── EXP-E05  Liste candidatures
│    │    │    ├── EXP-E08  Annuaire evenements (candidatures ouvertes)
│    │    │    ├── EXP-E10  Depot candidature
│    │    │    └── EXP-E07  Fiche candidature (detail/suivi)
│    │    ├── EXP-E06  Liste participations
│    │    │    ├── EXP-E11  Fiche participation (edition validee)
│    │    │    │    ├── EXP-E11b Plan de salle (emplacement)
│    │    │    │    └── EXP-E11c Programme public
│    │    │    └── EXP-E12  Documents par edition
│    │    │         └── EXP-E12b Envoi document signe
│    │    ├── EXP-E09  Agenda exposant (cross-evenements)
│    │    ├── EXP-E13  Devis et factures
│    │    └── EXP-E19  Notifications et preferences
│    ├── EXP-E17  Mon compte (profil + fiche entreprise)
│    └── EXP-E18  Fiche publique (repertoire)
│
└─── ESPACE VISITEUR
     ├── VIS-E01  Landing (passerelle depuis catalogue)
     ├── VIS-E02  Connexion
     ├── VIS-E03  Inscription visiteur (festival/groupe)
     ├── VIS-E04  Page d'accueil espace visiteur ──────── (hub central)
     │    ├── VIS-E05  Agenda personnel
     │    ├── VIS-E06  Billets et tickets
     │    ├── VIS-E07  Reservations (liste)
     │    ├── VIS-E08  Pass VIP et avantages
     │    ├── VIS-E09  Reservation (flux : atelier/creneau/pass)
     │    ├── VIS-E10  Fiche evenement (connecte, services actives)
     │    ├── VIS-E11  Suivi d'activites (historique)
     │    ├── VIS-E12  Jeux (liste et participation)
     │    └── VIS-E13  Concours (liste et participation)
     ├── VIS-E14  Mon compte
     └── VIS-E15  Preferences de notification
```

**Total** : **12 ecrans UNC** + **25 ecrans ORG** + **19 ecrans EXP** + **15 ecrans VIS** = **71 ecrans**

---

## 2. Navigation globale — Diagramme de flux

### 2.1 Point d'entree commun

```
                              ┌─────────────┐
                              │ URL / Lien   │
                              └──────┬───────┘
                                     │
                                     ▼
                          ┌──────────────────────┐
                          │   UNC-E01 LANDING     │
                          │   Facade publique      │
                          └──────────┬─────────────┘
                                     │
               ┌─────────────────────┼─────────────────────┐
               │                     │                     │
               ▼                     ▼                     ▼
     ┌──────────────────┐  ┌──────────────┐     ┌──────────────┐
     │ Navigation libre │  │  UNC-E12     │     │  UNC-E13     │
     │ Catalogue        │  │  CONNEXION   │     │  INSCRIPTION │
     │ (UNC-E02..E10)   │  └──────┬───────┘     │  Choix type  │
     └──────────────────┘         │              └──────┬───────┘
                                  │      ┌──────────────┼──────────────┐
                                  │      │              │              │
                                  │      ▼              ▼              ▼
                                  │  ORG-E03        EXP-E03        VIS-E03
                                  │  Inscription    Inscription    Inscription
                                  │  organisateur   exposant       visiteur
                                  │      │              │              │
                         ┌────────┴──────┴──────────────┴──────────────┘
                         │    (Apres connexion/inscription)
                         │
            ┌────────────┼────────────────┬────────────────┐
            ▼            ▼                ▼                ▼
     ┌────────────┐ ┌──────────┐  ┌────────────┐  ┌────────────┐
     │  ORG-E04   │ │ EXP-E04  │  │  VIS-E04   │  │ Retour     │
     │  Dashboard │ │ Dashboard│  │  Espace    │  │ contexte   │
     │  Orga      │ │ Exposant │  │  Visiteur  │  │ (fiche...) │
     └────────────┘ └──────────┘  └────────────┘  └────────────┘
```

### 2.2 Navigation conditionnelle depuis la facade

```
         UNC-E03 Fiche evenement
                │
     ┌──────────┼──────────┬──────────┐
     │          │          │          │
     ▼          ▼          ▼          ▼
  Reserver   Candidater  Voir Plan  Voir Programme
  (visiteur) (exposant)  (public)   (public)
     │          │
     ▼          ▼
  UNC-E11    UNC-E11
  Modal:     Modal:
  "Connectez-vous    "Connectez-vous
   ou inscrivez-vous  ou inscrivez-vous
   en tant que        en tant
   visiteur"          qu'exposant"
     │          │
     ▼          ▼
  UNC-E12    UNC-E12
  Connexion  Connexion
     │          │
     ▼          ▼
  VIS-E09    EXP-E10
  Flux       Depot
  Reservation candidature
```

---

## 3. Mocks ASCII — Facade Publique (UNC)

### 3.1 UNC-E01 — Landing

```
┌─────────────────────────────────────────────────────────────────┐
│ ┌─────┐  Evenements  Organisateurs  Exposants  [Connexion] [+] │
│ │LOGO │                                                         │
│ └─────┘                                                         │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│          Decouvrez les evenements et festivals                  │
│                                                                 │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ 🔍 Rechercher un evenement, organisateur, exposant...     │  │
│  └───────────────────────────────────────────────────────────┘  │
│                                                                 │
│  ┌─────────────────┐ ┌─────────────────┐ ┌─────────────────┐   │
│  │                 │ │                 │ │                 │   │
│  │  🎪 Evenements  │ │  🏢 Organisat.  │ │  🧑‍💼 Exposants   │   │
│  │  [Voir tout]    │ │  [Voir tout]    │ │  [Voir tout]    │   │
│  └─────────────────┘ └─────────────────┘ └─────────────────┘   │
│                                                                 │
│  ─────────── Prochains evenements ──────────                    │
│                                                                 │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ [img]  Festival du Jeu 2026            15-17 mars 2026   │  │
│  │        Lyon • Association Ludique      [Voir la fiche →] │  │
│  ├───────────────────────────────────────────────────────────┤  │
│  │ [img]  Salon des Createurs 2026        22-23 avr. 2026   │  │
│  │        Bordeaux • Mairie de Bordeaux   [Voir la fiche →] │  │
│  ├───────────────────────────────────────────────────────────┤  │
│  │ [img]  Marche de Noel Artisanal        5-24 dec. 2026    │  │
│  │        Strasbourg • Collectif Noel     [Voir la fiche →] │  │
│  └───────────────────────────────────────────────────────────┘  │
│                                                                 │
│  ─────────── Rejoignez la plateforme ──────────                 │
│                                                                 │
│  ┌─────────────────┐ ┌─────────────────┐ ┌─────────────────┐   │
│  │ Je suis         │ │ Je suis         │ │ Je suis         │   │
│  │ ORGANISATEUR    │ │ EXPOSANT        │ │ VISITEUR        │   │
│  │                 │ │                 │ │                 │   │
│  │ Creez et gerez  │ │ Participez a    │ │ Decouvrez et    │   │
│  │ vos evenements  │ │ des evenements  │ │ reservez        │   │
│  │ [S'inscrire →]  │ │ [S'inscrire →]  │ │ [S'inscrire →]  │   │
│  └─────────────────┘ └─────────────────┘ └─────────────────┘   │
│                                                                 │
├─────────────────────────────────────────────────────────────────┤
│ Mentions legales • CGU • Confidentialite • Accessibilite        │
└─────────────────────────────────────────────────────────────────┘
```

### 3.2 UNC-E02 — Liste des evenements

```
┌─────────────────────────────────────────────────────────────────┐
│ [LOGO]  Evenements  Organisateurs  Exposants  [Connexion] [+]  │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  Evenements                                      [Liste] [Map] │
│                                                                 │
│  ┌─ FILTRES ──────────────┐ ┌─ RESULTATS ───────────────────┐  │
│  │                        │ │                               │  │
│  │ Date                   │ │ 42 evenements                 │  │
│  │ [Du ___] [Au ___]      │ │                               │  │
│  │ [x] A venir seulement  │ │ ┌───────────────────────────┐ │  │
│  │                        │ │ │ [img] Festival du Jeu 2026│ │  │
│  │ Lieu                   │ │ │ 15-17 mars • Lyon         │ │  │
│  │ [____________]         │ │ │ Association Ludique       │ │  │
│  │                        │ │ │ #Jeu #Culture #Famille    │ │  │
│  │ Theme                  │ │ │            [Voir fiche →] │ │  │
│  │ [ ] Culture            │ │ └───────────────────────────┘ │  │
│  │ [ ] Artisanat          │ │                               │  │
│  │ [ ] Gastronomie        │ │ ┌───────────────────────────┐ │  │
│  │ [ ] Jeux               │ │ │ [img] Salon Createurs     │ │  │
│  │ [ ] Musique            │ │ │ 22-23 avr. • Bordeaux     │ │  │
│  │                        │ │ │ Mairie de Bordeaux        │ │  │
│  │ Organisateur           │ │ │ #Artisanat #Design       │ │  │
│  │ [____________]         │ │ │            [Voir fiche →] │ │  │
│  │                        │ │ └───────────────────────────┘ │  │
│  │ [Reinitialiser]        │ │                               │  │
│  └────────────────────────┘ │ ← 1 2 3 ... 5 →              │  │
│                             └───────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

### 3.3 UNC-E03 — Fiche evenement

```
┌─────────────────────────────────────────────────────────────────┐
│ [LOGO]  Evenements  Organisateurs  Exposants  [Connexion] [+]  │
├─────────────────────────────────────────────────────────────────┤
│ ← Retour aux evenements                                        │
│                                                                 │
│ ┌───────────────────────────────────────────────────────────┐   │
│ │                     [IMAGE BANNIERE]                      │   │
│ └───────────────────────────────────────────────────────────┘   │
│                                                                 │
│  Festival du Jeu 2026                                           │
│  ──────────────────────                                         │
│  📅 15 - 17 mars 2026                                          │
│  📍 Lyon, Parc des Expositions                                 │
│  🏢 Par : Association Ludique [Voir organisateur →]            │
│                                                                 │
│  Description                                                    │
│  Le Festival du Jeu reunit passionnes, createurs et editeurs   │
│  pour 3 jours de decouverte ludique...                         │
│                                                                 │
│  ─── Programme ─────────────────────────────────────────────    │
│  │ Ven. 15 │ Sam. 16 │ Dim. 17 │                               │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │ 10:00  Ouverture des portes                              │   │
│  │ 11:00  Atelier initiation jeux de role     [Salle A]     │   │
│  │ 14:00  Tournoi jeux de societe             [Salle B]     │   │
│  │ 16:00  Rencontre avec les auteurs          [Salle C]     │   │
│  └──────────────────────────────────────────────────────────┘   │
│                                                                 │
│  ─── Exposants (24) ────────────────────────────────────────    │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐          │
│  │ [logo]   │ │ [logo]   │ │ [logo]   │ │ [logo]   │          │
│  │ Editeur A│ │ Editeur B│ │ Artisan C│ │ Studio D │          │
│  │ [Voir →] │ │ [Voir →] │ │ [Voir →] │ │ [Voir →] │          │
│  └──────────┘ └──────────┘ └──────────┘ └──────────┘          │
│                                                  [Voir tous →] │
│                                                                 │
│  ─── Services proposes ─────────────────────────────────────    │
│  ┌───────────────────┐ ┌───────────────────┐                   │
│  │ 🎟️ Reserver un     │ │ 🏆 Participer     │                   │
│  │ atelier           │ │ au concours       │                   │
│  │ [Reserver →]      │ │ [Participer →]    │                   │
│  └───────────────────┘ └───────────────────┘                   │
│                                                                 │
│  ┌─ CTA CANDIDATURE EXPOSANT ────────────────────────────────┐  │
│  │  Vous etes professionnel ? Participez en tant qu'exposant │  │
│  │                                      [Candidater →]       │  │
│  └───────────────────────────────────────────────────────────┘  │
│                                                                 │
├─────────────────────────────────────────────────────────────────┤
│ Mentions legales • CGU • Confidentialite • Accessibilite        │
└─────────────────────────────────────────────────────────────────┘
```

---

## 4. Mocks ASCII — Espace Organisateur (ORG)

### 4.1 ORG-E04 — Tableau de bord organisateur

```
┌─────────────────────────────────────────────────────────────────┐
│ [LOGO]  JayFestival           Association Ludique  [Deconnexion]│
├──────────────┬──────────────────────────────────────────────────┤
│              │                                                  │
│  ⌂ Accueil   │  Bonjour, Association Ludique                    │
│              │                                                  │
│  📋 Editions │  ┌────────────┐ ┌────────────┐ ┌────────────┐   │
│              │  │ 3          │ │ 1          │ │ 12         │   │
│  👤 Compte   │  │ Editions   │ │ En cours   │ │ Candidat.  │   │
│              │  │ total      │ │            │ │ en attente │   │
│  👥 Equipe   │  └────────────┘ └────────────┘ └────────────┘   │
│              │                                                  │
│              │  ─── Mes editions ────────────────────────────    │
│              │                                                  │
│              │  ┌────────────────────────────────────────────┐   │
│              │  │ ● Festival du Jeu 2026     EN COURS       │   │
│              │  │   15-17 mars • Lyon                       │   │
│              │  │   24 exposants • 5 candidatures           │   │
│              │  │                        [Ouvrir dashboard →]│   │
│              │  ├────────────────────────────────────────────┤   │
│              │  │ ○ Salon Automne 2026       BROUILLON      │   │
│              │  │   15-17 oct. • Bordeaux                   │   │
│              │  │   0 exposant • 0 candidature              │   │
│              │  │                        [Ouvrir dashboard →]│   │
│              │  ├────────────────────────────────────────────┤   │
│              │  │ ✓ Festival du Jeu 2025     CLOTUREE       │   │
│              │  │   10-12 mars • Lyon                       │   │
│              │  │   18 exposants                            │   │
│              │  │                        [Consulter →]       │   │
│              │  └────────────────────────────────────────────┘   │
│              │                                                  │
│              │            [+ Creer une edition]                  │
│              │                                                  │
└──────────────┴──────────────────────────────────────────────────┘
```

### 4.2 ORG-E07 — Dashboard edition (hub central)

```
┌─────────────────────────────────────────────────────────────────┐
│ [LOGO]  JayFestival           Association Ludique  [Deconnexion]│
├──────────────┬──────────────────────────────────────────────────┤
│              │  Mes editions > Festival du Jeu 2026             │
│  ⌂ Accueil   │                                                  │
│              │  ┌──────────────────────────────────────────────┐ │
│  📋 Editions │  │ Vue d'ensemble │ Exposants │ Plan │ Programme││
│              │  │ Budget │ Documents │ Notif. │ Services │ Pub.││
│  EDITION:    │  └──────────────────────────────────────────────┘ │
│  ┈┈┈┈┈┈┈┈┈┈ │                                                  │
│  ⚙ Parametres│  ─── Indicateurs ────────────────────────────    │
│  👥 Exposants │                                                  │
│  🗺 Plan      │  ┌──────────┐┌──────────┐┌──────────┐┌────────┐ │
│  📅 Programme │  │ 24       ││ 5        ││ 18/30    ││ 12     │ │
│  💰 Budget    │  │ Exposants││ Candidat.││ Stands   ││ Anim.  │ │
│  📄 Documents │  │ valides  ││ attente  ││ attrib.  ││        │ │
│  📢 Annonces  │  └──────────┘└──────────┘└──────────┘└────────┘ │
│  🎮 Services  │                                                  │
│  🌐 Publier   │  ┌──────────┐┌──────────┐┌──────────┐          │
│              │  │ 45 200 € ││  8 500 € ││ 36 700 € │          │
│  ─────────── │  │ Revenus  ││ Depenses ││ Balance  │          │
│  👤 Compte   │  └──────────┘└──────────┘└──────────┘          │
│  👥 Equipe   │                                                  │
│              │  ─── Actions rapides ─────────────────────────    │
│              │                                                  │
│              │  [Traiter candidatures (5)]  [Voir le plan]      │
│              │  [Gerer programme]           [Editer budget]     │
│              │                                                  │
│              │  ─── Alertes ─────────────────────────────────    │
│              │  ⚠ 2 factures en attente de paiement             │
│              │  ⚠ 1 document a signer par exposant              │
│              │                                                  │
└──────────────┴──────────────────────────────────────────────────┘
```

### 4.3 ORG-E10 — Candidatures

```
┌─────────────────────────────────────────────────────────────────┐
│ [LOGO]  JayFestival           Association Ludique  [Deconnexion]│
├──────────────┬──────────────────────────────────────────────────┤
│              │  Mes editions > Festival du Jeu 2026 > Candidat. │
│  [sidebar]   │                                                  │
│              │  Candidatures en attente (5)                      │
│              │                                                  │
│              │  ┌──────────────────────────────────────────────┐ │
│              │  │ NOM          │ DATE    │ STATUT  │ ACTIONS   │ │
│              │  ├──────────────┼─────────┼─────────┼───────────┤ │
│              │  │ Studio Pixel │ 02/02   │ 🟡 Att. │ [Voir]    │ │
│              │  │ Atelier Bois │ 03/02   │ 🟡 Att. │ [Voir]    │ │
│              │  │ Jeux & Co    │ 04/02   │ 🟡 Att. │ [Voir]    │ │
│              │  │ Creations M. │ 05/02   │ 🟡 Att. │ [Voir]    │ │
│              │  │ ArtisanPro   │ 06/02   │ 🟡 Att. │ [Voir]    │ │
│              │  └──────────────┴─────────┴─────────┴───────────┘ │
│              │                                                  │
│              │  ┌── Detail candidature : Studio Pixel ────────┐  │
│              │  │                                             │  │
│              │  │  Entreprise : Studio Pixel                  │  │
│              │  │  Activite : Edition jeux de societe         │  │
│              │  │  Contact : contact@studiopixel.fr           │  │
│              │  │  Pieces jointes : [fiche.pdf] [logo.png]    │  │
│              │  │                                             │  │
│              │  │  ┌─────────────┐  ┌──────────────────────┐  │  │
│              │  │  │ ✓ VALIDER   │  │ ✗ REFUSER            │  │  │
│              │  │  │             │  │ Motif : [__________] │  │  │
│              │  │  └─────────────┘  └──────────────────────┘  │  │
│              │  └─────────────────────────────────────────────┘  │
└──────────────┴──────────────────────────────────────────────────┘
```

### 4.4 ORG-E14 — Plan de salle

```
┌─────────────────────────────────────────────────────────────────┐
│ [LOGO]  JayFestival           Association Ludique  [Deconnexion]│
├──────────────┬──────────────────────────────────────────────────┤
│              │  Plan de salle — Festival du Jeu 2026            │
│  [sidebar]   │                                                  │
│              │  ┌─── CANVAS ────────────────────────────────┐   │
│              │  │                                           │   │
│              │  │   ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐       │   │
│              │  │   │ A1  │ │ A2  │ │ A3  │ │ A4  │       │   │
│              │  │   │Pixel│ │     │ │Bois │ │     │       │   │
│              │  │   └─────┘ └─────┘ └─────┘ └─────┘       │   │
│              │  │                                           │   │
│              │  │   ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐       │   │
│              │  │   │ B1  │ │ B2  │ │ B3  │ │ B4  │       │   │
│              │  │   │J&Co │ │     │ │     │ │     │       │   │
│              │  │   └─────┘ └─────┘ └─────┘ └─────┘       │   │
│              │  │                                           │   │
│              │  │   ┌───────────────────┐  [SCENE]         │   │
│              │  │   │  ZONE TECHNIQUE   │                   │   │
│              │  │   └───────────────────┘                   │   │
│              │  │                                           │   │
│              │  └───────────────────────────────────────────┘   │
│              │                                                  │
│              │  ─── Legende ──────────────                      │
│              │  🟢 Attribue  🟡 Reserve  ⬜ Libre              │
│              │                                                  │
│              │  [Enregistrer]  [Export PDF]  [Export image]      │
└──────────────┴──────────────────────────────────────────────────┘
```

---

## 5. Mocks ASCII — Espace Exposant (EXP)

### 5.1 EXP-E04 — Dashboard exposant

```
┌─────────────────────────────────────────────────────────────────┐
│ [LOGO]  JayFestival                Studio Pixel    [Deconnexion]│
├──────────────┬──────────────────────────────────────────────────┤
│              │                                                  │
│  ⌂ Accueil   │  Bonjour, Studio Pixel                           │
│              │                                                  │
│  📋 Candidat.│  ┌────────────┐ ┌────────────┐ ┌────────────┐   │
│              │  │ 2          │ │ 3          │ │ 1          │   │
│  🎫 Particip.│  │ Candidat.  │ │ Particip.  │ │ Facture    │   │
│              │  │ en attente │ │ validees   │ │ a payer    │   │
│  📅 Agenda   │  └────────────┘ └────────────┘ └────────────┘   │
│              │                                                  │
│  📄 Documents│  ─── Prochain evenement ──────────────────────    │
│              │                                                  │
│  💰 Factures │  ┌────────────────────────────────────────────┐   │
│              │  │ 🗓 Festival du Jeu 2026                    │   │
│  🔔 Notif.   │  │   15-17 mars 2026 • Lyon                  │   │
│              │  │   Stand A1 • Statut : Valide               │   │
│  ─────────── │  │   ⚠ 1 document a signer                   │   │
│  👤 Compte   │  │                     [Voir participation →] │   │
│  📇 Fiche pub│  └────────────────────────────────────────────┘   │
│              │                                                  │
│              │  ─── Alertes ─────────────────────────────────    │
│              │  ⚠ Conflit de dates : Salon Createurs (22-23      │
│              │    avr.) chevauche avec Foire de Printemps        │
│              │  ⚠ Facture FJ-2026-042 a payer avant le 01/03    │
│              │                                                  │
│              │  ─── Candidatures recentes ───────────────────    │
│              │  ┌────────────────────────────────────────────┐   │
│              │  │ Salon Createurs      │ 🟡 En attente       │   │
│              │  │ Marche de Noel       │ 🟡 En attente       │   │
│              │  │ Foire Printemps      │ 🔴 Refusee          │   │
│              │  └────────────────────────────────────────────┘   │
│              │                                                  │
│              │         [Deposer une candidature →]               │
└──────────────┴──────────────────────────────────────────────────┘
```

### 5.2 EXP-E08 — Annuaire evenements (candidatures ouvertes)

```
┌─────────────────────────────────────────────────────────────────┐
│ [LOGO]  JayFestival                Studio Pixel    [Deconnexion]│
├──────────────┬──────────────────────────────────────────────────┤
│              │  Evenements ouverts aux candidatures              │
│  [sidebar]   │                                                  │
│              │  Filtres: [Date ▾] [Lieu ▾] [Theme ▾]            │
│              │                                                  │
│              │  ┌────────────────────────────────────────────┐   │
│              │  │                                            │   │
│              │  │  Festival du Jeu 2026                      │   │
│              │  │  📅 15-17 mars • 📍 Lyon                   │   │
│              │  │  🏢 Association Ludique                    │   │
│              │  │  Candidature avant le : 15/02/2026         │   │
│              │  │                          [Candidater →]     │   │
│              │  │                                            │   │
│              │  ├────────────────────────────────────────────┤   │
│              │  │                                            │   │
│              │  │  Salon des Createurs 2026                  │   │
│              │  │  📅 22-23 avr. • 📍 Bordeaux               │   │
│              │  │  🏢 Mairie de Bordeaux                     │   │
│              │  │  Candidature avant le : 01/03/2026         │   │
│              │  │  ⚠ Conflit dates : Foire Printemps         │   │
│              │  │                          [Candidater →]     │   │
│              │  │                                            │   │
│              │  ├────────────────────────────────────────────┤   │
│              │  │                                            │   │
│              │  │  Marche de Noel Artisanal 2026             │   │
│              │  │  📅 5-24 dec. • 📍 Strasbourg              │   │
│              │  │  🏢 Collectif Noel                         │   │
│              │  │  Candidature avant le : 15/10/2026         │   │
│              │  │                          [Candidater →]     │   │
│              │  │                                            │   │
│              │  └────────────────────────────────────────────┘   │
└──────────────┴──────────────────────────────────────────────────┘
```

### 5.3 EXP-E10 — Depot candidature

```
┌─────────────────────────────────────────────────────────────────┐
│ [LOGO]  JayFestival                Studio Pixel    [Deconnexion]│
├──────────────┬──────────────────────────────────────────────────┤
│              │  Mes candidatures > Festival du Jeu 2026         │
│  [sidebar]   │                                                  │
│              │  Candidater — Festival du Jeu 2026                │
│              │                                                  │
│              │  ┌── Reglement ────────────────────────────────┐  │
│              │  │ En candidatant, vous acceptez le reglement  │  │
│              │  │ des conditions de participation des          │  │
│              │  │ exposants.  [Lire le reglement]              │  │
│              │  │ [x] J'accepte le reglement                  │  │
│              │  └─────────────────────────────────────────────┘  │
│              │                                                  │
│              │  ─── Vos informations ────────────────────────    │
│              │  Entreprise    : Studio Pixel (JayXpose sync)     │
│              │  Activite      : Edition jeux de societe          │
│              │  Contact       : contact@studiopixel.fr           │
│              │                                                  │
│              │  ─── Informations complementaires (organisat.) ─  │
│              │  Description de votre offre :                     │
│              │  ┌────────────────────────────────────────────┐   │
│              │  │ Nous presentons nos dernieres creations    │   │
│              │  │ dont le jeu "Aventures & Compagnie"...     │   │
│              │  └────────────────────────────────────────────┘   │
│              │                                                  │
│              │  Besoins specifiques :                            │
│              │  [x] Table standard (3m)                          │
│              │  [ ] Prise electrique                             │
│              │  [ ] Cloison                                      │
│              │                                                  │
│              │  ─── Pieces jointes ──────────────────────────    │
│              │  ┌────────────────────────────────────────────┐   │
│              │  │  📎 Glisser-deposer ou [Parcourir]         │   │
│              │  │  ✓ fiche_entreprise.pdf (245 Ko)           │   │
│              │  │  ✓ logo_studiopixel.png (82 Ko)            │   │
│              │  └────────────────────────────────────────────┘   │
│              │                                                  │
│              │  ─── Verification agenda ─────────────────────    │
│              │  ✅ Aucun conflit de dates detecte                │
│              │                                                  │
│              │  [Previsualiser]   [Envoyer la candidature]       │
│              │                   [Annuler]                       │
└──────────────┴──────────────────────────────────────────────────┘
```

### 5.4 EXP-E09 — Agenda exposant

```
┌─────────────────────────────────────────────────────────────────┐
│ [LOGO]  JayFestival                Studio Pixel    [Deconnexion]│
├──────────────┬──────────────────────────────────────────────────┤
│              │  Mon agenda                      [Export iCal]    │
│  [sidebar]   │                                                  │
│              │  [< Fev 2026 >]     [Mois] [Semaine] [Liste]     │
│              │                                                  │
│              │  ┌───┬───┬───┬───┬───┬───┬───┐                   │
│              │  │Lun│Mar│Mer│Jeu│Ven│Sam│Dim│                   │
│              │  ├───┼───┼───┼───┼───┼───┼───┤                   │
│              │  │   │   │   │   │   │   │ 1 │                   │
│              │  ├───┼───┼───┼───┼───┼───┼───┤                   │
│              │  │ 2 │ 3 │ 4 │ 5 │ 6 │ 7 │ 8 │                   │
│              │  ├───┼───┼───┼───┼───┼───┼───┤                   │
│              │  │ 9 │10 │11 │12 │13 │14 │15 │                   │
│              │  │   │   │   │   │   │   │   │                   │
│              │  ├───┼───┼───┼───┼───┼───┼───┤                   │
│              │  │16 │17 │18 │19 │20 │21 │22 │                   │
│              │  ├───┼───┼───┼───┼───┼───┼───┤                   │
│              │  │23 │24 │25 │26 │27 │28 │   │                   │
│              │  └───┴───┴───┴───┴───┴───┴───┘                   │
│              │                                                  │
│              │  Mars 2026 :                                      │
│              │  ┌──────────────────────────────────────────┐     │
│              │  │ 🟢 15-17 mars  Festival du Jeu (VALIDE) │     │
│              │  │    Lyon • Stand A1                       │     │
│              │  └──────────────────────────────────────────┘     │
│              │  ┌──────────────────────────────────────────┐     │
│              │  │ 🟡 22-23 avr.  Salon Createurs (ATTENTE)│     │
│              │  │    Bordeaux                              │     │
│              │  │    ⚠ Conflit : Foire Printemps           │     │
│              │  └──────────────────────────────────────────┘     │
│              │                                                  │
│              │  Prochain evenement dans 34 jours                 │
└──────────────┴──────────────────────────────────────────────────┘
```

---

## 6. Mocks ASCII — Espace Visiteur (VIS)

### 6.1 VIS-E04 — Page d'accueil espace visiteur

```
┌─────────────────────────────────────────────────────────────────┐
│ [LOGO]  JayFestival                    Marie D.    [Deconnexion]│
├──────────────┬──────────────────────────────────────────────────┤
│              │                                                  │
│  ⌂ Accueil   │  Bonjour Marie !                                 │
│              │                                                  │
│  📅 Agenda   │  ┌────────────────────────────────────────────┐   │
│              │  │ 🗓 Prochain evenement                      │   │
│  🎟 Billets  │  │                                            │   │
│              │  │ Festival du Jeu 2026                       │   │
│  📝 Reserv.  │  │ 15-17 mars • Lyon                         │   │
│              │  │ Dans 34 jours                              │   │
│  🏆 Pass VIP │  └────────────────────────────────────────────┘   │
│              │                                                  │
│  🎮 Activites│  ┌────────────┐ ┌────────────┐ ┌────────────┐   │
│              │  │ 2          │ │ 3          │ │ 1          │   │
│  🔍 Catalogue│  │ Billets    │ │ Reservat.  │ │ Pass VIP   │   │
│              │  │            │ │ actives    │ │            │   │
│  ─────────── │  └────────────┘ └────────────┘ └────────────┘   │
│  👤 Compte   │                                                  │
│  🔔 Notif.   │  ─── Mon agenda (apercu) ────────────────────    │
│              │                                                  │
│              │  📅 15 mars 10:00  Ouverture Festival du Jeu     │
│              │  📅 15 mars 14:00  Atelier jeux de role (reserve)│
│              │  📅 16 mars 11:00  Tournoi jeux de societe       │
│              │                            [Voir agenda complet →]│
│              │                                                  │
│              │  ─── Mes activites recentes ──────────────────    │
│              │                                                  │
│              │  🏆 Quizz "Culture Ludique" — Score : 85/100     │
│              │  🏆 Chasse au tresor — En cours (3/5 etapes)     │
│              │                         [Voir toutes activites →]│
│              │                                                  │
│              │  ─── Decouvrir ───────────────────────────────    │
│              │                                                  │
│              │  [Parcourir le catalogue d'evenements →]          │
│              │                                                  │
└──────────────┴──────────────────────────────────────────────────┘
```

### 6.2 VIS-E10 — Fiche evenement (visiteur connecte)

```
┌─────────────────────────────────────────────────────────────────┐
│ [LOGO]  JayFestival                    Marie D.    [Deconnexion]│
├──────────────┬──────────────────────────────────────────────────┤
│              │  ← Retour au catalogue                           │
│  [sidebar]   │                                                  │
│              │  Festival du Jeu 2026                             │
│              │  📅 15-17 mars 2026 • 📍 Lyon                    │
│              │  🏢 Association Ludique                           │
│              │                                                  │
│              │  ─── Services disponibles ────────────────────    │
│              │                                                  │
│              │  ┌──────────┐ ┌──────────┐ ┌──────────┐         │
│              │  │ 🎟       │ │ 📝       │ │ 🏆       │         │
│              │  │ Billets  │ │ Ateliers │ │ Pass VIP │         │
│              │  │          │ │          │ │          │         │
│              │  │ [Acheter]│ │[Reserver]│ │ [Acheter]│         │
│              │  └──────────┘ └──────────┘ └──────────┘         │
│              │                                                  │
│              │  ┌──────────┐ ┌──────────┐                      │
│              │  │ 🎮       │ │ 🏅       │                      │
│              │  │ Jeux     │ │ Concours │                      │
│              │  │          │ │          │                      │
│              │  │[Jouer]   │ │[Particip]│                      │
│              │  └──────────┘ └──────────┘                      │
│              │                                                  │
│              │  ─── Programme ───────────────────────────────    │
│              │  │ Ven. 15 │ Sam. 16 │ Dim. 17 │                 │
│              │  ┌────────────────────────────────────────────┐   │
│              │  │ 10:00  Ouverture          [Ajouter agenda] │   │
│              │  │ 11:00  Atelier jeux role  [✓ Reserve]      │   │
│              │  │ 14:00  Tournoi societe    [Reserver →]     │   │
│              │  │ 16:00  Rencontre auteurs  [Ajouter agenda] │   │
│              │  └────────────────────────────────────────────┘   │
│              │                                                  │
│              │  ─── Exposants (24) ──────────────────────────    │
│              │  [Editeur A] [Editeur B] [Artisan C] [+21]       │
│              │                                                  │
└──────────────┴──────────────────────────────────────────────────┘
```

### 6.3 VIS-E09 — Flux de reservation

```
┌─────────────────────────────────────────────────────────────────┐
│ [LOGO]  JayFestival                    Marie D.    [Deconnexion]│
├──────────────┬──────────────────────────────────────────────────┤
│              │                                                  │
│  [sidebar]   │  Reserver — Festival du Jeu 2026                 │
│              │                                                  │
│              │  ─── Etape 1 : Type ──────────────────────────    │
│              │  (●) Atelier  ( ) Creneau  ( ) Pass               │
│              │                                                  │
│              │  ─── Etape 2 : Selection ─────────────────────    │
│              │                                                  │
│              │  ┌────────────────────────────────────────────┐   │
│              │  │ Atelier jeux de role                       │   │
│              │  │ 📅 Ven. 15 mars • 14:00-16:00 • Salle A   │   │
│              │  │ Places : 8/12 disponibles                  │   │
│              │  │                                (●) Choisi  │   │
│              │  ├────────────────────────────────────────────┤   │
│              │  │ Atelier creation jeu                       │   │
│              │  │ 📅 Sam. 16 mars • 10:00-12:00 • Salle C   │   │
│              │  │ Places : 2/10 disponibles                  │   │
│              │  │                                ( ) Choisi  │   │
│              │  ├────────────────────────────────────────────┤   │
│              │  │ Atelier illustration                       │   │
│              │  │ 📅 Dim. 17 mars • 14:00-16:00 • Salle B   │   │
│              │  │ Places : 0/8 — COMPLET [File d'attente →]  │   │
│              │  └────────────────────────────────────────────┘   │
│              │                                                  │
│              │  ─── Etape 3 : Verification agenda ───────────    │
│              │  ✅ Aucun conflit de dates detecte                │
│              │                                                  │
│              │  ─── Conditions ──────────────────────────────    │
│              │  Annulation possible jusqu'a 24h avant            │
│              │  [x] J'accepte les conditions                     │
│              │                                                  │
│              │           [Confirmer la reservation]              │
│              │           [Annuler]                               │
│              │                                                  │
└──────────────┴──────────────────────────────────────────────────┘
```

---

## 7. Passerelles inter-espaces

### 7.1 Flux de passage entre publics

```
┌──────────────────────────────────────────────────────────────────────┐
│                                                                      │
│  FACADE PUBLIQUE                                                     │
│  (UNC-E01..E14)                                                      │
│       │                                                              │
│       ├── [Se connecter] ──────→ UNC-E12 Connexion ─────────────┐    │
│       │                              │                          │    │
│       │                    ┌─────────┼─────────┐                │    │
│       │                    ▼         ▼         ▼                │    │
│       │               ORG-E04   EXP-E04   VIS-E04              │    │
│       │                                                         │    │
│       ├── [S'inscrire] ───→ UNC-E13 Choix type ────────────┐   │    │
│       │                       │         │         │         │   │    │
│       │                       ▼         ▼         ▼         │   │    │
│       │                  ORG-E03    EXP-E03    VIS-E03      │   │    │
│       │                                                      │   │    │
│       └── [CTA contextuel] → UNC-E11 Modal ────────────────┘   │    │
│                                                                  │    │
│  ESPACE ORGANISATEUR                                             │    │
│  (ORG-E04..E25)                                                  │    │
│       │                                                          │    │
│       └── ORG-E25 Publication → Fiche evenement visible sur ────┘    │
│                                   Facade publique (UNC-E03)          │
│                                                                      │
│  ESPACE EXPOSANT                                                     │
│  (EXP-E04..E19)                                                      │
│       │                                                              │
│       ├── EXP-E08 Annuaire evenements ← Catalogue (UNC-E02/E03)     │
│       ├── EXP-E18 Fiche publique → Repertoire (UNC-E09)             │
│       └── EXP-E11b/c Plan/Programme ← Publie par organisateur       │
│                                                                      │
│  ESPACE VISITEUR                                                     │
│  (VIS-E04..E15)                                                      │
│       │                                                              │
│       ├── VIS-E10 Fiche evenement ← Catalogue (UNC-E02/E03)         │
│       └── VIS-E05 Agenda ↔ JayKoa (synchronisation)                 │
│                                                                      │
└──────────────────────────────────────────────────────────────────────┘
```

### 7.2 Interconnexions avec services Jay

```
┌───────────────────────────────────────────────────────────────────┐
│                                                                   │
│  JayFestival (ce service)                                         │
│  ════════════════════════                                         │
│                                                                   │
│  ┌──────────── JayXpose ─────────────┐                            │
│  │                                   │                            │
│  │  Prerequis Organisateur :         │                            │
│  │  ORG-E03 → Verification profil    │                            │
│  │                                   │                            │
│  │  Prerequis Exposant :             │                            │
│  │  EXP-E03 → Verification profil    │                            │
│  │  EXP-E17 → Synchro "Mon Compte"  │                            │
│  │  EXP-E18 → Synchro fiche publique│                            │
│  │                                   │                            │
│  │  Pas de profil ? → Redirection    │                            │
│  │  vers JayXpose                    │                            │
│  └───────────────────────────────────┘                            │
│                                                                   │
│  ┌──────────── JayKoa ───────────────┐                            │
│  │                                   │                            │
│  │  ORG-E06 → Ajout dates edition    │                            │
│  │  ORG-E17b → Ajout dates animation │                            │
│  │  EXP-E09 → Agenda cross-events    │                            │
│  │  VIS-E05 → Agenda personnel       │                            │
│  │  VIS-E09 → Verif conflit dates    │                            │
│  └───────────────────────────────────┘                            │
│                                                                   │
│  ┌──────────── JayKonta ─────────────┐                            │
│  │                                   │                            │
│  │  ORG-E12 → Generation devis       │                            │
│  │  ORG-E13 → Emission factures      │                            │
│  │  ORG-E19 → Budget par edition     │                            │
│  │  EXP-E13 → Consultation factures  │                            │
│  └───────────────────────────────────┘                            │
│                                                                   │
│  ┌──────────── Central App ──────────┐                            │
│  │                                   │                            │
│  │  UNC-E12 → Authentification       │                            │
│  │  *-E02   → Connexion tous publics │                            │
│  │  *-E03   → Inscription            │                            │
│  └───────────────────────────────────┘                            │
│                                                                   │
└───────────────────────────────────────────────────────────────────┘
```

---

## 8. Layouts de navigation

### 8.1 Layout public (Facade — non connecte)

```
┌─────────────────────────────────────────────────────────────────┐
│  HEADER                                                         │
│  [Logo] [Evenements] [Organisateurs] [Exposants] [Connexion][+] │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│                        CONTENU                                  │
│                   (pleine largeur)                               │
│                                                                 │
├─────────────────────────────────────────────────────────────────┤
│  FOOTER                                                         │
│  Mentions legales • CGU • Confidentialite • Accessibilite       │
└─────────────────────────────────────────────────────────────────┘
```

### 8.2 Layout connecte (Organisateur / Exposant / Visiteur)

```
┌─────────────────────────────────────────────────────────────────┐
│  HEADER                                                         │
│  [Logo] [JayFestival]          [Nom utilisateur] [Deconnexion]  │
├──────────────┬──────────────────────────────────────────────────┤
│   SIDEBAR    │                                                  │
│              │                  CONTENU                          │
│  Menu items  │             (zone principale)                     │
│  (voir 8.3)  │                                                  │
│              │                                                  │
│  ──────────  │                                                  │
│  Compte      │                                                  │
│  Parametres  │                                                  │
├──────────────┴──────────────────────────────────────────────────┤
│  BOTTOM NAV (mobile uniquement)                                 │
│  [Accueil] [Editions] [Exposants] [Docs] [Budget] [Compte]     │
└─────────────────────────────────────────────────────────────────┘
```

### 8.3 Menus sidebar par role

**Organisateur :**
```
⌂  Accueil (ORG-E04)
📋 Editions (ORG-E05)
───────────
[Si edition selectionnee :]
⚙  Parametres (ORG-E08)
👥 Exposants (ORG-E09)
🗺  Plan de salle (ORG-E14)
📅 Programme (ORG-E17a)
💰 Budget (ORG-E19)
📄 Documents (ORG-E22)
📢 Annonces (ORG-E23)
🎮 Services visiteur (ORG-E24)
🌐 Publier (ORG-E25)
───────────
👤 Mon compte (ORG-E20)
👥 Equipe (ORG-E21)
```

**Exposant :**
```
⌂  Accueil (EXP-E04)
📋 Candidatures (EXP-E05)
🎫 Participations (EXP-E06)
📅 Agenda (EXP-E09)
📄 Documents (EXP-E12)
💰 Factures (EXP-E13)
🔔 Notifications (EXP-E19)
───────────
👤 Mon compte (EXP-E17)
📇 Fiche publique (EXP-E18)
```

**Visiteur :**
```
⌂  Accueil (VIS-E04)
📅 Agenda (VIS-E05)
🎟  Billets (VIS-E06)
📝 Reservations (VIS-E07)
🏆 Pass VIP (VIS-E08)
🎮 Activites (VIS-E11)
🔍 Catalogue (VIS-E10)
───────────
👤 Mon compte (VIS-E14)
🔔 Notifications (VIS-E15)
```

---

## 9. Navigation par onglets — Dashboard edition (ORG-E07)

Le dashboard edition utilise une navigation par onglets (pattern Catakana_Orga) :

```
┌───────────┬───────────┬──────┬───────────┬────────┬──────────┬───────┬──────────┬──────┐
│Vue d'ens. │ Parametres│Expos.│Plan salle │Programm│  Budget  │  Docs │  Notif.  │Publi.│
│ (ORG-E07) │ (ORG-E08) │(E09) │ (ORG-E14) │(E17a)  │ (ORG-E19)│(E22)  │ (ORG-E23)│(E25) │
└───────────┴───────────┴──────┴───────────┴────────┴──────────┴───────┴──────────┴──────┘
```

Sur mobile, ces onglets se transforment en menu scrollable horizontal ou en menu deroulant.

---

## 10. Recapitulatif des ecrans par public

### Facade publique (UNC) — 12 ecrans

| Id | Ecran | Type |
|-----|-------|------|
| UNC-E01 | Landing / Accueil catalogue | Page |
| UNC-E02 | Liste des evenements | Page |
| UNC-E03 | Fiche evenement | Page |
| UNC-E06 | Liste des organisateurs | Page |
| UNC-E07 | Fiche organisateur | Page |
| UNC-E08 | Liste des exposants | Page |
| UNC-E09 | Fiche exposant | Page |
| UNC-E10 | Recherche (resultats) | Page |
| UNC-E11 | CTA contextuels | Modal |
| UNC-E12 | Connexion | Page |
| UNC-E13 | Inscription (choix type) | Page |
| UNC-E14 | Mentions legales / CGU | Page |

### Espace Organisateur (ORG) — 25 ecrans

| Id | Ecran | Type |
|-----|-------|------|
| ORG-E01 | Landing (passerelle) | Partage UNC-E01 |
| ORG-E02 | Connexion | Page |
| ORG-E03 | Inscription organisateur | Page |
| ORG-E04 | Tableau de bord organisateur | Page |
| ORG-E05 | Liste des editions | Page |
| ORG-E06 | Creation d'une edition | Page/Modal |
| ORG-E07 | Dashboard edition | Page (hub) |
| ORG-E08 | Parametrage edition | Onglet |
| ORG-E09 | Liste exposants | Onglet |
| ORG-E10 | Candidatures | Onglet/Sous-page |
| ORG-E11 | Fiche exposant | Sous-page |
| ORG-E12 | Generation devis | Sous-page/Modal |
| ORG-E13 | Factures | Sous-page |
| ORG-E14 | Plan de salle (definition) | Onglet |
| ORG-E15 | Attribution emplacements | Sous-page |
| ORG-E16 | Visualisation plan | Sous-page |
| ORG-E17a | Programme (vues) | Onglet |
| ORG-E17b | Creation/edition animation | Modal/Sous-page |
| ORG-E18 | Import exposants | Modal |
| ORG-E19 | Budget | Onglet |
| ORG-E20 | Mon compte | Page |
| ORG-E21 | Equipe et invitations | Page |
| ORG-E22 | Documents et legal | Onglet |
| ORG-E23 | Annonces et notifications | Onglet |
| ORG-E24 | Services visiteur | Onglet |
| ORG-E25 | Publication et cloture | Onglet |

### Espace Exposant (EXP) — 19 ecrans

| Id | Ecran | Type |
|-----|-------|------|
| EXP-E01 | Landing (passerelle) | Partage UNC-E01 |
| EXP-E02 | Connexion | Page |
| EXP-E03 | Inscription exposant | Page |
| EXP-E04 | Dashboard exposant | Page (hub) |
| EXP-E05 | Liste candidatures | Page |
| EXP-E06 | Liste participations | Page |
| EXP-E07 | Fiche candidature | Sous-page |
| EXP-E08 | Annuaire evenements | Page |
| EXP-E09 | Agenda exposant | Page |
| EXP-E10 | Depot candidature | Page |
| EXP-E11 | Fiche participation | Sous-page |
| EXP-E11b | Plan de salle (emplacement) | Sous-page |
| EXP-E11c | Programme public | Sous-page |
| EXP-E12 | Documents par edition | Page |
| EXP-E12b | Envoi document signe | Modal |
| EXP-E13 | Devis et factures | Page |
| EXP-E17 | Mon compte | Page |
| EXP-E18 | Fiche publique | Page |
| EXP-E19 | Notifications et preferences | Page |

### Espace Visiteur (VIS) — 15 ecrans

| Id | Ecran | Type |
|-----|-------|------|
| VIS-E01 | Landing (passerelle) | Partage UNC-E01 |
| VIS-E02 | Connexion | Page |
| VIS-E03 | Inscription visiteur | Page |
| VIS-E04 | Page d'accueil espace visiteur | Page (hub) |
| VIS-E05 | Agenda personnel | Page |
| VIS-E06 | Billets et tickets | Page |
| VIS-E07 | Reservations (liste) | Page |
| VIS-E08 | Pass VIP et avantages | Page |
| VIS-E09 | Reservation (flux) | Page (multi-etapes) |
| VIS-E10 | Fiche evenement (connecte) | Page |
| VIS-E11 | Suivi d'activites | Page |
| VIS-E12 | Jeux | Page |
| VIS-E13 | Concours | Page |
| VIS-E14 | Mon compte | Page |
| VIS-E15 | Preferences notification | Page |

---

## 11. Ecrans partages et mutualisables

Certains ecrans sont **partages** ou **mutualisables** entre publics :

| Ecran partage | Publics concernes | Notes |
|---------------|-------------------|-------|
| **Landing (UNC-E01)** | UNC, ORG, EXP, VIS | Point d'entree unique, meme ecran |
| **Connexion (UNC-E12)** | UNC, ORG, EXP, VIS | Meme ecran, redirection selon role |
| **Inscription choix (UNC-E13)** | UNC → ORG, EXP, VIS | Choix du type puis formulaire dedie |
| **Fiche evenement** | UNC-E03, VIS-E10 | Meme structure, VIS-E10 ajoute services + CTAs connecte |
| **Programme public** | UNC-E03 (section), EXP-E11c, VIS-E10 (section) | Meme composant, contextes differents |
| **Plan de salle (lecture)** | ORG-E16, EXP-E11b | Lecture seule, ORG voit tout, EXP voit son stand |
| **Mentions legales (UNC-E14)** | Tous | Accessible depuis tout footer |

---

## 12. Voir aussi

- [JayFestival - Parcours Utilisateurs Schema Flux](./reference/JayFestival%20-%20Parcours%20Utilisateurs%20Schema%20Flux.md) — Flux metier par type d'utilisateur
- [JayFestival - Specification UI Conforme Catakana](./JayFestival%20-%20Specification%20UI%20Conforme%20Catakana.md) — Specifications detaillees des composants UI
- [JayFestival - Reference UI Transcription Catakana](./JayFestival%20-%20Reference%20UI%20Transcription%20Catakana.md) — Transcription UI Catakana vers Dioxus
- [JayFestival - Analyse Approfondie Catakana Orga](./JayFestival%20-%20Analyse%20Approfondie%20Catakana%20Orga.md) — Analyse technique Catakana
- **Ecrans par public** :
  - [Organisateurs - Ecrans et cycle](./publics/Organisateurs/Organisateurs%20-%20Ecrans%20et%20cycle.md)
  - [Exposants - Ecrans et cycle](./publics/Exposants/Exposants%20-%20Ecrans%20et%20cycle.md)
  - [Visiteurs - Ecrans et cycle](./publics/Visiteurs/Visiteurs%20-%20Ecrans%20et%20cycle.md)
  - [UtilisateurNonConnecte - Ecrans et cycle](./publics/UtilisateurNonConnecte/UtilisateurNonConnecte%20-%20Ecrans%20et%20cycle.md)
- [JayFestival - Bornage Implementation](./JayFestival%20-%20Bornage%20Implementation.md) — Perimetre par phase

---

**Document** : JayFestival — Plan du Service  
**Version** : 1.0  
**Date** : 2026-02-09  
**Statut** : Document de reference — plan du service, sitemap, mocks de navigation  
**Sources** : Schema de flux, documentation par public, Catakana_Orga
