# MiyuClicker â€” Ressources et catÃ©gories

## Contexte

Ce document fixe la **terminologie officielle** des ressources affichÃ©es et des **catÃ©gories** de ressources dans MiyuClicker. Il Ã©vite de confondre une **ressource** (quantitÃ© affichÃ©e et manipulÃ©e par le joueur) avec une **catÃ©gorie** (regroupement conceptuel pour rÃ¨gles de jeu : caps de stockage, coÃ»ts, etc.).

## PortÃ©e / Scope

- **Applicable Ã  :** UI (barre de ressources), simulation (idlesim), sauvegarde, documentation.
- **Audience :** DÃ©veloppeurs, designers, rÃ©dacteurs.
- **Statut :** Document de rÃ©fÃ©rence normatif.

---

## 1. Ressources (affichÃ©es)

Les **ressources** sont les quantitÃ©s affichÃ©es dans la barre et utilisÃ©es par les rÃ¨gles de jeu. Chaque ressource a une valeur courante (et Ã©ventuellement une capacitÃ© max).

| Ressource    | Description courte                    | Cap / max                    |
|-------------|----------------------------------------|------------------------------|
| **Or**      | Monnaie                                | â€”                            |
| **Gens**    | Population                             | cap_gens (maisons Ã— 4)       |
| **Soldats** | Troupes                                | cap_soldats()                |
| **MaÃ§ons**  | Ouvriers affectÃ©s aux constructions   | â€”                            |
| **Recherche** | Points de recherche                  | â€”                            |
| **Bonheur** | Moral ( %)                             | â€”                            |
| **Nourriture** | Stock nourriture                    | cap_nourriture() (Grenier)    |
| **Bois**    | MatiÃ¨re premiÃ¨re (bois)               | part du cap matiÃ¨res premiÃ¨res |
| **Pierre**  | MatiÃ¨re premiÃ¨re (pierre)              | part du cap matiÃ¨res premiÃ¨res |
| **Fer**     | MatiÃ¨re premiÃ¨re (fer)                 | part du cap matiÃ¨res premiÃ¨res |
| **Outils**  | Produit manufacturÃ©                    | part du cap produits manufacturÃ©s |
| **Armes**   | Produit manufacturÃ©                    | part du cap produits manufacturÃ©s |

---

## 2. CatÃ©gories (pas des ressources)

Les **catÃ©gories** sont des regroupements utilisÃ©s pour les **rÃ¨gles de jeu** (caps de stockage, coÃ»ts agrÃ©gÃ©s, etc.). Elles ne sont **pas** des ressources affichÃ©es en tant que telles : on nâ€™affiche pas Â« MatiÃ¨re premiÃ¨re Â» ou Â« Produits manufacturÃ©s Â» comme une ligne de ressource, mais on affiche les ressources qui les composent.

### 2.1 MatiÃ¨re premiÃ¨re (matiÃ¨res premiÃ¨res)

- **DÃ©finition :** CatÃ©gorie regroupant **Bois**, **Pierre** et **Fer**.
- **Usage :** Cap de stockage global (DÃ©pÃ´t) : `cap_matiÃ¨res()`. Le total (bois + pierre + fer) ne peut pas dÃ©passer ce cap.
- **Ã€ ne pas faire :** Afficher Â« MatiÃ¨re premiÃ¨re Â» ou Â« MatiÃ¨res Â» comme une ressource avec une quantitÃ© propre. Afficher **Bois**, **Pierre**, **Fer** sÃ©parÃ©ment.

### 2.2 Produits manufacturÃ©s

- **DÃ©finition :** CatÃ©gorie regroupant **Outils** et **Armes**.
- **Usage :** Cap de stockage global (EntrepÃ´t) : `cap_manufacturÃ©s()`. Le total (outils + armes) ne peut pas dÃ©passer ce cap.
- **Ã€ ne pas faire :** Afficher Â« Produits manufacturÃ©s Â» ou Â« ManufacturÃ©s Â» comme une ressource avec une quantitÃ© propre. Afficher **Outils** et **Armes** sÃ©parÃ©ment.

---

## 3. RÃ©capitulatif

| Terme                  | Nature      | AffichÃ© dans la barre ? | Exemple |
|------------------------|------------|---------------------------|---------|
| Nourriture, Bois, Pierre, Fer, Outils, Armes | Ressource | Oui (chaque ressource)    | Bois: 17, Pierre: 26 |
| MatiÃ¨re premiÃ¨re       | CatÃ©gorie  | Non                       | Regroupe Bois, Pierre, Fer ; utilisÃ© pour cap_matiÃ¨res() |
| Produits manufacturÃ©s  | CatÃ©gorie  | Non                       | Regroupe Outils, Armes ; utilisÃ© pour cap_manufacturÃ©s() |

---

## 4. RÃ©fÃ©rences

- [MiyuClicker - Document Fondateur](MiyukiniClicker%20-%20Document%20Fondateur.md) â€” section Gameplay, ressources.
- Code : `crates/miyuclicker/src/state.rs` â€” `cap_matiÃ¨res()`, `cap_manufacturÃ©s()`, `cap_nourriture()`.
- Code : `crates/miyuclicker/src/app.rs` â€” barre de ressources (`ui_bar`).

---

**Date de crÃ©ation :** 2026-02-01  
**Statut :** Document de rÃ©fÃ©rence â€” ressources vs catÃ©gories

