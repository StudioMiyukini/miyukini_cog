# JayFestival — Parcours Utilisateurs Schema Flux

## Contexte

Ce document décrit les **parcours utilisateurs** de JayFestival tels que définis dans le schéma de flux de référence. Il identifie les **3 types d'utilisateurs principaux** (Organisateur, Exposant, Visiteur) et détaille leurs interactions avec le système ainsi que les interconnexions avec les services Jay partenaires (JayKoa, JayXpose, JayKonta).

**Source** : Schéma de flux `reference/Untitled*.jpg` (7 images découpées).

## Portée / Scope

- **Périmètre** : Parcours complets des 3 rôles utilisateurs, points d'entrée, flux de décision, interconnexions services.
- **Hors périmètre** : Spécifications techniques des API, maquettes UI détaillées (voir `publics/` pour les écrans).
- **Audience** : Équipes produit, développement, UX.

---

## 1. Vue d'ensemble des interconnexions

```
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│   Central App   │────▶│  JayFestival    │◀───▶│    JayKoa       │
│ (Authentification)    │  (Portail)      │     │ (Calendrier)    │
└─────────────────┘     └────────┬────────┘     └─────────────────┘
                                 │
                    ┌────────────┼────────────┐
                    ▼            ▼            ▼
            ┌───────────┐ ┌───────────┐ ┌───────────┐
            │ JayXpose  │ │ JayKonta  │ │ JayFaim   │
            │ (Profil)  │ │ (Compta)  │ │ (Restau)  │
            └───────────┘ └───────────┘ └───────────┘
```

| Service | Rôle dans le flux |
|---------|-------------------|
| **Central App** | Authentification centralisée, point d'entrée unique |
| **JayFestival** | Portail événementiel principal |
| **JayKoa** | Gestion calendrier/agenda, synchronisation des dates |
| **JayXpose** | Profil entreprise/exposant (prérequis obligatoire) |
| **JayKonta** | Budget, devis, facturation, encaissements |

---

## 2. Parcours ORGANISATEUR

### 2.1 Point d'entrée et prérequis

```
"Je suis organisateur"
        │
        ▼
┌─────────────────────────────────────┐
│ Récupération données JayXpose       │
│ avec statut compatible              │
└─────────────────────────────────────┘
        │
        ├──── Pas de donnée JayXpose ────▶ Renvoi vers JayXpose
        │                                   (création profil requise)
        │
        ▼ Compatible
┌─────────────────────────────────────┐
│ Création de l'événement             │
│ Formulaire avec toutes les          │
│ informations nécessaires            │
└─────────────────────────────────────┘
```

**Prérequis** : Profil JayXpose valide avec statut compatible.

### 2.2 Gestion d'une édition

```
Nouvelle "édition"
        │
        ▼
┌─────────────────────────────────────┐
│ Formulaire des informations         │
│ de l'édition                        │
└─────────────────────────────────────┘
        │
        ▼
┌─────────────────────────────────────┐
│ JayKoa                              │
│ Ajout des dates automatiquement     │
└─────────────────────────────────────┘
```

### 2.3 Configuration de l'édition

L'organisateur définit les éléments suivants, chacun générant automatiquement des documents ou données système :

| Élément configuré | Action organisateur | Génération automatique |
|-------------------|---------------------|------------------------|
| **Conditions d'accès Visiteur** | Remplissage formulaire | Règlement officiel des conditions d'accès aux visiteurs |
| **Animations** | Création via formulaire (infos, conditions de participation, horaires, inscription) | Ajout au "PROGRAMME" de l'édition + Ajout automatique des Dates/heures d'ouverture |
| **Plan d'implantation** | Dessin/configuration du plan | Plan final (avec statuts : Réserve / Valide / Confirme) |
| **Conditions de candidature exposant** | Rédaction des conditions | Règlement officiel des conditions de participation des exposants |

### 2.4 Gestion des ressources humaines

L'organisateur peut configurer :
- **L'équipe organisatrice** — membres avec droits d'administration
- **Les équipes de bénévoles** — planning et affectations

### 2.5 Outputs automatiques de l'édition

Une fois l'édition configurée, le système génère automatiquement :
- **"LE PROGRAMME" de l'édition** — agrégation des animations créées
- **Liste des exposants** — mise à jour au fil des validations
- **Plan final** — version consolidée du plan d'implantation
- **Fiche événement** — dans la version de l'édition active (visible publiquement)

