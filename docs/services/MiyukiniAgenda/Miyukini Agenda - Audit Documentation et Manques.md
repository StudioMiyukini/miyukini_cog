# Miyukini Agenda — Audit de la documentation et manques pour un service complet

## Contexte

Ce document constitue un **audit** de la documentation Miyukini Agenda pour identifier ce qui **manque** afin d’avoir un **service complet** : Opérateurs et Toolkits, Équipes d’Opérateurs, intégration détaillée avec les autres services, filtres de l’agenda, vue des événements publics et sélection pour valider sur son propre agenda.

## Portée / Scope

- **Périmètre** : Audit des documents existants ; liste des manques ; référence aux documents créés ou enrichis pour les combler.
- **Hors périmètre** : Implémentation technique (référencée dans Bornage et Opérateurs/Toolkits).

---

## 1. État des lieux (avant audit)

| Document | Contenu actuel | Manques identifiés |
|----------|----------------|---------------------|
| **Document Fondateur** | Besoins, positionnement, intégration synthétique, sécurité. | Pas de section dédiée Opérateurs/Toolkits/Équipe ; pas de parcours « événements publics → ajout à mon agenda ». |
| **Écrans et UI** | Composants AGD-UI-01 à 08 (calendrier, alerte, export, filtre source/type, prochaine entrée, conflit, liste, libre/occupé). | Filtres **détaillés** (période, statut, visibilité public/privé) peu explicites ; **vue événements publics** et **sélection pour ajout à l’agenda** absents. |
| **Parcours Utilisateurs** | Consultation agenda, vérification conflit, export, agrégation ; parcours côté service (publication, interrogation). | Parcours **« Vue événements publics → Sélection → Validation sur mon agenda »** absent. |
| **Integration Services Consommateurs** | Principe, JayRDV, MFS, agrégation, futurs services. | **Séquence d’intégration** (qui appelle quoi, dans quel ordre) et **contrat des filtres** (paramètres passés à Miyukini Agenda) peu détaillés. |
| **Opérateurs et Toolkits** | **Absent** : aucun document équivalent à « Exposants - Operateurs et Toolkits » (MFS) ou « Professionnels - Operateurs et Toolkits » (JayRDV). | Besoin d’un document **Miyukini Agenda - Operateurs et Toolkits** : quels Opérateurs exposent le service, quels Kits agrègent les outils, Contrat d’équipe. |
| **Niveaux Sécurité, Bornage, Referentiel Google** | Présents et structurés. | Pas de manque majeur pour un service complet. |

---

## 2. Manques détaillés et actions

### 2.1 Opérateurs, Toolkits et Équipe d’Opérateurs

| Manque | Description | Action |
|--------|-------------|--------|
| **Opérateurs Miyukini Agenda** | Les documents citent « Opérateurs et Kits » (ex. `agenda.entries.list`, `agenda.conflict.check`) mais aucun document ne **liste et décrit** les Opérateurs du service (ex. « Miyukini Agenda Entrées », « Miyukini Agenda Conflits », « Miyukini Agenda Vue & Export ») ni leur rôle par rapport aux services consommateurs. | Créer **Miyukini Agenda - Operateurs et Toolkits.md** : Opérateurs exposés par Miyukini Agenda, Kits d’outils (Kit Entrées, Kit Conflits, Kit Vue, Kit Export, Kit Événements publics), Consommation par JayRDV/MFS, Contrat d’équipe si applicable. |
| **Toolkits (Kits d’outils)** | Les outils agrégés (`agenda.entries.list`, `agenda.conflict.check`, `agenda.export.ical`, etc.) ne sont pas regroupés en **Kits** nommés ni en lien avec les Opérateurs. | Document Operateurs et Toolkits : pour chaque Kit, rôle, outils agrégés (exemples), composants sous-jacents (MiyuClock, Miyubooking, KindMother), consommé par quels Opérateurs / services. |
| **Équipe d’Opérateurs** | Miyukini Agenda en tant que **service** peut être exposé par une **Équipe d’Opérateurs** (plusieurs Opérateurs collaborant sous Contrat d’équipe) ; non explicité. | Dans Operateurs et Toolkits : préciser si Miyukini Agenda est exposé par un seul Opérateur ou une Équipe d’Opérateurs ; Contrat d’équipe et Mandats. |

### 2.2 Intégration des autres services avec Miyukini Agenda

| Manque | Description | Action |
|--------|-------------|--------|
| **Séquence d’intégration** | L’ordre des appels (service consommateur → Miyukini Agenda → réponse) et les **points d’entrée** (à quel moment JayRDV/MFS appellent publication, conflit, vue, export) ne sont pas décrits étape par étape. | Enrichir **Integration Services Consommateurs** : section « Séquence d’intégration » (flux publication, flux interrogation conflit, flux vue/export) ; tableau qui appelle quoi et quand. |
| **Contrat d’intégration** | Les **paramètres** passés à Miyukini Agenda (utilisateur, période, **filtres**) et le format des réponses (liste d’entrées, conflit oui/non + liste) ne sont pas formalisés dans un contrat conceptuel. | Enrichir **Integration Services Consommateurs** : section « Contrat conceptuel » (paramètres d’entrée, filtres supportés, format de sortie). |
| **Filtres côté API** | Les filtres (source, type, période, statut, visibilité) sont mentionnés en UI (AGD-UI-04) mais pas **explicitement** comme paramètres d’interrogation vers Miyukini Agenda. | Enrichir **Integration** et **Operateurs et Toolkits** : liste des **filtres** supportés par Miyukini Agenda (période, source, type, statut, visibilité public/privé) et comment les services les passent. |

