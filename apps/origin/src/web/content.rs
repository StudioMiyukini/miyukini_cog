//! Contenu dynamique du site web Origin.
//!
//! Gère les articles de blog, annonces et téléchargements.
//!
//! Note : Code préparé pour fonctionnalités futures (filtres et méthodes).
#![allow(dead_code)]

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

// ═══════════════════════════════════════════════════════════════════════════
// Articles de blog
// ═══════════════════════════════════════════════════════════════════════════

/// Article de blog.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogPost {
    /// Identifiant unique (slug).
    pub id: String,
    /// Titre de l'article.
    pub title: String,
    /// Résumé.
    pub summary: String,
    /// Contenu complet (Markdown).
    pub content: String,
    /// Auteur.
    pub author: String,
    /// Date de publication.
    pub published_at: DateTime<Utc>,
    /// Tags.
    pub tags: Vec<String>,
    /// Image de couverture (optionnel).
    pub cover_image: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════════════
// Annonces
// ═══════════════════════════════════════════════════════════════════════════

/// Type d'annonce.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AnnouncementType {
    /// Nouvelle version.
    Release,
    /// Alerte de sécurité.
    Security,
    /// Maintenance.
    Maintenance,
    /// Information générale.
    Info,
}

/// Annonce globale.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Announcement {
    /// Identifiant unique.
    pub id: String,
    /// Type d'annonce.
    pub announcement_type: AnnouncementType,
    /// Titre.
    pub title: String,
    /// Contenu.
    pub content: String,
    /// Date de publication.
    pub published_at: DateTime<Utc>,
    /// Date d'expiration (optionnel).
    pub expires_at: Option<DateTime<Utc>>,
    /// Priorité (0 = normal, 1 = important, 2 = critique).
    pub priority: u8,
}

// ═══════════════════════════════════════════════════════════════════════════
// Services (catalogue public)
// ═══════════════════════════════════════════════════════════════════════════

/// Catégorie de service.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ServiceCategory {
    /// Outils utilitaires.
    Outils,
    /// Développement et programmation.
    Developpement,
    /// Commerce et vente.
    Commerce,
    /// Jeux et divertissement.
    Jeux,
    /// Style de vie et productivité.
    StyleDeVie,
    /// Social et communication.
    Social,
    /// Autre.
    Autre,
}

impl ServiceCategory {
    /// Retourne le libellé français de la catégorie.
    pub fn label(&self) -> &'static str {
        match self {
            Self::Outils => "Outils",
            Self::Developpement => "Développement",
            Self::Commerce => "Commerce",
            Self::Jeux => "Jeux",
            Self::StyleDeVie => "Style de vie",
            Self::Social => "Social",
            Self::Autre => "Autre",
        }
    }

    /// Retourne toutes les catégories.
    pub fn all() -> &'static [ServiceCategory] {
        &[
            Self::Outils,
            Self::Developpement,
            Self::Commerce,
            Self::Jeux,
            Self::StyleDeVie,
            Self::Social,
            Self::Autre,
        ]
    }
}

/// Type de service.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ServiceType {
    /// Service Miyukini officiel.
    Officiel,
    /// Service tiers.
    Tiers,
}

impl ServiceType {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Officiel => "Officiel",
            Self::Tiers => "Tiers",
        }
    }
}

/// Fiche complète d'un service pour le catalogue.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInfo {
    /// Identifiant unique (slug).
    pub id: String,
    /// Nom du service.
    pub name: String,
    /// Description courte (1 ligne).
    pub short_description: String,
    /// Description complète (Markdown).
    pub full_description: String,
    /// Catégorie.
    pub category: ServiceCategory,
    /// Éditeur / développeur.
    pub editor: String,
    /// Version COG minimale.
    pub cog_version: String,
    /// Type de service (officiel/tiers).
    pub service_type: ServiceType,
    /// Licence d'utilisation.
    pub license: String,
    /// Prix (0.0 = gratuit).
    pub price: f64,
    /// Date de parution.
    pub release_date: DateTime<Utc>,
    /// URL du site officiel.
    pub website_url: Option<String>,
    /// URL de téléchargement/achat.
    pub download_url: Option<String>,
    /// URLs des captures d'écran.
    pub screenshots: Vec<String>,
    /// URL de la petite bannière.
    pub banner_url: Option<String>,
}

impl ServiceInfo {
    /// Retourne si le service est gratuit.
    pub fn is_free(&self) -> bool {
        self.price <= 0.0
    }

    /// Retourne le prix formaté.
    pub fn price_label(&self) -> String {
        if self.is_free() {
            "Gratuit".to_string()
        } else {
            format!("{:.2} €", self.price)
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Téléchargements
// ═══════════════════════════════════════════════════════════════════════════

/// Catégorie de téléchargement.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum DownloadCategory {
    /// COG complet.
    Cog,
    /// Cores système.
    Cores,
    /// Toolkit individuel.
    Toolkit,
    /// Service/Opérateur.
    Service,
    /// Outils de développement.
    DevTools,
}

/// Plateforme cible.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Platform {
    Windows,
    Linux,
    MacOs,
    Android,
    Ios,
    Universal,
}

/// Téléchargement disponible.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Download {
    /// Identifiant unique.
    pub id: String,
    /// Nom du package.
    pub name: String,
    /// Description.
    pub description: String,
    /// Version.
    pub version: String,
    /// Catégorie.
    pub category: DownloadCategory,
    /// Plateformes supportées.
    pub platforms: Vec<Platform>,
    /// URL de téléchargement.
    pub download_url: String,
    /// Taille en octets.
    pub size_bytes: u64,
    /// Hash SHA-256.
    pub sha256: String,
    /// Date de publication.
    pub published_at: DateTime<Utc>,
    /// Notes de version.
    pub release_notes: String,
    /// Version minimale des Cores requise.
    pub min_core_version: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════════════
// Documentation
// ═══════════════════════════════════════════════════════════════════════════

/// Section de documentation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocSection {
    /// Identifiant (slug).
    pub id: String,
    /// Titre.
    pub title: String,
    /// Description.
    pub description: String,
    /// Icône (emoji ou nom d'icône).
    pub icon: String,
    /// Articles dans cette section.
    pub articles: Vec<DocArticle>,
}

/// Article de documentation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocArticle {
    /// Identifiant (slug).
    pub id: String,
    /// Titre.
    pub title: String,
    /// Contenu (Markdown).
    pub content: String,
    /// Dernière mise à jour.
    pub updated_at: DateTime<Utc>,
}

// ═══════════════════════════════════════════════════════════════════════════
// Gestionnaire de contenu
// ═══════════════════════════════════════════════════════════════════════════

/// Gestionnaire de contenu du site web.
pub struct ContentManager {
    /// Articles de blog.
    blog_posts: Arc<RwLock<Vec<BlogPost>>>,
    /// Annonces.
    announcements: Arc<RwLock<Vec<Announcement>>>,
    /// Téléchargements.
    downloads: Arc<RwLock<Vec<Download>>>,
    /// Documentation.
    doc_sections: Arc<RwLock<Vec<DocSection>>>,
    /// Services (catalogue).
    services: Arc<RwLock<Vec<ServiceInfo>>>,
}

impl ContentManager {
    /// Crée un nouveau gestionnaire avec le contenu par défaut.
    #[must_use]
    pub fn new() -> Self {
        let manager = Self {
            blog_posts: Arc::new(RwLock::new(Vec::new())),
            announcements: Arc::new(RwLock::new(Vec::new())),
            downloads: Arc::new(RwLock::new(Vec::new())),
            doc_sections: Arc::new(RwLock::new(Vec::new())),
            services: Arc::new(RwLock::new(Vec::new())),
        };

        // Charger le contenu initial dans un spawn
        let manager_clone = Self {
            blog_posts: Arc::clone(&manager.blog_posts),
            announcements: Arc::clone(&manager.announcements),
            downloads: Arc::clone(&manager.downloads),
            doc_sections: Arc::clone(&manager.doc_sections),
            services: Arc::clone(&manager.services),
        };

        tokio::spawn(async move {
            manager_clone.load_default_content().await;
        });

        manager
    }

    /// Charge le contenu par défaut selon le sitemap complet.
    async fn load_default_content(&self) {
        self.load_blog_posts().await;
        self.load_announcements().await;
        self.load_downloads().await;
        self.load_documentation().await;
        self.load_services().await;
    }

    /// Charge les articles de blog.
    async fn load_blog_posts(&self) {
        let mut posts = self.blog_posts.write().await;

        posts.push(BlogPost {
            id: "miyukini-cog-announcement".to_string(),
            title: "Bienvenue dans l'écosystème Miyukini COG".to_string(),
            summary: "Découvrez Miyukini COG, un environnement de gouvernance orchestré par des Cores, conçu pour l'autonomie et la souveraineté numérique.".to_string(),
            content: include_str!("../../content/blog/welcome.md").to_string(),
            author: "Équipe Miyukini".to_string(),
            published_at: Utc::now(),
            tags: vec!["annonce".to_string(), "introduction".to_string(), "cog".to_string()],
            cover_image: None,
        });

        posts.push(BlogPost {
            id: "mws-origin-launch".to_string(),
            title: "Lancement d'Origin — Le cœur du Webway".to_string(),
            summary: "Origin est maintenant en ligne ! Découvrez comment connecter votre COG au réseau Miyukini.".to_string(),
            content: include_str!("../../content/blog/origin-launch.md").to_string(),
            author: "Équipe Miyukini".to_string(),
            published_at: Utc::now() - chrono::Duration::hours(2),
            tags: vec!["mws".to_string(), "origin".to_string(), "lancement".to_string()],
            cover_image: None,
        });
    }

