# MiyukiniWatch — Architecture et Positionnement

## Contexte

**MiyukiniWatch** est un **Service interne COG (Type 1)** qui mesure les habitudes et les interactions de l'utilisateur avec son environnement Miyukini. Ce document décrit son positionnement dans la Pyramide Miyukini, ses relations avec les Cores, les Outils et les autres Services, ainsi que le flux d'exécution standard.

## Portée / Scope

- **Applicable à :** Architecture, positionnement pyramidal, dépendances inter-strates, flux de gouvernance.
- **Audience :** Architectes, développeurs, équipes sécurité.
- **Statut :** Document normatif — référence architecturale du Service MiyukiniWatch.

---

## 1. Positionnement dans la Pyramide Miyukini

MiyukiniWatch s'inscrit dans la Pyramide des strates comme un **Service** visible par l'utilisateur, porté par des **Opérateurs** gouvernés.

| Strate | Élément | Rôle vis-à-vis de MiyukiniWatch |
|--------|---------|---------------------------------|
| **7** | Opérateurs MiyukiniWatch | Exécutent la collecte des métriques, l'agrégation, la présentation des données et l'effacement sur demande. |
| **6** | Outils & Kits d'Outils | Capacités exécutables utilisées par les Opérateurs : écriture/lecture locale (MiyuStorage), horodatage (MiyuTime), agrégation statistique. |
| **5** | BondingBrother | Médiation entre les Opérateurs MiyukiniWatch et les Cores. Traduit les intentions de collecte et d'effacement vers les autorités. |
| **4** | Cores | Gouvernent le comportement : KindMother (persistance), StrongFather (décision), Caring Nanny (observation d'état), WorrySentinel (sécurité), Master Butler (permissions). |
| **3** | Invariants & Contrats | Principes non négociables : pas de lecture de contenus, données locales uniquement, transparence totale. |
| **K** | Kernel | Substrat technique neutre : scheduling, I/O, identifiants. |

### 1.1 Flux d'exécution standard

```
Utilisateur → MiyukiniWatch (Service) → Opérateurs MiyukiniWatch → BondingBrother → Cores → Outils → Exécution
```

Le flux se décompose en deux voies :

**Voie passive (collecte silencieuse) :**
```
Événement utilisateur (clic, ouverture service, connexion)
  → Opérateur Collecteur MiyukiniWatch
    → BondingBrother
      → StrongFather : "Cette métrique peut-elle être enregistrée ?"
      → KindMother : WriteIntent (enregistrement de la métrique)
    → Outils d'écriture locale
      → Métrique persistée dans le stockage COG
```

**Voie active (consultation par l'utilisateur) :**
```
Utilisateur ouvre MiyukiniWatch depuis Central
  → Opérateur Consultation MiyukiniWatch
    → BondingBrother
      → KindMother : ReadIntent (lecture des métriques)
      → Master Butler : "L'utilisateur a-t-il accès à ces données ?"
    → Outils de lecture locale
      → Données présentées à l'utilisateur
```

---

## 2. Type de Service et espaces

| Attribut | Valeur |
|----------|--------|
| **Type** | Service interne COG (Type 1) |
| **Espace** | Miyukini Central uniquement |
| **Surface externe** | Aucune — pas de surface web, pas de protocole Inter-COG |
| **Visibilité** | Apparaît dans la liste des services de Central (Salon / Bibliothèque) |

**Règle canonique :** MiyukiniWatch est un service purement interne. Il ne communique jamais avec l'extérieur du COG. Aucune donnée ne transite vers un autre COG, un serveur tiers ou le MWS.

---

## 3. Opérateurs MiyukiniWatch

Les Opérateurs sont les entités fonctionnelles gouvernées qui exécutent les capacités du service. MiyukiniWatch déploie trois Opérateurs principaux :

| Opérateur | Responsabilité | Mode |
|-----------|---------------|------|
| **MiyukiniWatchCollector** | Collecte des métriques en temps réel (sessions, services, amis, clics). Écoute les événements du COG et produit des WriteIntent vers KindMother. | Passif — tourne en arrière-plan. |
| **MiyukiniWatchAggregator** | Agrège les métriques brutes en résumés exploitables (par jour, semaine, mois). Produit les agrégats consommés par Miou. | Périodique — déclenché à intervalles ou à la demande. |
| **MiyukiniWatchPresenter** | Présente les données à l'utilisateur lorsqu'il ouvre le service. Gère l'effacement et la désactivation de la collecte. | Actif — déclenché par l'utilisateur. |

### 3.1 Règle fondamentale des Opérateurs

> « Les Opérateurs sont gouvernés, jamais autonomes. »

Les Opérateurs MiyukiniWatch ne prennent aucune décision seuls. Chaque action de collecte, d'agrégation ou d'effacement est soumise aux Cores via BondingBrother. Le Collector ne peut enregistrer une métrique que si StrongFather l'autorise et que KindMother accepte l'écriture.

---

## 4. Interactions avec les Cores

### 4.1 Matrice Core × Responsabilité

| Core | Rôle vis-à-vis de MiyukiniWatch |
|------|---------------------------------|
| **StrongFather** | Décide si la collecte d'une métrique est autorisée. Valide les opérations d'effacement. Émet les Mandats de Permission pour les Opérateurs. |
| **KindMother** | Autorité exclusive de persistance locale. Reçoit et traite les WriteIntent (enregistrement de métriques) et ReadIntent (consultation). Gère la résidence des données et la purge par rétention. |
| **Caring Nanny** | Observe l'état du système. Peut restreindre la collecte si le COG est en état dégradé (T2, T3, T4) pour préserver les ressources. |
| **Master Butler** | Registre des capacités et permissions : qui peut consulter les métriques, qui peut effacer, qui peut désactiver la collecte. |
| **WorrySentinel** | Gouverne le niveau de sécurité des données MiyukiniWatch. Définit les politiques de chiffrement au repos et de classification des métriques. |
| **Border Guard** | Garantit que les données MiyukiniWatch ne franchissent jamais les frontières du COG. Toute tentative d'export est bloquée par défaut. |
| **Ever Buddy** | Gère l'évolution du Service (nouvelles métriques, changement de format, dépréciation). Assure la compatibilité des agrégats lors des mises à jour. |
| **TAMR** | Point d'intervention humaine : l'utilisateur peut demander un effacement total, une désactivation, ou une consultation des règles en cours. |

### 4.2 Flux de gouvernance détaillé — Collecte d'une métrique

```
1. Événement détecté (ouverture d'un service, par exemple)
2. MiyukiniWatchCollector prépare un WriteIntent
3. BondingBrother transmet l'intention à StrongFather
   → StrongFather vérifie :
     - La collecte est-elle activée ? (TAMR / préférence utilisateur)
     - L'état du COG permet-il la collecte ? (Caring Nanny)
     - Le type de métrique est-il autorisé ? (Master Butler)
   → Si refusé : WriteIntent rejeté, la métrique n'est pas enregistrée
4. StrongFather émet un Mandat de Permission
5. BondingBrother transmet le WriteIntent validé à KindMother
   → KindMother persiste la métrique dans le stockage local
6. Confirmation remontée à l'Opérateur
```

---

## 5. Dépendances architecturales

### 5.1 Dépendances amont (MiyukiniWatch consomme)

| Dépendance | Nature | Description |
|------------|--------|-------------|
| **Miyukini Central** | Service hôte | MiyukiniWatch est un service dans Central ; il dépend de Central pour le cycle de vie (démarrage, arrêt). |
| **KindMother** | Core (Strate 4) | Persistance locale des métriques. MiyukiniWatch ne peut pas fonctionner sans KindMother. |
| **Événements COG** | Bus d'événements | MiyukiniWatch écoute les événements du COG (ouverture de service, connexion, etc.) pour déclencher la collecte. |

### 5.2 Dépendances aval (d'autres consomment MiyukiniWatch)

| Consommateur | Nature | Description |
|--------------|--------|-------------|
| **Miou** | Sous-service de Central | Consomme les **agrégats** produits par MiyukiniWatch pour adapter ses bulles, rappels et suggestions. Voir [MiyukiniWatch - Intégration Miou et Agrégats](./MiyukiniWatch%20-%20Integration%20Miou%20et%20Agregats.md). |
| **Salon (Miyukini Central)** | Vue de Central | Les suggestions affichées sur le Salon peuvent être dérivées des métriques MiyukiniWatch. |

### 5.3 Indépendance vis-à-vis du MWS

MiyukiniWatch **n'a aucune dépendance sur le MWS** (Miyukini Webway System). Conformément à LOI-1 et LOI-2 :

- Aucune donnée MiyukiniWatch ne transite sur le réseau.
- Le service fonctionne identiquement que le COG soit connecté au Webway ou totalement isolé.
- Un COG de type LONE (structurellement isolé) bénéficie de MiyukiniWatch sans aucune restriction.

---

## 6. Conformité aux Lois d'Autonomie

| Loi | Conformité MiyukiniWatch |
|-----|--------------------------|
| **LOI-1** | Aucune dépendance externe critique. MiyukiniWatch ne requiert que KindMother (locale) et les événements internes du COG. |
| **LOI-2** | L'isolement est un état normal. MiyukiniWatch fonctionne sans réseau. |
| **LOI-3** | L'état local est souverain. Les métriques appartiennent au COG et à son utilisateur. |
| **LOI-4** | Pas de temps global requis. Les horodatages sont locaux au COG. |
| **LOI-5** | Coût proportionnel au hardware. La collecte et l'agrégation sont légères et ne requièrent pas de ressources disproportionnées. |
| **LOI-6** | L'autonomie n'empêche pas la fédération. MiyukiniWatch ne participe pas à la fédération mais n'empêche pas les services qui consomment ses agrégats (ex. Miou) de fonctionner dans un contexte fédéré. |
| **LOI-7** | La strate Cores est immuable. MiyukiniWatch évolue par environnement ; ses métriques sont versionnées. |
| **LOI-8** | Migration = diplomatie. Lors d'une migration de COG, les données MiyukiniWatch suivent la procédure formelle (pas de copie brute). |

---

## 7. Diagramme de contexte

```
┌─────────────────────────────────────────────────┐
│                   COG (souverain)                │
│                                                   │
│  ┌──────────────────────────────────────────┐    │
│  │          Miyukini Central                 │    │
│  │                                            │    │
│  │  ┌──────────────┐   ┌──────────────────┐ │    │
│  │  │ MiyukiniWatch│──▶│      Miou        │ │    │
│  │  │  (Service)   │   │ (Sous-service)   │ │    │
│  │  └──────┬───────┘   └──────────────────┘ │    │
│  │         │                                  │    │
│  │         ▼                                  │    │
│  │  ┌──────────────┐                         │    │
│  │  │  Opérateurs  │                         │    │
│  │  │  MW Collect.  │                         │    │
│  │  │  MW Aggreg.   │                         │    │
│  │  │  MW Present.  │                         │    │
│  │  └──────┬───────┘                         │    │
│  └─────────┼────────────────────────────────┘    │
│            │                                      │
│            ▼                                      │
│  ┌──────────────────┐                            │
│  │  BondingBrother   │                            │
│  └────────┬─────────┘                            │
│           │                                       │
│           ▼                                       │
│  ┌────────────────────────────────────┐          │
│  │           Cores (Strate 4)         │          │
│  │  StrongFather │ KindMother │ ...   │          │
│  └────────────────────────────────────┘          │
│                                                   │
│  ╳ Aucune sortie réseau (Border Guard)           │
└─────────────────────────────────────────────────┘
```

---

## 8. Références

| Document | Rôle |
|----------|------|
| [MiyukiniWatch — Document Fondateur](./MiyukiniWatch%20-%20Document%20Fondateur.md) | Vision, principes fondateurs, métriques, non-lecture des contenus. |
| [MiyukiniWatch — Contraintes et Invariants](./MiyukiniWatch%20-%20Contraintes%20et%20Invariants.md) | Règles non négociables. |
| [MiyukiniWatch — Intégration Miou et Agrégats](./MiyukiniWatch%20-%20Integration%20Miou%20et%20Agregats.md) | Lien avec Miou, format des agrégats. |
| [Miyukini Central — Miou, avatar, bulles et rôle](../MiyukiniCentral/Miyukini%20Central%20-%20Miou%20avatar%20bulles%20et%20role.md) | Rôle de Miou et consommation des agrégats. |
| [Types de Services et Espaces](../../reference/Miyukini%20Conceptual%20References%20-%20Types%20de%20Services%20et%20Espaces.md) | Définition du Type 1 (Service interne COG). |
| Architecture Miyukini (skill miyukini-architecture) | Pyramide, Cores, Lois d'Autonomie. |

---

**Document** : MiyukiniWatch — Architecture et Positionnement  
**Version** : 1.0  
**Date** : 2026-02-14  
**Statut** : Document normatif — référence architecturale du Service MiyukiniWatch
