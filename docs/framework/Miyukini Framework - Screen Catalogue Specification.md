# 📘 Miyukini Framework - Screen Catalogue Specification

**(Cursor · IA Guardrail · Web → Native Safe)**

---

## 0. Objectif du document

Ce document définit **un catalogue officiel d’écrans types (Screens)** utilisables dans tous les SaaS construits avec le **Miyukini Framework** (booking, pro, account, admin, etc.).

Il sert à :

* standardiser les **patterns UX**
* générer des **bases d’écrans modulaires**
* empêcher la dette technique mobile
* guider les agents IA (Cursor) lors de la génération de code

👉 **Toute génération d’écran DOIT se baser sur ce catalogue.**

---

## 1. Définitions fondamentales (NON NÉGOCIABLE)

### 1.1 Screen

Un **Screen** est :

* un artefact UX
* indépendant du routing
* indépendant de la plateforme (Web / Mobile / Native)
* responsable uniquement de l’affichage et de l’intention utilisateur

Un Screen :

* ❌ ne connaît pas l’URL
* ❌ ne connaît pas la navigation
* ❌ ne connaît pas la source des données

---

### 1.2 Page (Web only)

Une **Page** :

* est spécifique au Web
* sert uniquement à **monter un Screen**
* est jetable

```ts
// Web-only
export default function Page() {
  return <SomeScreen />
}
```

---

## 2. Règles UX globales (rappel)

Tous les Screens DOIVENT respecter :

* 1 Screen = 1 intention utilisateur
* 1 action primaire maximum
* lecture ≠ édition
* pas de formulaire long
* pas de scroll sauf justification explicite
* FAB = action principale (mobile)

⚠️ Toute violation invalide le Screen.

---

## 3. Structure obligatoire d’un Screen

Chaque Screen DOIT :

1. Déclarer un **ScreenContract**
2. Utiliser le **layout triptyque**
3. Être composé uniquement du **UI Kit**
4. Contenir uniquement :

   * structure
   * placeholders
   * callbacks

---

### 3.1 ScreenContract (OBLIGATOIRE)

```ts
/**
 * ScreenContract
 *
 * screenName: string
 * module: ModuleName
 * intent: string
 *
 * primaryAction:
 *   - label: string
 *   - type: 'fab' | 'button' | 'none'
 *
 * layout:
 *   - header: boolean
 *   - body: 'list' | 'form' | 'content'
 *   - bottom: 'fab' | 'actionTray' | 'none'
 *
 * rules:
 *   - singlePrimaryAction: true
 *   - scrollAllowed: boolean
 *   - editable: boolean
 */
```

---

## 4. Catalogue officiel des écrans types

Cursor DOIT utiliser **EXCLUSIVEMENT** ces types pour générer des Screens.

---

### 4.1 LIST SCREEN

**(Consultation / navigation)**

#### Intention

> Consulter une collection d’éléments et naviguer vers un détail.

#### Caractéristiques

* body : list
* action primaire : créer (FAB) ou aucune
* scroll : autorisé (liste)
* aucune édition inline

#### Exemple

* Booking list
* Clients list
* Appointments list

---

### 4.2 READ-ONLY DETAIL SCREEN

**(Consultation d’un élément)**

#### Intention

> Lire les informations d’un élément unique.

#### Caractéristiques

* body : content
* action primaire : aucune
* actions secondaires : menu / navigation
* scroll : autorisé si contenu long
* ❌ aucune modification possible

---

### 4.3 CREATE FORM SCREEN

**(Création)**

#### Intention

> Créer un nouvel élément.

#### Caractéristiques

* body : form
* action primaire : Save (FAB ou bouton)
* scroll : interdit
* formulaire court
* écran dédié

---

### 4.4 EDIT FORM SCREEN

**(Modification)**

#### Intention

> Modifier un élément existant.

#### Caractéristiques

* body : form
* action primaire : Save
* scroll : interdit
* jamais combiné avec lecture

---

### 4.5 CONFIRMATION SCREEN

**(Validation / décision)**

#### Intention

> Confirmer ou annuler une action critique.

#### Caractéristiques

* body : content
* action primaire : confirmer
* action secondaire : annuler
* aucun scroll
* contenu minimal

---

### 4.6 STEP SCREEN

**(Processus multi-écrans)**

#### Intention

> Avancer dans un processus découpé.

#### Caractéristiques

* 1 sous-action par écran
* pas de formulaire long
* progression explicite
* navigation contrôlée

---

### 4.7 EMPTY STATE SCREEN

**(Point d’entrée)**

#### Intention

> Comprendre pourquoi il n’y a rien et quoi faire.

#### Caractéristiques

* body : content
* action primaire : créer
* aucun scroll
* rôle pédagogique

---

## 5. Ce que Cursor DOIT générer

Pour chaque Screen :

1. Le ScreenContract commenté
2. Le composant `*Screen.tsx`
3. Une structure JSX vide mais valide
4. Des `TODO` explicites

---

## 6. Ce que Cursor NE DOIT JAMAIS générer

❌ Routing
❌ useNavigate / useParams
❌ appels Supabase / fetch
❌ logique métier
❌ logique responsive
❌ styles custom
❌ composants hors UI Kit

---

## 7. Processus de génération imposé à Cursor

```
1. Identifier le type d’écran (depuis le catalogue)
2. Écrire le ScreenContract
3. Valider les règles UX
4. Générer le squelette
5. Ajouter des TODOs
```

Si une étape échoue → **STOP**.

---

## 8. Principe final (à graver)

> Le catalogue d’écrans est la colonne vertébrale UX du produit.
>
> Les routes changent.
> Les plateformes changent.
>
> **Les Screens restent.**

---

## 9. Instruction Cursor (à copier)

```
This project follows the "Screen Catalogue Specification".
You MUST generate screens using ONLY the screen types defined there.
If a requested screen does not match the catalogue, you MUST refuse.
```

---

## 🔚 Conclusion

Avec ce document :

* Cursor devient **un générateur d’écrans industriels**
* tu élimines **90 % des dérives UX**
* tu sécurises **Web → Mobile → Native**
* tu transformes ton framework en **produit durable**
