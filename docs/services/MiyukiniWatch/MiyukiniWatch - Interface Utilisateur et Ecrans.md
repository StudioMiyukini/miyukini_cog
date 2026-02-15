# MiyukiniWatch — Interface Utilisateur et Écrans

## Contexte

**MiyukiniWatch** est un service **silencieux mais pas opaque**. Bien qu'il fonctionne en arrière-plan, l'utilisateur peut l'ouvrir comme n'importe quel service depuis Miyukini Central. Ce document décrit les écrans, la navigation, les composants UI et l'expérience utilisateur de MiyukiniWatch lorsqu'il est consulté.

## Portée / Scope

- **Applicable à :** Spécification des écrans, composants UI, flux de navigation, interactions.
- **Audience :** Développeurs frontend, designers UX/UI, équipes produit.
- **Statut :** Spécification fonctionnelle — référence UI du Service MiyukiniWatch.

### Hors périmètre

- Choix techniques d'implémentation (framework, composants Dioxus spécifiques).
- Style visuel détaillé (couleurs, typographie) — voir les packs UI CSS de Central.
- Bulles Miou (gérées par le sous-service Miou, pas par MiyukiniWatch).

---

## 1. Accès au service

### 1.1 Point d'entrée

MiyukiniWatch apparaît dans la **liste des services** de Miyukini Central, visible depuis le **Salon** et la **Bibliothèque**.

| Attribut | Valeur |
|----------|--------|
| **Nom affiché** | MiyukiniWatch |
| **Icône** | Icône thématique (œil bienveillant / montre / cœur — à définir par le design) |
| **Description courte** | « Tes habitudes et tes mesures — consulte, comprends, efface. » |
| **Badge/indicateur** | Aucun badge en fonctionnement normal. Un indicateur discret « Collecte désactivée » si la collecte est off. |

### 1.2 Comportement à l'ouverture

