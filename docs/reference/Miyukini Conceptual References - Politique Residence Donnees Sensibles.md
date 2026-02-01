# Miyukini Conceptual References — Politique de résidence des données sensibles

## Contexte

Ce document définit la **politique de centralisation et de résidence** des données sensibles dans l'écosystème Miyukini. Il établit le mécanisme par lequel certains **COG officiels (COG de référence)** détiennent la copie canonique de données qui ne doivent pas être dupliquées ou hébergées comme seule copie sur des terminaux tiers.

**Problème adressé :** Certains services manipulent des données (ex. données personnelles d'un exposant) qui doivent rester disponibles pour les acteurs autorisés (ex. organisateurs) même en cas de coupure ou d'indisponibilité du terminal de l'utilisateur (ex. exposant). Si les données restent uniquement dans l'environnement du terminal, elles deviennent inaccessibles.

## Portée / Scope

- **Applicable à :** Architecture des Services, conception des flux de données, déploiement COG
- **Audience :** Architectes, développeurs, responsables sécurité, product owners
- **Statut :** Document de référence normatif

---

## 1. Principe fondateur

> **Les données sensibles à résidence centralisée ne doivent pas avoir pour seule copie un terminal ou un COG tiers. Leur copie canonique réside sur un COG de référence désigné.**

---

## 2. Définitions

### 2.1 COG de référence (Reference COG / Official COG)

**COG désigné comme détenteur canonique** des données sensibles d'un domaine donné.

| Aspect | Description |
|--------|-------------|
| **Rôle** | Héberge l'Instance Mère KindMother (ou l'équivalent « serveur ») pour le domaine concerné |
| **Source de vérité** | La copie canonique des données à résidence centralisée réside sur ce COG |
| **Accès** | Les terminaux et autres COG accèdent via Visite gouvernée ou synchronisation ; ils ne sont pas propriétaires de la seule copie |

**Voir aussi :** [Glossaire — COG de référence](./Miyukini%20Conceptual%20References%20-%20Glossaire.md)

### 2.2 Données à résidence centralisée

Données pour lesquelles la politique impose que la **copie canonique** réside sur un COG de référence. Typiquement :

- Données personnelles (niveau WorrySentinel 2 — Sensitive et au-delà)
- Données métier critiques dont la disponibilité doit être garantie pour des acteurs autres que le détenteur du terminal (ex. organisateurs, support)
- Données qui ne doivent pas être dupliquées ou hébergées comme seule copie sur des terminaux tiers

---

## 3. Règles de la politique

### 3.1 Résidence canonique

| Règle | Description |
|-------|-------------|
| **RÉS-1** | La copie canonique des données à résidence centralisée réside sur un COG de référence désigné |
| **RÉS-2** | Un terminal ou un COG tiers ne peut pas être la seule copie de ces données |
| **RÉS-3** | Les écritures (WriteIntent) sont validées et persistées sur l'Instance Mère (COG de référence) ; les Filles ou terminaux peuvent soumettre des intentions, pas détenir la vérité seule |
| **RÉS-4** | L'accès en lecture depuis un terminal ou un COG tiers s'effectue via Visite gouvernée ou synchronisation (miroir lecture seule ou session gouvernée) |

### 3.2 Lien avec les niveaux de sécurité (WorrySentinel)

| Niveau WorrySentinel | Nom | Politique de résidence |
|----------------------|-----|------------------------|
| 0 | Public | Non concerné |
| 1 | Standard | Optionnel selon domaine |
| **2** | **Sensitive** | **Données personnelles / sensibles : résidence centralisée recommandée ou obligatoire selon contrat Service** |
| 3 | Critical | Résidence centralisée obligatoire |
| 4 | Highest | Résidence centralisée obligatoire |

Les Services et Opérateurs qui manipulent des données de niveau 2 et au-delà déclarent le **COG de référence** (ou le type de COG) autorisé à les détenir. StrongFather / Border Guard peuvent refuser qu'un Mandat ou une Instance Fille constitue la seule copie.

### 3.3 Désignation du COG de référence

Le COG de référence pour un domaine donné est désigné par :

