# Miyukini Core System — Caring Nanny Documentation Fondatrice

## 1. Introduction

### Objet du document

Ce document définit le **Caring Nanny — Documentation Fondatrice** : un contrat normatif, non négociable, et de statut FONDATION qui établit ce que signifie observer et rapporter l'état du système dans Caring Nanny, les caractéristiques conceptuelles du moteur d'état, et les garanties associées à l'observation de l'état dans le Miyukini Core System.

Ce contrat précise la nature conceptuelle de l'état, les invariants d'observation, les notions d'état système et d'état applicatif, sans jamais introduire de détail d'implémentation technique.

### Question fondamentale

Caring Nanny répond à une question essentielle : **Dans quel état se trouve le système à un instant donné ?**

Cette question apparemment simple cache une complexité considérable. L'état d'un système distribué, modulaire, et offline-first n'est pas une valeur unique : c'est une composition d'états partiels, de transitions en cours, de conditions temporaires, et de dépendances croisées. Caring Nanny apporte une réponse structurée, cohérente, et traçable à cette question.

### Portée

Ce contrat s'applique à **toutes les opérations d'observation d'état** dans Caring Nanny et définit de manière absolue :
- la définition formelle de l'état système et de l'état applicatif,
- la notion d'observation conceptuelle,
- les catégories d'états,
- les invariants d'observation,
- les propagations d'états entre modules,
- les garanties d'observation offertes,
- les distinctions entre observation, décision, et exécution.

### Statut contractuel

Ce document est **contractuel, normatif, non discutable, et de statut FONDATION**. Il établit des règles absolues qui ne peuvent être contournées, négociées, ou modifiées. Le contrat prime sur toute considération pratique.

### Relation avec les autres contrats

Ce contrat complète et respecte les documents contractuels existants :
- **KindMother — Documentation Fondatrice** : Caring Nanny observe l'état des données gérées par KindMother, mais ne modifie jamais ces données
- **StrongFather — Documentation Fondatrice** : Caring Nanny informe StrongFather de l'état, mais ne prend jamais de décision
- **BondingBrother — Documentation Fondatrice** : Caring Nanny collabore avec BondingBrother pour la propagation des états, mais ne médiatise jamais les intentions

Il n'introduit aucune contradiction et constitue la définition formelle de ce que signifie observer l'état dans Caring Nanny.

---

## 2. Raison d'être

### Problème que Caring Nanny résout

Dans l'architecture actuelle de MCS, la connaissance de l'état du système est dispersée dans les modules, les adaptateurs, et les produits. Cette dispersion présente plusieurs limitations :

1. **Absence de vision globale** : Chaque composant connaît uniquement son propre état, sans vision de l'état global du système
2. **États incohérents** : Sans coordination, différents composants peuvent avoir des perceptions contradictoires de l'état système
3. **Pas de détection proactive** : Les problèmes ne sont détectés que lorsqu'une opération échoue, pas de manière préventive
4. **Diagnostic difficile** : Sans observateur centralisé, le diagnostic des problèmes nécessite une investigation dispersée
5. **Propagation manuelle** : Les changements d'état doivent être propagés manuellement entre composants, créant des incohérences

Caring Nanny résout ces problèmes en fournissant un observateur unifié qui :
- Centralise l'observation de l'état du système
- Garantit une vision cohérente et non contradictoire
- Détecte proactivement les dégradations et anomalies
- Facilite le diagnostic par une observation structurée
- Propage automatiquement les changements d'état pertinents

### Nécessité du core

Sans Caring Nanny, chaque composant du système devrait :
- Implémenter sa propre logique de détection d'état
- Gérer ses propres notifications de changement d'état
- Déduire l'état des autres composants de manière indirecte
- Gérer les incohérences entre perceptions d'état différentes

Cette approche dispersée conduit à :
- Des erreurs de diagnostic
- Des réactions tardives aux problèmes
- Une complexité accrue dans chaque composant
- Une impossibilité de maintenir une vue d'ensemble cohérente

Caring Nanny centralise cette responsabilité en un point unique, cohérent, et fiable.

---

## 3. Positionnement familial

### Relation avec KindMother

Caring Nanny reconnaît KindMother comme l'autorité absolue des données. La relation entre Caring Nanny et KindMother est une relation d'observation, pas d'interaction bidirectionnelle.

