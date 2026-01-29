# Protocole d'implémentation générale jusqu'au niveau système
**Version : v2.0**  
**Statut : Normatif – Processus contrôlé**  

---

## 1. Scope

Ce document définit un **protocole strict d'utilisation d'agents IA** pour l'implémentation d'un système logiciel **à partir d'une documentation de référence**.

Le processus :
- est **suivi et validé par un humain**,
- repose sur un **cycle fermé**, déterminé et explicite,
- impose des **contrôles stricts à chaque étape**,
- autorise le **travail en parallèle**, sous contraintes,
- intègre la **gestion des limites de contexte et de coûts LLM**,
- interdit toute dérive implicite ou interprétation libre.

Certaines étapes nécessitent explicitement une **intervention humaine**.

### 1.1 Dépendances normatives

Ce protocole s'appuie sur les standards suivants :

| Standard | Rôle | Référence |
|----------|------|-----------|
| **MSCM v1** | Balisage sémantique du code | Miyukini Semantic Code Markup |
| **MIP v1** | Indexation structurelle globale | [Miyukini Prompt Protocol - MIP v1 MSCM Index Protocol](Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md) |

👉 Tout code produit DOIT être conforme MSCM.  
👉 Tout projet DOIT maintenir un index MIP valide.

---

## 2. Cycle global d'implémentation (obligatoire)

Le cycle est **ordonné, fermé et non contournable** :

1. **Planification**
2. **Distribution des tâches aux agents**
3. **Vérification, corrections et tests**
4. **Gel et versionnement**

👉 Aucune étape ne peut être sautée ou fusionnée.

---

## 3. Phase 1 — Planification

### 3.1 Titre de l'étape en cours

---

### 3.2 Explication rapide

Description synthétique de :
- l'objectif de l'implémentation,
- le périmètre couvert,
- les limites explicites.

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

Contraintes :

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

#### c) Construction du plan d'implémentation
- 1 étape = 1 fichier
- 1 agent = 1 étape

Chaque étape doit être indépendamment implémentable

Les dépendances doivent être :
- explicitées,
- annotées,
- accompagnées d'un ordre strict si nécessaire

#### d) Contraintes absolues
- ❌ Ne pas anticiper les étapes suivantes
- ❌ Ne pas fusionner plusieurs fichiers
- ❌ Ne pas corriger hors périmètre

#### e) Tests
- Tests unitaires console si possible
- Sinon : justification explicite de leur absence

#### f) Mini log de planification
- Ambiguïtés détectées
- Dépendances critiques
- Décisions structurantes

#### g) Définition du balisage MSCM attendu

Le plan DOIT inclure pour chaque fichier à produire :

- Les **blocs MSCM obligatoires** à créer
- Les **rôles sémantiques** (`@role`) attendus
- Les **couches architecturales** (`@layer`) concernées
- Les **dépendances inter-blocs** à déclarer

👉 Cette définition préalable garantit la cohérence du balisage et facilite la régénération de l'index MIP.

---

## 4. Phase 2 — Distribution des tâches

Chaque tâche issue du plan est **déléguée par l'agent de planification** à un nouvel agent si les conditions sont réunies.

Contexte vierge obligatoire.
**Pas de tâche mutualisation**
**Pas de batch/vague/groupe de tâches**
**1 agent = 1 fichier**

Chaque tâche du plan ou to-do a une nomenclature défini : [xx] - [nom du fichier à produire]
Le [xx] est préfixe de regroupement d'écriture parallèle. Je peux lancer toutes les tâches "01" en même temps avec des agents différents.

Une limite maximale d'agents simultanés est définie à **4**. Donc aucun groupement de préfixe ne doit avoir plus de 4 itérations.

### 4.1 Obligation de balisage MSCM

Chaque fichier produit DOIT respecter le protocole MSCM v1 :

**Obligations minimales :**
- Chaque bloc fonctionnel DOIT avoir un identifiant unique (`@id`)
- Chaque bloc DOIT avoir une description fonctionnelle (`@do`)

**Méta-données optionnelles :**
- Le rôle sémantique peut être explicite (`@role`) — optionnel
- La couche architecturale peut être déclarée (`@layer`) — optionnel
- Une description humaine peut accompagner le bloc (`@human` ou `@humain`) — optionnel