- MiyukiniWatch s'ouvre **dans la zone de contenu principal** de Central (comme tout autre service).
- Le chargement est rapide : les agrégats récents sont disponibles immédiatement (déjà en mémoire via l'Aggregator).
- Aucune animation d'introduction intrusive ; transition standard de Central.

---

## 2. Architecture des écrans

MiyukiniWatch est organisé en **quatre écrans principaux** accessibles via une barre de navigation locale (tabs ou sidebar) :

```
MiyukiniWatch
├── Tableau de bord (défaut)
├── Détail des métriques
├── Paramètres et vie privée
└── Historique des actions (audit)
```

---

## 3. Écran 1 — Tableau de bord

**Rôle :** Vue synthétique et rassurante. L'utilisateur comprend en un coup d'œil ce que MiyukiniWatch enregistre et les résumés de ses habitudes.

### 3.1 Structure de l'écran

| Zone | Contenu | Position |
|------|---------|----------|
| **En-tête** | Titre « MiyukiniWatch » + phrase d'explication : « Ce service mesure tes habitudes pour aider Miou à te faire de meilleures suggestions. Il ne lit jamais le contenu de tes messages. » | Haut de page |
| **Bloc périmètre** | Encadré clair listant les 4 dimensions collectées : Quand, Où, Qui, Combien. Avec une icône ✓ devant chaque dimension active et ✗ devant chaque dimension désactivée. | Sous l'en-tête |
| **Carte Sessions** | Dernière session (date, durée), jours depuis dernière visite, tranche horaire habituelle, nombre total de sessions. | Zone principale, gauche |
| **Carte Services** | Top 3 services les plus utilisés (semaine), dernier service ouvert, service le plus long. | Zone principale, droite |
| **Carte Amis** | Amis contactés récemment, ami non contacté depuis le plus longtemps, top 3 par temps de discussion. (Masquée si aucune métrique d'amis disponible.) | Zone principale, en dessous |
| **Carte Activité** | Indicateur d'activité (clics par jour, sessions par semaine), streak de jours actifs. | Zone principale, en dessous |
| **Pied de page** | Lien « Détail complet », lien « Paramètres », lien « Effacer tout ». | Bas de page |

### 3.2 États spéciaux

| État | Affichage |
|------|-----------|
| **Première utilisation** | Message d'accueil : « MiyukiniWatch vient de commencer à mesurer tes habitudes. Reviens dans quelques jours pour voir tes premières statistiques. » Les cartes sont vides avec un placeholder. |
| **Collecte désactivée** | Bandeau informatif en haut : « La collecte est désactivée. Les données existantes sont toujours consultables. » Lien « Réactiver ». |
| **Données effacées** | Cartes vides avec message : « Aucune donnée disponible. La collecte (si active) reconstituera progressivement le contexte. » |

---

## 4. Écran 2 — Détail des métriques

**Rôle :** Consultation approfondie des données par catégorie et par période.

### 4.1 Navigation par catégorie

L'écran est organisé en onglets (tabs) par catégorie de métriques :

| Onglet | Contenu |
|--------|---------|
| **Sessions** | Liste chronologique des sessions (date, heure, durée). Graphique de durée par jour/semaine. Tranche horaire la plus fréquente. |
| **Services** | Classement des services par fréquence et temps passé. Historique d'ouverture par service. Graphique d'utilisation. |
| **Amis** | Liste des amis contactés avec temps depuis dernière discussion. Classement par temps passé. Historique des interactions (dates uniquement, pas de contenu). |
| **Activité** | Compteur de clics par jour/semaine (si exposé). Jours actifs consécutifs. Badges débloqués. |

### 4.2 Filtrage par période

| Filtre | Comportement |
|--------|-------------|
| **Aujourd'hui** | Métriques brutes de la session en cours et des sessions précédentes du jour. |
| **Cette semaine** | Agrégats quotidiens de la semaine en cours. |
| **Ce mois** | Agrégats quotidiens du mois en cours. |
| **Personnalisé** | Sélection d'une plage de dates (dans la limite de la rétention configurée). |

### 4.3 Affichage des données

| Composant | Description |
|-----------|-------------|
| **Tableau** | Affichage tabulaire pour les listes (sessions, services, amis). Colonnes triables. |
| **Graphique barres** | Durée de session ou nombre d'ouvertures de service par jour/semaine. |
| **Graphique circulaire** | Répartition du temps par service (période sélectionnée). |
| **Indicateur textuel** | Phrases lisibles : « Tu as passé 2h30 dans JayXpose cette semaine » ; « Ton ami [pseudo] n'a pas été contacté depuis 12 jours ». |

### 4.4 Rappel de transparence

Chaque onglet affiche en bas un rappel :

> « MiyukiniWatch enregistre uniquement quand, où, qui et combien. Il ne lit pas le contenu de tes messages, tes saisies ou tes fichiers. »

---

## 5. Écran 3 — Paramètres et vie privée

**Rôle :** Contrôle total de l'utilisateur sur la collecte, la rétention et l'effacement.

### 5.1 Sections

| Section | Contenu |
|---------|---------|
| **État de la collecte** | Toggle on/off pour activer ou désactiver la collecte globale. Indicateur visuel clair (vert = active, gris = désactivée). |
| **Collecte par catégorie** | Toggles individuels pour chaque catégorie : Sessions, Services, Amis, Activité (clics). Permet de désactiver une catégorie sans tout couper. |
| **Rétention** | Sliders ou sélecteurs pour ajuster la rétention : métriques brutes (7-90 j.), agrégats quotidiens (30-365 j.), agrégats hebdomadaires (90-730 j.). |
| **Espace utilisé** | Indicateur de l'espace de stockage utilisé par MiyukiniWatch (ex. « 12 Mo / 50 Mo »). |
| **Effacement** | Boutons d'effacement : « Effacer la dernière semaine », « Effacer le dernier mois », « Effacer tout l'historique ». Chaque bouton déclenche une confirmation. |
| **Effacement par catégorie** | Boutons pour effacer une catégorie spécifique (ex. « Effacer toutes les données Amis »). |

### 5.2 Confirmation d'effacement

Toute action d'effacement déclenche une modale de confirmation :

| Élément | Contenu |
|---------|---------|
| **Titre** | « Effacer les données ? » |
| **Message** | Description précise de ce qui sera effacé (période, catégorie, volume estimé). |
| **Avertissement** | « Cette action est irréversible. Miou utilisera des messages génériques tant que de nouvelles données ne seront pas collectées. » |
| **Actions** | Bouton « Confirmer l'effacement » (destructif, rouge), bouton « Annuler » (neutre). |

### 5.3 Liens vers d'autres paramètres

| Lien | Destination |
|------|-------------|
| « Comment Miou utilise ces données » | Lien vers une section explicative ou vers la documentation Miou. |
| « Paramètres Miyukini Central » | Lien vers la section « Données et vie privée » de Central. |

---

## 6. Écran 4 — Historique des actions (audit)

**Rôle :** Transparence totale. L'utilisateur voit ce que MiyukiniWatch a fait en matière de gestion de données.

### 6.1 Contenu

| Élément | Description |
|---------|-------------|
| **Journal chronologique** | Liste des événements d'audit (collecte activée/désactivée, effacements, purges automatiques, modifications de rétention). |
| **Filtrage** | Par type d'événement, par période. |
| **Détail** | Clic sur une entrée pour voir le détail (ex. « Purge automatique : 342 métriques brutes de plus de 30 jours supprimées »). |

### 6.2 Format d'une entrée

| Champ | Exemple |
|-------|---------|
| **Date** | 14/02/2026, 10:32 |
| **Type** | Purge automatique |
| **Description** | « 342 métriques brutes expirées supprimées (rétention : 30 jours). » |
| **Catégorie** | Sessions, Services |

---

## 7. Principes UX

### 7.1 Ton et langage

| Principe | Application |
|----------|-------------|
| **Tutoiement** | Cohérent avec Miou et Central : « Tes habitudes », « Tu n'as pas… ». |
| **Bienveillance** | Aucun jugement sur les habitudes. Pas de « tu utilises trop » ou « tu devrais ». |
| **Clarté** | Vocabulaire simple. Pas de jargon technique (pas de « WriteIntent » dans l'interface). |
| **Transparence** | Rappels fréquents de ce qui est et n'est pas collecté. |

### 7.2 Responsive

| Écran | Adaptation |
|-------|------------|
| **Desktop** | Grille 2 colonnes pour le tableau de bord ; sidebar de navigation. |
| **Tablette** | Grille 1-2 colonnes ; navigation par tabs en haut. |
| **Mobile** | Colonne unique ; cartes empilées ; tabs en bas. |

### 7.3 Accessibilité

| Critère | Exigence |
|---------|----------|
| **Contraste** | Ratios conformes WCAG AA minimum. |
| **Navigation clavier** | Tous les éléments interactifs accessibles au clavier. |
| **Lecteur d'écran** | Labels ARIA sur les graphiques, les toggles et les boutons d'effacement. |
| **Taille de texte** | Respecte le zoom navigateur jusqu'à 200 %. |

---

## 8. Flux de navigation

```
Salon / Bibliothèque
    │
    ▼ Clic sur "MiyukiniWatch"
┌──────────────────────────────────────────────┐
│           MiyukiniWatch                       │
│                                                │
│  [Tableau de bord] [Détail] [Paramètres] [Audit]
│                                                │
│  Tableau de bord (par défaut)                  │
│    ├── Carte Sessions                          │
│    ├── Carte Services                          │
│    ├── Carte Amis                              │
│    └── Carte Activité                          │
│                                                │
│  Détail des métriques                          │
│    ├── Onglet Sessions                         │
│    ├── Onglet Services                         │
│    ├── Onglet Amis                             │
│    └── Onglet Activité                         │
│                                                │
│  Paramètres et vie privée                      │
│    ├── Toggle collecte                         │
│    ├── Toggles par catégorie                   │
│    ├── Sliders rétention                       │
│    └── Boutons effacement                      │
│                                                │
│  Historique des actions                         │
│    └── Journal d'audit                         │
└──────────────────────────────────────────────┘
```

---

## 9. Références

| Document | Rôle |
|----------|------|
| [MiyukiniWatch — Document Fondateur](./MiyukiniWatch%20-%20Document%20Fondateur.md) | Vision « silencieux mais pas opaque », interface utilisateur. |
| [MiyukiniWatch — Gouvernance Données et Rétention](./MiyukiniWatch%20-%20Gouvernance%20Donnees%20et%20Retention.md) | Droits de l'utilisateur, effacement, désactivation. |
| [MiyukiniWatch — Spécification Fonctionnelle : Métriques et Collecte](./MiyukiniWatch%20-%20Specification%20Fonctionnelle%20Metriques%20et%20Collecte.md) | Catalogue des métriques affichées. |
| [Miyukini Central — Packs UI CSS Style Chrome](../MiyukiniCentral/reference/Miyukini%20Central%20-%20Packs%20UI%20CSS%20Style%20Chrome.md) | Référence de style visuel. |
| [Miyukini Central — Écrans et UI](../MiyukiniCentral/Miyukini%20Central%20-%20Ecrans%20et%20UI.md) | Référence de navigation Central. |

---

**Document** : MiyukiniWatch — Interface Utilisateur et Écrans  
**Version** : 1.0  
**Date** : 2026-02-14  
**Statut** : Spécification fonctionnelle — référence UI du Service MiyukiniWatch
