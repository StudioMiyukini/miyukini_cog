# Miyukini Agenda — Niveaux de sécurité et protection des données

## Contexte

Ce document détaille les **niveaux de sécurité** (WorrySentinel 0–4) appliqués aux données et flux du service **Miyukini Agenda**, ainsi que les **solutions de protection** associées. Il complète le [Document fondateur](../Miyukini%20Agenda%20-%20Document%20Fondateur.md) et s’aligne sur la [Politique de résidence des données sensibles](../../../reference/Miyukini%20Conceptual%20References%20-%20Politique%20Residence%20Donnees%20Sensibles.md) et le [Glossaire Miyukini](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) (Niveaux de sécurité, WorrySentinel).

## Portée / Scope

- **Périmètre** : Niveaux de sécurité des données agenda (entrées, agrégations, export) et mesures de protection (résidence, chiffrement, audit, visibilité).
- **Hors périmètre** : Implémentation technique détaillée (référencée dans les contrats d’Opérateurs et Kits).

---

## 1. Sensibilité des données agenda

### 1.1 Typologie des données traitées

| Type de donnée | Exemple | Sensibilité | Niveau WorrySentinel |
|----------------|---------|-------------|----------------------|
| **Plage temporelle** | Début, fin, fuseau, durée | Faible | 0 |
| **Type d’entrée** | RDV, édition, atelier, participation | Faible | 0 |
| **Identifiant opaque** | ID d’entrée (sans nom, sans détail métier) | Faible à standard | 0–1 |
| **Contexte « mes entrées »** | Liste d’entrées agrégées par utilisateur (sans détail métier) | Standard | 1 |
| **Métadonnées utilisateur** | Qui est concerné (référence utilisateur) | Standard à sensible | 1–2 |
| **Données personnelles ou métier** | Nom du client, objet du RDV, nom de l’exposant, détail candidature | Sensible à critique | 2–3 |

Miyukini Agenda **ne détient pas** la copie canonique des données personnelles ou métier des services consommateurs (JayRDV, MFS). Il travaille sur **références** (plage, type, id opaque) et **synthèses** (conflits, vues agrégées) ; la résidence des données sensibles reste définie par le contrat de chaque service consommateur.

### 1.2 Niveaux WorrySentinel (rappel)

| Niveau | Nom | Description |
|-------|-----|-------------|
| **0** | Public | Données publiques, aucune contrainte stricte |
| **1** | Standard | Données standard, contraintes de base |
| **2** | Sensitive | Données sensibles, contraintes renforcées |
| **3** | Critical | Données critiques, contraintes strictes |
| **4** | Highest | Sécurité maximale, contraintes maximales |

**Gouvernance** : WorrySentinel gouverne les niveaux de sécurité et les états de confiance ; Master Butler gère les permissions ; StrongFather émet les Mandats.

---

## 2. Solutions de protection par niveau

### 2.1 Niveau 0 — Public

| Aspect | Mesure |
|--------|--------|
| **Données concernées** | Plages temporelles anonymisées, types d’entrée génériques, pas de lien utilisateur. |
| **Accès** | Aucune contrainte stricte ; pas de Mandat obligatoire pour lecture. |
| **Résidence** | Non concerné. |
| **Audit** | Optionnel (trace minimale). |
| **Export** | Export public possible (ex. calendrier anonyme d’événements). |

### 2.2 Niveau 1 — Standard

| Aspect | Mesure |
|--------|--------|
| **Données concernées** | Références d’entrées (id opaque), contexte « mes entrées » sans détail métier, vues calendrier agrégées (sans noms de tiers). |
| **Accès** | Mandat de Permission ou Mandat public d’accès selon contexte ; permissions (Master Butler). |
| **Résidence** | Optionnel selon domaine ; pas d’obligation de résidence centralisée. |
| **Audit** | Traçabilité des accès (qui a consulté quoi, quand). |
| **Export** | Export autorisé si Mandat et niveau du destinataire compatibles ; pas de données personnelles de tiers. |

### 2.3 Niveau 2 — Sensitive

| Aspect | Mesure |
|--------|--------|
| **Données concernées** | Données personnelles ou métier liées aux entrées agenda (noms, objets, détails) ; agrégation incluant des données sensibles. |
| **Accès** | Mandat de Permission obligatoire ; résidence centralisée sur COG de référence (voir [Politique de résidence](../../../reference/Miyukini%20Conceptual%20References%20-%20Politique%20Residence%20Donnees%20Sensibles.md)). |
| **Résidence** | COG de référence désigné par le service consommateur (JayRDV, MFS) ; Miyukini Agenda ne détient pas la copie canonique ; accès via Visite gouvernée ou session. |
| **Audit** | Audit des lectures et écritures ; traçabilité complète. |
| **Export** | Export contrôlé ; pas d’exposition hors périmètre autorisé ; pas de noms de tiers en export partagé. |

