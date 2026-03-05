# JayKoa â€” Niveaux de sÃ©curitÃ© et protection des donnÃ©es

## Contexte

Ce document dÃ©taille les **niveaux de sÃ©curitÃ©** (WorrySentinel 0â€“4) appliquÃ©s aux donnÃ©es et flux du service **JayKoa**, ainsi que les **solutions de protection** associÃ©es. Il complÃ¨te le [Document fondateur](../JayKoa%20-%20Document%20Fondateur.md) et sâ€™aligne sur la [Politique de rÃ©sidence des donnÃ©es sensibles](..//..//..//miyukini-webway-system//reference//_index.md) et le [Glossaire Miyukini](..//..//..//miyukini-webway-system//reference//_index.md) (Niveaux de sÃ©curitÃ©, WorrySentinel).

## PortÃ©e / Scope

- **PÃ©rimÃ¨tre** : Niveaux de sÃ©curitÃ© des donnÃ©es agenda (entrÃ©es, agrÃ©gations, export) et mesures de protection (rÃ©sidence, chiffrement, audit, visibilitÃ©).
- **Hors pÃ©rimÃ¨tre** : ImplÃ©mentation technique dÃ©taillÃ©e (rÃ©fÃ©rencÃ©e dans les contrats dâ€™OpÃ©rateurs et Kits).

---

## 1. SensibilitÃ© des donnÃ©es agenda

### 1.1 Typologie des donnÃ©es traitÃ©es

| Type de donnÃ©e | Exemple | SensibilitÃ© | Niveau WorrySentinel |
|----------------|---------|-------------|----------------------|
| **Plage temporelle** | DÃ©but, fin, fuseau, durÃ©e | Faible | 0 |
| **Type dâ€™entrÃ©e** | RDV, Ã©dition, atelier, participation | Faible | 0 |
| **Identifiant opaque** | ID dâ€™entrÃ©e (sans nom, sans dÃ©tail mÃ©tier) | Faible Ã  standard | 0â€“1 |
| **Contexte Â« mes entrÃ©es Â»** | Liste dâ€™entrÃ©es agrÃ©gÃ©es par utilisateur (sans dÃ©tail mÃ©tier) | Standard | 1 |
| **MÃ©tadonnÃ©es utilisateur** | Qui est concernÃ© (rÃ©fÃ©rence utilisateur) | Standard Ã  sensible | 1â€“2 |
| **DonnÃ©es personnelles ou mÃ©tier** | Nom du client, objet du RDV, nom de lâ€™exposant, dÃ©tail candidature | Sensible Ã  critique | 2â€“3 |

JayKoa **ne dÃ©tient pas** la copie canonique des donnÃ©es personnelles ou mÃ©tier des services consommateurs (JayRDV, JayFestival). Il travaille sur **rÃ©fÃ©rences** (plage, type, id opaque) et **synthÃ¨ses** (conflits, vues agrÃ©gÃ©es) ; la rÃ©sidence des donnÃ©es sensibles reste dÃ©finie par le contrat de chaque service consommateur.

### 1.2 Niveaux WorrySentinel (rappel)

| Niveau | Nom | Description |
|-------|-----|-------------|
| **0** | Public | DonnÃ©es publiques, aucune contrainte stricte |
| **1** | Standard | DonnÃ©es standard, contraintes de base |
| **2** | Sensitive | DonnÃ©es sensibles, contraintes renforcÃ©es |
| **3** | Critical | DonnÃ©es critiques, contraintes strictes |
| **4** | Highest | SÃ©curitÃ© maximale, contraintes maximales |

**Gouvernance** : WorrySentinel gouverne les niveaux de sÃ©curitÃ© et les Ã©tats de confiance ; Master Butler gÃ¨re les permissions ; StrongFather Ã©met les Mandats.

---

## 2. Solutions de protection par niveau

### 2.1 Niveau 0 â€” Public

| Aspect | Mesure |
|--------|--------|
| **DonnÃ©es concernÃ©es** | Plages temporelles anonymisÃ©es, types dâ€™entrÃ©e gÃ©nÃ©riques, pas de lien utilisateur. |
| **AccÃ¨s** | Aucune contrainte stricte ; pas de Mandat obligatoire pour lecture. |
| **RÃ©sidence** | Non concernÃ©. |
| **Audit** | Optionnel (trace minimale). |
| **Export** | Export public possible (ex. calendrier anonyme dâ€™Ã©vÃ©nements). |

### 2.2 Niveau 1 â€” Standard

| Aspect | Mesure |
|--------|--------|
| **DonnÃ©es concernÃ©es** | RÃ©fÃ©rences dâ€™entrÃ©es (id opaque), contexte Â« mes entrÃ©es Â» sans dÃ©tail mÃ©tier, vues calendrier agrÃ©gÃ©es (sans noms de tiers). |
| **AccÃ¨s** | Mandat de Permission ou Mandat public dâ€™accÃ¨s selon contexte ; permissions (Master Butler). |
| **RÃ©sidence** | Optionnel selon domaine ; pas dâ€™obligation de rÃ©sidence centralisÃ©e. |
| **Audit** | TraÃ§abilitÃ© des accÃ¨s (qui a consultÃ© quoi, quand). |
| **Export** | Export autorisÃ© si Mandat et niveau du destinataire compatibles ; pas de donnÃ©es personnelles de tiers. |

### 2.3 Niveau 2 â€” Sensitive

| Aspect | Mesure |
|--------|--------|
| **DonnÃ©es concernÃ©es** | DonnÃ©es personnelles ou mÃ©tier liÃ©es aux entrÃ©es agenda (noms, objets, dÃ©tails) ; agrÃ©gation incluant des donnÃ©es sensibles. |
| **AccÃ¨s** | Mandat de Permission obligatoire ; rÃ©sidence centralisÃ©e sur COG de rÃ©fÃ©rence (voir [Politique de rÃ©sidence](..//..//..//miyukini-webway-system//reference//_index.md)). |
| **RÃ©sidence** | COG de rÃ©fÃ©rence dÃ©signÃ© par le service consommateur (JayRDV, JayFestival) ; JayKoa ne dÃ©tient pas la copie canonique ; accÃ¨s via Visite gouvernÃ©e ou session. |
| **Audit** | Audit des lectures et Ã©critures ; traÃ§abilitÃ© complÃ¨te. |
| **Export** | Export contrÃ´lÃ© ; pas dâ€™exposition hors pÃ©rimÃ¨tre autorisÃ© ; pas de noms de tiers en export partagÃ©. |

### 2.4 Niveau 3 â€” Critical

| Aspect | Mesure |
|--------|--------|
| **DonnÃ©es concernÃ©es** | DonnÃ©es critiques (santÃ©, finance, identitÃ© renforcÃ©e) si un service consommateur en publie. |
| **AccÃ¨s** | Mandat strict ; rÃ©sidence centralisÃ©e obligatoire ; chiffrement au repos et en transit. |
| **RÃ©sidence** | COG de rÃ©fÃ©rence unique ; pas de copie sur terminal ou COG tiers sans gouvernance. |
| **Audit** | Audit complet ; rÃ©vocation immÃ©diate possible (StrongFather, WorrySentinel). |
| **Export** | Export trÃ¨s restreint ; procÃ©dure dâ€™autorisation explicite. |

### 2.5 Niveau 4 â€” Highest

| Aspect | Mesure |
|--------|--------|
| **DonnÃ©es concernÃ©es** | DonnÃ©es de sÃ©curitÃ© maximale (ex. accÃ¨s MiyukiniAdmin, interventions TAMR). |
| **AccÃ¨s** | Isolement renforcÃ© ; procÃ©dures dâ€™accÃ¨s exceptionnel (TAMR, MiyukiniAdmin). |
| **RÃ©sidence** | COG de rÃ©fÃ©rence dÃ©diÃ© ; contraintes maximales. |
| **Audit** | Audit exhaustif ; pas dâ€™accÃ¨s sans traÃ§abilitÃ©. |
| **Export** | Interdit ou procÃ©dure exceptionnelle validÃ©e. |

---

## 3. RÃ¨gles de sÃ©curitÃ© spÃ©cifiques JayKoa

| RÃ¨gle | Description |
|-------|-------------|
| **AGD-SEC-1** | JayKoa ne persiste pas la copie canonique des donnÃ©es personnelles ou mÃ©tier des services consommateurs ; il travaille sur rÃ©fÃ©rences et synthÃ¨ses. |
| **AGD-SEC-2** | Toute agrÃ©gation cross-service (vue utilisateur, export) est soumise Ã  Mandat de Permission et au niveau de sÃ©curitÃ© du contexte (WorrySentinel). |
| **AGD-SEC-3** | Lâ€™export (iCal, PDF) ne doit pas inclure de donnÃ©es au-delÃ  du niveau autorisÃ© pour le destinataire (ex. pas de noms de tiers en export public). |
| **AGD-SEC-4** | En Ã©tat de confiance dÃ©gradÃ© (T2â€“T4), les capacitÃ©s dâ€™agrÃ©gation ou dâ€™export peuvent Ãªtre restreintes (Caring Nanny, WorrySentinel). |
| **AGD-SEC-5** | Les services consommateurs (JayRDV, JayFestival) dÃ©clarent le niveau WorrySentinel des donnÃ©es quâ€™ils publient vers JayKoa ; JayKoa applique les rÃ¨gles de visibilitÃ© et dâ€™export en consÃ©quence. |
| **AGD-SEC-6** | Pour les Ã©vÃ©nements de type **prÃ©sence physique** en conflit : pas de blocage de la rÃ©servation ou de lâ€™entrÃ©e dans lâ€™agenda ; notification systÃ©matique ; indicateurs UI (alerte, rouge clignotant) pour pousser Ã  la rÃ©solution du conflit jusquâ€™Ã  ce que lâ€™utilisateur le rÃ©solve. |

---

## 4. Ã‰tats de confiance (T0â€“T4)

En cas de dÃ©gradation de lâ€™intÃ©gritÃ© du systÃ¨me (Ã©tats de confiance T1â€“T4 gouvernÃ©s par WorrySentinel), les capacitÃ©s de JayKoa peuvent Ãªtre restreintes :

| Ã‰tat | Effet possible sur JayKoa |
|------|------------------------------------|
| **T0** | Normal â€” toutes capacitÃ©s disponibles. |
| **T1** | Instable â€” surveillance accrue ; pas de restriction par dÃ©faut. |
| **T2** | DÃ©gradÃ© â€” agrÃ©gation cross-service ou export peuvent Ãªtre limitÃ©s. |
| **T3** | Restreint â€” capacitÃ©s dâ€™agrÃ©gation et dâ€™export restreintes ; lecture des entrÃ©es de base possible. |
| **T4** | BloquÃ© â€” uniquement diagnostics ; pas dâ€™agrÃ©gation ni dâ€™export. |

Caring Nanny et WorrySentinel gouvernent ces restrictions ; JayKoa ne dÃ©cide pas seul.

---

## 5. RÃ©fÃ©rences

| Document | RÃ´le |
|----------|------|
| [JayKoa - Document Fondateur](../JayKoa%20-%20Document%20Fondateur.md) | Contexte, besoins, positionnement, sÃ©curitÃ© synthÃ©tique. |
| [Politique de rÃ©sidence des donnÃ©es sensibles](..//..//..//miyukini-webway-system//reference//_index.md) | RÃ©sidence centralisÃ©e, COG de rÃ©fÃ©rence, niveaux 2+. |
| [Glossaire â€” Niveaux de sÃ©curitÃ©, WorrySentinel, Ã‰tats de confiance](..//..//..//miyukini-webway-system//reference//_index.md) | DÃ©finitions officielles. |

---

**Document** : JayKoa â€” Niveaux de sÃ©curitÃ© et protection des donnÃ©es  
**Version** : 1.0  
**Date** : 2026-01-31  
**Statut** : Document de rÃ©fÃ©rence (sÃ©curitÃ©)