**Vérifications avant livraison :**
- [ ] Les identifiants sont uniques globalement
- [ ] Les rôles sont cohérents avec la documentation de référence
- [ ] Les couches respectent l'architecture définie
- [ ] Les dépendances inter-blocs sont déclarées

👉 Un fichier sans balisage MSCM conforme est considéré comme **non livrable**.

---

### 4.2 Règle d'arrêt stricte

Un agent DOIT S'ARRÊTER IMMÉDIATEMENT si :

- une ambiguïté bloquante est détectée,
- une dépendance manquante est rencontrée,
- la fenêtre de contexte devient insuffisante.
- Si le fichier et le test unitaire si présent sont terminé et correct

👉 Dans ces cas :

- aucun fichier partiel n'est généré,
- l'agent rend la main à l'humain.

---

## 5. Phase 3 — Vérification, corrections et tests

### 5.1 Vérification globale

Le système est analysé pour :
- incohérences inter-fichiers,
- non-conformités à la documentation,
- violations de règles ou d'invariants,
- comportements implicites ou non documentés.

### 5.2 Corrections

Toute correction est traitée comme une nouvelle tâche  
Les règles de la Phase 2 s'appliquent intégralement  
Aucune correction "rapide" hors protocole n'est autorisée

### 5.3 Tests

- Exécution des tests unitaires et globaux
- Validation fonctionnelle complète
- Aucune validation partielle n'est acceptée

### 5.4 Vérification de conformité MSCM

Avant passage en Phase 4, vérification obligatoire :

**Contrôles MSCM :**
- [ ] Tous les blocs critiques sont balisés MSCM
- [ ] Les identifiants sont uniques globalement
- [ ] Les couches (`@layer`) sont cohérentes avec l'architecture
- [ ] Aucun bloc orphelin (sans `@id` ou `@role`)
- [ ] Les dépendances inter-blocs sont déclarées

**Régénération de l'index MIP :**
- L'index MIP DOIT être régénéré après chaque cycle de corrections
- La régénération DOIT réussir sans erreur
- Le graphe de dépendances DOIT être cohérent
- La hiérarchie DOIT être valide

👉 Toute erreur MIP bloque le passage en Phase 4.

---

### 5.5 Audit du système

Rédaction d'un audit formel incluant :
- erreurs rencontrées,
- corrections appliquées,
- risques évités,
- points de vigilance futurs,
- **rapport de conformité MSCM/MIP**.

---

## 6. Phase 4 — Gel et versionnement

### 6.1 Gel

- Rédaction d'un document de gel officiel
- Liste exhaustive des éléments gelés
- Interdiction de toute modification implicite

### 6.2 Génération de l'index MIP final

**Livrable obligatoire :** L'index MIP final DOIT être généré et inclus dans le gel.

**Contenu de l'index MIP :**
```
mscm_index/
├── registry.json      # Métadonnées et intégrité
├── blocks.json        # Identité sémantique des blocs
├── hierarchy.json     # Structure hiérarchique
├── graph.json         # Relations transverses
├── flows.json         # Processus métier
├── domains.json       # Vision métier
├── layers.json        # Architecture technique
├── dependencies.json  # Dépendances logiques
├── files.json         # Cartographie code
└── stats.json         # Métriques
```

**Vérifications avant gel :**
- [ ] L'index MIP peut être régénéré sans erreur
- [ ] Aucun bloc orphelin détecté
- [ ] Aucun cycle invalide dans le graphe
- [ ] Intégrité validée (`registry.json → integrity: "ok"`)

👉 Un projet sans index MIP valide ne peut pas être gelé.

### 6.3 Versionnement

- Attribution d'une version explicite (ex : v1.2.0)
- Distinction versions majeures / mineures
- Règles d'évolution futures
- Conditions de dégel et de migration
- **Version de l'index MIP associée**

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
- ne génère aucun fichier, même partiel.

---

## 8. Conclusion

Ce protocole garantit :

- une implémentation conforme à la documentation,
- une discipline stricte des agents IA,
- une traçabilité complète,
- une maîtrise des coûts et du contexte,
- une base stable pour audit, gel et certification.

Toute implémentation hors de ce protocole est considérée comme non conforme.
