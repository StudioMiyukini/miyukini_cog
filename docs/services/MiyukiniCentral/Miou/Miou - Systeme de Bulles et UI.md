# Miou â€” SystÃ¨me de Bulles et UI

SpÃ©cification du canal de communication principal de Miou : les **bulles** affichÃ©es en bas Ã  droite de Miyukini Central.

---

## 1. Positionnement

| Attribut | SpÃ©cification |
|----------|----------------|
| **Position** | **En bas Ã  droite** de la fenÃªtre Miyukini Central. Zone fixe (pas de drag). |
| **Z-index** | Au-dessus de tout le contenu (services, Salon, TabBar) mais sous les modales systÃ¨mes (profile_window, dialogues OS). |
| **Marge** | ~16px du bord droit, ~16px du bord infÃ©rieur (au-dessus de la status bar). |
| **Taille max** | Largeur max ~360px ; hauteur adaptative au contenu. Sur Ã©cran < 480px de large : pleine largeur avec marges rÃ©duites. |

---

## 2. Anatomie d'une bulle

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚  ðŸŒ¸  â”€â”€â”€ Miou                     âœ•   â”‚
â”‚                                        â”‚
â”‚  Â« Bonsoir Kaito ! Tu as passÃ© un      â”‚
â”‚    bon moment sur JayXpose ce soir.    â”‚
â”‚    Pense Ã  faire une pause. Â»          â”‚
â”‚                                        â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”‚
â”‚  â”‚ ðŸ• Bonne idÃ©e â”‚  â”‚ C'est notÃ©     â”‚  â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

| Ã‰lÃ©ment | Description |
|---------|-------------|
| **Avatar** | IcÃ´ne Miou (ðŸŒ¸ ou mascotte illustrÃ©e) en haut Ã  gauche. |
| **Nom** | Â« Miou Â» â€” identifie clairement la source. |
| **Bouton fermer** | âœ• en haut Ã  droite ; ferme la bulle pour la session en cours. |
| **Corps** | Texte court (1â€“3 phrases max). Ton Miou (bienveillant, tutoiement). |
| **Actions rapides** | 0 Ã  2 boutons en bas. Optionnels selon le type de bulle. |

---

## 3. Types de bulles

| Type | DÃ©clencheur | Contenu | Actions |
|------|-------------|---------|---------|
| **Accueil** | ArrivÃ©e dans Central (aprÃ¨s Connexion) | Â« Bonjour/Bonsoir [pseudo] Â» + phrase contextuelle | Aucune ou Â« Voir les suggestions Â» |
| **Suggestion** | MÃ©triques MiyukiniWatch (service dÃ©laissÃ©, ami non contactÃ©) | Phrase courte + suggestion | Â« Ouvrir [service] Â» / Â« C'est notÃ© Â» |
| **Rappel** | Ã‰vÃ©nement JayKoa, durÃ©e de session > seuil | Rappel discret | Â« Voir le calendrier Â» / Â« Pause Â» / Â« Plus tard Â» |
| **FÃ©licitation** | Badge dÃ©bloquÃ©, Ã©tape franchie | CÃ©lÃ©bration (ton lÃ©ger) | Â« Super ! Â» |
| **Notification** | Ami connectÃ© (Jay1Tribu), nouveau message en attente | Info courte | Â« Ouvrir Jay1Tribu Â» / Â« Plus tard Â» |
| **Conversation** (futur, LLM) | RÃ©ponse Ã  une question de l'utilisateur | Phrase gÃ©nÃ©rÃ©e par le LLM | Aucune ou Â« En savoir plus Â» |

---

## 4. Comportement

### 4.1 Apparition

| RÃ¨gle | Description |
|-------|-------------|
| **Animation** | Fade-in + lÃ©ger slide-up (200ms ease-out). Pas de pop brutal. |
| **DÃ©lai initial** | La premiÃ¨re bulle apparaÃ®t **2â€“3 secondes** aprÃ¨s l'arrivÃ©e dans Central (laisser le temps au Salon de se charger). |
| **Pas au-dessus des modales** | Si une modale est ouverte (profil, formulaire), la bulle attend la fermeture. |

### 4.2 Fermeture

| RÃ¨gle | Description |
|-------|-------------|
| **Bouton âœ•** | Ferme la bulle immÃ©diatement (fade-out 150ms). |
| **Auto-dismiss** | Les bulles d'accueil et de fÃ©licitation disparaissent aprÃ¨s ~15 secondes si l'utilisateur ne clique pas. Les rappels restent jusqu'Ã  dismiss ou action. |
| **Dismiss = respectÃ©** | AprÃ¨s fermeture, Miou ne repropose pas le mÃªme message dans la mÃªme session. |

