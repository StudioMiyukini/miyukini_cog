# Miyukini Core System — Phase 1 : Capacités CMS Cœur

## 1. Objectif de la Phase 1

La Phase 1 vise à transformer les fondations validées en Phase 0
en un **socle CMS réellement exploitable par des produits**, sans
introduire de logique métier, d'UI ou de dépendances techniques lourdes.

Elle introduit des **capacités CMS cœur**, génériques, réutilisables
et indépendantes du contexte produit.

---

## 2. Positionnement architectural

Rappel des couches :

- Kernel : infra pure (config, id, time, log, lifecycle)
- SPM CMS Phase 0 : fondations fonctionnelles (Contenu, Hiérarchie, Taxonomies)
- **SPM CMS Phase 1 : capacités cœur (cette phase)**
- Produit : métier, règles business, UI, intégrations

👉 La dépendance reste strictement unidirectionnelle.

---

## 3. Modules candidats Phase 1

### 3.1 Module Médias (PRIORITÉ 1)

**Responsabilité :**
Gestion générique des assets (images, vidéos, fichiers) et de leurs métadonnées.

**Pourquoi Phase 1 :**
- Mutualisable par ≥5 types de produits (CMS, Event, SaaS, Jeux, E-shop)
- Peu de logique métier
- Très forte valeur produit

**Capacités attendues (haut niveau) :**
- Création d'un média
- Lecture d'un média
- Suppression
- Métadonnées (format opaque)
- Lien média ↔ entité externe (via Id)

**Hors-scope explicite :**
- Upload HTTP
- Transformation d'image
- CDN
- Rendu
- Permissions

---

### 3.2 Module Publication (PRIORITÉ 2)

**Responsabilité :**
Gestion du cycle éditorial générique des entités de contenu.

**Capacités attendues :**
- États éditoriaux (draft, published, archived, scheduled)
- Dates de publication
- Publication différée (scheduling)

**Risque identifié :**
- Forte tentation de logique métier (workflows, validations)

👉 À cadrer très strictement.

---

### 3.3 Module Blocs (PRIORITÉ 3)

**Responsabilité :**
Composition structurelle de contenu par blocs.

**Capacités attendues :**
- Blocs typés
- Ordonnancement
- Données de bloc opaques

**Risque majeur :**
- Dérive vers le rendu / page builder

👉 À n'attaquer qu'après validation Media + Publication.

---

## 4. Ordre officiel de développement

1. **Module Médias**
2. **Module Publication**
3. **Module Blocs**

⚠️ Règle ferme : **un seul module Phase 1 à la fois**.

---

## 5. Règles Phase 1 (NON NÉGOCIABLES)

1. Contrat fonctionnel écrit AVANT le code
2. Implémentation mémoire obligatoire
3. Tests unitaires obligatoires (≥10)
4. Aucune UI, aucun rendu
5. Aucune auth, permission, SEO
6. Aucun accès réseau
7. Aucun ajout au kernel
8. Si une règle métier apparaît → STOP

---

## 6. Critères de validation Phase 1

Un module Phase 1 est validé lorsque :

- Le contrat est stable et documenté
- Les tests unitaires passent
- Une démo console existe
- Il est consommable par `mini-cms`
- Aucun invariant Phase 0 n'est violé

---

## 7. Risques principaux et garde-fous

| Risque | Garde-fou |
|------|----------|
| Dérive métier | Hors-scope explicite |
| Couplage UI | Interdiction de rendu |
| Anticipation | Besoin réel ou rejet |
| Bloat | ≥2 produits ou refus |

---

## 8. Statut

Phase 1 : **PLANIFIÉE**

Prochaine action : démarrage du **Module Médias**.
