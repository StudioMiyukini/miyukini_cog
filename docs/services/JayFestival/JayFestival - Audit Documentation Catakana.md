# JayFestival — Audit documentation vs projet Catakana

## Contexte

Ce document constitue l’**audit de la documentation JayFestival** par rapport au projet **Catakana** (.Catakana) : qualité de la transcription/traduction des fonctionnalités Catakana vers JayFestival, couverture des toolkits, opérateurs, services imbriqués, et niveau de détail conceptuel, détaillé et de guidage/bornage pour l’implémentation.

**Références** : [Liste des fonctionnalités Catakana](../../../.Catakana/.cursor/rules/Liste%20des%20fonctionnalités.md), [APPLICATION_COMPLETE_DOCUMENTATION Catakana](../../../.Catakana/docs/APPLICATION_COMPLETE_DOCUMENTATION.md), documentation JayFestival (Document fondateur, publics Organisateurs / Exposants / Visiteurs / Utilisateur non connecté, référence Interpolarité).

## Portée / Scope

- **Périmètre** : Comparaison fonctionnalités Catakana ↔ documentation JayFestival ; toolkits, opérateurs, services imbriqués ; qualité conceptuelle, détaillée et bornage implémentation.
- **Hors périmètre** : Code source Catakana, implémentation effective des crates Miyukini.

---

## 1. Synthèse des métriques (%)

| Critère | Score | Commentaire |
|--------|-------|-------------|
| **Transcription/traduction des fonctionnalités Catakana → JayFestival** | **88 %** | Couverture très bonne ; manques : journal des modifications (programme), commentaires internes/notes privées exposants, import Google Sheet explicite, modules complémentaires (matériel, reporting, urgences). |
| **Documentation des toolkits nécessaires** | **92 %** | Toolkits identifiés et décrits par public (Organisateurs 9, Exposants 6, Visiteurs 5+) ; outils agrégés en exemples ; pas de contrat formel par outil. |
| **Documentation des opérateurs** | **90 %** | Opérateurs clairement nommés (JayFestival Organisateur, JayFestival Édition, MFS Exposant, JayFestival Candidatures, JayFestival Visiteur) ; lien opérateur ↔ livrables présent ; MFS non défini dans le glossaire. |
| **Services imbriqués et interpolarité** | **95 %** | Référence Interpolarité complète (JayXpose, JayFaim, JayKoa, JayKonta) ; Document fondateur et besoins citent Miyauth, Miyuprofile, Miyunotify, Miyuinvoice, MiyuClock, Miyubooking, Miyucms, Miyumedia, Miyucptaledger, Miyuexpense, Miyucomptareports. |
| **Documentation conceptuelle (niveau fondateur)** | **95 %** | Vision, B2B2C, Store, comptes cross-événements, héritage Catakana, macro, distribution, politique résidence données ; alignement Glossaire Miyukini. |
| **Documentation dans le détail (besoins, écrans, parcours)** | **93 %** | Besoins numérotés (ORG/EXP/VIS/UNC), critères d’acceptation, user stories, NFR, cas limites, MoSCoW ; écrans et cycle par public ; quelques besoins Catakana non détaillés (voir § 2). |
| **Guidage / bornage de l’implémentation** | **55 %** | Pas de document « Bornage Implementation » dédié (contrairement à JayKoa) ; besoins et critères d’acceptation servent de bornes ; pas de phases MVP/phase 2, pas de hors scope explicite par phase, pas de critères de livraison formalisés. |

**Score global pondéré (qualité doc vs Catakana + implémentation)** : **≈ 85 %**.

---

## 2. Transcription/traduction des fonctionnalités Catakana → JayFestival

### 2.1 Fonctionnalités de la « Liste des fonctionnalités » Catakana