### 2.6 Traitement des candidatures exposants

```
Parcours de candidature en fonction
des conditions des organisateurs
                │
                ▼
        Bon pour accord
        (Exposant accepte le règlement)
                │
                ▼
┌───────────────────────────────────────────────────────────────┐
│                  DÉCISION DES ORGANISATEURS                   │
├───────────────────┬───────────────────┬───────────────────────┤
│     ACCEPTÉ       │      REFUS        │   LISTE D'ATTENTE     │
└─────────┬─────────┴─────────┬─────────┴───────────┬───────────┘
          │                   │                     │
          ▼                   ▼                     ▼
┌─────────────────┐  ┌─────────────────┐  ┌─────────────────────┐
│ Envoi facture   │  │ Notification    │  │ Ajout sur liste     │
│ à régler        │  │ à l'exposant    │  │ d'attente           │
│                 │  │                 │  │ Ordonnée par ordre  │
│ Accès aux       │  │ Enregistrement  │  │ d'arrivée           │
│ moyens de       │  │ historique      │  │                     │
│ paiement        │  │ avec motif      │  │                     │
└─────────────────┘  └─────────────────┘  └─────────────────────┘
```

### 2.7 Suivi des paiements exposants

```
            Paiement effectué par l'exposant
                        │
          ┌─────────────┴─────────────┐
          │                           │
          ▼ OUI                       ▼ NON
┌─────────────────────┐     ┌─────────────────────┐
│ Validation de la    │     │ Relance de paiement │
│ participation       │     └──────────┬──────────┘
│                     │                │
│ Ajout de la fiche   │                ▼
│ de l'exposant       │     ┌─────────────────────┐
│ à l'édition         │     │ Décision des        │
│                     │     │ organisateurs       │
│ → Ajout automatique │     └──────────┬──────────┘
│   Liste exposants   │                │
└─────────────────────┘      ┌─────────┴─────────┐
                             │                   │
                             ▼                   ▼
                   ┌──────────────┐    ┌──────────────────┐
                   │   Accepté    │    │ Refus ou délai   │
                   │   (relance)  │    │ dépassé          │
                   └──────────────┘    └────────┬─────────┘
                                                │
                                                ▼
                                      ┌──────────────────┐
                                      │ Notification     │
                                      │ exposant         │
                                      │ Historique +     │
                                      │ motif            │
                                      └──────────────────┘
```

**Automatisation** : Ajout automatique de la date J-3 avant la fin du délai de paiement pour déclenchement de la relance.

---

## 3. Parcours EXPOSANT

### 3.1 Point d'entrée et prérequis

```
Connexion par Central App
        │
        ▼
Portail JayFestival
        │
        ▼
"Devenir exposant"
        │
        ▼
┌─────────────────────────────────────┐
│ Récupération données JayXpose       │
│ avec statut compatible              │
└─────────────────────────────────────┘
        │
        ├──── Pas de donnée JayXpose ────▶ Renvoi vers JayXpose
        │                                   (création profil requise)
        │
        ▼ Compatible
┌─────────────────────────────────────┐
│ Accès au portail exposant           │
└─────────────────────────────────────┘
```

**Prérequis** : Authentification via Central App + Profil JayXpose valide avec statut compatible.

### 3.2 Navigation et gestion du profil

Une fois authentifié, l'exposant dispose de :

| Fonctionnalité | Description |
|----------------|-------------|
| **Navigation événements** | Parcours des différents événements avec bouton "Candidater" |
| **"Mon Profil Exposant"** | Accès à la gestion de son profil |
| **Synchronisation JayXpose** | Informations automatiquement synchronisées avec JayXposant |
| **Visibilité publique** | Sélection des informations visibles en public |

### 3.3 Processus de candidature

```
Navigation dans les différents événements
avec un bouton "Candidater"
                │
                ▼
┌─────────────────────────────────────┐
│ Parcours de candidature en fonction │
│ des conditions des organisateurs    │
└─────────────────────────────────────┘
                │
                ▼
        Bon pour accord
        (Acceptation du règlement officiel
         des conditions de participation)
                │
                ▼
┌─────────────────────────────────────┐
│      DÉCISION DES ORGANISATEURS     │
├─────────┬─────────┬─────────────────┤
│ ACCEPTÉ │  REFUS  │ LISTE D'ATTENTE │
└────┬────┴────┬────┴────────┬────────┘
     │         │             │
     ▼         ▼             ▼
```