    /// Charge les annonces.
    async fn load_announcements(&self) {
        let mut announcements = self.announcements.write().await;

        announcements.push(Announcement {
            id: "origin-online".to_string(),
            announcement_type: AnnouncementType::Release,
            title: "Origin v0.1.0 est en ligne".to_string(),
            content: "Le serveur Origin est maintenant opérationnel. Tous les COGs peuvent se connecter au réseau Miyukini Webway.".to_string(),
            published_at: Utc::now(),
            expires_at: None,
            priority: 1,
        });

        announcements.push(Announcement {
            id: "beta-testing".to_string(),
            announcement_type: AnnouncementType::Info,
            title: "Phase de test bêta".to_string(),
            content: "Nous sommes actuellement en phase de test. Merci de signaler tout problème via le système de feedback.".to_string(),
            published_at: Utc::now() - chrono::Duration::hours(1),
            expires_at: Some(Utc::now() + chrono::Duration::days(30)),
            priority: 0,
        });
    }

    /// Charge les téléchargements.
    async fn load_downloads(&self) {
        let mut downloads = self.downloads.write().await;

        // COG v0.2.0 - Installateur Windows
        downloads.push(Download {
            id: "miyukini-cog-windows".to_string(),
            name: "Miyukini Central — Installateur Windows".to_string(),
            description: "Installateur Miyukini Central v0.2.0 pour Windows — Inclut Central, KindMother, voix Miou et tous les Services (JayKonta, JayKoa, Jay1Tribu, JayManga, MiyukiniWatch, MiyuClicker, Lord of the Castle). Installation sans droits admin dans AppData.".to_string(),
            version: "0.2.0".to_string(),
            category: DownloadCategory::Cog,
            platforms: vec![Platform::Windows],
            download_url: "/files/MiyukiniCentral-0.2.0-Setup.exe".to_string(),
            size_bytes: 6_522_680,
            sha256: "".to_string(),
            published_at: Utc::now(),
            release_notes: "v0.2.0 — Service Market, refonte Central, JayManga, installation AppData sans admin.".to_string(),
            min_core_version: None,
        });

        // Central Mobile v0.1.0 - APK Android (client léger via COG Bridge)
        downloads.push(Download {
            id: "miyukini-central-mobile-android".to_string(),
            name: "Miyukini Central Mobile — APK Android".to_string(),
            description: "Client léger Android pour piloter votre COG Host à distance. Pairing par QR-code, voix Miou, accès Market et Services. Nécessite un Central installé sur PC.".to_string(),
            version: "0.1.0".to_string(),
            category: DownloadCategory::Cog,
            platforms: vec![Platform::Android],
            download_url: "/files/MiyukiniCentralMobile-0.1.0.apk".to_string(),
            size_bytes: 18_874_368,
            sha256: String::new(),
            published_at: Utc::now(),
            release_notes: "v0.1.0 — Première release Android : pairing COG Bridge, CentralRemote WebSocket, Market intégré.".to_string(),
            min_core_version: Some("0.2.0".to_string()),
        });
    }

