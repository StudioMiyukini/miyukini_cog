Miyukini — Règles d’Interaction UI/UX
Un écran = une action • Anti-scroll • Pro-mobile

> ⚠️ RÈGLES UX STRUCTURANTES — NON NÉGOCIABLES  
> Ces règles priment sur toute considération esthétique ou technique.  
> Toute IA ou développeur DOIT les appliquer par défaut.

---

## 1. Principe fondamental : Un écran = une action

### 1.1 Définition

Un écran Famitura DOIT :
- avoir **un objectif principal unique**
- proposer **une action principale unique**
- guider l’utilisateur vers **UNE décision claire**

❌ Un écran ne doit JAMAIS :
- demander plusieurs décisions importantes
- mélanger consultation et modification
- proposer plusieurs actions critiques concurrentes

---

### 1.2 Action principale

Chaque écran DOIT définir explicitement :
- son **action principale**
- matérialisée par :
  - un bouton primary
  - OU un geste principal (ex : validation)

⚠️ Il ne peut y avoir :
- qu’**UN bouton primary visible**
- ou **AUCUNE action primary** (écran lecture seule)

---

## 2. Séparation stricte des intentions

### 2.1 Lecture vs Action

Les intentions suivantes DOIVENT être séparées :

| Intention | Écran dédié |
|---------|------------|
| Lire une information | Écran de lecture |
| Modifier | Écran de modification |
| Ajouter | Écran d’ajout |
| Configurer | Écran de configuration |
| Valider / confirmer | Écran de confirmation |

❌ Interdit :
- éditer directement dans un écran de lecture
- afficher un formulaire long sous une liste

---

### 2.2 Exemple concret (Courses)

- Liste de courses → **lecture / progression**
- Modifier item → **écran dédié**
- Modifier produit → **écran dédié**
- Configurer rayons magasin → **écran dédié**

---

## 3. Anti-scroll : éviter le défilement si possible

### 3.1 Règle générale

Le scroll vertical DOIT être :
- **évité autant que possible**
- **accepté uniquement si justifié**

Objectif :
- compréhension immédiate
- actions visibles sans chercher
- usage terrain (courses, une main)

---

### 3.2 Cas où le scroll est AUTORISÉ

Le scroll est autorisé UNIQUEMENT pour :

- listes longues (items, tâches, produits)
- contenus dynamiques (historique)
- documents (médical)

⚠️ Même dans ces cas :
- l’action principale DOIT rester visible
- ou accessible sans effort

---

### 3.3 Cas où le scroll est INTERDIT

❌ Scroll interdit pour :
- formulaires d’ajout / édition
- écrans de configuration
- écrans de validation

➡️ Dans ces cas :
- découper en **plusieurs écrans**
- ou utiliser un **stepper explicite**

---

## 4. Formulaires : règle des écrans courts

### 4.1 Taille maximale

Un formulaire NE DOIT PAS :
- dépasser la hauteur d’un écran mobile standard
- nécessiter un scroll pour comprendre l’action attendue

---

### 4.2 Découpage obligatoire

Si un formulaire dépasse :
- découper en écrans successifs
- chaque écran = une sous-action claire

Exemples :
- Infos de base → écran 1
- Détails → écran 2
- Confirmation → écran 3

---

## 5. FAB & Action principale

### 5.1 Mobile

- Le FAB représente TOUJOURS l’action principale
- Un seul FAB visible
- Position basse, accessible au pouce

---

### 5.2 Tablet / Desktop

- FAB OU bouton primary dans le header
- JAMAIS les deux simultanément

---

## 6. Listes & Actions

### 6.1 Règle

Dans un écran de liste :
- la liste = le contenu principal
- les actions sont :
  - contextuelles (swipe, long press)
  - OU via FAB

❌ Pas de boutons multiples sous chaque item.

---

### 6.2 Détail d’un item

- Tap → détail
- Actions lourdes → écran dédié
- Actions rapides → geste simple (swipe / tap)

---

## 7. Application stricte au module Courses

### 7.1 Mode Parcours Magasin

- Un écran = une action :
  - avancer dans le parcours
- Pas de scroll inutile
- Rayons affichés séquentiellement
- Action visible en permanence

---

### 7.2 Fiche produit

- Lecture : écran lecture
- Modifier :
  - écran séparé
- Ajouter à liste :
  - action directe OU écran court

---

## 8. Anti-hallucination IA — Règles obligatoires

Toute IA DOIT :

1. Identifier l’action principale AVANT de générer un écran
2. Refuser un écran avec plusieurs actions primaires
3. Découper automatiquement un écran trop long
4. Éviter le scroll par défaut
5. Demander validation si un scroll est nécessaire

---

## 9. Règle finale (NON DISCUTABLE)

> Si un écran nécessite du scroll pour comprendre  
> **quelle action l’utilisateur doit faire**,  
> alors l’écran est **mal conçu**.

La clarté d’action prime sur la densité d’information.