### 4.3 File d'attente et prioritÃ©

Si plusieurs messages sont disponibles au mÃªme moment :

| PrioritÃ© | Type | Raison |
|----------|------|--------|
| **1 (haute)** | Rappel (Ã©vÃ©nement imminent, pause santÃ©) | LiÃ© au bien-Ãªtre ou Ã  un timing critique. |
| **2** | Notification (ami, message) | Interaction sociale. |
| **3** | Accueil | PremiÃ¨re bulle de la session. |
| **4** | Suggestion | Service ou activitÃ© proposÃ©e. |
| **5 (basse)** | FÃ©licitation | Peut attendre. |

**RÃ¨gle de dÃ©bit :** Maximum **1 bulle Ã  la fois**. Espacement minimum **30 secondes** entre deux bulles (configurable). Maximum **5 bulles par session** par dÃ©faut (Ã©viter la fatigue).

### 4.4 Interaction avec les services

| RÃ¨gle | Description |
|-------|-------------|
| **Clic sur action** | Ouvre le service correspondant (ex. Â« Ouvrir JayKoa Â» â†’ bascule sur l'onglet JayKoa) puis ferme la bulle. |
| **Pas de blocage** | La bulle ne bloque jamais les interactions avec le reste de Central. L'utilisateur peut ignorer et continuer son activitÃ©. |
| **Clic hors bulle** | Ne ferme pas la bulle (sauf auto-dismiss timer). |

---

## 5. Adaptation responsive

| Largeur fenÃªtre | Comportement |
|-----------------|-------------|
| **> 800px** | Bulle standard (360px max) en bas Ã  droite. |
| **480â€“800px** | Bulle rÃ©duite (280px max), mÃªme position. |
| **< 480px** | Bulle pleine largeur en bas (type notification mobile), hauteur minimale. |

---

## 6. Personnalisation utilisateur

Accessible dans **ParamÃ¨tres Miyukini > Miou** :

| Option | DÃ©faut | Description |
|--------|--------|-------------|
| **Bulles activÃ©es** | Oui | DÃ©sactiver = aucune bulle ne s'affiche. |
| **Ne pas dÃ©ranger (DND)** | DÃ©sactivÃ© | Aucune bulle (sauf rappels critiques optionnels). Voir [Miou - Roadmap et AmÃ©liorations](_index.md). |
| **FrÃ©quence** | Normale (max 5/session) | Choix : DiscrÃ¨te (max 2), Normale (max 5), Bavarde (max 10). |
| **Son des bulles** | DÃ©sactivÃ© | Petit son discret Ã  l'apparition d'une bulle. |
| **Rappels de pause** | AprÃ¨s 2h | Seuil configurable (1h, 2h, 3h, dÃ©sactivÃ©). |

---

## 7. ImplÃ©mentation technique (repÃ¨res)

| Ã‰lÃ©ment | Emplacement actuel ou prÃ©vu |
|---------|---------------------------|
| **Composant UI** | Ã€ crÃ©er : `apps/central/src/components/miou_bubble.rs` (overlay Dioxus, position absolute). |
| **Moteur de dÃ©cision** | Ã€ crÃ©er : `apps/central/src/miou/engine.rs` â€” dÃ©cide quelle bulle afficher selon MiyukiniWatch, profil, contexte. |
| **Ã‰tat Miou** | Ã€ ajouter dans `AppState` : `miou_current_bubble`, `miou_queue`, `miou_session_count`, `miou_last_shown`. |
| **Rendu** | InjectÃ© dans `App` (app.rs) aprÃ¨s le `main` et avant le `footer`, visible quel que soit `MainTab`. |

---

## 8. RÃ©fÃ©rences

- [Miou - Document Fondateur](./Miou%20-%20Document%20Fondateur.md)
- [Miou - Moteur de GÃ©nÃ©ration Templates et LLM](./Miou%20-%20Moteur%20de%20Generation%20Templates%20et%20LLM.md)
- [MiyukiniWatch â€” Document Fondateur](../../MiyukiniWatch/MiyukiniWatch%20-%20Document%20Fondateur.md)

---

*Bulles Miou : en bas Ã  droite, discrÃ¨tes, respectueuses, adaptatives. Le canal principal de la relation entre Miou et l'utilisateur.*