### 2.3 Filtres de l’agenda

| Manque | Description | Action |
|--------|-------------|--------|
| **Filtres détaillés** | Seul le filtre **source / type** (AGD-UI-04) est décrit. Manquent : **période** (date début/fin), **statut** (candidat, inscrit, confirmé), **visibilité** (public vs privé), **nature** (présence physique ou autre). | Enrichir **Écrans et UI** : section « Filtres de l’agenda » détaillée (liste des filtres, données fournies par Miyukini Agenda, comportement). Référencer dans Integration et Operateurs et Toolkits. |
| **Filtres et niveau de sécurité** | Quels filtres sont autorisés selon le Mandat et le niveau WorrySentinel (ex. filtre « public » uniquement pour certains contextes). | Documenter dans Niveaux Sécurité ou Operateurs et Toolkits : filtres et permissions. |

### 2.4 Vue des événements publics sur son agenda et sélection pour valider sur son propre agenda

| Manque | Description | Action |
|--------|-------------|--------|
| **Vue des événements publics** | L’utilisateur doit pouvoir **voir** un catalogue ou une liste d’**événements publics** (ex. festivals ouverts aux inscriptions, ateliers ouverts) **sur** ou **à côté de** son agenda, pour décider lesquels ajouter. Non documenté. | Ajouter composant **AGD-UI-09** (Vue catalogue / liste événements publics) : objectif, données (événements publics éligibles, plage, type, source), intégration (page « Découvrir », « Ajouter à mon agenda », ou zone dans « Mon agenda »). Les **données** événements publics peuvent être fournies par le **service consommateur** (MFS, JayRDV) ; Miyukini Agenda peut exposer une **liste d’entrées publiques** (sans détail utilisateur) ou le consommateur agrège catalogue métier + appels Miyukini Agenda pour conflit/vue. |
| **Sélection et validation sur son propre agenda** | L’utilisateur **sélectionne** un ou plusieurs événements publics et **valide** pour les **ajouter à son agenda** (inscription, réservation). Le flux : sélection → vérification conflit (Miyukini Agenda) → confirmation → enregistrement métier (service consommateur) → publication entrée (Miyukini Agenda). Non documenté. | Ajouter composant **AGD-UI-10** (Sélection / Ajouter à mon agenda) : actions « Sélectionner », « Ajouter à mon agenda » ; vérification conflit avant validation ; enregistrement côté service consommateur puis publication vers Miyukini Agenda. Ajouter parcours **« Vue événements publics et ajout à mon agenda »** dans Parcours Utilisateurs. |

---

## 3. Synthèse des actions réalisées (post-audit)

| Action | Document créé ou enrichi |
|--------|---------------------------|
| Création **Miyukini Agenda - Operateurs et Toolkits.md** | Nouveau document : Opérateurs, Kits, Équipe, Consommation par JayRDV/MFS. |
| Enrichissement **Integration Services Consommateurs** | Section Séquence d’intégration ; Contrat conceptuel (paramètres, filtres) ; Filtres supportés. |
| Enrichissement **Écrans et UI** | Section Filtres de l’agenda (détaillée) ; AGD-UI-09 (Vue événements publics) ; AGD-UI-10 (Sélection / Ajouter à mon agenda). |
| Enrichissement **Parcours Utilisateurs** | Parcours « Vue événements publics → Sélection → Validation sur mon agenda ». |
| Référence depuis **Document Fondateur** | Lien vers Operateurs et Toolkits ; mention parcours événements publics. |

---

## 4. Références

| Document | Rôle |
|----------|------|
| [Miyukini Agenda - Document Fondateur](./Miyukini%20Agenda%20-%20Document%20Fondateur.md) | Contexte, besoins, positionnement. |
| [Miyukini Agenda - Operateurs et Toolkits](./Miyukini%20Agenda%20-%20Operateurs%20et%20Toolkits.md) | Opérateurs, Kits, Équipe, Consommation (créé suite à l’audit). |
| [Miyukini Agenda - Integration Services Consommateurs](./reference/Miyukini%20Agenda%20-%20Integration%20Services%20Consommateurs.md) | Intégration ; enrichi avec séquence, contrat, filtres. |
| [Miyukini Agenda - Ecrans et UI](./Miyukini%20Agenda%20-%20Ecrans%20et%20UI.md) | Composants UI ; enrichi avec filtres détaillés, événements publics, sélection. |
| [Miyukini Agenda - Parcours Utilisateurs](./Miyukini%20Agenda%20-%20Parcours%20Utilisateurs.md) | Parcours ; enrichi avec événements publics et ajout à mon agenda. |

---

**Document** : Miyukini Agenda — Audit documentation et manques  
**Version** : 1.0  
**Date** : 2026-01-31  
**Statut** : Document d’audit — référence pour compléter le service
