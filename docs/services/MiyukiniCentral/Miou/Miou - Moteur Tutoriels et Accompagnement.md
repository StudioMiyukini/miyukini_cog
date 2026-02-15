# Miou — Moteur Tutoriels et Accompagnement

Miou propose ou répond à des sollicitations pour **montrer, expliquer et accompagner** l'utilisateur dans le COG. Elle est l'incarnation des tutoriels de Miyukini Central.

---

## 1. Vision et principes

### 1.1 Rôle de Miou-tuteur

| Aspect | Description |
|--------|-------------|
| **Proposition** | Miou propose un tutoriel quand le contexte le suggère (première visite Salon, nouveau sur Webway, etc.) |
| **Sollicitation** | L'utilisateur peut demander explicitement (« Explique-moi Central », « Comment rejoindre le réseau ? ») |
| **Accès doc** | Miou a accès à **toute la documentation du projet** (`docs/`) pour répondre avec précision |
| **Flèche verte** | Miou peut indiquer visuellement où cliquer avec une **flèche verte** pointant vers l'élément UI concerné |

### 1.2 Précision et efficacité

Le moteur de tutoriels doit être :
- **Précis** : chaque étape cible un élément UI identifiable via `data-tutorial-id`
- **Efficace** : étapes courtes, ordre logique, pas de redondance
- **Adaptable** : Miou propose ou saute des étapes selon le niveau perçu de l'utilisateur

---

## 2. Architecture du moteur

### 2.1 Flux général

```
┌─────────────────────────────────────────────────────────────────┐
│  TRIGGER (proposition ou sollicitation)                          │
│  - Premier accès Salon                                          │
│  - « Explique-moi Central » / « Comment ça marche ? »           │
│  - Changement vers Webway (première fois)                       │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│  SÉLECTION TUTORIEL                                             │
│  - tutoriel_central_intro                                        │
│  - tutoriel_mws_connexion                                        │
│  - etc.                                                         │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│  EXÉCUTION ÉTAPES                                                │
│  Pour chaque étape :                                            │
│    1. Texte explicatif (bulle Miou)                              │
│    2. [Optionnel] Flèche verte → data-tutorial-id cible         │
│    3. Validation (clic utilisateur ou "Suivant")                 │
└─────────────────────────────────────────────────────────────────┘
```

### 2.2 Sources de contenu

| Source | Usage |
|--------|-------|
| **Registre des tutoriels** | Étapes pré-définies, ordre, cibles UI |
| **Documentation projet** | Référence pour rédiger et mettre à jour les tutoriels |
| **Banque de templates** | Variantes de phrases Miou pour chaque étape |

### 2.3 Intégration avec le Bot existant

- Nouvelle catégorie de triggers : `tutoriel_*`
- Nouveaux triggers : `E-T01` (demande tutoriel Central), `E-T02` (demande tutoriel MWS), `T-T01` (premier accès Salon)
- Variables de contexte : `tutorial_in_progress`, `tutorial_current_step`, `tutorial_id`
- Voir [Bot - Catalogue Complet des Triggers](./Bot/Bot%20-%20Catalogue%20Complet%20des%20Triggers.md) pour l'extension

---

## 3. Flèche verte — Pointeur UI

### 3.1 Spécification visuelle

- **Icône** : flèche verte pointant vers le bas ou vers l'élément (style Steam / Gaming)
- **Position** : superposition ou ancrage à proximité de l'élément cible (`data-tutorial-id`)
- **Animation** : légère pulsation ou glow pour attirer l'œil
- **Dismiss** : disparaît à l'étape suivante ou au clic sur l'élément cible

### 3.2 Identifiants UI (Registre)

Chaque élément ciblable doit avoir un attribut `data-tutorial-id` (ou équivalent) :