**Ce que Caring Nanny observe de KindMother :**
- L'état de santé de la persistance (disponible, dégradé, indisponible)
- L'état de synchronisation (synchronisé, en cours, désynchronisé, conflits)
- L'état des instances (DB Mère accessible, DB Filles connectées)
- L'état des opérations en cours (écritures en attente, deltas non propagés)

**Ce que Caring Nanny ne fait JAMAIS vis-à-vis de KindMother :**
- Modifier des données
- Déclencher des opérations de synchronisation
- Valider ou invalider des WriteIntent
- Accéder directement à la couche de persistance

La relation est strictement unidirectionnelle : KindMother produit des faits sur les données, Caring Nanny observe l'état de ces données.

### Relation avec StrongFather

Caring Nanny reconnaît StrongFather comme l'autorité absolue des décisions. La relation entre Caring Nanny et StrongFather est une relation d'information, pas de délégation.

**Ce que Caring Nanny informe StrongFather :**
- L'état actuel du système (healthy, degraded, offline, syncing, error)
- Les transitions d'état en cours
- Les conditions qui pourraient affecter les décisions

**Ce que Caring Nanny ne fait JAMAIS vis-à-vis de StrongFather :**
- Prendre une décision basée sur l'état observé
- Modifier une politique ou une contrainte
- Refuser ou accepter une intention
- Influencer le résultat d'une évaluation

StrongFather peut consulter Caring Nanny pour connaître l'état du système, mais toute décision basée sur cet état est prise par StrongFather, jamais par Caring Nanny.

### Relation avec BondingBrother

Caring Nanny collabore avec BondingBrother pour la propagation des états aux produits. La relation est de collaboration passive, pas de médiation active.

**Ce que Caring Nanny fournit à BondingBrother :**
- Les notifications de changement d'état à propager
- L'état des composants concernés par une intention
- Les informations de diagnostic pour le filtrage

**Ce que Caring Nanny ne fait JAMAIS vis-à-vis de BondingBrother :**
- Médiatiser des intentions
- Traduire des demandes de produits
- Filtrer des réponses d'autorités
- Prendre des décisions de routage

Caring Nanny informe, BondingBrother propage. La distinction est fondamentale.

### La famille Miyukini

Dans la famille Miyukini, Caring Nanny est la **nounou attentive** : elle observe, elle surveille, elle rapporte, mais elle n'agit jamais directement. Son rôle est de savoir ce qui se passe, de détecter les anomalies, et d'informer ceux qui ont l'autorité d'agir.

Caring Nanny ne détient aucune autorité sur les données (KindMother), sur les décisions (StrongFather), ou sur la médiation (BondingBrother). Elle est l'observatrice privilégiée, la gardienne de la connaissance de l'état, mais jamais une actrice.

---

## 4. Concepts fondamentaux

### État système

L'**état système** est la condition globale du Miyukini Core System à un instant donné. C'est une synthèse de tous les états partiels des composants, agrégée en une représentation unifiée.

**Caractéristiques :**
- Agrégé : synthèse de multiples états partiels
- Instantané : valide à un moment précis
- Cohérent : sans contradiction interne
- Observable : accessible par interrogation

**Catégories d'état système :**
- **healthy** : Tous les composants fonctionnent normalement, aucune anomalie détectée
- **degraded** : Certains composants fonctionnent en mode dégradé, le système reste opérationnel
- **offline** : Le système fonctionne en mode déconnecté, sans accès aux autorités centrales
- **syncing** : Une synchronisation est en cours, certaines opérations peuvent être différées
- **error** : Une erreur critique a été détectée, certaines opérations ne sont pas possibles

### État applicatif

L'**état applicatif** est la condition d'un module ou composant spécifique au sein du système. C'est un état partiel qui contribue à l'état système global.

**Caractéristiques :**
- Partiel : concerne un composant spécifique
- Contributif : participe à l'état système global
- Autonome : peut être observé indépendamment
- Spécialisé : sémantique propre au composant

**Exemples d'états applicatifs :**
- État d'un module Content : prêt, en chargement, erreur de schéma
- État d'une instance KindMother : connectée, déconnectée, en synchronisation
- État d'une politique StrongFather : active, suspendue, en cours de validation

### Transition d'état

Une **transition d'état** est le passage d'un état à un autre. Elle représente un changement observable dans le système.

**Caractéristiques :**
- Déterministe : un état donné conduit à un ensemble fini d'états possibles
- Observable : la transition elle-même est un fait observable
- Traçable : chaque transition est enregistrée avec son contexte
- Causale : une transition a toujours une cause identifiable