- **Contrat du Service** (ex. Miyukini Festival Service : COG organisateur ou COG du Service)
- **Politique d'environnement** (déclaration Master Butler, validation StrongFather)
- **Règles métier** : qui doit avoir accès en permanence aux données ? → le COG de cet acteur est candidat COG de référence (ex. organisateur pour les données exposant)

---

## 4. Cas d'usage : Exposants (Miyukini Festival Service)

### 4.1 Contexte

Les données personnelles et métier des **exposants** (fiche, candidatures, documents, facturation) doivent rester **disponibles pour les organisateurs** même si le terminal de l'exposant est hors ligne (coupure réseau, appareil éteint, perte).

### 4.2 Application de la politique

| Élément | Décision |
|---------|----------|
| **COG de référence** | COG de l'organisateur (ou COG du Service Festival, selon architecture retenue) |
| **Données concernées** | Fiche exposant, candidatures, documents, facturation (niveau Sensitive) |
| **Terminal exposant** | Utilisateur Visiteur sur le COG de référence, ou Instance Fille dont les écritures remontent à la Mère ; aucune « seule copie » sur le terminal |
| **Effet** | En cas de coupure du terminal exposant, les données restent accessibles sur le COG de référence pour les organisateurs |

### 4.3 Référence documentaire

Voir [Miyukini Festival Service - Document Fondateur](../services/MiyukiniFestivalService/Miyukini%20Festival%20Service%20-%20Document%20Fondateur.md) et [Exposants - Analyse des besoins](../services/MiyukiniFestivalService/publics/Exposants/Exposants%20-%20Analyse%20des%20besoins.md) pour l'application explicite au public Exposants.

---

## 5. Relation avec les autres références

| Document | Relation |
|----------|----------|
| **KindMother** (Instance Mère / Filles) | La Mère est hébergée sur le COG de référence ; les Filles ou terminaux ne détiennent pas la seule copie des données à résidence centralisée |
| **Souveraineté Environnement** | Un COG de référence est un COG souverain ; la résidence des données renforce la disponibilité côté « serveur » désigné |
| **Connexion Inter-COG** | Accès aux données du COG de référence via Visite gouvernée (Utilisateur Visiteur, Visa) ou protocole de synchronisation gouverné |
| **WorrySentinel / Niveaux de sécurité** | Niveaux 2+ déclenchent l'application de la politique de résidence centralisée selon contrat Service |
| **Migration** | Les données migrées vers un COG de référence respectent la diplomatie entre environnements (Border Guard, BondingBrother, KindMother) |

---

## 6. Synthèse

| # | Règle | Statut |
|---|-------|--------|
| 1 | La copie canonique des données sensibles à résidence centralisée réside sur un COG de référence | **NON NÉGOCIABLE** |
| 2 | Un terminal ou un COG tiers ne peut pas être la seule copie de ces données | **NON NÉGOCIABLE** |
| 3 | Le COG de référence est désigné par contrat Service ou politique d'environnement | **NORMATIF** |
| 4 | Niveaux WorrySentinel 2+ : résidence centralisée selon contrat Service | **NORMATIF** |

---

**Date de création :** 2026-01-31  
**Version :** 1.0  
**Statut :** Document de référence normatif

**Références croisées :**

- [Miyukini Conceptual References - Glossaire](./Miyukini%20Conceptual%20References%20-%20Glossaire.md) : COG de référence, Politique de résidence des données sensibles
- [Miyukini Conceptual References - Souverainete Environnement](./Miyukini%20Conceptual%20References%20-%20Souverainete%20Environnement.md)
- [Miyukini Conceptual References - Connexion Inter-COG](./Miyukini%20Conceptual%20References%20-%20Connexion%20Inter-COG.md)
- [Miyukini Conceptual References - Security Levels](./Miyukini%20Conceptual%20References%20-%20Security%20Levels.md) (ou Niveaux de sécurité dans le Glossaire)
- [KindMother — Documentation Fondatrice](../core/KindMother/foundation/KindMother%20-%20Documentation%20Fondatrice.md)
- [Miyukini Festival Service - Document Fondateur](../services/MiyukiniFestivalService/Miyukini%20Festival%20Service%20-%20Document%20Fondateur.md)