| ID | Zone | Élément |
|----|------|---------|
| `nav-salon` | Header | Bouton SALON |
| `nav-bibliotheque` | Header | Bouton BIBLIOTHÈQUE |
| `nav-webway` | Header | Bouton WEBWAY |
| `nav-miyukini` | Header | Bouton MIYUKINI |
| `search-input` | Header | Champ de recherche |
| `tab-accueil` | TabBar | Onglet Accueil |
| `tab-add` | TabBar | Bouton « + » (ouvrir Bibliothèque) |
| `service-card-{id}` | Grille | Carte service (ex. `service-card-jayxpose`) |
| `filter-tous` | Grille | Filtre Tous |
| `filter-installes` | Grille | Filtre Installés |
| `filter-favoris` | Grille | Filtre Favoris |
| `mws-btn-connect` | MWS | Bouton Se connecter |
| `mws-btn-lone` | MWS | Toggle Mode Lone / Réseau |
| `mws-search` | MWS | Champ recherche COGs/Lobbys |
| `mws-conformity` | MWS | Bloc protocole de conformité |

Voir [Miou - Registre Identifiants UI Tutorial](./Miou%20-%20Registre%20Identifiants%20UI%20Tutorial.md) pour la liste complète et les mises à jour.

### 3.3 Implémentation technique (Dioxus / web)

- Utiliser `data-tutorial-id="{id}"` sur les éléments concernés
- Composant `TutorialPointer` : absolu, `querySelector` ou `getElementById` sur `[data-tutorial-id="..."]`, position calculée (getBoundingClientRect)
- Ou : overlay avec flèche SVG positionnée dynamiquement

---

## 4. Proposition vs sollicitation

### 4.1 Miou propose un tutoriel

| Condition | Tutoriel proposé | Bulle type |
|-----------|------------------|------------|
| Première connexion session + jamais vu tutoriel Central | `tutoriel_central_intro` | « Tu découvres Central ? Je peux te guider. » |
| Premier clic sur Webway | `tutoriel_mws_intro` | « Le Webway, c’est le réseau entre COGs. Tu veux que je t’explique ? » |
| Premier service installé | (optionnel) | Rappel rapide « Tu peux cliquer sur une carte pour ouvrir le service. » |

### 4.2 L'utilisateur sollicite

| Phrase / intention | Tutoriel déclenché |
|--------------------|--------------------|
| « Explique-moi Central », « Comment ça marche ? », « C’est quoi Central ? » | `tutoriel_central_intro` |
| « Comment me connecter au réseau ? », « C’est quoi le Webway ? », « Comment rejoindre le MWS ? » | `tutoriel_mws_intro` ou `tutoriel_mws_connexion` |
| « Où sont mes services ? » | Étape ciblée : navigation Bibliothèque |
| « Comment ouvrir un service ? » | Étape ciblée : clic sur ServiceCard |

---

## 5. Liens avec la doc projet

Miou s’appuie sur :

| Document | Usage |
|----------|-------|
| `docs/reference/Miyukini Conceptual References - Miyukini Central Hub Services.md` | Définition Central, rôle, flux |
| `docs/miyukini-webway-system/MWS - Document Fondateur.md` | Définition MWS, acteurs, principes |
| `docs/services/MiyukiniCentral/Miyukini Central - Ecrans et UI.md` | Structure écrans, zones |
| `docs/services/MiyukiniCentral/Miou/Tutoriels/Tutoriel - Miyukini Central Introduction.md` | Central : étapes, variantes |
| `docs/services/MiyukiniCentral/Miou/Tutoriels/Tutoriel - MWS Webway Introduction et Connexion.md` | MWS : étapes, variantes |

---

## 6. Références

- [Bot - Integration et Flux de Donnees](./Bot/Bot%20-%20Integration%20et%20Flux%20de%20Donnees.md)
- [Bot - Catalogue Complet des Triggers](./Bot/Bot%20-%20Catalogue%20Complet%20des%20Triggers.md)
- [Miou - Moteur de Generation Templates et LLM](./Miou%20-%20Moteur%20de%20Generation%20Templates%20et%20LLM.md)

---

**Version :** 1.0  
**Statut :** Spécification moteur tutoriels