### Condition

Une **condition** est un fait observable qui peut influencer l'état. C'est un élément d'information brut, avant interprétation en termes d'état.

**Caractéristiques :**
- Factuelle : représente un fait, pas une interprétation
- Observable : peut être détectée par Caring Nanny
- Temporelle : valide à un moment donné
- Contextuelle : a un contexte d'observation

**Exemples de conditions :**
- La connexion réseau est disponible
- Le temps de réponse dépasse un seuil
- Un composant ne répond pas
- Une synchronisation a échoué

### Propagation

La **propagation** est le mécanisme par lequel un changement d'état est communiqué aux composants concernés. C'est une diffusion d'information, pas une modification d'état.

**Caractéristiques :**
- Passive : Caring Nanny informe, elle ne modifie pas
- Sélective : seuls les composants concernés sont informés
- Traçable : chaque propagation est enregistrée
- Non bloquante : la propagation n'attend pas de confirmation d'action

---

## 5. Responsabilités exclusives

### Observation de l'état système

Caring Nanny est **exclusivement responsable** de l'observation de l'état système global. Aucun autre composant ne peut prétendre fournir une vision unifiée de l'état du système.

Cette responsabilité inclut :
- L'agrégation des états partiels en état global
- La détection des transitions d'état
- La résolution des contradictions apparentes
- La maintenance d'un historique d'états

### Détection des anomalies

Caring Nanny est **exclusivement responsable** de la détection proactive des anomalies dans le système. Une anomalie est une condition qui s'écarte du comportement attendu.

Cette responsabilité inclut :
- La surveillance des conditions de santé
- La détection des dégradations progressives
- L'identification des patterns anormaux
- L'alerte précoce avant défaillance

### Classification des états

Caring Nanny est **exclusivement responsable** de la classification des états selon les catégories définies (healthy, degraded, offline, syncing, error).

Cette responsabilité inclut :
- L'évaluation des conditions observées
- La catégorisation selon les critères établis
- La cohérence de la classification dans le temps
- La documentation des critères de classification

### Propagation des changements d'état

Caring Nanny est **exclusivement responsable** de la propagation des changements d'état aux composants concernés.

Cette responsabilité inclut :
- L'identification des destinataires d'une notification
- La formulation du message de changement d'état
- Le déclenchement de la propagation via BondingBrother
- La traçabilité des propagations effectuées

### Historique d'observation

Caring Nanny est **exclusivement responsable** de la maintenance d'un historique des observations d'état.

Cette responsabilité inclut :
- L'enregistrement de chaque observation
- La conservation des transitions d'état
- La mise à disposition de l'historique pour audit
- La gestion de la rétention de l'historique

---

## 6. Ce que Caring Nanny ne fait PAS

### Ne modifie aucune donnée

Caring Nanny **ne modifie jamais** aucune donnée dans le système. Elle observe, elle rapporte, mais elle n'écrit jamais. Toute modification de données est du ressort de KindMother, jamais de Caring Nanny.

### Ne prend aucune décision

Caring Nanny **ne prend jamais** de décision basée sur l'état observé. Elle informe StrongFather de l'état, mais la décision de réagir à cet état appartient à StrongFather, jamais à Caring Nanny.

### N'exécute aucune action corrective

Caring Nanny **n'exécute jamais** d'action corrective en réponse à une anomalie détectée. Elle détecte, elle informe, mais elle n'agit jamais. L'action corrective est du ressort du composant concerné ou du produit.

### Ne médiatise pas les intentions

Caring Nanny **ne médiatise jamais** les intentions des produits vers les autorités. La médiation est du ressort de BondingBrother, jamais de Caring Nanny.

### Ne détient pas d'autorité

Caring Nanny **ne détient aucune autorité** sur les données, les décisions, ou les actions. Elle est un observateur privilégié, pas une autorité.

### Ne valide pas les opérations

Caring Nanny **ne valide jamais** les opérations avant leur exécution. La validation est du ressort de KindMother (pour la cohérence des données) ou de StrongFather (pour les permissions et politiques).

### Ne gère pas la persistance

Caring Nanny **ne gère jamais** la persistance de ses observations dans un système externe. Si une persistance est nécessaire, elle est déléguée à KindMother via les canaux appropriés.

### Ne définit pas de règles

Caring Nanny **ne définit jamais** de règles pour la classification des états ou la détection des anomalies. Les règles sont définies par le produit ou l'écosystème, Caring Nanny les applique.