### 3.4 Issue ACCEPTÉ — Flux de paiement

```
ACCEPTÉ
    │
    ▼
┌─────────────────────────────────────┐
│ Envoi de la facture à régler        │
│ Accès aux moyens de paiement        │
└─────────────────────────────────────┘
    │
    ├──── Paiement effectué ────▶ Validation de la participation
    │                             Ajout fiche exposant à l'édition
    │                             → Ajout automatique Liste exposants
    │
    ├──── Paiement non effectué ────▶ Relance de paiement
    │                                  → Décision organisateurs
    │
    └──── Refus ou délai dépassé ────▶ Notification exposant
                                        Historique + motif
```

### 3.5 Issue REFUS

```
REFUS
    │
    ▼
┌─────────────────────────────────────┐
│ Notification à l'exposant           │
│                                     │
│ Enregistrement dans l'historique    │
│ de l'exposant et des organisateurs  │
│ avec le motif                       │
└─────────────────────────────────────┘
```

### 3.6 Issue LISTE D'ATTENTE

```
LISTE D'ATTENTE
    │
    ▼
┌─────────────────────────────────────┐
│ Ajout sur liste d'attente           │
│ Liste ordonnée par ordre d'arrivée  │
└─────────────────────────────────────┘
    │
    ├──── Place disponible ────▶ Retour au flux ACCEPTÉ
    │
    └──── Refus ou délai dépassé ────▶ Notification exposant
                                        Historique + motif
```

### 3.7 Résultat final positif

Après validation complète du paiement :
1. **Validation de la participation de l'exposant**
2. **Ajout de la fiche de l'exposant à l'édition**
3. **Ajout automatique à la Liste des exposants** (visible sur le plan final et le programme)

---

## 4. Parcours VISITEUR

### 4.1 Navigation dans le catalogue d'événements

```
┌─────────────────────────────────────────────────────────────┐
│         Navigation dans le catalogue d'événements          │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  Critères de tri et filtrage :                              │
│  • Ordonné par date                                         │
│  • Filtre par catégories                                    │
│  • Filtre par ville                                         │
│  • Filtre par département                                   │
│  • Filtre par conditions d'accès                            │
│  • Filtre par prix d'entrée                                 │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### 4.2 Actions disponibles

| Action | Description | Service concerné |
|--------|-------------|------------------|
| **Consultation fiche** | Fiche événement dans la version de l'édition active | JayFestival |
| **Favoris** | Liste des événements favoris (sauvegarde personnelle) | JayFestival |
| **Sélection de visite** | Ajout de l'événement à son calendrier | JayKoa |

### 4.3 Flux de sélection de visite

```
Catalogue d'événements
        │
        ▼
Sélection de visite
        │
        ▼
┌─────────────────────────────────────┐
│ JayKoa                              │
│ Synchronisation calendrier          │
│                                     │
│ • Ajout automatique des dates       │
│   d'installation et d'ouverture     │
│   au public                         │
└─────────────────────────────────────┘
```

### 4.4 Informations accessibles au visiteur

Sur la **Fiche événement dans la version de l'édition active** :
- Informations générales de l'événement
- Programme de l'édition
- Liste des exposants (avec liens vers fiches exposants)
- Plan final (si publié)
- Conditions d'accès visiteur
- Règlement officiel des conditions d'accès aux visiteurs

---

## 5. Synthèse des flux par service

### 5.1 JayKoa — Points d'intégration

| Flux | Déclencheur | Action JayKoa |
|------|-------------|---------------|
| Création édition | Organisateur crée une édition | Ajout des dates automatiquement |
| Sélection visite | Visiteur sélectionne un événement | Synchronisation calendrier personnel |
| Création animation | Organisateur crée une animation | Ajout automatique des Dates/heures d'ouverture |
| Installation | Validation participation exposant | Ajout automatique des dates d'installation |

### 5.2 JayXpose — Points d'intégration

| Flux | Déclencheur | Action JayXpose |
|------|-------------|-----------------|
| Entrée organisateur | "Je suis organisateur" | Récupération données avec statut compatible |
| Entrée exposant | "Devenir exposant" | Récupération données avec statut compatible |
| Profil exposant | Accès "Mon Profil Exposant" | Synchronisation informations JayXposant |
| Redirection | Pas de donnée JayXpose | Renvoi vers JayXpose |

### 5.3 JayKonta — Points d'intégration (via flux facturation)

| Flux | Déclencheur | Action JayKonta |
|------|-------------|-----------------|
| Candidature acceptée | Décision organisateurs = ACCEPTÉ | Envoi de la facture à régler |
| Paiement | Exposant accède aux moyens de paiement | Traitement encaissement |
| Relance | Délai J-3 avant fin | Relance de paiement |

---

## 6. États et transitions clés

### 6.1 États d'une candidature exposant

```
┌─────────────┐
│   SOUMISE   │
└──────┬──────┘
       │ Décision organisateurs
       │