| Bloc Catakana | Élément | JayFestival | Statut |
|---------------|---------|-------------|--------|
| **Authentification Orga** | Comptes organisateurs, rôles (bénévole, exposant, manager, admin) | Compte organisateur, rôles Admin / Manager / Bénévole ; Exposant = compte distinct | ✅ Transcrit (terminologie COG) |
| | Connexion email/mdp ou lien magique | ORG-01, Miyauth, lien magique cité | ✅ |
| | Protection des routes (middleware) | Mandat, Master Butler, NFR-ORG-07 | ✅ |
| | Interface gestion des rôles (admin) | ORG-46, ORG-47, ORG-48, Équipe & Permissions | ✅ |
| **Exposants** | Liste filtrable | ORG-12, EXP-05 à EXP-08 | ✅ |
| | Fiche exposant détaillée (contact, statut, historique) | ORG-15, EXP-17, fiche par édition | ✅ |
| | Ajout manuel ou import Google Sheet / CSV | ORG-18 import CSV/tableur | ⚠️ Google Sheet non explicite |
| | Changement de statut (en attente, validé, refusé) | ORG-13, ORG-14, EXP-12 | ✅ |
| | Commentaires internes ou notes privées | — | ❌ Non documenté |
| | Téléversement de documents par exposant | EXP-21 (documents signés/complétés) | ✅ |
| **Devis & Factures** | Génération devis, conversion en facture, PDF, historique, envoi email, marquage payé/en attente | ORG-16, ORG-17, EXP-22 à EXP-24, Miyuinvoice / JayKonta | ✅ |
| **Plan & Emplacement** | Plan interactif, attribution (drag & drop ou formulaire), zones, tailles, légende, export, zones techniques | ORG-19 à ORG-22, Kit Plan de salle | ✅ |
| **Programme** | Animations, association scène/salle/lieu, horaires, chevauchement bloqué | ORG-23 à ORG-26, Kit Programme | ✅ |
| | Vue chronologique ou par salle, filtres | ORG-25 | ✅ |
| | Édition rapide du programme en live | — | ⚠️ Non explicite |
| | Journal des modifications | — | ❌ Non documenté |
| **Documents & Légal** | Contrats types, envoi à signer, historique, accès par rôle | ORG-32 à ORG-34, Kit Documents & Légal | ✅ |
| **Notifications** | Annonces globales, notifications ciblées, journal messages, rédaction/planification alerte | ORG-35 à ORG-37, Miyunotify | ✅ |
| **Modules complémentaires** | Gestion matériel prêté/placé | — | ❌ Non documenté |
| | Gestion bénévoles (planning, équipes) | ORG-47, ORG-48 (zones, créneaux) | ⚠️ Partiel |
| | Outils de reporting (participation, paiements, retards) | ORG-30 balance, exports ; pas de reporting dédié | ⚠️ Partiel |
| | Suivi interventions techniques ou urgences | — | ❌ Non documenté |

### 2.2 Éléments Catakana (APPLICATION_COMPLETE) non transcrits ou hors scope

| Élément Catakana | Présence JayFestival | Remarque |
|------------------|----------------------|----------|
| Module Actualités (News) | Annonces (ORG-35, ORG-36) ; pas de flux « Actualités » éditorial public | Peut relever de Miyucms / Miyufeeds ; non formalisé pour JayFestival |
| Invités (Guests), prestations, réservations créneaux | Non documenté comme bloc dédié | Peut être couvert par « Services visiteur » (ateliers, créneaux) ; à préciser |
| RPG (stats, inventaire) | Non documenté | Hors scope ou à traiter dans un autre service |
| Galeries par édition | Non documenté | Peut relever de Miyumedia ; non formalisé |
| Réservation de stands avec tarifs types (stand intérieur/extérieur/restauration, options) | Devis/facture (Miyuinvoice) ; plan de salle avec attribution | Tarifs types et options (chaises, tables, etc.) dans le détail des devis, pas en tant que catalogue de stands tarifés |
| Liste des emplacements (statut libre/réservé/occupé) | Plan de salle, stands, attribution | Couvert conceptuellement ; détail « statut temps réel » à confirmer |

---

## 3. Toolkits, opérateurs, services imbriqués

### 3.1 Toolkits documentés

- **Organisateurs** : Éditions, Exposants (côté organisateur), Plan de salle, Programme, Budget, Documents & Légal, Services visiteur, Publication catalogue, Équipe & Permissions — **9 toolkits**, avec exemples d’outils agrégés.
- **Exposants** : Candidatures Exposant, Participations & Éditions, Agenda cross-événements, Documents Exposant, Facturation Exposant (Miyuinvoice), Répertoire Exposants — **6 toolkits**.
- **Visiteurs** : Agenda Visiteur, Billets & Réservations, Pass VIP, Suivi d’activités, etc. — **5+ toolkits** (décrits dans Visiteurs - Operateurs et Toolkits).

**Score** : Couverture très bonne ; manque une liste consolidée « Tous les toolkits JayFestival » et des contrats d’outils formels (nom, paramètres, garanties).

### 3.2 Opérateurs documentés

- **Organisateurs** : JayFestival Organisateur, JayFestival Édition.
- **Exposants** : MFS Exposant, JayFestival Candidatures (« MFS » non défini dans le glossaire).
- **Visiteurs** : JayFestival Visiteur (ou équivalent).
- **Catalogue / UNC** : Façade publique gouvernée, pas d’opérateur nommé spécifiquement.

### 3.3 Services imbriqués et interpolarité