---

## 7. Invariants non négociables

### INV-CN-1 : Observateur pur

Caring Nanny est **exclusivement** un observateur. Elle observe, elle rapporte, elle propage des informations d'état, mais elle ne modifie jamais l'état du système qu'elle observe.

**Conséquence :** Aucune opération de Caring Nanny ne peut avoir d'effet de bord sur les données, les décisions, ou les actions du système.

### INV-CN-2 : Aucune capacité d'exécution

Caring Nanny ne possède **aucune capacité d'exécution**. Elle ne peut pas déclencher d'action, ni directement ni indirectement. Si une action est nécessaire en réponse à un état observé, cette action doit être décidée et exécutée par un autre composant.

**Conséquence :** Caring Nanny ne peut jamais être la cause d'une modification du système.

### INV-CN-3 : Non-autoritaire

Caring Nanny ne détient **aucune autorité** sur aucun aspect du système. Elle ne peut pas valider, invalider, accepter, ou refuser quoi que ce soit.

**Conséquence :** Caring Nanny ne peut jamais bloquer une opération ou imposer une contrainte.

### INV-CN-4 : État cohérent

L'état rapporté par Caring Nanny est **toujours cohérent**. Il n'y a jamais de contradiction dans l'état observé : si un composant est rapporté comme "healthy", il ne peut pas être simultanément rapporté comme "error".

**Conséquence :** Les consommateurs de l'état peuvent se fier à la cohérence de l'information fournie.

### INV-CN-5 : Traçabilité complète

Chaque observation, chaque transition, chaque propagation est **entièrement traçable**. L'historique permet de reconstituer l'évolution de l'état du système dans le temps.

**Conséquence :** L'audit et le diagnostic sont toujours possibles a posteriori.

### INV-CN-6 : Non-bloquant

Caring Nanny ne bloque **jamais** les opérations du système. L'observation est passive et n'interfère pas avec le fonctionnement normal.

**Conséquence :** La présence de Caring Nanny n'a aucun impact sur les performances ou la disponibilité du système.

### INV-CN-7 : Propagation fidèle

Caring Nanny propage les changements d'état **sans modification**. L'information transmise est exactement celle observée, sans interprétation, sans filtrage, sans transformation.

**Conséquence :** Les destinataires reçoivent une information fiable et non altérée.

---

## 8. Interactions avec l'écosystème

### Flux d'observation

Le flux d'observation décrit comment Caring Nanny collecte l'information d'état.

**1. Détection de condition**
- Une condition est détectée dans un composant (KindMother, StrongFather, module, etc.)
- La condition est transmise à Caring Nanny via les canaux d'observation

**2. Évaluation de l'état**
- Caring Nanny évalue la condition selon les critères de classification
- La condition est traduite en état partiel (healthy, degraded, offline, syncing, error)

**3. Agrégation**
- Les états partiels sont agrégés en état système global
- Les contradictions sont résolues selon les règles de priorité

**4. Détection de transition**
- Si l'état global a changé, une transition est enregistrée
- La transition est associée à la condition qui l'a provoquée

### Flux de propagation

Le flux de propagation décrit comment Caring Nanny communique les changements d'état.

**1. Identification des destinataires**
- Caring Nanny identifie les composants concernés par la transition
- La liste des destinataires dépend de la nature de la transition

**2. Formulation du message**
- Le message de notification est construit avec l'état précédent, l'état actuel, et la cause
- Le message est structuré selon le format attendu par BondingBrother

**3. Délégation à BondingBrother**
- Caring Nanny transmet le message à BondingBrother pour propagation
- BondingBrother gère la distribution aux destinataires

**4. Enregistrement**
- La propagation est enregistrée dans l'historique
- La traçabilité est assurée

### Flux de consultation

Le flux de consultation décrit comment les composants interrogent Caring Nanny.

**1. Demande d'état**
- Un composant (StrongFather, produit, module) demande l'état actuel
- La demande peut porter sur l'état global ou sur un composant spécifique

**2. Réponse**
- Caring Nanny retourne l'état demandé
- La réponse inclut l'horodatage de l'observation et le contexte

**3. Aucune modification**
- La consultation n'a aucun effet de bord
- L'état n'est pas modifié par la consultation

### Relations avec les composants

**Avec KindMother :**
- Caring Nanny observe l'état de santé, de synchronisation, et de disponibilité
- Aucune interaction vers KindMother (lecture seule)