┌──────┴──────┬───────────────┬──────────────┐
▼             ▼               ▼              ▼
┌─────────┐ ┌─────────────┐ ┌────────────┐ (Timeout)
│ ACCEPTÉE│ │LISTE ATTENTE│ │  REFUSÉE   │
└────┬────┘ └──────┬──────┘ └────────────┘
     │             │
     │ Facture     │ Place dispo
     ▼             │
┌─────────────┐    │
│ EN PAIEMENT │◀───┘
└──────┬──────┘
       │
┌──────┴──────┬──────────────┐
▼             ▼              ▼
┌─────────┐ ┌────────────┐ ┌────────────┐
│ VALIDÉE │ │  RELANCÉE  │ │  EXPIRÉE   │
└─────────┘ └────────────┘ └────────────┘
```

### 6.2 États d'un emplacement (Plan)

| État | Description |
|------|-------------|
| **Réserve** | Emplacement pré-réservé, en attente de confirmation |
| **Valide** | Emplacement validé par l'organisateur |
| **Confirme** | Emplacement confirmé (paiement effectué) |

---

## 7. Règles métier extraites du schéma

1. **Prérequis JayXpose obligatoire** — Tout organisateur ou exposant doit avoir un profil JayXpose valide avec statut compatible avant d'accéder aux fonctionnalités JayFestival.

2. **Synchronisation automatique JayKoa** — Les dates sont automatiquement propagées vers JayKoa lors de la création d'éditions, d'animations, ou de sélections de visite.

3. **Liste d'attente FIFO** — La liste d'attente est ordonnée par ordre d'arrivée (First In, First Out).

4. **Relance automatique J-3** — Une relance de paiement est automatiquement déclenchée 3 jours avant la fin du délai de paiement.

5. **Traçabilité des décisions** — Toute décision (acceptation, refus, expiration) est enregistrée dans l'historique de l'exposant ET des organisateurs avec le motif.

6. **Génération automatique des règlements** — Les conditions d'accès et de participation génèrent automatiquement les règlements officiels correspondants.

7. **Visibilité contrôlée exposant** — L'exposant choisit quelles informations de son profil JayXpose sont visibles publiquement sur JayFestival.

---

## 8. Voir aussi

- [JayFestival - Interpolarite Services Jay](./JayFestival%20-%20Interpolarite%20Services%20Jay.md) — Détails des couplages techniques avec JayKoa, JayXpose, JayKonta
- [JayFestival - Document Fondateur](../JayFestival%20-%20Document%20Fondateur.md) — Vision et raison d'être
- **Parcours détaillés par public** :
  - [Organisateurs - Parcours Capacites Livrables](../publics/Organisateurs/Organisateurs%20-%20Parcours%20Capacites%20Livrables.md)
  - [Exposants - Parcours Capacites Dashboard](../publics/Exposants/Exposants%20-%20Parcours%20Capacites%20Dashboard.md)
  - [Visiteurs - Parcours Capacites Services](../publics/Visiteurs/Visiteurs%20-%20Parcours%20Capacites%20Services.md)
- **Écrans par public** :
  - [Organisateurs - Ecrans et cycle](../publics/Organisateurs/Organisateurs%20-%20Ecrans%20et%20cycle.md)
  - [Exposants - Ecrans et cycle](../publics/Exposants/Exposants%20-%20Ecrans%20et%20cycle.md)
  - [Visiteurs - Ecrans et cycle](../publics/Visiteurs/Visiteurs%20-%20Ecrans%20et%20cycle.md)

---

**Document** : JayFestival — Parcours Utilisateurs Schema Flux  
**Version** : 1.0  
**Date** : 2026-02-09  
**Statut** : Document de référence — parcours utilisateurs issus du schéma de flux  
**Source** : Schéma `reference/Untitled*.jpg` (7 images)