    /// Charge la documentation complète selon le sitemap.
    async fn load_documentation(&self) {
        let mut sections = self.doc_sections.write().await;

        // ═══════════════════════════════════════════════════════════════════════
        // VUE D'ENSEMBLE
        // ═══════════════════════════════════════════════════════════════════════
        sections.push(DocSection {
            id: "overview".to_string(),
            title: "Vue d'ensemble".to_string(),
            description: "Introduction générale à Miyukini COG et son écosystème".to_string(),
            icon: "📖".to_string(),
            articles: vec![
                DocArticle {
                    id: "introduction".to_string(),
                    title: "Introduction à Miyukini COG".to_string(),
                    content: r#"# Introduction à Miyukini COG

**Miyukini COG** (Core-Orchestrated Governance) est un environnement applicatif révolutionnaire conçu pour garantir votre **souveraineté numérique**.

## Qu'est-ce qu'un COG ?

Un COG n'est pas un système d'exploitation traditionnel. C'est un **environnement de gouvernance** où 8 Cores orchestrent toutes les opérations, garantissant :

- **Autonomie complète** — Fonctionne sans dépendance externe
- **Souveraineté des données** — Vos données restent sur votre matériel
- **Gouvernance transparente** — 8 Lois d'Autonomie immuables
- **Fédération optionnelle** — Connexion sécurisée via le Miyukini Webway System

## Pourquoi Miyukini ?

Les solutions cloud traditionnelles vous rendent dépendant de services externes. Miyukini prend le contrepied :

| Aspect | Cloud traditionnel | Miyukini COG |
|--------|-------------------|--------------|
| Données | Chez le fournisseur | Chez vous |
| Disponibilité | Dépend d'Internet | Fonctionne hors ligne |
| Coût | Abonnement mensuel | Hardware seul |
| Contrôle | Limité | Total |

## Composants principaux

1. **Cores** — Les 8 décideurs immuables (StrongFather, KindMother, etc.)
2. **Toolkits** — Outils spécialisés gouvernés par les Cores
3. **Opérateurs** — Exécutants des décisions
4. **Services** — Applications utilisateur (JayKoa, JayKonta, etc.)

## Prochaines étapes

- [Installation](/docs/overview/installation) — Installer votre premier COG
- [Architecture](/docs/architecture/stack) — Comprendre la pyramide des strates
- [Les Cores](/docs/cores/strongfather) — Découvrir les 8 Cores"#.to_string(),
                    updated_at: Utc::now(),
                },
                DocArticle {
                    id: "installation".to_string(),
                    title: "Installation".to_string(),
                    content: r#"# Installation de Miyukini COG

## Prérequis système

### Matériel minimum
- **Processeur** : x64 ou ARM64
- **RAM** : 4 Go minimum (8 Go recommandé)
- **Stockage** : 2 Go d'espace libre
- **Réseau** : Optionnel (mode Lone disponible)

### Systèmes supportés
- Windows 10/11 (x64)
- Linux (Ubuntu 22.04+, Debian 12+, Fedora 38+)
- macOS 12+ (Intel et Apple Silicon)

## Installation rapide

### Windows
```powershell
# Télécharger l'installateur
Invoke-WebRequest -Uri "https://miyukini.com/files/MiyukiniCentral-0.2.0-Setup.exe" -OutFile "MiyukiniCentral-Setup.exe"

# Lancer l'installation guidée
.\MiyukiniCentral-Setup.exe
```

### Linux
```bash
# Télécharger et extraire
curl -L https://origin.miyukini.net/downloads/miyukini-cog-linux.tar.gz | tar xz

# Lancer
./miyukini-cog/miyukini-central
```

## Premier démarrage

Au premier lancement, Central vous guide à travers le **Rite d'Entrée** :

1. **Création du profil administrateur** — Votre compte principal
2. **Configuration des Cores** — Vérification de l'intégrité
3. **Mode réseau** — Connecté (MWS) ou Lone (isolé)

## Vérification de l'installation

```bash
# Vérifier la version
miyukini-central --version

# Tester les Cores
miyukini-central --check-cores
```

## Prochaines étapes

- [Démarrage rapide](/docs/overview/quickstart)
- [Configuration](/docs/overview/configuration)"#.to_string(),
                    updated_at: Utc::now(),
                },
                DocArticle {
                    id: "quickstart".to_string(),
                    title: "Démarrage rapide".to_string(),
                    content: r#"# Démarrage rapide

Ce guide vous permet d'être opérationnel avec Miyukini COG en 5 minutes.

## 1. Lancer Central

Central est le hub de votre COG, semblable à Steam pour les jeux :

```bash
miyukini-central
```

## 2. Créer votre premier profil

L'écran de connexion apparaît. Si c'est un **COG vierge** :
1. Cliquez sur "Créer un compte"
2. Renseignez email et mot de passe
3. Choisissez un pseudonyme

## 3. Explorer les Services

Dans la bibliothèque, vous trouverez les Services installés :
- **JayKonta** — Comptabilité
- **JayKoa** — Calendrier

## 4. Se connecter au réseau (optionnel)

Dans l'onglet "Communauté" :
1. Cliquez sur "Se connecter"
2. Le protocole de conformité s'exécute (Origin → Relay → Tracker)
3. Une fois "En ligne", vous pouvez découvrir d'autres COGs

## 5. Mode Lone

Si vous préférez rester hors réseau :
1. Activez le toggle "Mode Lone"
2. Vos données restent 100% locales
3. Aucune connexion externe

## Félicitations !

Vous avez un COG fonctionnel. Explorez maintenant :
- [Les Services](/docs/services/overview)
- [Le réseau MWS](/docs/mws/overview)
- [L'architecture](/docs/architecture/stack)"#.to_string(),
                    updated_at: Utc::now(),
                },
            ],
        });

        // ═══════════════════════════════════════════════════════════════════════
        // ARCHITECTURE ET STACK
        // ═══════════════════════════════════════════════════════════════════════
        sections.push(DocSection {
            id: "architecture".to_string(),
            title: "Architecture et Stack".to_string(),
            description: "La pyramide des strates et l'organisation technique de Miyukini COG"
                .to_string(),
            icon: "🏛️".to_string(),
            articles: vec![
                DocArticle {
                    id: "stack".to_string(),
                    title: "Stack technique".to_string(),
                    content: r#"# Stack technique Miyukini

## Langage principal : Rust

Miyukini COG est entièrement écrit en **Rust**, choisi pour :
- **Performance** — Comparable au C/C++
- **Sécurité mémoire** — Pas de buffer overflow
- **Concurrence** — Gestion native des threads
- **Écosystème** — Cargo, crates.io

## Stack par composant

### Cores et Toolkits
- Rust pur sans dépendances runtime
- Sérialisation avec `serde`
- Async avec `tokio`

### Base de données (KindMother)
- **SQLite** (rusqlite) pour le stockage local
- **libSQL** pour la réplication optionnelle
- Chiffrement transparent avec SQLCipher

### Interface utilisateur (Central)
- **Dioxus** — Framework UI déclaratif
- Rendu natif via WGPU
- Pas de webview (performance maximale)

### Réseau (MWS)
- **tokio** pour l'async networking
- **rustls** pour TLS natif
- Protocole binaire personnalisé

## Organisation du code

```
miyukini-cog/
├── crates/           # Bibliothèques Rust
│   ├── miyukini-kernel/   # Noyau et utilitaires
│   ├── kindmother/        # Core de persistance
│   ├── strongfather/      # Core de décision
│   └── ...
├── apps/             # Applications
│   ├── central/      # Hub principal
│   └── origin/       # Serveur MWS
└── docs/             # Documentation
```"#
                        .to_string(),
                    updated_at: Utc::now(),
                },
                DocArticle {
                    id: "pyramid".to_string(),
                    title: "Pyramide des strates".to_string(),
                    content: r#"# Pyramide des strates

L'architecture Miyukini est organisée en 10 strates hiérarchiques (0-9).

## Vue d'ensemble

| Strate | Nom | Rôle |
|--------|-----|------|
| 0 | **Kernel** | Utilitaires fondamentaux |
| 1 | **Cores** | Les 8 décideurs immuables |
| 2 | **Toolkits** | Outils spécialisés gouvernés |
| 3 | **Operators** | Exécutants des décisions |
| 4 | **Services** | Applications utilisateur |
| 5 | **Interfaces** | Points d'entrée (Central, CLI) |
| 6 | **Network** | Communication (MWS) |
| 7 | **External** | Intégrations tierces |
| 8 | **User** | Configuration utilisateur |
| 9 | **Runtime** | État d'exécution |

## Règles de dépendance

1. Une strate ne peut dépendre que des strates **inférieures**
2. La strate **Cores (1)** est **immuable** après certification
3. Les **Toolkits (2)** ne prennent pas de décisions, ils exécutent

## Flux de décision

```
Utilisateur demande une action
        ↓
    Service (4)
        ↓
    Opérateur (3)
        ↓
    Toolkit (2) — Exécute
        ↓
    Core (1) — DÉCIDE
        ↓
    Kernel (0) — Support
```

## Les 8 Lois d'Autonomie

Ces lois gouvernent l'ensemble de l'architecture :

1. **LOI-1** : Aucune dépendance externe critique à l'exécution
2. **LOI-2** : Le système accepte l'isolement comme état normal
3. **LOI-3** : L'état local est souverain
4. **LOI-4** : Pas de temps global requis
5. **LOI-5** : Le coût doit être proportionnel au hardware
6. **LOI-6** : L'autonomie n'empêche pas la fédération
7. **LOI-7** : La strate Cores est immuable
8. **LOI-8** : Migration = diplomatie entre environnements"#
                        .to_string(),
                    updated_at: Utc::now(),
                },
            ],
        });

        // ═══════════════════════════════════════════════════════════════════════
        // LES 9 CORES (8 en Strate 4 + BondingBrother en Strate 5)
        // ═══════════════════════════════════════════════════════════════════════
        sections.push(DocSection {
            id: "cores".to_string(),
            title: "Les Cores".to_string(),
            description: "Les 9 Cores système qui gouvernent votre COG — décision, persistance, médiation, observation, capacités, frontières, cycle de vie, sécurité, intervention humaine".to_string(),
            icon: "⚙️".to_string(),
            articles: vec![
                DocArticle {
                    id: "overview".to_string(),
                    title: "Vue d'ensemble des Cores".to_string(),
                    content: r#"# Les 9 Cores — Vue d'ensemble

Les Cores sont les **moteurs conceptuels** de gouvernance du système. Ils décident ou gouvernent, mais **n'exécutent jamais**.

## Règle fondamentale

> **Les Cores décident ou gouvernent, mais n'exécutent jamais.**

## Tableau des 9 Cores

| Core | Strate | Domaine | Question fondamentale |
|------|--------|---------|----------------------|
| [StrongFather](/docs/cores/strongfather) | 4 | Décision stratégique | "Devrait-on faire cette action ?" |
| [KindMother](/docs/cores/kindmother) | 4 | Données et persistance | "Comment les données sont-elles persistées ?" |
| [BondingBrother](/docs/cores/bondingbrother) | 5 | Médiation | "Comment traduire cette intention pour les autorités ?" |
| [CaringNanny](/docs/cores/caringnanny) | 4 | Observation d'état | "Dans quel état est le système ?" |
| [MasterButler](/docs/cores/masterbutler) | 4 | Capacités et permissions | "Qu'est-ce qui est possible ?" |
| [BorderGuard](/docs/cores/borderguard) | 4 | Frontières et confiance | "Où sont les frontières ?" |
| [EverBuddy](/docs/cores/everbuddy) | 4 | Cycle de vie et évolution | "Comment évoluer sans rompre ?" |
| [WorrySentinel](/docs/cores/worrysentinel) | 4 | Gouvernance de sécurité | "Quel niveau de sécurité ?" |
| [TAMR](/docs/cores/tamr) | 4 | Intervention humaine | "Quand l'humain intervient-il ?" |

## Strates

- **Strate 4** : Huit Cores gouvernent les décisions, données, état, capacités, frontières, évolution, sécurité et intervention humaine.
- **Strate 5** : BondingBrother assure la médiation entre les Opérateurs et les Cores.

## Immuabilité (LOI-7)

La strate Cores est **immuable** à version donnée. Toute évolution se fait par la création d'un nouvel environnement complet. Les Cores (StrongFather, KindMother, CaringNanny, MasterButler, BorderGuard, EverBuddy, WorrySentinel, TAMR) sont figés après certification."#.to_string(),
                    updated_at: Utc::now(),
                },
                DocArticle {
                    id: "strongfather".to_string(),
                    title: "StrongFather".to_string(),
                    content: r#"# StrongFather — Le Décideur Stratégique

**Core de décision** (Strate 4). Moteur de décision stratégique et politique. **Émetteur des Mandats de Permission.**

## Question fondamentale

> *"Devrait-on faire cette action ?"*

## Rôle

Décider si une action devrait être faite, **sans jamais l'exécuter**.

## Responsabilités clés

- **Décision stratégique** — Évalue les intentions et émet ou refuse les Mandats
- **Validation des Contrats d'Équipe** — Vérifie la cohérence des règles de collaboration
- **Émission des Mandats de Permission** — Autorisation déléguée, temporaire et encadrée
- **Révocation des mandats** — En cas de violation ou de changement de contexte
- **Architecture structurelle** — Définit les politiques globales (ressources, sécurité)

## Invariants clés

- Ne possède **jamais** d'autorité d'exécution
- Ne modifie **jamais** un état ou un fait directement
- Décision ≠ Exécution — StrongFather décide, d'autres exécutent

## Collaborations

| Core | Rôle dans la collaboration |
|------|---------------------------|
| KindMother | Persistance des décisions et des Mandats |
| BorderGuard | Application des politiques de sécurité et frontières |
| MasterButler | Catalogue des capacités et permissions |
| BondingBrother | Réception des intentions, traduction en demandes |

## Mandat de Permission

Un Mandat émis par StrongFather permet à des Opérateurs de collaborer sans repasser en permanence par la gouvernance centrale. Il est :

- **Temporaire** — Durée limitée
- **Encadré** — Périmètre défini
- **Révocable** — À tout moment par StrongFather

## Immuabilité

StrongFather, comme tous les Cores, est **immuable** après certification (LOI-7). Ses politiques peuvent évoluer via configuration, mais son code reste figé.

**Voir aussi :** Mandat de Permission, Contrat d'Équipe, KindMother"#.to_string(),
                    updated_at: Utc::now(),
                },
                DocArticle {
                    id: "kindmother".to_string(),
                    title: "KindMother".to_string(),
                    content: r#"# KindMother — La Gardienne des Données

**Core de données** (Strate 4). Autorité absolue des données et de la persistance.

## Question fondamentale

> *"Comment les données sont-elles persistées et synchronisées ?"*

## Rôle

Persistance, synchronisation, cohérence des données. KindMother est le **seul** point d'accès aux bases.

## Responsabilités

- **Stockage** — Seul accès aux bases SQLite, isolation par Service
- **WriteIntent** — Validation des intentions d'écriture avant toute modification
- **Chiffrement** — Protection des données au repos (AES-256)
- **Synchronisation** — Réplication mère/fille si activée
- **Intégrité** — Vérification lors de l'attestation d'environnement

## Principe absolu

> "KindMother est la seule à toucher aux données. Personne ne lit ni écrit directement dans les bases."

## Architecture de délégation

```
Service → Opérateur → Toolkit → KindMother → SQLite
```

Aucun contournement possible. Toute lecture ou écriture passe par le client KindMother.

## Bases gérées

- `central.db` — Profils, authentification, Mandats
- `jaykonta.db` — Comptabilité
- `jaykoa.db` — Calendrier

## Attestation d'environnement

Lors de la vérification MWS, KindMother vérifie l'intégrité des données persistantes et contribue à l'attestation signée envoyée au Relay.

## API (via kindmother-client)

```rust
let client = KindMotherClient::connect("127.0.0.1:50051", "jaykoa", "jaykoa").await?;
let entries = client.query("SELECT * FROM entries WHERE active = ?", vec!["1"]).await?;
```

**Voir aussi :** WriteIntent, Attestation d'environnement, StrongFather"#.to_string(),
                    updated_at: Utc::now(),
                },
                DocArticle {
                    id: "bondingbrother".to_string(),
                    title: "BondingBrother".to_string(),
                    content: r#"# BondingBrother — Le Médiateur Fraternel

**Core de médiation** (Strate 5). Interface fraternelle qui traduit les intentions des Opérateurs en demandes pour les Cores, et les réponses en résultats.

## Question fondamentale

> *"Comment traduire cette intention pour les autorités ?"*

## Rôle

**Médiation uniquement, jamais d'autorité.** BondingBrother ne décide jamais ; il transmet.

## Responsabilités

- **Traduction des intentions** — Opérateur → requête vers le Core approprié
- **Pont vers l'extérieur** — MWS, Bridge inter-COG, APIs externes
- **Interfaces utilisateur** — Coordination Central, Dioxus, thèmes
- **Protocoles** — REST, GraphQL, WebSocket, adaptation des formats
- **Internationalisation** — Support multilingue

## Règle fondamentale

> "Le Bridge ne fait jamais confiance, il transporte."

BondingBrother transporte les identités et autorisations sans les évaluer. L'évaluation relève des Cores.

## Collaborations MWS

| Rôle | Description |
|------|-------------|
| Participant Webway | Traduit intentions (annoncer, découvrir) en appels Opérateurs |
| Tracker Webway | Traduit intentions (accepter, rejeter) en appels Opérateurs |
| Bridge inter-COG | Canal diplomatique pour visites gouvernées |

## Flux typique (Central → Service)

```
Utilisateur clique "Ouvrir" → Hub envoie intention à BondingBrother
→ BondingBrother → StrongFather (évaluation, Mandat)
→ BondingBrother reçoit Mandat ou refus → Hub ouvre l'UI ou affiche erreur
```

## Ce que BondingBrother ne fait PAS

- Ne prend aucune décision d'autorisation
- Ne modifie aucun état
- Ne persiste rien
- Ne valide pas les politiques (BorderGuard, WorrySentinel le font)

**Voir aussi :** Bridge inter-COG, Collaboration Mandatée, Mandat de Permission"#.to_string(),
                    updated_at: Utc::now(),
                },
                DocArticle {
                    id: "caringnanny".to_string(),
                    title: "CaringNanny".to_string(),
                    content: r#"# CaringNanny — L'Observatrice d'État

**Core d'observation d'état** (Strate 4). Observateur du système qui détecte, classe et propage les états.

## Question fondamentale

> *"Dans quel état se trouve le système à un instant donné ?"*

## Rôle

Observer et rapporter l'état du système, **sans jamais modifier, décider, ou exécuter**.

## Responsabilités

- **Détection d'état** — Santé des composants, niveaux de dégradation
- **Classification** — États de confiance (T0 à T4), état de santé (healthy, degraded, unhealthy)
- **Propagation** — Notification aux consommateurs (Central, Services)
- **Cohérence** — Bloque les Outils si l'environnement est dégradé

## Ce que CaringNanny fait

- Observe l'état en temps réel
- Fournit des métriques (HealthStatus, métriques d'observation)
- Signale les anomalies et changements d'état
- Empêche l'exécution en environnement dégradé

## Ce que CaringNanny ne fait PAS

- Ne modifie aucun état
- Ne prend aucune décision
- N'exécute aucune action corrective

## États de santé

| État | Signification |
|------|---------------|
| Healthy | Système opérationnel normal |
| Degraded | Fonctionnalités réduites, vigilance requise |
| Unhealthy | Le système bloque les opérations sensibles |

## Intégration Central

Le Hub interroge CaringNanny (via BondingBrother) pour adapter l'affichage : masquer des actions sensibles, afficher des messages d'avertissement, limiter les fonctionnalités en mode dégradé.

## Outils gouvernés

CaringNanny gouverne les Outils d'observation : HealthChecker, MetricsCollector, ObserverSystemEvent.

**Voir aussi :** États de confiance, WorrySentinel, Niveaux de sécurité"#.to_string(),
                    updated_at: Utc::now(),
                },
                DocArticle {
                    id: "masterbutler".to_string(),
                    title: "MasterButler".to_string(),
                    content: r#"# MasterButler — Le Registre des Capacités

**Core de capacités** (Strate 4). Registre central des capacités et permissions du système.

## Question fondamentale

> *"Qu'est-ce qui est possible dans cet environnement ?"*

## Rôle

Catalogue des capacités, définition des permissions, découverte. MasterButler déclare **ce qui existe**, pas ce qui doit être fait.

## Responsabilités

- **Catalogue** — Inventaire des Outils et Kits d'Outils disponibles
- **Capability → Tool** — Lien entre capacité déclarée et Outil qui l'implémente
- **Permissions** — Définition des permissions d'accès par rôle, Mandat, contexte
- **Découverte** — Réponse aux requêtes "quels Services sont disponibles ?"

## Ce que MasterButler fait

- Déclare quels Tools existent
- Lie Capability → Tool
- Définit les permissions d'accès
- Fournit le catalogue au Central (via BondingBrother)

## Ce que MasterButler NE fait PAS

- N'implémente pas les Tools
- N'exécute pas les Tools
- Ne décide pas si un Tool doit être appelé (StrongFather)

## Flux Central

| Action | Flux |
|--------|------|
| Chargement catalogue | Hub → BondingBrother → MasterButler |
| Services disponibles | MasterButler répond avec métadonnées (nom, description, Opérateur(s), version) |
| Activation Service | StrongFather évalue et émet Mandat ; MasterButler fournit les métadonnées |

## Capability vs Permission

- **Capability** : Pouvoir technique intrinsèque à un composant
- **Permission** : Droit d'usage accordé dans un contexte (Mandat, rôle, etc.)

MasterButler gère les deux : le registre des capacités et les règles de permission.

**Voir aussi :** Capability, Permission, Outil, StrongFather"#.to_string(),
                    updated_at: Utc::now(),
                },
                DocArticle {
                    id: "borderguard".to_string(),
                    title: "BorderGuard".to_string(),
                    content: r#"# BorderGuard — Le Gardien des Frontières

**Core de frontières** (Strate 4). Définit les frontières du système et les niveaux de confiance.

## Question fondamentale

> *"Où sont les frontières du système, et quelles règles gouvernent leur franchissement ?"*

## Rôle

Définition conceptuelle des frontières, pas d'application directe. BorderGuard établit les **règles** ; d'autres les appliquent.

## Responsabilités

- **Frontières** — Définition des Boundary (external, internal, integration)
- **Règles de franchissement** — CrossingRule : min_trust_level, allowed
- **Niveaux de confiance** — Classification des acteurs (T0 à T4)
- **Identité** — Cohérence de l'identité et des certificats (attestation)
- **Migration** — Règles de passage entre environnements (LOI-8)

## Types de frontières

| Type | Description |
|------|-------------|
| External | Frontière avec le monde extérieur (réseau, utilisateurs externes) |
| Internal | Frontière entre composants internes du COG |
| Integration | Frontière avec des systèmes tiers (APIs, plugins) |

## Règles de franchissement

Chaque CrossingRule définit :
- `boundary_id` — Quelle frontière
- `min_trust_level` — Niveau de confiance minimum requis
- `allowed` — Autorisation ou interdiction

## Attestation d'environnement

Lors de la vérification MWS, BorderGuard vérifie la cohérence de l'identité et des certificats du COG.

## Collaboration MWS

Les décisions (accepter, rejeter, filtrer) impliquent BorderGuard avec WorrySentinel et StrongFather ; BondingBrother traduit en intentions.

## Migrations (LOI-8)

> "Migration = diplomatie entre environnements."

BorderGuard définit les protocoles de migration : Parentalité COG, Passeport, Visa.

**Voir aussi :** États de confiance, Migration, WorrySentinel, Niveaux de sécurité"#.to_string(),
                    updated_at: Utc::now(),
                },
                DocArticle {
                    id: "everbuddy".to_string(),
                    title: "EverBuddy".to_string(),
                    content: r#"# EverBuddy — Le Gardien du Cycle de Vie

**Core de cycle de vie** (Strate 4). Gouverne l'évolution des structures, des contrats, et des entités dans le temps.

## Question fondamentale

> *"Comment le système évolue-t-il sans jamais se rompre ?"*

## Rôle

Observer ce qui a été, ce qui est, et ce qui sera — **sans jamais exécuter de migration**. EverBuddy gouverne l'évolution ; il ne la fait pas.

## Responsabilités

- **Versions** — Gestion des versions (MAJOR.MINOR.PATCH), compatibilité
- **États de vie** — ACTIF, DÉPRÉCIÉ, RETIRÉ, BROUILLON
- **Dépréciation** — Périodes de transition, avertissements aux consommateurs
- **Compatibilité** — Niveaux de compatibilité entre versions (Compatible, Incompatible)
- **Évolution** — Comment migrer sans rupture

## États de vie

| État | Signification |
|------|---------------|
| **ACTIF** | En usage normal, stable, supporté |
| **DÉPRÉCIÉ** | Toujours fonctionnel, usage déconseillé, successeur en préparation |
| **RETIRÉ** | Plus disponible, migration obligatoire |
| **BROUILLON** | En développement, non publié |

## Règle d'évolution

> "Miyukini ne maintient pas le code à la place de l'humain. Il rend le code maintenable sans ambiguïté."

## Compatibilité COG

Deux COGs peuvent interagir si et seulement si leur `core_version.MAJOR` est identique. EverBuddy gouverne ces règles.

## Migration COG ( père/fils )

Le passage à des Cores plus récents suit un protocole gouverné par EverBuddy : parentalité, archivage par strates, migration DB, installation des Services compatibles.

## Outils gouvernés

EverBuddy gouverne les Outils de versioning : VersionManager, CompatibilityChecker, MigrationExecutor.

**Voir aussi :** Version des Cores, États de vie, Migration, Empreinte de version COG"#.to_string(),
                    updated_at: Utc::now(),
                },
                DocArticle {
                    id: "worrysentinel".to_string(),
                    title: "WorrySentinel".to_string(),
                    content: r#"# WorrySentinel — Le Gardien de la Sécurité

**Core de gouvernance de sécurité** (Strate 4). Gouverne les niveaux de sécurité et les états de confiance.

## Question fondamentale

> *"Quel niveau de sécurité et quel état de confiance sont applicables ?"*

## Rôle

Gouverner la sécurité **sans exécuter de contrôle technique**. WorrySentinel décide ; les Outils exécutent.

## Ce que WorrySentinel décide

- Niveau de confiance global (T0 Normal → T4 Blacklist)
- Niveau de sécurité actif
- Mode de fonctionnement autorisé (degradation, restriction, blocage)

## Ce que WorrySentinel ne décide PAS

- Des actions concrètes
- Des permissions détaillées (MasterButler)
- Des données (KindMother)

## États de confiance

| État | Description |
|------|-------------|
| T0 | Normal — Conformité complète |
| T1 | Méfiance légère — Anomalie mineure |
| T2 | Quarantaine — Comportement suspect |
| T3 | Blacklist temporaire |
| T4 | Blacklist permanente |

## Attestation d'environnement

WorrySentinel vérifie les checksums des binaires installés et produit l'attestation signée envoyée au Relay lors du REGISTER.

## Responsabilité Outils

- Niveau de sécurité requis
- Blocage en cas de menace
- Audit des accès et des incidents

## Dégradation

WorrySentinel gouverne les états de dégradation : Normal, Unstable, Degraded, Restricted, Blocked. Les Outils appliquent ces états (throttle, downgrade, freeze, block).

## COG Hébergeur

Lors d'une visite inter-COG, WorrySentinel surveille la session et peut révoquer à tout moment.

**Voir aussi :** États de confiance, Niveaux de sécurité, Attestation d'environnement, CaringNanny"#.to_string(),
                    updated_at: Utc::now(),
                },
                DocArticle {
                    id: "tamr".to_string(),
                    title: "TAMR".to_string(),
                    content: r#"# TAMR — Le Médiateur d'Intervention Humaine

**Core d'intervention humaine** (Strate 4). Trust & Authority Mediation Resolver.

## Question fondamentale

> *"Quand l'humain a-t-il le droit d'intervenir dans le système ?"*

## Rôle

Définir les **points d'intervention humaine** dans le système. TAMR établit où et quand l'humain peut prendre le relais.

## Responsabilités

- **Seuils d'escalade** — Quand passer le contrôle à l'utilisateur
- **Processus humains** — Validation manuelle, approbation, résolution de conflit
- **Cas d'erreur** — Quand proposer "Contacter le support" ou une action humaine
- **Délégation** — Définir les situations où l'automatisation s'arrête

## Cas d'usage

| Situation | Intervention TAMR |
|-----------|-------------------|
| Erreur / Refus | Message gouverné, option "Réessayer" ou "Contacter le support" |
| Conflit de données | Demande de résolution manuelle |
| Décision ambiguë | Escalade vers l'utilisateur |
| Sécurité dégradée | Choix utilisateur (continuer en mode limité, arrêter) |

## Relation avec WorrySentinel

TAMR et WorrySentinel sont complémentaires :
- **WorrySentinel** — Niveaux de sécurité, états de confiance
- **TAMR** — Points où l'humain doit ou peut intervenir

## Services et TAMR

Chaque Service déclare ses points TAMR : où l'utilisateur intervient (validation, choix, résolution). Exemple : JayKoa définit les points d'intervention humaine pour les conflits d'agenda.

## Règle fondamentale

> "L'humain intervient quand le système le demande, selon les règles établies par TAMR."

**Voir aussi :** WorrySentinel, Processus humains, Erreur gouvernée"#.to_string(),
                    updated_at: Utc::now(),
                },
            ],
        });

        // ═══════════════════════════════════════════════════════════════════════
        // LES TOOLKITS
        // ═══════════════════════════════════════════════════════════════════════
        sections.push(DocSection {
            id: "toolkits".to_string(),
            title: "Les Toolkits".to_string(),
            description: "Outils spécialisés gouvernés par les Cores".to_string(),
            icon: "🧰".to_string(),
            articles: vec![DocArticle {
                id: "overview".to_string(),
                title: "Vue d'ensemble des Toolkits".to_string(),
                content: r#"# Les Toolkits

Les Toolkits sont des **outils spécialisés** de la strate 2. Ils exécutent mais ne décident pas.

## Principe

> "Un Toolkit fait ce qu'on lui dit. Il n'a pas d'initiative propre."

## Structure standard

Chaque Toolkit possède :
- `admin_cell.rs` — Cellule de gouvernance
- `context.rs` — Contexte d'exécution
- `errors.rs` — Types d'erreurs
- `lib.rs` — Point d'entrée

## Toolkits principaux

| Toolkit | Domaine | Gouverné par |
|---------|---------|--------------|
| `kindmother-client` | Accès DB | KindMother |
| `miyusql-bridge` | Requêtes SQL | KindMother |
| `miyuwebway-participant` | Réseau MWS | BondingBrother |
| `miyu-crypto` | Cryptographie | BorderGuard |

## Création d'un Toolkit

```rust
// Définir l'admin cell
pub fn my_toolkit_admin_cell() -> MyToolkitAdminCell {
    MyToolkitAdminCell {
        identification: MyToolkitIdentification { ... },
        integrity: MyToolkitIntegrity { ... },
    }
}

// Le Toolkit est gouverné par un Core
impl GoverningCore for MyToolkit {
    fn governing_core() -> CoreType {
        CoreType::KindMother
    }
}
```"#
                    .to_string(),
                updated_at: Utc::now(),
            }],
        });

        // ═══════════════════════════════════════════════════════════════════════
        // LES OPÉRATEURS
        // ═══════════════════════════════════════════════════════════════════════
        sections.push(DocSection {
            id: "operators".to_string(),
            title: "Les Opérateurs".to_string(),
            description: "Exécutants des décisions des Cores".to_string(),
            icon: "🔧".to_string(),
            articles: vec![
                DocArticle {
                    id: "overview".to_string(),
                    title: "Vue d'ensemble des Opérateurs".to_string(),
                    content: r#"# Les Opérateurs

Les Opérateurs sont les **exécutants** de la strate 3. Ils traduisent les décisions des Cores en actions concrètes.

## Rôle

- Reçoivent les demandes des Services
- Consultent les Cores pour validation
- Utilisent les Toolkits pour l'exécution
- Rapportent le résultat

## Exemples d'Opérateurs

| Opérateur | Fonction |
|-----------|----------|
| `jaykonta-operator` | Mouvements financiers |
| `jaykoa-operator` | Événements calendrier |

## Flux d'exécution

```
1. Service envoie une demande
2. Opérateur vérifie les permissions (via Core)
3. Opérateur utilise le Toolkit approprié
4. Résultat retourné au Service
```

## Bonnes pratiques

- Ne jamais contourner les Cores
- Utiliser uniquement les Toolkits gouvernés
- Logger toutes les opérations
- Gérer les erreurs proprement"#.to_string(),
                    updated_at: Utc::now(),
                },
            ],
        });

        // ═══════════════════════════════════════════════════════════════════════
        // LES SERVICES
        // ═══════════════════════════════════════════════════════════════════════
        sections.push(DocSection {
            id: "services".to_string(),
            title: "Les Services".to_string(),
            description: "Applications utilisateur disponibles dans le COG".to_string(),
            icon: "📦".to_string(),
            articles: vec![
                DocArticle {
                    id: "overview".to_string(),
                    title: "Vue d'ensemble des Services".to_string(),
                    content: r#"# Les Services

Les Services sont les **applications utilisateur** de la strate 4. C'est ce que vous utilisez au quotidien.

## Services Miyukini

Services officiels développés par l'équipe Miyukini :

| Service | Description |
|---------|-------------|
| MiyuClicker | Jeu idle/clicker |
| Lord of the Castle | Jeu de stratégie coopératif |

## Services Jay

Services de productivité de la gamme Jay :

| Service | Description |
|---------|-------------|
| JayKonta | Comptabilité et finances |
| JayKoa | Calendrier universel |
| JayShop | Boutique en ligne |

## Jeux

Jeux disponibles dans le COG :

- **MiyuClicker** — Clicker avec progression infinie
- **Lord of the Castle** — Tower defense coopératif
- **Survivor** — Jeu de survie multijoueur

## Installation d'un Service

Dans Central :
1. Aller dans le Magasin
2. Trouver le Service souhaité
3. Cliquer sur "Installer"
4. Le Service apparaît dans la Bibliothèque"#.to_string(),
                    updated_at: Utc::now(),
                },
            ],
        });

        // ═══════════════════════════════════════════════════════════════════════
        // MIYUKINI WEBWAY SYSTEM
        // ═══════════════════════════════════════════════════════════════════════
        sections.push(DocSection {
            id: "mws".to_string(),
            title: "Miyukini Webway System".to_string(),
            description: "Le réseau P2P de présence et découverte des COGs".to_string(),
            icon: "🌐".to_string(),
            articles: vec![
                DocArticle {
                    id: "overview".to_string(),
                    title: "Vue d'ensemble du MWS".to_string(),
                    content: r#"# Miyukini Webway System

Le MWS est le réseau qui permet aux COGs de se découvrir et communiquer.

## Principes

1. **Optionnel** — Un COG peut fonctionner en mode Lone (isolé)
2. **Souverain** — Chaque COG reste maître de ses données
3. **Vérifié** — Seuls les COGs conformes accèdent au réseau
4. **Sécurisé** — Tout le trafic est chiffré TLS

## Les acteurs

| Acteur | Rôle |
|--------|------|
| **Origin** | Point central de vérité |
| **Relay** | Vérification de conformité |
| **Tracker** | Découverte et pools |
| **COG** | Participant au réseau |

## Flux de connexion

1. Le COG contacte Origin
2. Connexion TLS au Relay
3. Enregistrement (REGISTER)
4. Obtention du Permis de circulation
5. Annonce au Tracker (ANNOUNCE)
6. Conformité complète ✓

## Modes de fonctionnement

- **Connecté** — Participe au réseau MWS
- **Lone** — Isolé, données 100% locales"#
                        .to_string(),
                    updated_at: Utc::now(),
                },
                DocArticle {
                    id: "cogs-mesh".to_string(),
                    title: "Les COGs en maillage".to_string(),
                    content: r#"# Les COGs en maillage

## Architecture du réseau

Le MWS crée un **maillage fédéré** de COGs :

```
    [COG A] ←→ [Relay] ←→ [COG B]
        ↓         ↓         ↓
    [Tracker] ←→ [Origin] ←→ [Tracker]
        ↓         ↓         ↓
    [COG C] ←→ [Relay] ←→ [COG D]
```

## Types de connexion

### Relay (port 7000)
- Vérification de conformité
- Obtention du Permis
- Tunnel de données relayées

### Tracker (port 21000)
- Annonce de présence
- Découverte de COGs
- Gestion des lobbys

## Pools de version

Les COGs sont regroupés par version de Cores :
- Pool `1.0.x` — COGs version 1.0
- Pool `1.1.x` — COGs version 1.1

## Communication inter-COG

1. COG A cherche COG B via le Tracker
2. Le Tracker retourne l'adresse de COG B
3. COG A se connecte via le Relay
4. Communication chiffrée de bout en bout"#
                        .to_string(),
                    updated_at: Utc::now(),
                },
                DocArticle {
                    id: "security-protocols".to_string(),
                    title: "Sécurité et protocoles".to_string(),
                    content: r#"# Sécurité et protocoles MWS

## Protocole Relay

### Format des trames

```
+--------+--------+--------+--------+----------------+--------+
| Magic  | Ver    | Type   | Flags  | Session ID     | Length |
| 4 oct  | 1 oct  | 1 oct  | 2 oct  | 16 octets      | 4 oct  |
+--------+--------+--------+--------+----------------+--------+
|                      Payload                                |
+------------------------------------------------------------+
```

### Types de messages
- `REGISTER` (0x01) — Enregistrement initial
- `REGISTER_ACK` (0x02) — Accusé d'enregistrement
- `HEARTBEAT` (0x50) — Maintien de connexion
- `DATA` (0x40) — Données relayées

## Protocole Tracker

### Format des trames
```
+--------+--------+--------+--------+--------+--------+
| Magic  | Ver    | Type   | Flags  | ReqID  | Length |
+--------+--------+--------+--------+--------+--------+
```

### Types de messages
- `ANNOUNCE` (0x01) — Annonce de présence
- `SEARCH_COGS` (0x20) — Recherche de COGs
- `SEARCH_LOBBYS` (0x22) — Recherche de lobbys

## Sécurité

- TLS 1.3 obligatoire
- Certificats vérifiés
- Permis signé cryptographiquement
- Rate limiting anti-DDoS"#
                        .to_string(),
                    updated_at: Utc::now(),
                },
                DocArticle {
                    id: "how-it-works".to_string(),
                    title: "Fonctionnement".to_string(),
                    content: r#"# Fonctionnement du MWS

## Cycle de vie d'une connexion

### 1. Résolution Origin
Le COG résout l'adresse d'Origin (DNS ou configuration).

### 2. Connexion Relay
```
COG → TLS Handshake → Relay (port 7000)
```

### 3. Enregistrement
```json
{
    "cog_id": "cog-abc123",
    "core_version": "0.1.0",
    "services": ["jaykoa"],
    "nonce": "...",
    "timestamp": 1707840000
}
```

### 4. Vérification en 3 phases
- **Phase A** : Vérification des Cores (signature)
- **Phase B** : Vérification des Services
- **Phase C** : État de santé

### 5. Obtention du Permis
```json
{
    "permis_id": "...",
    "ttl": 3600,
    "scopes": ["tracker", "relay"]
}
```

### 6. Annonce Tracker
```json
{
    "cog_id": "cog-abc123",
    "permis_id": "...",
    "address": "192.168.1.100:8080",
    "services": ["jaykoa"]
}
```

### 7. Heartbeat
Toutes les 30 secondes pour maintenir la présence.

## États de confiance

| État | Description |
|------|-------------|
| T0 | Normal — Conformité complète |
| T1 | Méfiance légère — Anomalie mineure |
| T2 | Quarantaine — Comportement suspect |
| T3 | Blacklist temporaire |
| T4 | Blacklist permanente |"#
                        .to_string(),
                    updated_at: Utc::now(),
                },
                DocArticle {
                    id: "actors".to_string(),
                    title: "Les acteurs".to_string(),
                    content: r#"# Les acteurs du MWS

## Origin

**Point central de vérité** du réseau Miyukini.

### Rôles
- Source de vérité pour les versions
- Registre des services officiels
- Point d'entrée unique (DNS unique)

### Composants
- Relay intégré
- Tracker intégré
- Site web public
- API de catalogue

### Adresse
- `origin.miyukini.net:7000` (Relay)
- `origin.miyukini.net:21000` (Tracker)
- `origin.miyukini.net:80` (Web)

## Relay

**Vérificateur de conformité**.

### Responsabilités
- Vérification des Cores (signatures)
- Vérification des Services
- Émission des Permis de circulation
- Relais de données chiffrées

### Haute disponibilité
Plusieurs Relays peuvent exister pour la redondance.

## Tracker

**Gestionnaire de découverte**.

### Responsabilités
- Pools de COGs par version
- Recherche de COGs et Services
- Gestion des lobbys
- Statistiques réseau

## COG (Participant)

**Membre du réseau**.

### États possibles
- **Connecté** — Présent sur le réseau
- **Lone** — Mode isolé, hors réseau
- **Déconnecté** — En attente de connexion"#
                        .to_string(),
                    updated_at: Utc::now(),
                },
            ],
        });

        // ═══════════════════════════════════════════════════════════════════════
        // CONCEPTION, WORKFLOW ET DÉVELOPPEMENT
        // ═══════════════════════════════════════════════════════════════════════
        sections.push(DocSection {
            id: "development".to_string(),
            title: "Conception et développement".to_string(),
            description: "Guide pour les développeurs et contributeurs".to_string(),
            icon: "💻".to_string(),
            articles: vec![
                DocArticle {
                    id: "workflow".to_string(),
                    title: "Workflow de développement".to_string(),
                    content: r#"# Workflow de développement

## Structure du projet

```
miyukini-cog/
├── .cursor/         # Configuration Cursor
│   └── skills/      # Skills de l'agent
├── apps/            # Applications
│   ├── central/     # Hub principal
│   └── origin/      # Serveur MWS
├── crates/          # Bibliothèques Rust
│   ├── miyukini-kernel/
│   ├── kindmother/
│   └── ...
├── docs/            # Documentation
└── scripts/         # Scripts d'automatisation
```

## Commandes utiles

```bash
# Compiler tout le projet
cargo build

# Lancer les tests
cargo test

# Lancer Central en mode développement
cargo run -p miyukini-central-native

# Lancer Origin
cargo run -p miyukini-origin

# Vérifier le formatage
cargo fmt --check

# Linter
cargo clippy
```

## Conventions

### Nommage
- Crates : `miyukini-*` ou `nom-service`
- Modules : `snake_case`
- Types : `PascalCase`
- Fonctions : `snake_case`

### Documentation
- Tous les éléments publics documentés
- Exemples dans les docstrings
- README pour chaque crate"#
                        .to_string(),
                    updated_at: Utc::now(),
                },
                DocArticle {
                    id: "creating-service".to_string(),
                    title: "Créer un Service".to_string(),
                    content: r#"# Créer un nouveau Service

## Structure d'un Service

```
crates/mon-service/
├── Cargo.toml
├── src/
│   ├── lib.rs          # Point d'entrée
│   ├── admin_cell.rs   # Gouvernance
│   ├── context.rs      # Contexte
│   ├── errors.rs       # Erreurs
│   └── data.rs         # Accès données
```

## Étapes de création

### 1. Créer le crate

```toml
[package]
name = "mon-service"
version = "0.1.0"
edition = "2021"

[dependencies]
kindmother-client = { path = "../kindmother-client" }
serde = { version = "1", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
```

### 2. Définir l'admin cell

```rust
pub struct MonServiceAdminCell {
    pub identification: MonServiceIdentification,
    pub integrity: MonServiceIntegrity,
}
```

### 3. Implémenter l'accès aux données

```rust
pub struct MonServiceDb {
    client: KindMotherClient,
}

impl MonServiceDb {
    pub async fn open(db_path: PathBuf) -> Result<Self, Error> {
        // Via KindMother uniquement !
    }
}
```

### 4. Intégrer dans Central

Dans `apps/central/src/services/` :
- Créer le module d'affichage
- Ajouter au catalogue
- Définir la sidebar et les vues"#
                        .to_string(),
                    updated_at: Utc::now(),
                },
            ],
        });
    }

    /// Charge les services du catalogue.
    async fn load_services(&self) {
        let mut services = self.services.write().await;

        // Services Jay (productivité)
        services.push(ServiceInfo {
            id: "jaykonta".to_string(),
            name: "JayKonta".to_string(),
            short_description: "Comptabilité et gestion financière personnelle".to_string(),
            full_description: r#"# JayKonta

**JayKonta** est un service de comptabilité personnelle et professionnelle.

## Fonctionnalités

### Gestion des comptes
- Création de porte-monnaies (comptes)
- Suivi des soldes en temps réel
- Historique des mouvements

### Mouvements financiers
- Revenus et dépenses
- Transferts entre comptes
- Catégorisation automatique

### Récurrences
- Dépenses récurrentes (loyer, abonnements)
- Revenus réguliers (salaire)
- Alertes de paiement

### Prévisions
- Projection du solde futur
- Graphiques de tendance
- Alertes de découvert

## Données souveraines

Toutes vos données financières restent sur votre COG, jamais partagées sans votre accord."#
                .to_string(),
            category: ServiceCategory::Outils,
            editor: "Miyukini".to_string(),
            cog_version: "0.1.0".to_string(),
            service_type: ServiceType::Officiel,
            license: "Propriétaire Miyukini".to_string(),
            price: 0.0,
            release_date: Utc::now() - chrono::Duration::days(20),
            website_url: Some("https://miyukini.com/services/jaykonta".to_string()),
            download_url: Some("/downloads".to_string()),
            screenshots: vec![],
            banner_url: None,
        });

        services.push(ServiceInfo {
            id: "jaykoa".to_string(),
            name: "JayKoa".to_string(),
            short_description: "Calendrier universel et gestion d'agenda".to_string(),
            full_description: r#"# JayKoa

**JayKoa** est un calendrier universel pour gérer vos événements et rendez-vous.

## Fonctionnalités

### Vues multiples
- Vue mois complète
- Vue semaine détaillée
- Vue jour heure par heure
- Vue agenda (liste)

### Gestion des événements
- Création rapide d'événements
- Rappels et notifications
- Récurrence (quotidien, hebdomadaire, mensuel)
- Couleurs et catégories

### Synchronisation
- Synchronisation via MWS entre COGs
- Import/Export CalDAV
- Partage de calendriers

## Intégration

JayKoa s'intègre avec JayKonta pour les échéances financières."#.to_string(),
            category: ServiceCategory::StyleDeVie,
            editor: "Miyukini".to_string(),
            cog_version: "0.1.0".to_string(),
            service_type: ServiceType::Officiel,
            license: "Propriétaire Miyukini".to_string(),
            price: 0.0,
            release_date: Utc::now() - chrono::Duration::days(15),
            website_url: Some("https://miyukini.com/services/jaykoa".to_string()),
            download_url: Some("/downloads".to_string()),
            screenshots: vec![],
            banner_url: None,
        });

        // Jeux
        services.push(ServiceInfo {
            id: "miyuclicker".to_string(),
            name: "MiyuClicker".to_string(),
            short_description: "Jeu idle/clicker avec progression infinie".to_string(),
            full_description: r#"# MiyuClicker

**MiyuClicker** est un jeu idle/clicker addictif avec une progression infinie.

## Gameplay

### Clics et automatisation
- Gagnez des ressources en cliquant
- Débloquez des améliorations automatiques
- Progression exponentielle

### Améliorations
- Upgrades de clic
- Générateurs automatiques
- Multiplicateurs de bonus

### Achievements
- Objectifs à débloquer
- Récompenses spéciales
- Classements

## Mode hors ligne

Le jeu continue à progresser même quand vous ne jouez pas, grâce à la simulation de temps écoulé."#
                .to_string(),
            category: ServiceCategory::Jeux,
            editor: "Miyukini".to_string(),
            cog_version: "0.1.0".to_string(),
            service_type: ServiceType::Officiel,
            license: "Propriétaire Miyukini".to_string(),
            price: 0.0,
            release_date: Utc::now() - chrono::Duration::days(45),
            website_url: Some("https://miyukini.com/games/miyuclicker".to_string()),
            download_url: Some("/downloads".to_string()),
            screenshots: vec![],
            banner_url: None,
        });

        services.push(ServiceInfo {
            id: "lord-of-the-castle".to_string(),
            name: "Lord of the Castle".to_string(),
            short_description: "Tower defense coopératif en réseau".to_string(),
            full_description: r#"# Lord of the Castle

**Lord of the Castle** est un jeu de tower defense coopératif jouable en réseau via le MWS.

## Gameplay

### Défense
- Construisez des tours de défense
- Améliorez vos bâtiments
- Gérez vos ressources

### Vagues d'ennemis
- Vagues progressives de difficulté
- Types d'ennemis variés
- Boss redoutables

### Mode coopératif
- Jouez avec d'autres COGs via le MWS
- Partagez les ressources
- Stratégie collaborative

## Lobbys publics

Créez ou rejoignez des lobbys pour jouer avec d'autres membres du réseau Miyukini."#
                .to_string(),
            category: ServiceCategory::Jeux,
            editor: "Miyukini".to_string(),
            cog_version: "0.1.0".to_string(),
            service_type: ServiceType::Officiel,
            license: "Propriétaire Miyukini".to_string(),
            price: 0.0,
            release_date: Utc::now() - chrono::Duration::days(60),
            website_url: Some("https://miyukini.com/games/lord-of-the-castle".to_string()),
            download_url: Some("/downloads".to_string()),
            screenshots: vec![],
            banner_url: None,
        });

        // Outils système
        services.push(ServiceInfo {
            id: "miyukini-central".to_string(),
            name: "Miyukini Central".to_string(),
            short_description: "Hub principal de votre environnement COG".to_string(),
            full_description: r#"# Miyukini Central

**Miyukini Central** est le hub principal de votre environnement COG, comparable à Steam pour les jeux.

## Fonctionnalités

### Bibliothèque
- Accès à tous vos Services installés
- Lancement rapide des applications
- Gestion des installations

### Communauté
- Connexion au réseau MWS
- Découverte des COGs connectés
- Lobbys et sessions multijoueur

### Magasin
- Catalogue des Services disponibles
- Installation en un clic
- Mises à jour automatiques

### Profil
- Gestion de votre identité COG
- Paramètres de confidentialité
- Configuration du mode Lone/Connecté

## Interface

Central utilise une interface native performante construite avec Dioxus, sans webview."#.to_string(),
            category: ServiceCategory::Outils,
            editor: "Miyukini".to_string(),
            cog_version: "0.1.0".to_string(),
            service_type: ServiceType::Officiel,
            license: "Propriétaire Miyukini".to_string(),
            price: 0.0,
            release_date: Utc::now() - chrono::Duration::days(90),
            website_url: Some("https://miyukini.com".to_string()),
            download_url: Some("/downloads".to_string()),
            screenshots: vec![],
            banner_url: None,
        });

        // Social & Communication
        services.push(ServiceInfo {
            id: "jay1tribu".to_string(),
            name: "Jay1Tribu".to_string(),
            short_description: "Messagerie P2P, tribus et salons — communication souveraine".to_string(),
            full_description: r#"# Jay1Tribu

**Jay1Tribu** est le service de messagerie pair-à-pair et de gestion de communautés de Miyukini.

## Fonctionnalités

### Messagerie directe
- Conversations privées entre deux COGs
- Envoi de fichiers et images (restreint aux amis)
- File d'attente hors-ligne : vos messages partent dès que la connexion revient

### Tribus
- Créez des groupes (tribus) avec vos proches ou collègues
- Rôles : Chef, Admin, Membre (personnalisables)
- Invitations sécurisées entre COGs

### Salons
- Salons directs (1-à-1) ou collectifs (au sein d'une tribu)
- Historique complet stocké localement par KindMother
- Recherche dans les messages

### Présence & hors-ligne
- Indicateur de présence en temps réel via le MWS
- Mode hors-ligne natif : lecture, écriture, création de salons — tout fonctionne
- Les messages en attente sont distribués automatiquement à la reconnexion

## Intégration MWS

Jay1Tribu utilise le transport MWS chiffré pour :
- Dispatcher les messages en temps réel
- Synchroniser la présence des amis
- Distribuer les invitations de tribu

## Architecture

Persistance via KindMother ou SQLite direct (optionnel). Transport via MwsTransportSender avec chiffrement C-2."#.to_string(),
            category: ServiceCategory::Social,
            editor: "Miyukini".to_string(),
            cog_version: "0.1.0".to_string(),
            service_type: ServiceType::Officiel,
            license: "Propriétaire Miyukini".to_string(),
            price: 0.0,
            release_date: Utc::now() - chrono::Duration::days(10),
            website_url: Some("https://miyukini.com/services/jay1tribu".to_string()),
            download_url: Some("/downloads".to_string()),
            screenshots: vec![],
            banner_url: None,
        });

        // Rendez-vous & Réservation
        services.push(ServiceInfo {
            id: "jayrdv".to_string(),
            name: "JayRDV".to_string(),
            short_description: "Prise de rendez-vous et réservation en ligne".to_string(),
            full_description: r#"# JayRDV

**JayRDV** est le service de prise de rendez-vous et de réservation pour les professionnels et les particuliers.

## Fonctionnalités

### Pour les professionnels
- Profil professionnel avec secteur d'activité, photo, contact
- Gestion des praticiens/collaborateurs avec rôles (Admin, Manager, Praticien)
- Définition des services/prestations (durée, prix, catégorie)
- Gestion des ressources (salles, équipements, personnel)
- Plannings récurrents par jour de la semaine
- Exceptions (congés, fermetures, absences)
- Paramètres avancés : politique d'annulation, préavis, liste d'attente, acomptes

### Pour les clients
- Réservation de créneaux en ligne
- Historique des rendez-vous
- Rappels multi-canal (SMS, Email, Push) via CaringNanny
- Annulation et report

### Gestion des créneaux
- Statuts : Disponible → Réservé temporairement → Confirmé → Bloqué
- Hold temporaire de 10 min (panier)
- Round-robin entre praticiens (optionnel)

### Cycle de vie des rendez-vous
- Pending → Confirmed → Completed / NoShow / Cancelled
- Suivi des no-shows par client
- Annulation par client ou professionnel

## Intégrations

- **JayKoa** : synchronisation avec le calendrier
- **MiyuNotify** : rappels automatiques

## Données souveraines

Toutes les données de rendez-vous restent sur votre COG. Aucun intermédiaire cloud."#.to_string(),
            category: ServiceCategory::Commerce,
            editor: "Miyukini".to_string(),
            cog_version: "0.1.0".to_string(),
            service_type: ServiceType::Officiel,
            license: "Propriétaire Miyukini".to_string(),
            price: 0.0,
            release_date: Utc::now() - chrono::Duration::days(5),
            website_url: Some("https://miyukini.com/services/jayrdv".to_string()),
            download_url: Some("/downloads".to_string()),
            screenshots: vec![],
            banner_url: None,
        });

        // Commerce & Point de vente
        services.push(ServiceInfo {
            id: "jayshop".to_string(),
            name: "JayShop".to_string(),
            short_description: "Point de vente et vente en ligne — caisse et tickets".to_string(),
            full_description: r#"# JayShop

**JayShop** est le service commerce famille Jay pour la gestion des ventes, du point de vente (PoS) et de la vente en ligne.

## Fonctionnalités

### Gestion des tickets (ventes)
- Création de tickets : Draft → Open → Paid / Refunded / Cancelled
- Source : Point de vente (PoS) ou vente en ligne
- Historique complet des transactions

### Point de vente
- Interface caisse rapide
- Scan de produits
- Calcul automatique des taxes (multi-taux)
- Réductions et promotions prédéfinies
- Gestion des sessions de caisse

### Paiements
- Encaissement multi-moyens
- Rendu de monnaie
- Suivi des paiements en attente

### Rapports et KPIs
- Chiffre d'affaires par période
- Produits les plus vendus
- Rapports de caisse journaliers

## Intégrations

- **JayKonta** : pipeline comptable automatique (factures, TVA, rapports)

## Architecture

Persistance via KindMother avec chiffrement libSQL. Pipeline comptable vers JayKonta automatisé."#.to_string(),
            category: ServiceCategory::Commerce,
            editor: "Miyukini".to_string(),
            cog_version: "0.1.0".to_string(),
            service_type: ServiceType::Officiel,
            license: "Propriétaire Miyukini".to_string(),
            price: 0.0,
            release_date: Utc::now() - chrono::Duration::days(3),
            website_url: Some("https://miyukini.com/services/jayshop".to_string()),
            download_url: Some("/downloads".to_string()),
            screenshots: vec![],
            banner_url: None,
        });
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Accesseurs
    // ─────────────────────────────────────────────────────────────────────────

    /// Récupère tous les articles de blog.
    pub async fn get_blog_posts(&self) -> Vec<BlogPost> {
        let posts = self.blog_posts.read().await;
        let mut sorted = posts.clone();
        sorted.sort_by(|a, b| b.published_at.cmp(&a.published_at));
        sorted
    }

    /// Récupère un article de blog par ID.
    pub async fn get_blog_post(&self, id: &str) -> Option<BlogPost> {
        let posts = self.blog_posts.read().await;
        posts.iter().find(|p| p.id == id).cloned()
    }

    /// Récupère les annonces actives.
    pub async fn get_announcements(&self) -> Vec<Announcement> {
        let announcements = self.announcements.read().await;
        let now = Utc::now();
        let mut active: Vec<_> = announcements
            .iter()
            .filter(|a| a.expires_at.map(|e| e > now).unwrap_or(true))
            .cloned()
            .collect();
        active.sort_by(|a, b| {
            b.priority
                .cmp(&a.priority)
                .then_with(|| b.published_at.cmp(&a.published_at))
        });
        active
    }

    /// Récupère tous les téléchargements.
    pub async fn get_downloads(&self) -> Vec<Download> {
        let downloads = self.downloads.read().await;
        downloads.clone()
    }

    /// Récupère les téléchargements par catégorie.
    pub async fn get_downloads_by_category(&self, category: DownloadCategory) -> Vec<Download> {
        let downloads = self.downloads.read().await;
        downloads
            .iter()
            .filter(|d| d.category == category)
            .cloned()
            .collect()
    }

    /// Récupère les sections de documentation.
    pub async fn get_doc_sections(&self) -> Vec<DocSection> {
        let sections = self.doc_sections.read().await;
        sections.clone()
    }

    /// Récupère une section de documentation.
    pub async fn get_doc_section(&self, id: &str) -> Option<DocSection> {
        let sections = self.doc_sections.read().await;
        sections.iter().find(|s| s.id == id).cloned()
    }

    /// Récupère un article de documentation.
    pub async fn get_doc_article(&self, section_id: &str, article_id: &str) -> Option<DocArticle> {
        let sections = self.doc_sections.read().await;
        sections
            .iter()
            .find(|s| s.id == section_id)
            .and_then(|s| s.articles.iter().find(|a| a.id == article_id).cloned())
    }

    /// Récupère tous les services.
    pub async fn get_services(&self) -> Vec<ServiceInfo> {
        let services = self.services.read().await;
        services.clone()
    }

    /// Récupère un service par ID.
    pub async fn get_service(&self, id: &str) -> Option<ServiceInfo> {
        let services = self.services.read().await;
        services.iter().find(|s| s.id == id).cloned()
    }

    /// Récupère les services par catégorie.
    pub async fn get_services_by_category(&self, category: ServiceCategory) -> Vec<ServiceInfo> {
        let services = self.services.read().await;
        services
            .iter()
            .filter(|s| s.category == category)
            .cloned()
            .collect()
    }

    /// Récupère les services gratuits.
    pub async fn get_free_services(&self) -> Vec<ServiceInfo> {
        let services = self.services.read().await;
        services.iter().filter(|s| s.is_free()).cloned().collect()
    }
}

impl Default for ContentManager {
    fn default() -> Self {
        Self::new()
    }
}
