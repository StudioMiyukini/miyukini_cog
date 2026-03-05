# MiyukiniAdmin â€” Affichage Dynamique et Metriques

## 1. Contexte

Ce document definit la **strategie technique** pour afficher des donnees evolutives en temps (ou quasi temps) dans MiyukiniAdmin : metriques DB (requetes/s, latence, pool), infos DB (tables, lignes), logs, statut operateurs. L'objectif est de privilegier **Rust au maximum** et de n'introduire une nouvelle dependance que si necessaire.

**References :** [DB Metrics Contract](../contracts/monitoring/MiyukiniAdmin%20-%20DB%20Metrics%20Contract.md), [Dashboard & Metrics Display](./MiyukiniAdmin%20-%20Dashboard%20&%20Metrics%20Display.md), [Consumption Metrics Contract](../contracts/monitoring/MiyukiniAdmin%20-%20Consumption%20Metrics%20Contract.md).

---

## 2. Portee / Scope

Ce document definit :
- Le besoin d'affichage dynamique (metriques, infos DB, logs).
- Les options techniques sans nouvelle dependance JavaScript lourde (polling, SSE).
- Les options pouvant necessiter une dependance (WebSockets, onglet Chart cote client).
- La recommandation : privilegier polling + SSE (Rust seul, Axum).

Ce document **ne remplace pas** les contrats de metriques ; il complete la strategie d'implementation cote UI et backend.

---

## 3. Besoin

- **Metriques DB :** requetes/s, latence (P95), etat du pool de connexions, sante du moteur SQL (voir [DB Metrics Contract](../contracts/monitoring/MiyukiniAdmin%20-%20DB%20Metrics%20Contract.md)).
- **Infos DB :** liste des tables, nombre de lignes, taille, index ; mise a jour periodique ou a la demande.
- **Logs :** flux des logs d'audit et operationnels ; affichage continu ou par pages.
- **Statut operateurs :** etat des Cores (KindMother, StrongFather, etc.) ; mise a jour periodique.
- **Dashboard :** cartes fonctionnalites, section PROJECT API ; donnees statiques ou rafraichies (ex. statut systeme).

---

## 4. Options sans nouvelle dependance JavaScript lourde

### 4.1 Polling (Rust + vanilla JS)

- **Principe :** le frontend appelle periodiquement des endpoints REST (ex. `GET /api/metrics`, `GET /api/tables`, `GET /api/status`) avec `fetch` et `setInterval`.
- **Backend :** Axum existant ; aucun nouveau package. Les reponses JSON sont generees en Rust.
- **Frontend :** vanilla JavaScript (deja utilise pour Daynight Admin) ; pas de React, Vue, ni build Node.
- **Usage :** suffisant pour metriques rafraichies toutes les 5 s, 15 s ou 30 s selon niveau de securite et charge.
- **Conclusion :** aucune nouvelle dependance.

### 4.2 SSE (Server-Sent Events)

- **Principe :** le serveur pousse des evenements au client sur un canal unidirectionnel (serveur -> client). Le client utilise l'API native `EventSource`.
- **Backend :** Axum supporte les reponses en stream (body stream) ; pas de librairie externe obligatoire pour formater les evenements SSE (format texte simple).
- **Frontend :** `new EventSource('/api/metrics/stream')` et ecoute des messages ; vanilla JS.
- **Usage :** adapte pour logs (flux continu) ou metriques poussees par le serveur a intervalle regulier.
- **Conclusion :** pas de nouvelle dependance ; tout en Rust cote backend.

---

## 5. Options pouvant necessiter une dependance

### 5.1 WebSockets

- **Principe :** canal bidirectionnel pour un "Realtime" type Supabase (push instantane des changements).
- **Backend :** Axum 0.7 peut gerer les WebSockets avec la feature **`ws`** (a activer dans [Cargo.toml](..//..//..//_index.md) : `axum = { version = "0.7", features = ["json", "ws"] }`). Pas de nouveau crate obligatoire.
- **Frontend :** `new WebSocket(...)` en vanilla JavaScript ; pas de framework.
- **Conclusion :** pas de nouvelle dependance npm ; eventuellement feature Cargo `ws` pour Axum.

### 5.2 Onglet Chart (SQL Editor)

- **Besoin :** visualisation des resultats de requete ou des metriques en courbes/barres (onglet Chart du SQL Editor ou page Metriques).
- **Options :**
  1. **Serveur Rust genere SVG/HTML :** le backend produit un SVG ou un fragment HTML representant le graphique. Aucune dependance frontend.
  2. **Client :** librairie JS legere (ex. Chart.js) ou canvas vanilla pour dessiner les graphiques. Meilleure interactivite mais introduit une dependance JS (ou CDN).
- **A signaler :** si choix client pour de beaux graphiques interactifs, une dependance JS legere (ou chargement CDN) sera necessaire ; sinon rester sur tableaux + export (et eventuellement SVG serveur).

---

## 6. Recommandation

- **Privilegier polling + SSE** (Rust seul, Axum) pour metriques et logs. Pas de nouveau package npm.
- **WebSockets :** activer la feature `ws` d'Axum si un push temps reel type Realtime est requis ; cote client, WebSocket natif suffit.
- **Onglet Chart :** documenter que l'onglet peut rester **tableau + export** (Rust seul) ou, si graphiques requis, proposer une option "serveur SVG" ou indiquer la possibilite d'une lib JS legere (a decider en implementation).

---

## 7. Pages concernees (affichage dynamique)

| Page | Donnees dynamiques | Methode recommandee |
|------|--------------------|----------------------|
| Dashboard / Project Overview | Statut systeme, cartes, PROJECT API | Polling (ex. 30 s) ou statique |
| Metriques / Observability | Metriques DB, systeme | SSE ou polling (Rust) |
| Logs | Flux logs audit / operationnels | SSE pour flux continu (Rust) |
| Table Editor | Liste tables, recent items | Polling ou bouton "Refresh" |
| Table Editor â€” Donnees table | Lignes, pagination | Polling ou refresh a la demande |
| SQL Editor | Resultats, Explain | Un appel API par execution ; pas de push |

---

## 8. Documents associes

- [MiyukiniAdmin - DB Metrics Contract](../contracts/monitoring/MiyukiniAdmin%20-%20DB%20Metrics%20Contract.md)
- [MiyukiniAdmin - Consumption Metrics Contract](../contracts/monitoring/MiyukiniAdmin%20-%20Consumption%20Metrics%20Contract.md)
- [MiyukiniAdmin - Dashboard & Metrics Display](./MiyukiniAdmin%20-%20Dashboard%20&%20Metrics%20Display.md)
- [MiyukiniAdmin - Organisation Pages et UX DB](./MiyukiniAdmin%20-%20Organisation%20Pages%20et%20UX%20DB.md)
- [MiyukiniAdmin - Reference SQL et DB](../reference/MiyukiniAdmin%20-%20Reference%20SQL%20et%20DB.md)

---

**Date de creation :** 2026-01-29  
**Version :** 1.0.0  
**Statut :** Document de reference (strategie technique affichage dynamique)