- **Document [JayFestival - Interpolarite Services Jay](reference/JayFestival%20-%20Interpolarite%20Services%20Jay.md)** : JayXpose (fiche/répertoire exposants), JayFaim (restauration événement), JayKoa (agenda, conflits de dates), JayKonta (budget, devis/factures).
- **Document fondateur et besoins** : Miyauth, Miyuprofile, Miyunotify, Miyuinvoice, MiyuClock, Miyubooking, Miyucms, Miyumedia, Miyucptaledger, Miyuexpense, Miyucomptareports, StrongFather, Master Butler, KindMother, WorrySentinel.

**Score** : Très bon ; référence centralisée et cohérente.

---

## 4. Qualité conceptuelle, détaillée et bornage implémentation

### 4.1 Niveau conceptuel

- Document fondateur : raison d’être, vision, B2B2C, Store, comptes cross-événements, héritage Catakana, macro, distribution (organisateurs, exposants, visiteurs), politique de résidence des données sensibles.
- Par public : Parcours, capacités et livrables ; Analyse des besoins ; Écrans et cycle ; Opérateurs et Toolkits.
- Terminologie alignée sur le Glossaire Miyukini (Opérateur, Mandat, Kit d’outils, etc.).

**Score** : 95 %.

### 4.2 Niveau détaillé

- Besoins numérotés avec critères d’acceptation (ORG-xx, EXP-xx, VIS-xx, UNC-xx).
- User stories, pain points, opportunités, MoSCoW, NFR, cas limites, métriques de succès.
- Écrans listés avec phase, objectif, organisation, besoins, navigation.

**Score** : 93 %.

### 4.3 Guidage / bornage implémentation

- **Présent** : Besoins et critères d’acceptation utilisables comme bornes ; priorités MoSCoW ; dépendances entre publics et avec les services Jay.
- **Absent** : Document dédié « Bornage Implementation » (in scope / hors scope par phase, critères de livraison, dépendances techniques formalisées) comme pour JayKoa ; pas de phases MVP / phase 2 explicites ; pas de liste « hors scope v1 » consolidée.

**Score** : 55 %.

---

## 5. Recommandations

1. ~~**Créer un document « JayFestival - Bornage Implementation »**~~ **Fait** : [JayFestival - Bornage Implementation](./JayFestival%20-%20Bornage%20Implementation.md) ; périmètre MVP / phase 2, hors scope, dépendances, critères de livraison.
2. **Compléter les manques de transcription** : commentaires internes / notes privées sur exposants (besoin ORG ou EXP) ; journal des modifications du programme (optionnel) ; mention explicite de l’import depuis Google Sheet (ou décision de ne pas supporter).
3. **Préciser le sigle « MFS »** (ex. Miyukini Festival Service ou équivalent) dans le glossaire ou dans le document fondateur JayFestival.
4. **Documenter ou décider** : module Actualités (News) public (Miyufeeds / Miyucms vs Annonces) ; gestion matériel, reporting avancé, interventions techniques/urgences (hors scope v1 ou backlog).
5. **Ajouter une référence « Niveaux Sécurité et Protection Données »** pour JayFestival (sur le modèle JayKonta / JayKoa) si nécessaire pour les audits et la conformité.
6. **Référence UI et implémentation** : [JayFestival - Reference UI Transcription Catakana](./JayFestival%20-%20Reference%20UI%20Transcription%20Catakana.md) retranscrit l’UI complète Catakana (Atomic, thème, ui-kit, écrans) dans la stack actuelle (egui/eframe) pour préparer l’implémentation.

---

## 6. Références

- [JayFestival - Document Fondateur](./JayFestival%20-%20Document%20Fondateur.md)
- [JayFestival - Bornage Implementation](./JayFestival%20-%20Bornage%20Implementation.md)
- [JayFestival - Reference UI Transcription Catakana](./JayFestival%20-%20Reference%20UI%20Transcription%20Catakana.md)
- [JayFestival - Interpolarite Services Jay](./reference/JayFestival%20-%20Interpolarite%20Services%20Jay.md)
- [JayKoa - Bornage Implementation](../JayKoa/JayKoa%20-%20Bornage%20Implementation.md)
- [Liste des fonctionnalités Catakana](../../../.Catakana/.cursor/rules/Liste%20des%20fonctionnalités.md)
- Documentation publics : [Organisateurs](./publics/Organisateurs/_index.md), [Exposants](./publics/Exposants/_index.md), [Visiteurs](./publics/Visiteurs/_index.md), [Utilisateur non connecté](./publics/UtilisateurNonConnecte/_index.md)

---

**Document** : JayFestival — Audit documentation vs projet Catakana  
**Version** : 1.0  
**Date** : 2026-02-03  
**Statut** : Rapport d’audit — métriques et recommandations.
