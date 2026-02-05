# MiyuClicker — Ressources et catégories

## Contexte

Ce document fixe la **terminologie officielle** des ressources affichées et des **catégories** de ressources dans MiyuClicker. Il évite de confondre une **ressource** (quantité affichée et manipulée par le joueur) avec une **catégorie** (regroupement conceptuel pour règles de jeu : caps de stockage, coûts, etc.).

## Portée / Scope

- **Applicable à :** UI (barre de ressources), simulation (idlesim), sauvegarde, documentation.
- **Audience :** Développeurs, designers, rédacteurs.
- **Statut :** Document de référence normatif.

---

## 1. Ressources (affichées)

Les **ressources** sont les quantités affichées dans la barre et utilisées par les règles de jeu. Chaque ressource a une valeur courante (et éventuellement une capacité max).

| Ressource    | Description courte                    | Cap / max                    |
|-------------|----------------------------------------|------------------------------|
| **Or**      | Monnaie                                | —                            |
| **Gens**    | Population                             | cap_gens (maisons × 4)       |
| **Soldats** | Troupes                                | cap_soldats()                |
| **Maçons**  | Ouvriers affectés aux constructions   | —                            |
| **Recherche** | Points de recherche                  | —                            |
| **Bonheur** | Moral ( %)                             | —                            |
| **Nourriture** | Stock nourriture                    | cap_nourriture() (Grenier)    |
| **Bois**    | Matière première (bois)               | part du cap matières premières |
| **Pierre**  | Matière première (pierre)              | part du cap matières premières |
| **Fer**     | Matière première (fer)                 | part du cap matières premières |
| **Outils**  | Produit manufacturé                    | part du cap produits manufacturés |
| **Armes**   | Produit manufacturé                    | part du cap produits manufacturés |

---

## 2. Catégories (pas des ressources)

Les **catégories** sont des regroupements utilisés pour les **règles de jeu** (caps de stockage, coûts agrégés, etc.). Elles ne sont **pas** des ressources affichées en tant que telles : on n’affiche pas « Matière première » ou « Produits manufacturés » comme une ligne de ressource, mais on affiche les ressources qui les composent.

### 2.1 Matière première (matières premières)

- **Définition :** Catégorie regroupant **Bois**, **Pierre** et **Fer**.
- **Usage :** Cap de stockage global (Dépôt) : `cap_matières()`. Le total (bois + pierre + fer) ne peut pas dépasser ce cap.
- **À ne pas faire :** Afficher « Matière première » ou « Matières » comme une ressource avec une quantité propre. Afficher **Bois**, **Pierre**, **Fer** séparément.

### 2.2 Produits manufacturés

- **Définition :** Catégorie regroupant **Outils** et **Armes**.
- **Usage :** Cap de stockage global (Entrepôt) : `cap_manufacturés()`. Le total (outils + armes) ne peut pas dépasser ce cap.
- **À ne pas faire :** Afficher « Produits manufacturés » ou « Manufacturés » comme une ressource avec une quantité propre. Afficher **Outils** et **Armes** séparément.

---

## 3. Récapitulatif

| Terme                  | Nature      | Affiché dans la barre ? | Exemple |
|------------------------|------------|---------------------------|---------|
| Nourriture, Bois, Pierre, Fer, Outils, Armes | Ressource | Oui (chaque ressource)    | Bois: 17, Pierre: 26 |
| Matière première       | Catégorie  | Non                       | Regroupe Bois, Pierre, Fer ; utilisé pour cap_matières() |
| Produits manufacturés  | Catégorie  | Non                       | Regroupe Outils, Armes ; utilisé pour cap_manufacturés() |

---

## 4. Références

- [MiyuClicker - Document Fondateur](MiyuClicker%20-%20Document%20Fondateur.md) — section Gameplay, ressources.
- Code : `crates/miyuclicker/src/state.rs` — `cap_matières()`, `cap_manufacturés()`, `cap_nourriture()`.
- Code : `crates/miyuclicker/src/app.rs` — barre de ressources (`ui_bar`).

---

**Date de création :** 2026-02-01  
**Statut :** Document de référence — ressources vs catégories
