MIYUKINI Framework - Nomenclature de la documentation
Objectif :
- Standardiser le nommage des fichiers de documentation
- En langue française
- Identifier rapidement la section concernée entre le framework, l'app en cours de dev, la documentation client (préfixe)
- Garantir un rangement cohérent dans des dossiers standards
- Faciliter la recherche, l’archivage et la maintenance

Cette règle s’applique à TOUTE documentation générée
automatiquement ou semi-automatiquement (IA ou humain).
---------------------------------------------------------
1. FORMAT DE NOMMAGE DES FICHIERS
---------------------------------------------------------
Format général :
<PREFIX> - <SUJET> <DETAIL_OPTIONNEL>.<ext>

Conventions :
- pas d’accents
- Majuscule permises

Exemple :
Miyukini Framework - Compte Utilisateur.md
---------------------------------------------------------
2. PREFIXES (CADRE DU DOCUMENT)
---------------------------------------------------------
Un SEUL préfixe obligatoire par document
à définir en fonction du cadre d'emploi. Est-ce pour le framework, l'app en cours de dev, didacticiel/mode d'emploi pour le client?
---------------------------------------------------------
3. SUJET
---------------------------------------------------------
Sujet principal du document
Doit être précis et orienté module ou fonctionnalité
Exemples :
Fonctionnalité Authentication
Module Facturation
Fonctionnalité Paiement
Module Notifications
---------------------------------------------------------
4. DETAIL OPTIONNEL
---------------------------------------------------------
Facultatif mais recommandé si nécessaire
Sert à éviter les fichiers fourre-tout
Exemples :
V1
Alpha
Beta
MVP
Medge_cases
Supabase
Stripe
Offline
PWA
Localstorage
---------------------------------------------------------
7. EXTENSIONS AUTORISÉES
---------------------------------------------------------
.md    = Documentation principale (PRIORITAIRE)
.txt   = Notes brutes
.pdf   = Export figé
.drawio= Schémas
.json  = Spécifications machine
.yaml  = Config / infra
.csv   = Données de référence
---------------------------------------------------------
8. ARBORESCENCE STANDARD DES DOSSIERS
---------------------------------------------------------
docs/
│
├── readme/
│   └── readme.md
│
├── audit/
│   ├── Miyukini Framework - audit du framework.md
|
├── framework/
│   ├── Miyukini Framework - Compte Utilisateur.md
|
├── architecture/
│   └── diagrams/
│
├── specifications/
│   ├── backend/
│   ├── frontend/
│   ├── api/
│   └── database/
│
├── guides/
│   ├── dev/
│   ├── user/
│   └── admin/
│
├── reference/
│   ├── api/
│   ├── schemas/
│   └── types/
│
├── ux_ui/
│   ├── ux/
│   ├── ui/
│   └── design_system/
│
├── qa/
│   ├── tests/
│   ├── checklists/
│   └── reports/
│
├── ps/
│   ├── deploy/
│   ├── monitoring/
│   └── incidents/
│
├── security/
│   └── sec_policies__human.md
│
├── legal/
│   └── legal_documents__human.md
│
└── archive/
    └── deprecated/
---------------------------------------------------------
9. REGLES DE RANGEMENT AUTOMATIQUE
---------------------------------------------------------
Cursor / agent IA DOIT :
1. Déduire le dossier
2. Déduire le sous-dossier
3. Refuser la création si le nom ne respecte pas la règle
4. Ne JAMAIS écraser un fichier existant sans versioning

Versioning :
- Ajouter _vX si conflit
---------------------------------------------------------
10. REGLES DE QUALITÉ
---------------------------------------------------------
Tout document doit :
- Avoir un titre clair en H1
- Avoir une section "Contexte"
- Avoir une section "Portée / Scope"
- Être orienté action ou décision
- Titre de l'app en cours de dev dans le README général
Les documents "brain_" ne sont PAS contractuels
---------------------------------------------------------
11. EXEMPLES COMPLETS
---------------------------------------------------------
Miyukini Framework - Compte Utilisateur.md
Le Patate - Module Agenda.md
Catakana - Module Facturation.md
Miyukini Framework - Compte Utilisateur.md
Miyukini Website - Module Commission.md