**Avec StrongFather :**
- Caring Nanny informe StrongFather des états pour enrichir le contexte des décisions
- StrongFather peut consulter Caring Nanny avant une évaluation
- Aucune influence sur les décisions

**Avec BondingBrother :**
- Caring Nanny utilise BondingBrother pour propager les notifications d'état
- Aucune médiation d'intentions

**Avec les modules SPM :**
- Caring Nanny observe l'état de chaque module
- Aucune interaction directe avec les modules

**Avec les produits :**
- Les produits peuvent consulter Caring Nanny pour connaître l'état
- Les produits reçoivent les notifications de changement d'état via BondingBrother

---

## 9. Vocabulaire canonique

### État

Un **état** est une condition observable d'un composant ou du système à un instant donné. Un état est toujours catégorisé (healthy, degraded, offline, syncing, error), daté, et contextualisé.

### Observation

Une **observation** est l'acte par lequel Caring Nanny détecte et enregistre une condition ou un état. L'observation est passive, non intrusive, et sans effet de bord.

### Transition

Une **transition** est le passage d'un état à un autre. Une transition est toujours causale (provoquée par une condition), traçable (enregistrée avec son contexte), et observable (détectable par Caring Nanny).

### Propagation

La **propagation** est l'acte par lequel Caring Nanny communique un changement d'état aux composants concernés. La propagation est passive (informative, pas directive), fidèle (sans altération), et traçable.

### Condition

Une **condition** est un fait observable qui peut influencer l'état. Une condition est factuelle (représente un fait), temporelle (valide à un moment donné), et contextuelle (a un contexte d'observation).

### Anomalie

Une **anomalie** est une condition qui s'écarte du comportement attendu. Une anomalie est détectée par Caring Nanny, rapportée aux composants concernés, mais jamais corrigée par Caring Nanny.

### Santé

La **santé** est la catégorie d'état qui indique un fonctionnement normal (healthy) ou anormal (degraded, error) d'un composant ou du système.

### Diagnostic

Le **diagnostic** est l'analyse de l'historique d'observations pour identifier la cause d'un problème. Caring Nanny fournit les données pour le diagnostic, mais ne réalise pas le diagnostic lui-même.

### Agrégation

L'**agrégation** est l'opération par laquelle Caring Nanny synthétise les états partiels des composants en état système global. L'agrégation est déterministe, cohérente, et reproductible.

### Historique

L'**historique** est l'ensemble des observations enregistrées par Caring Nanny. L'historique permet la traçabilité, l'audit, et le diagnostic.

---

## 10. Conformité aux Lois d'Autonomie Système

Ce core respecte les **Lois d'Autonomie Système** définies dans [Miyukini Framework - Lois Autonomie Systeme.md](../../reference/Miyukini%20Framework%20-%20Lois%20Autonomie%20Systeme.md). Caring Nanny est **compatible** avec ces lois, avec une extension requise pour la distinction explicite des états d'isolement.

### LOI-1 : Aucune dépendance externe critique à l'exécution

**Conformité :** ✅ **Conforme**

Caring Nanny respecte intégralement LOI-1 :
- L'**observation d'état fonctionne localement**, sans appel externe
- Les observations sont enregistrées localement dans l'historique
- L'absence de connexion ne bloque jamais l'observation
- Les états sont classifiés à partir du contexte local disponible

**Architecture :** Caring Nanny est un observateur passif, fonctionnant uniquement sur les informations locales.

### LOI-2 : Le système accepte l'isolement comme état normal

**Conformité :** ✅ **Conforme — Extension requise**

Caring Nanny respecte LOI-2, avec une extension requise :
- Reconnaît et signale l'état **"isolé" (offline)** comme un état normal, pas comme une anomalie
- Les catégories d'état incluent `offline` comme état valide (Section 4, État système)
- **Extension requise :** Doit distinguer explicitement "isolé" (état normal) de "erreur" (anomalie)

**Architecture :** Les états reconnus sont : `healthy`, `degraded`, `offline`, `syncing`, `error`. L'état `offline` est un état normal, pas une erreur.

### LOI-3 : L'état local est souverain

**Conformité :** ✅ **Conforme**

Caring Nanny respecte intégralement LOI-3 :
- Enregistre l'**historique local de manière complète et autonome**
- Les observations locales constituent une trace d'audit complète
- Les transitions d'état sont enregistrées localement, sans dépendance externe
- L'historique local est la source de vérité pour l'observation d'état

