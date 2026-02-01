# Miyukini Purse — Parcours, capacités et livrables

## Contexte

Ce document détaille le **parcours**, les **capacités** et les **livrables** du point d’entrée **Miyukini Purse** (perso/individuel) du service COG Miyukini Account. Il complète le [Document fondateur Miyukini Account](../../Miyukini%20Account%20-%20Document%20Fondateur.md) et s’appuie sur l’[analyse des besoins](./Purse%20-%20Analyse%20des%20besoins.md) et le document [Operateurs et Toolkits](./Purse%20-%20Operateurs%20et%20Toolkits.md).

## Portée / Scope

- **Public** : Particuliers, foyers (point d’entrée Miyukini Purse).
- **Périmètre** : Parcours (onboarding, tableau de bord, mouvements, budgets occasionnels, objectifs, rapports, export, alertes), capacités et livrables associés.
- **Hors périmètre** : Devis et facturation légale (réservés au point d’entrée Miyukini Account) ; spécifications techniques (API, schémas).

---

## 1. Profil du public

| Critère | Description |
|---------|-------------|
| **Qui** | Particuliers ou foyers qui souhaitent tenir un budget personnel et/ou gérer des budgets occasionnels (vacances, Noël, mariage, travaux). |
| **Compte** | Compte Miyukini Purse (email, mot de passe ou lien magique, identité minimale) ; pas d’exigence SIRET ni de facturation. |
| **Accès** | Authentification (Miyauth) ; session gouvernée par Mandat ; point d’entrée Miyukini Purse. |
| **Espace** | Tableau de bord Purse (solde, synthèse, mouvements, budgets occasionnels, objectifs, rapports, export, alertes). |

---

## 2. Parcours utilisateur

### 2.1 Parcours onboarding (inscription Purse)

1. **Accès** : L’utilisateur accède à la page d’inscription Miyukini Purse.
2. **Formulaire** : Saisie email, mot de passe ou lien magique, identité minimale (nom, prénom optionnel).
3. **Validation** : Validation email si configurée ; pas d’exigence SIRET ni de facturation.
4. **Résultat** : Compte Purse créé ; redirection vers le tableau de bord.

**Livrables sollicités** : Formulaire d’inscription dédié Purse ; validation email ; tableau de bord (PUR-01, PUR-02).

### 2.2 Parcours tableau de bord et mouvements

1. **Connexion** : L’utilisateur se connecte (Miyauth) ; session gouvernée par Mandat.
2. **Tableau de bord** : Affichage du solde courant, synthèse (revenus vs dépenses), répartition par catégorie, évolution (filtres : mois, trimestre, année).
3. **Saisie d’un mouvement** : Saisie manuelle (date, montant, libellé, catégorie) ou import CSV ; correction et suppression selon règles.
4. **Historique** : Liste paginée et filtrable des mouvements ; recherche par libellé ; export liste (CSV) pour usage personnel.

**Livrables sollicités** : Tableau de bord (solde, synthèse, répartition) ; formulaire de saisie mouvement ; liste historique ; export CSV (PUR-04 à PUR-07).

### 2.3 Parcours budgets occasionnels (vacances, Noël, etc.)

1. **Création** : L’utilisateur crée un budget occasionnel (nom, montant cible ou plafond, date de début/fin optionnelle).
2. **Affectation** : Lors de la saisie d’un mouvement, possibilité d’affecter à un budget occasionnel ; solde du budget (dépensé / restant) mis à jour.
3. **Liste et détail** : Liste des budgets avec indicateur (solde, % utilisé) ; fiche par budget (mouvements, solde, objectif) ; archivage ou clôture quand projet terminé.
4. **Alerte** : Alerte si dépassement ou seuil configuré (Miyunotify).

**Livrables sollicités** : Formulaire création budget occasionnel ; affectation mouvement à un budget ; liste et fiche budgets ; archivage/clôture ; alerte dépassement (PUR-08 à PUR-10, PUR-15).

### 2.4 Parcours objectifs (épargne, dépense)

