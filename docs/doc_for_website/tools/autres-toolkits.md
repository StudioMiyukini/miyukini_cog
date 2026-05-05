# Autres Toolkits

## Toolkits Complémentaires

Cette page présente les autres toolkits disponibles dans Miyukini, chacun avec une responsabilité spécifique.

---

## MiyuStore

### Toolkit de Stockage de Fichiers

**Fonction** : Gestion des fichiers et médias en dehors de la base de données.

| Capacité | Description |
|----------|-------------|
| Upload | Réception de fichiers |
| Download | Récupération de fichiers |
| Métadonnées | Informations sur les fichiers |
| Organisation | Dossiers et catégories |

**Cas d'usage** : Stocker les images de profil, documents PDF, pièces jointes.

---

## MiyuText

### Toolkit de Manipulation de Texte

**Fonction** : Traitement et transformation de texte.

| Capacité | Description |
|----------|-------------|
| Formatage | Markdown, HTML |
| Sanitization | Nettoyage sécurisé |
| Recherche | Full-text search |
| Traduction | Interface i18n |

**Cas d'usage** : Rendu de contenu Markdown, recherche dans les documents.

---

## MiyuValidate

### Toolkit de Validation de Données

**Fonction** : Vérification et validation des entrées.

| Capacité | Description |
|----------|-------------|
| Format | Email, téléphone, URL |
| Structure | JSON Schema, règles métier |
| Sanitization | Échappement, nettoyage |
| Limites | Longueur, plages de valeurs |

**Cas d'usage** : Valider les formulaires utilisateur, vérifier les imports.

---

## MiyuWidgets

### Toolkit de Composants UI

**Fonction** : Fournir des composants d'interface utilisateur réutilisables.

| Capacité | Description |
|----------|-------------|
| Composants | Boutons, formulaires, tables |
| Thèmes | Styles personnalisables |
| Accessibilité | ARIA, navigation clavier |
| Responsive | Adaptation écrans |

**Cas d'usage** : Construire les interfaces des services.

---

## MiyuTreasury

### Toolkit de Gestion Financière

**Fonction** : Opérations financières et comptables.

| Capacité | Description |
|----------|-------------|
| Calculs | Montants, TVA, arrondis |
| Devises | Conversion, formatage |
| Rapports | Bilans, exports |
| Conformité | Règles comptables |

**Cas d'usage** : Calculs dans JayKonta, génération de factures.

---

## MiyuStory

### Toolkit de Gestion de Contenu

**Fonction** : Organisation et structuration du contenu narratif.

| Capacité | Description |
|----------|-------------|
| Articles | Création, édition |
| Versions | Historique des modifications |
| Catégories | Organisation hiérarchique |
| Publication | États (brouillon, publié) |

**Cas d'usage** : Blog, documentation, pages de présentation.

---

## MiyuClock

### Toolkit de Gestion du Temps

**Fonction** : Manipulation des dates et horaires.

| Capacité | Description |
|----------|-------------|
| Parsing | Lecture de dates |
| Calculs | Durées, intervalles |
| Fuseaux | Conversion timezone |
| Formatage | Affichage localisé |

**Cas d'usage** : Planification dans JayRDV, agenda universel JayKoa.

---

## MiyuSocialProfile

### Toolkit de Profils Utilisateurs

**Fonction** : Gestion des profils sociaux.

| Capacité | Description |
|----------|-------------|
| Profil | Création, édition |
| Avatar | Gestion d'images |
| Préférences | Paramètres utilisateur |
| Visibilité | Contrôle de confidentialité |

**Cas d'usage** : Profils utilisateurs dans tous les services.

---

## MiyuSocialModeration

### Toolkit de Modération

**Fonction** : Modération de contenu généré par les utilisateurs.

| Capacité | Description |
|----------|-------------|
| Signalement | Réception de reports |
| Filtrage | Mots interdits, spam |
| Actions | Masquer, supprimer |
| Historique | Audit des décisions |

**Cas d'usage** : Modération dans Jay1Tribu, commentaires.

---

## Caractéristiques Communes

Tous ces toolkits partagent :

### Architecture Standard

```
miyu-<toolkit>/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── admin_cell.rs
│   ├── context.rs
│   └── errors.rs
└── contracts/
    └── governance/
```

### Principes

| Principe | Application |
|----------|-------------|
| Non-autonomie | Pas de décision propre |
| Déterminisme | Résultats prévisibles |
| Isolation | Pas d'accès croisé |
| Sécurité | `unsafe_code = "forbid"` |

### Invocation

Tous les toolkits sont invoqués uniquement via BondingBrother :

```
Core ──► BondingBrother ──► Toolkit
                              │
                              ▼
                          Résultat
                              │
Core ◄── BondingBrother ◄─────┘
```

### Contrats

Chaque toolkit possède :
- **Contrat de Frontière** : Ce qu'il peut/ne peut pas faire
- **Contrat de Gouvernance** : Conformité aux Lois
- **Contrat d'Intégration** : API et interfaces

## Liste Complète

Pour une documentation détaillée de chaque toolkit, consultez :
- La documentation technique des crates
- Les contrats dans `docs/tools/`
- Le code source dans `crates/miyu*/`