**Architecture :** L'historique d'observation est maintenu localement, sans synchronisation externe obligatoire.

### LOI-4 : Pas de temps global requis

**Conformité :** ✅ **Conforme**

Caring Nanny respecte intégralement LOI-4 :
- Les observations sont **horodatées localement** (via le kernel Clock)
- La comparaison inter-nœuds est **explicitement encadrée** (pas de comparaison automatique de timestamps)
- Les transitions d'état sont basées sur des conditions locales, pas sur des timestamps synchronisés

**Architecture :** Le temps est local et contextuel pour Caring Nanny.

### LOI-5 : Le coût doit être proportionnel au hardware

**Conformité :** ✅ **Conforme**

Caring Nanny respecte intégralement LOI-5 :
- **Observateur passif**, consommation minimale
- Pas de workers permanents coûteux
- Historique géré de manière optimisée (rétention configurable)
- Mémoire prévisible (historique limité, pas de croissance infinie)

**Architecture :** Caring Nanny est conçue pour être légère et prévisible en termes de ressources.

### Extension requise pour LOI-2

**Action nécessaire :** Caring Nanny doit explicitement distinguer :
- **État "isolé" (offline)** : État normal où le système fonctionne sans connexion externe
- **État "erreur" (error)** : Anomalie où le système ne peut pas fonctionner correctement

Cette distinction est critique pour respecter LOI-2 : l'isolement n'est pas une erreur, c'est un état normal.

---

## 11. Conclusion et statut contractuel

### Résumé

Caring Nanny est l'**observateur d'état** du Miyukini Core System. Elle observe, elle détecte, elle classe, elle propage, elle historise. Elle ne modifie jamais, ne décide jamais, n'exécute jamais, ne bloque jamais.

Son rôle est de fournir une **vision unifiée, cohérente, et traçable** de l'état du système à tout instant. Cette vision permet aux autres composants (StrongFather, BondingBrother, produits) de prendre des décisions éclairées, de diagnostiquer des problèmes, et de réagir aux changements.

### Phrase fondatrice

**Caring Nanny est l'observateur d'état privilégié du système, fournissant une vision cohérente et traçable de l'état global et des transitions, sans jamais modifier, décider, ou exécuter.**

Cette phrase résume l'essence de Caring Nanny : observateur (pas acteur), privilégié (vision globale), cohérent (pas de contradiction), traçable (historique complet), passif (aucun effet de bord).

### Garanties contractuelles

Ce contrat garantit que :
- Caring Nanny fournit une vision cohérente de l'état du système
- L'observation n'a aucun effet de bord sur le système
- Les transitions d'état sont traçables et auditables
- La propagation des états est fidèle et non altérée
- Aucune décision n'est prise par Caring Nanny
- Aucune action corrective n'est exécutée par Caring Nanny

### Conformité

Toute implémentation de Caring Nanny doit respecter intégralement ce document. Toute évolution de Caring Nanny doit préserver les invariants définis ici. Toute spécialisation de Caring Nanny doit rester fidèle à la nature décrite ici.

Ce contrat est de statut **FONDATION**. Aucune exception n'est autorisée.

---

**Version :** 1.5  
**Date :** 2026-01-26  
**Statut :** FONDATION — Contrat normatif non négociable  
**Référence :** Miyukini Core System, KindMother Documentation Fondatrice, StrongFather Documentation Fondatrice, BondingBrother Documentation Fondatrice, Miyukini Framework - Lois Autonomie Systeme, [Miyukini Framework - Integrity & Degradation System](../../reference/Miyukini%20Framework%20-%20Integrity%20Degradation%20System.md), [Miyukini Framework - External Signal & Trust Reinforcement Contract](../../reference/Miyukini%20Framework%20-%20External%20Signal%20Trust%20Reinforcement%20Contract.md), [Miyukini Framework - Mobile & WebApp Strategy](../../reference/Miyukini%20Framework%20-%20Mobile%20WebApp%20Strategy.md) (état réseau et dégradation mobile), [Miyukini Framework - Security Protocols](../../reference/Miyukini%20Framework%20-%20Security%20Protocols.md) (authentification en couches RT-SEC-2, détection anomalie RT-SEC-4, dégradation AS-SEC-5), [Miyukini Framework - Security Levels](../../reference/Miyukini%20Framework%20-%20Security%20Levels.md) (adaptation monitoring selon niveau sécurité 0-4)