### 2.4 Niveau 3 — Critical

| Aspect | Mesure |
|--------|--------|
| **Données concernées** | Données critiques (santé, finance, identité renforcée) si un service consommateur en publie. |
| **Accès** | Mandat strict ; résidence centralisée obligatoire ; chiffrement au repos et en transit. |
| **Résidence** | COG de référence unique ; pas de copie sur terminal ou COG tiers sans gouvernance. |
| **Audit** | Audit complet ; révocation immédiate possible (StrongFather, WorrySentinel). |
| **Export** | Export très restreint ; procédure d’autorisation explicite. |

### 2.5 Niveau 4 — Highest

| Aspect | Mesure |
|--------|--------|
| **Données concernées** | Données de sécurité maximale (ex. accès MiyukiniAdmin, interventions TAMR). |
| **Accès** | Isolement renforcé ; procédures d’accès exceptionnel (TAMR, MiyukiniAdmin). |
| **Résidence** | COG de référence dédié ; contraintes maximales. |
| **Audit** | Audit exhaustif ; pas d’accès sans traçabilité. |
| **Export** | Interdit ou procédure exceptionnelle validée. |

---

## 3. Règles de sécurité spécifiques Miyukini Agenda

| Règle | Description |
|-------|-------------|
| **AGD-SEC-1** | Miyukini Agenda ne persiste pas la copie canonique des données personnelles ou métier des services consommateurs ; il travaille sur références et synthèses. |
| **AGD-SEC-2** | Toute agrégation cross-service (vue utilisateur, export) est soumise à Mandat de Permission et au niveau de sécurité du contexte (WorrySentinel). |
| **AGD-SEC-3** | L’export (iCal, PDF) ne doit pas inclure de données au-delà du niveau autorisé pour le destinataire (ex. pas de noms de tiers en export public). |
| **AGD-SEC-4** | En état de confiance dégradé (T2–T4), les capacités d’agrégation ou d’export peuvent être restreintes (Caring Nanny, WorrySentinel). |
| **AGD-SEC-5** | Les services consommateurs (JayRDV, MFS) déclarent le niveau WorrySentinel des données qu’ils publient vers Miyukini Agenda ; Miyukini Agenda applique les règles de visibilité et d’export en conséquence. |
| **AGD-SEC-6** | Pour les événements de type **présence physique** en conflit : pas de blocage de la réservation ou de l’entrée dans l’agenda ; notification systématique ; indicateurs UI (alerte, rouge clignotant) pour pousser à la résolution du conflit jusqu’à ce que l’utilisateur le résolve. |

---

## 4. États de confiance (T0–T4)

En cas de dégradation de l’intégrité du système (états de confiance T1–T4 gouvernés par WorrySentinel), les capacités de Miyukini Agenda peuvent être restreintes :

| État | Effet possible sur Miyukini Agenda |
|------|------------------------------------|
| **T0** | Normal — toutes capacités disponibles. |
| **T1** | Instable — surveillance accrue ; pas de restriction par défaut. |
| **T2** | Dégradé — agrégation cross-service ou export peuvent être limités. |
| **T3** | Restreint — capacités d’agrégation et d’export restreintes ; lecture des entrées de base possible. |
| **T4** | Bloqué — uniquement diagnostics ; pas d’agrégation ni d’export. |

Caring Nanny et WorrySentinel gouvernent ces restrictions ; Miyukini Agenda ne décide pas seul.

---

## 5. Références

| Document | Rôle |
|----------|------|
| [Miyukini Agenda - Document Fondateur](../Miyukini%20Agenda%20-%20Document%20Fondateur.md) | Contexte, besoins, positionnement, sécurité synthétique. |
| [Politique de résidence des données sensibles](../../../reference/Miyukini%20Conceptual%20References%20-%20Politique%20Residence%20Donnees%20Sensibles.md) | Résidence centralisée, COG de référence, niveaux 2+. |
| [Glossaire — Niveaux de sécurité, WorrySentinel, États de confiance](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) | Définitions officielles. |

---

**Document** : Miyukini Agenda — Niveaux de sécurité et protection des données  
**Version** : 1.0  
**Date** : 2026-01-31  
**Statut** : Document de référence (sécurité)
