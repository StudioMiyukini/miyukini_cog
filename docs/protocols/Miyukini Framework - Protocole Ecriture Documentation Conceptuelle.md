# Protocole d'écriture de la documentation conceptuelle et contractuelle d'un nouveau système
**Version : v1.0**  
**Statut : Normatif – Processus contrôlé**  

---

## 1. Contexte

Ce protocole complète le [Protocole d'implémentation générale](./Miyukini%20Framework%20-%20Protocole%20d'implémentation%20générale.md) en définissant les règles spécifiques à la **rédaction de documentation** (et non de code).

### 1.1 Portée / Scope

Ce document définit un **protocole strict d'utilisation d'agents IA** pour l'écriture de la documentation d'un système logiciel — **cette documentation fera référence**.

Le processus :
- est **suivi et automatisé**,
- repose sur un **cycle fermé**, déterminé et explicite,
- impose des **contrôles stricts à chaque étape**,
- autorise le **travail en parallèle**, sous contraintes,
- intègre la **gestion des limites de contexte et de coûts LLM**,
- interdit toute dérive implicite ou interprétation libre.

### 1.2 Différence avec le protocole d'implémentation

| Aspect | Implémentation | Documentation |
|--------|----------------|---------------|
| Unité de travail | 1 agent = 1 fichier code | 1 agent = 1 document |
| En cas de blocage | Rend la main à l'humain | Informe l'agent planificateur |
| Agents simultanés | Défini à l'avance | Maximum 4 |
| Tests | Tests unitaires console | Recherche d'incohérences/ambiguïtés |

---

## 2. Cycle global d'écriture (obligatoire)

Le cycle est **ordonné, fermé et non contournable** :

1. **Planification**
2. **Distribution des tâches aux agents**
3. **Vérification, corrections et tests**
4. **Gel et versionnement**

👉 Aucune étape ne peut être sautée ou fusionnée.

---

## 3. Phase 1 — Planification

### 3.1 Titre de l'étape en cours

Renseigner obligatoirement le titre de l'étape de planification active.

---

### 3.2 Explication rapide

Description synthétique de :
- le périmètre couvert,
- les limites explicites,
- le maintien de la liste des tâches par l'agent planificateur,
- le lancement des nouvelles tâches à de nouveaux agents Auto Cursor si les conditions sont réunies.

---

### 3.3 Sélection du modèle IA (obligatoire)

En **entête de tout prompt** destiné à Cursor :

```
COMPLEXITÉ : Simple | Complexe | Extreme
CHARGE CONTEXTUELLE : Faible | Moyenne | Élevée

MODÈLE AUTORISÉ :
- Simple  → Cursor Auto
- Complexe → 1 modèle premium explicitement nommé
- Extreme → LLM étendu (jusqu'à 1M tokens)

MODE IA ACTIF : AI Mode 1 | AI Mode 2 | AI Mode 3
```

**Contraintes :**
- Le mode IA actif doit être cohérent avec la complexité déclarée
- Aucun changement de modèle en cours de tâche sans justification explicite
- En Mode 2 ou 3, privilégier systématiquement Cursor Auto

---

### 3.4 Prompt engineering — Mode PLAN

Le prompt de planification DOIT inclure :

#### a) Définition de l'agent
- Rôle précis
- Poste
- Compétences requises
- Responsabilités
- Ce que l'agent ne doit jamais faire

#### b) Cadre de travail
- Documentation autorisée (liste fermée)
- Outils autorisés
- Outils interdits

#### c) Construction de la structure
- 1er document = document "fondateur" si pas déjà présent
- Création des documents nommés mais vides
- Respecter la taxonomie et les nomenclatures du projet

Les dépendances doivent être :
- explicitées,
- annotées,
- accompagnées d'un ordre strict si nécessaire

#### d) Contraintes absolues
- ❌ Ne pas anticiper les étapes suivantes
- ❌ Ne pas fusionner plusieurs documents
- ❌ Ne pas corriger hors périmètre

