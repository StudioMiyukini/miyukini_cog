# Miou — Système de Bulles et UI

Spécification du canal de communication principal de Miou : les **bulles** affichées en bas à droite de Miyukini Central.

---

## 1. Positionnement

| Attribut | Spécification |
|----------|----------------|
| **Position** | **En bas à droite** de la fenêtre Miyukini Central. Zone fixe (pas de drag). |
| **Z-index** | Au-dessus de tout le contenu (services, Salon, TabBar) mais sous les modales systèmes (profile_window, dialogues OS). |
| **Marge** | ~16px du bord droit, ~16px du bord inférieur (au-dessus de la status bar). |
| **Taille max** | Largeur max ~360px ; hauteur adaptative au contenu. Sur écran < 480px de large : pleine largeur avec marges réduites. |

---

## 2. Anatomie d'une bulle

```
┌────────────────────────────────────────┐
│  🌸  ─── Miou                     ✕   │
│                                        │
│  « Bonsoir Kaito ! Tu as passé un      │
│    bon moment sur JayXpose ce soir.    │
│    Pense à faire une pause. »          │
│                                        │
│  ┌──────────────┐  ┌────────────────┐  │
│  │ 🕐 Bonne idée │  │ C'est noté     │  │
│  └──────────────┘  └────────────────┘  │
└────────────────────────────────────────┘
```

| Élément | Description |
|---------|-------------|
| **Avatar** | Icône Miou (🌸 ou mascotte illustrée) en haut à gauche. |
| **Nom** | « Miou » — identifie clairement la source. |
| **Bouton fermer** | ✕ en haut à droite ; ferme la bulle pour la session en cours. |
| **Corps** | Texte court (1–3 phrases max). Ton Miou (bienveillant, tutoiement). |
| **Actions rapides** | 0 à 2 boutons en bas. Optionnels selon le type de bulle. |

---

## 3. Types de bulles

| Type | Déclencheur | Contenu | Actions |
|------|-------------|---------|---------|
| **Accueil** | Arrivée dans Central (après Connexion) | « Bonjour/Bonsoir [pseudo] » + phrase contextuelle | Aucune ou « Voir les suggestions » |
| **Suggestion** | Métriques MiyukiniWatch (service délaissé, ami non contacté) | Phrase courte + suggestion | « Ouvrir [service] » / « C'est noté » |
| **Rappel** | Événement JayKoa, durée de session > seuil | Rappel discret | « Voir le calendrier » / « Pause » / « Plus tard » |
| **Félicitation** | Badge débloqué, étape franchie | Célébration (ton léger) | « Super ! » |
| **Notification** | Ami connecté (Jay1Tribu), nouveau message en attente | Info courte | « Ouvrir Jay1Tribu » / « Plus tard » |
| **Conversation** (futur, LLM) | Réponse à une question de l'utilisateur | Phrase générée par le LLM | Aucune ou « En savoir plus » |

---

## 4. Comportement

### 4.1 Apparition

| Règle | Description |
|-------|-------------|
| **Animation** | Fade-in + léger slide-up (200ms ease-out). Pas de pop brutal. |
| **Délai initial** | La première bulle apparaît **2–3 secondes** après l'arrivée dans Central (laisser le temps au Salon de se charger). |
| **Pas au-dessus des modales** | Si une modale est ouverte (profil, formulaire), la bulle attend la fermeture. |

### 4.2 Fermeture

| Règle | Description |
|-------|-------------|
| **Bouton ✕** | Ferme la bulle immédiatement (fade-out 150ms). |
| **Auto-dismiss** | Les bulles d'accueil et de félicitation disparaissent après ~15 secondes si l'utilisateur ne clique pas. Les rappels restent jusqu'à dismiss ou action. |
| **Dismiss = respecté** | Après fermeture, Miou ne repropose pas le même message dans la même session. |

### 4.3 File d'attente et priorité

Si plusieurs messages sont disponibles au même moment :

| Priorité | Type | Raison |
|----------|------|--------|
| **1 (haute)** | Rappel (événement imminent, pause santé) | Lié au bien-être ou à un timing critique. |
| **2** | Notification (ami, message) | Interaction sociale. |
| **3** | Accueil | Première bulle de la session. |
| **4** | Suggestion | Service ou activité proposée. |
| **5 (basse)** | Félicitation | Peut attendre. |

**Règle de débit :** Maximum **1 bulle à la fois**. Espacement minimum **30 secondes** entre deux bulles (configurable). Maximum **5 bulles par session** par défaut (éviter la fatigue).

### 4.4 Interaction avec les services

| Règle | Description |
|-------|-------------|
| **Clic sur action** | Ouvre le service correspondant (ex. « Ouvrir JayKoa » → bascule sur l'onglet JayKoa) puis ferme la bulle. |
| **Pas de blocage** | La bulle ne bloque jamais les interactions avec le reste de Central. L'utilisateur peut ignorer et continuer son activité. |
| **Clic hors bulle** | Ne ferme pas la bulle (sauf auto-dismiss timer). |

---

## 5. Adaptation responsive

| Largeur fenêtre | Comportement |
|-----------------|-------------|
| **> 800px** | Bulle standard (360px max) en bas à droite. |
| **480–800px** | Bulle réduite (280px max), même position. |
| **< 480px** | Bulle pleine largeur en bas (type notification mobile), hauteur minimale. |

---

## 6. Personnalisation utilisateur

Accessible dans **Paramètres Miyukini > Miou** :

| Option | Défaut | Description |
|--------|--------|-------------|
| **Bulles activées** | Oui | Désactiver = aucune bulle ne s'affiche. |
| **Ne pas déranger (DND)** | Désactivé | Aucune bulle (sauf rappels critiques optionnels). Voir [Miou - Roadmap et Améliorations](./Miou%20-%20Roadmap%20et%20Améliorations.md). |
| **Fréquence** | Normale (max 5/session) | Choix : Discrète (max 2), Normale (max 5), Bavarde (max 10). |
| **Son des bulles** | Désactivé | Petit son discret à l'apparition d'une bulle. |
| **Rappels de pause** | Après 2h | Seuil configurable (1h, 2h, 3h, désactivé). |

---

## 7. Implémentation technique (repères)

| Élément | Emplacement actuel ou prévu |
|---------|---------------------------|
| **Composant UI** | À créer : `apps/central/src/components/miou_bubble.rs` (overlay Dioxus, position absolute). |
| **Moteur de décision** | À créer : `apps/central/src/miou/engine.rs` — décide quelle bulle afficher selon MiyukiniWatch, profil, contexte. |
| **État Miou** | À ajouter dans `AppState` : `miou_current_bubble`, `miou_queue`, `miou_session_count`, `miou_last_shown`. |
| **Rendu** | Injecté dans `App` (app.rs) après le `main` et avant le `footer`, visible quel que soit `MainTab`. |

---

## 8. Références

- [Miou - Document Fondateur](./Miou%20-%20Document%20Fondateur.md)
- [Miou - Moteur de Génération Templates et LLM](./Miou%20-%20Moteur%20de%20Generation%20Templates%20et%20LLM.md)
- [MiyukiniWatch — Document Fondateur](../../MiyukiniWatch/MiyukiniWatch%20-%20Document%20Fondateur.md)

---

*Bulles Miou : en bas à droite, discrètes, respectueuses, adaptatives. Le canal principal de la relation entre Miou et l'utilisateur.*
