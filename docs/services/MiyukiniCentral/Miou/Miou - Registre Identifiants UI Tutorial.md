# Miou — Registre des identifiants UI pour tutoriels

Liste des `data-tutorial-id` utilisés par le moteur de tutoriels pour la **flèche verte** et le ciblage précis des éléments.

---

## Convention

- Format : `{zone}-{element}` ou `{zone}-{element}-{suffixe}`
- Minuscules, tirets
- Suffixe dynamique pour éléments en liste : `{prefix}-{id}` (ex. `service-card-jayxpose`)

---

## 1. Header et navigation principale

| ID | Composant | Emplacement | Description |
|----|-----------|-------------|-------------|
| `nav-salon` | Header | `header.rs` | Bouton SALON — page d'accueil, lieu de vie avec Miou |
| `nav-bibliotheque` | Header | `header.rs` | Bouton BIBLIOTHÈQUE — services installés |
| `nav-webway` | Header | `header.rs` | Bouton WEBWAY — réseau MWS |
| `nav-miyukini` | Header | `header.rs` | Bouton MIYUKINI — paramètres du COG |
| `search-input` | Header | `header.rs` | Champ de recherche « Rechercher dans le magasin » |
| `btn-profile` | Header | `header.rs` | Zone profil (avatar + pseudo) |
| `btn-notifications` | Header | `header.rs` | Bouton notifications (🔔) |

---

## 2. TabBar (onglets services ouverts)

| ID | Composant | Emplacement | Description |
|----|-----------|-------------|-------------|
| `tab-accueil` | TabBar | `tab_bar.rs` | Onglet Accueil (🏠) |
| `tab-service-{id}` | TabBar | `tab_bar.rs` | Onglet d'un service (ex. `tab-service-jayxpose`) |
| `tab-add` | TabBar | `tab_bar.rs` | Bouton « + » — ouvre Bibliothèque pour ajouter un onglet |

---

## 3. Home / Salon — Grille de services

| ID | Composant | Emplacement | Description |
|----|-----------|-------------|-------------|
| `home-welcome` | HomeView | `home.rs` | Zone bannière « Bienvenue dans Miyukini Central » |
| `filter-tous` | ServiceGrid | `service_grid.rs` | Bouton filtre « Tous » |
| `filter-installes` | ServiceGrid | `service_grid.rs` | Bouton filtre « Installés » |
| `filter-favoris` | ServiceGrid | `service_grid.rs` | Bouton filtre « Favoris » |
| `service-card-{id}` | ServiceCard | `service_card.rs` | Carte d'un service (ex. `service-card-jayxpose`, `service-card-jaykoa`) |

---

## 4. Webway (MWS)

| ID | Composant | Emplacement | Description |
|----|-----------|-------------|-------------|
| `mws-header` | MwsNetworkView | `mws_view.rs` | En-tête « Réseau MWS » |
| `mws-btn-connect` | MwsConnectionButton | `mws_view.rs` | Bouton Se connecter / Déconnecter |
| `mws-btn-lone` | MwsLoneModeToggle | `mws_view.rs` | Toggle Mode Lone / Réseau |
| `mws-status-card` | MwsStatusCard | `mws_view.rs` | Carte état de connexion |
| `mws-conformity` | MwsConformityProgress | `mws_view.rs` | Bloc protocole de conformité (étapes 1–10) |
| `mws-search-input` | MwsSearchSection | `mws_view.rs` | Champ recherche COGs / Lobbys (visible une fois connecté) |
| `mws-lobbys-section` | MwsLobbysSection | `mws_view.rs` | Liste des lobbys découverts |
| `mws-cogs-section` | MwsCogsSection | `mws_view.rs` | Liste des COGs découverts |

---

## 5. Miyukini (Paramètres)

| ID | Composant | Emplacement | Description |
|----|-----------|-------------|-------------|
| `settings-general` | SettingsSection | `home.rs` | Carte Général (langue, thème, notifications) |
| `settings-securite` | SettingsSection | `home.rs` | Carte Sécurité |
| `settings-cog-cores` | SettingsSection | `home.rs` | Carte COG & Cores |
| `settings-stockage` | SettingsSection | `home.rs` | Carte Stockage |

---

## 6. Mapping pour les tutoriels

### Tutoriel Central (tutoriel_central_intro)

| Étape | data-tutorial-id cible | Action suggérée |
|-------|------------------------|-----------------|
| 1 | `home-welcome` | Lire le message d'accueil |
| 2 | `nav-salon` | Expliquer Salon |
| 3 | `nav-bibliotheque` | Expliquer Bibliothèque |
| 4 | `nav-webway` | Expliquer Webway |
| 5 | `service-card-jayxpose` (ou premier installé) | Montrer comment ouvrir un service |
| 6 | `tab-accueil` | Montrer les onglets ouverts |

### Tutoriel MWS (tutoriel_mws_connexion)

| Étape | data-tutorial-id cible | Action suggérée |
|-------|------------------------|-----------------|
| 1 | `mws-header` | Présenter le Webway |
| 2 | `mws-conformity` | Expliquer le protocole de conformité |
| 3 | `mws-btn-connect` | Inviter à cliquer pour se connecter |
| 4 | `mws-btn-lone` | Expliquer le mode Lone (optionnel) |
| 5 | `mws-search-input` | Une fois connecté : recherche COGs / Lobbys |

---

**Version :** 1.0  
**Statut :** Registre de référence — à maintenir lors des changements UI