#### e) Tests
- Tests unitaires pour recherche d'incohérence ou d'ambiguïté si nécessaires
- Sinon : justification explicite de leur absence

#### f) Mini log de planification
- Ambiguïtés détectées
- Dépendances critiques
- Décisions structurantes

---

## 4. Phase 2 — Distribution des tâches

Chaque tâche issue du plan est **déléguée par l'agent de planification** à un nouvel agent si les conditions sont réunies.

Contexte vierge obligatoire.
**Pas de tâche mutualisation**
**Pas de batch/vague/groupe de tâches**
**1 agent = 1 document**

Chaque tâche du plan ou to-do a une nomenclature défini : [xx] - [du document à produire]
Le [xx] est préfixe de regroupement d'écriture parallèle. Je peux lancer toutes les tâches "01" en même temps avec des agents différents.

Une limite maximale d'agents simultanés est définie à **4**. Donc aucun groupement de préfixe ne doit avoir plus de 4 itérations.

### 4.1 Règle d'arrêt stricte

Un agent DOIT S'ARRÊTER IMMÉDIATEMENT si :

- une ambiguïté bloquante est détectée,
- une dépendance manquante est rencontrée,
- la fenêtre de contexte devient insuffisante,
- la tâche et le test unitaire (si présent) sont terminés et corrects.

👉 Dans ces cas :

- aucun fichier partiel n'est généré,
- l'agent informe l'agent planificateur qui continuera son orchestration.

---

## 5. Phase 3 — Vérification, corrections et tests

### 5.1 Vérification globale

Le système documentaire est analysé pour :
- incohérences inter-documents,
- non-conformités à la documentation de référence,
- violations de règles ou d'invariants,
- comportements implicites ou non documentés.

### 5.2 Corrections

Toute correction est traitée comme une nouvelle tâche.  
Les règles de la Phase 2 s'appliquent intégralement.  
Aucune correction "rapide" hors protocole n'est autorisée.

### 5.3 Tests

- Exécution des tests de cohérence et de complétude
- Validation de la structure documentaire complète
- Aucune validation partielle n'est acceptée

### 5.4 Audit du système

Rédaction d'un audit formel incluant :
- erreurs rencontrées,
- corrections appliquées,
- risques évités,
- points de vigilance futurs.

---

## 6. Phase 4 — Gel et versionnement

### 6.1 Gel

- Rédaction d'un document de gel officiel
- Liste exhaustive des éléments gelés
- Interdiction de toute modification implicite

### 6.2 Versionnement

- Attribution d'une version explicite (ex : v1.2.0)
- Distinction versions majeures / mineures
- Règles d'évolution futures
- Conditions de dégel et de migration

👉 Après gel :

- toute modification impose un nouveau cycle complet

---

## 7. Annexes

### 7.1 Modes IA

**AI Mode 1 : Libre**
- Abonnement non limité

**AI Mode 2 : Dégradé**
- Budget On Demand limité (50$)
- → Priorité au mode Auto

**AI Mode 3 : Fermé**
- Cursor Auto uniquement

### 7.2 Modèles premium accessibles

- Composer 1
- Opus 4.5
- Sonnet 4.5
- GPT 5.2 Codex (Low / High / Extra High)
- GPT 5.2
- Gemini 3 Flash
- GPT 5.1 mini
- Kimi K2

### 7.3 Règle de consommation IA

À chaque [pause], l'agent doit :

- soit continuer explicitement,
- soit passer la main à un nouvel agent.

Un agent dont la fenêtre de contexte est saturée :

- s'arrête,
- ne génère aucun document, même partiel.

---

## 8. Conclusion

Ce protocole garantit :

- une documentation conforme aux standards du projet,
- une discipline stricte des agents IA,
- une traçabilité complète,
- une maîtrise des coûts et du contexte,
- une base stable pour audit, gel et certification.

Toute rédaction de documentation hors de ce protocole est considérée comme non conforme.