1. **Définition** : L’utilisateur définit un objectif (libellé, montant cible, période ou récurrent) ; lien optionnel à un budget occasionnel ou une catégorie.
2. **Suivi** : Tableau de bord objectifs : progression (atteint, en cours, en retard) ; alerte si objectif atteint ou en retard (Miyunotify optionnel).
3. **Rappels (optionnel)** : Intégration Miyukini Agenda pour rappels (échéance objectif) ; références temporelles uniquement ; source de vérité reste Miyukini Account.

**Livrables sollicités** : Formulaire objectif ; tableau de bord objectifs ; alertes ; rappels agenda optionnels (PUR-11, PUR-12, PUR-16).

### 2.5 Parcours rapports et export

1. **Rapports** : Consultation de rapports prédéfinis (mensuel, trimestriel, annuel) : synthèse, évolution, répartition par catégorie, comparaison de périodes ; graphiques et tableaux ; niveau 2 (pas de données au-delà du niveau autorisé).
2. **Export** : Export PDF (synthèse, rapport) ou CSV (mouvements) ; périmètre limité aux données de l’utilisateur ; pas d’export de données de paiement brutes.

**Livrables sollicités** : Rapports prédéfinis ; graphiques et tableaux ; export PDF/CSV (PUR-13, PUR-14).

### 2.6 Parcours alertes

1. **Configuration** : L’utilisateur configure les alertes (seuil solde, dépassement budget occasionnel, objectif atteint/en retard).
2. **Réception** : Notification (Miyunotify) par email et/ou in-app ; préférences utilisateur.

**Livrables sollicités** : Page configuration alertes ; notifications email/in-app ; préférences (PUR-15).

---

## 3. Capacités et livrables (synthèse)

| Capacité | Description | Livrable | Besoin(s) couvert(s) |
|----------|-------------|----------|----------------------|
| **Compte Purse** | Inscription, connexion, déconnexion, récupération mot de passe. | Formulaire inscription/connexion ; session gouvernée. | PUR-01, PUR-02 |
| **Données niveau 2** | Données Purse au minimum niveau 2 ; résidence selon politique. | Gouvernance WorrySentinel, KindMother ; pas d’exposition hors Mandat. | PUR-03 |
| **Mouvements** | Enregistrement revenus/dépenses, catégories, solde, synthèse, historique. | Tableau de bord ; formulaire saisie ; liste historique ; export CSV. | PUR-04 à PUR-07 |
| **Budgets occasionnels** | Création, suivi dépenses, solde, liste/détail, archivage/clôture. | Formulaire budget ; affectation mouvement ; liste et fiche budgets ; alerte dépassement. | PUR-08 à PUR-10, PUR-15 |
| **Objectifs** | Définition, suivi, alerte atteint/en retard. | Formulaire objectif ; tableau de bord objectifs ; notifications. | PUR-11, PUR-12 |
| **Rapports et export** | Rapports prédéfinis, graphiques, export PDF/CSV. | Rapports mensuel/trimestriel/annuel ; export PDF/CSV. | PUR-13, PUR-14 |
| **Alertes** | Configuration et réception alertes (seuil, budget, objectif). | Page configuration ; notifications email/in-app. | PUR-15 |
| **Rappels (optionnel)** | Rappels via Miyukini Agenda (échéance, clôture). | Références temporelles dans l’agenda ; pas de donnée financière. | PUR-16 |

---

## 4. Références

| Document | Rôle |
|----------|------|
| [Purse - Analyse des besoins](./Purse%20-%20Analyse%20des%20besoins.md) | Liste exhaustive des besoins PUR-01 à PUR-16, NFR-PUR-01 à NFR-PUR-07. |
| [Purse - Operateurs et Toolkits](./Purse%20-%20Operateurs%20et%20Toolkits.md) | Matrice Besoin → Service / Opérateur / Toolkit. |
| [Miyukini Account - Document Fondateur](../../Miyukini%20Account%20-%20Document%20Fondateur.md) | Contexte service COG, points d’entrée Purse/Account. |
| [Points d’entrée Purse et Account](../../reference/Miyukini%20Account%20-%20Points%20Entree%20Purse%20et%20Account.md) | Périmètre Purse, capacités exposées. |

---

**Document** : Miyukini Purse — Parcours, capacités et livrables  
**Version** : 1.0  
**Date** : 2026-01-31  
**Statut** : Document de référence (parcours, capacités, livrables)
