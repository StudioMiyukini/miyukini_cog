# Bot Miou — Capacités Avancées et Jalons

Ce document décrit les **capacités avancées** du moteur Bot : gestion des streaks, jalons, observations contextuelles, et règles de personnalisation fine. Il définit comment Miou exploite les agrégats MiyukiniWatch pour des bulles plus intelligentes.

---

## 1. Contexte

Les agrégats MiyukiniWatch fournissent des données riches : `AGG_MILESTONES`, `AGG_NEW_MILESTONE`, `AGG_TOP_FRIENDS`, `AGG_FAVORITE_SERVICE`, etc. Ce document spécifie comment le Bot les **consomme** et **transforme** en bulles pertinentes.

---

## 2. Jalons (Milestones) — Catalogue complet

### 2.1 Jalons de session

| Type | Condition | Valeur | Catégorie | Exemple bulle |
|------|-----------|--------|-----------|---------------|
| `first_session` | Première session ever | 1 | bienvenue_premiere_fois | « Bienvenue dans Miyukini Central. » |
| `streak_7` | 7 jours consécutifs | 7 | felicitation_streak | « 7 jours d'affilée — bravo ! » |
| `streak_30` | 30 jours consécutifs | 30 | felicitation_streak | « Un mois ensemble — belle fidélité. » |
| `total_sessions_100` | 100 sessions totales | 100 | felicitation_milestone | « 100 visites — ton COG te connaît. » |

### 2.2 Jalons de services

| Type | Condition | Catégorie | Exemple bulle |
|------|-----------|-----------|---------------|
| `first_service` | Premier service ouvert | felicitation_milestone | « Premier service — bienvenue dans l'aventure. » |
| `three_services` | 3 services différents ouverts | felicitation_milestone | « Tu as exploré 3 coins de ton COG. » |
| `service_favorite` | Service le plus utilisé identifié | observation_service_favori | « JayXpose — ton coin préféré. » |
| `service_neglected` | Service non ouvert > 14j | suggestion_service | « Tu n'as pas ouvert JayKoa depuis un moment. » |

### 2.3 Jalons sociaux

| Type | Condition | Catégorie | Exemple bulle |
|------|-----------|-----------|---------------|
| `first_friend_contact` | Premier échange avec un ami | felicitation_milestone | « Premier ami contacté — le Webway prend vie. » |
| `three_friends` | 3 amis distincts contactés | felicitation_milestone | « 3 amis — ton cercle grandit. » |
| `friend_neglected` | Ami non contacté > 7j | rappel_ami | « Pense à {ami}, ça fait {jours} jours. » |
| `friend_top` | Ami le plus contacté (top 1) | observation_ami_proche | « Tu passes beaucoup de temps avec {ami}. » |

### 2.4 Jalons MWS et externes

| Type | Condition | Catégorie | Exemple bulle |
|------|-----------|-----------|---------------|
| `webway_connected` | Connexion MWS réussie | felicitation_badge | « Webway connecté — bienvenue sur le réseau. » |
| `vitrine_published` | Vitrine JayXpose publiée | felicitation_badge | « Ta vitrine est en ligne. » |
| `first_event` | Premier événement JayKoa créé | felicitation_milestone | « Premier événement — ça se fête ! » |

---

## 3. Streaks — Règles de gestion

### 3.1 Calcul du streak

- **Source :** `consecutive_active_days` (MiyukiniWatch, AGG_SESSION_SUMMARY).
- **Définition :** Nombre de jours **consécutifs** avec au moins une session.
- **Reset :** Un jour sans session remet le streak à 0.

### 3.2 Seuils de félicitation

| Streak | Quand afficher | Cooldown |
|--------|----------------|----------|
| 7 | Dès que atteint | Une fois par déblocage |
| 14 | Dès que atteint | Une fois |
| 30 | Dès que atteint | Une fois |
| 100 | Dès que atteint | Une fois |

### 3.3 Règle anti-spam

- Ne pas afficher une bulle streak **chaque jour** une fois le seuil dépassé.
- Exemple : Streak à 8 jours → pas de nouvelle bulle « 8 jours ». Attendre 14 ou 30.

---

## 4. Capacités d'observation

### 4.1 Détection du service favori

- **Agrégat :** `AGG_FAVORITE_SERVICE` ou `AGG_TOP_SERVICES`.
- **Usage :** Adapter la variante de suggestion.
  - JayXpose → « Ta vitrine est à jour ? »
  - JayKoa → « Un événement à rappeler ? »
  - Jay1Tribu → « Un ami à saluer ? »

### 4.2 Détection du profil d'activité

- **Agrégat :** `AGG_ACTIVITY_LEVEL` (`inactive`, `low`, `moderate`, `active`, `very_active`).
- **Usage :**
  - `inactive` + `jours_absent >= 7` → catégorie `encouragement_retour`
  - `very_active` → éviter les suggestions « fais plus » ; privilégier pause ou félicitation

### 4.3 Tranche horaire

- **Agrégat :** `AGG_SESSION_TIME` (`MORNING`, `AFTERNOON`, `EVENING`, `NIGHT`).
- **Usage :** Sélection catégorie accueil, ton nocturne si NIGHT.

---

## 5. Personnalisation par service favori

| Service favori | Variante de suggestion délaissé | Variante service favori |
|----------------|--------------------------------|---------------------------|
| JayXpose | « Tu n'as pas ouvert JayKoa — une vitrine à visiter ? » | « JayXpose — ton profil exposant est à jour ? » |
| JayKoa | « JayKoa te manque peut-être — un événement à créer ? » | « Tu aimes ton calendrier — un rappel ? » |
| Jay1Tribu | « Jay1Tribu — un ami t'attend peut-être. » | « Tu passes du temps avec tes amis — c'est bien. » |

---

## 6. Règles de priorité pour jalons

Lorsque plusieurs jalons sont « fraîchement » débloqués :

| Priorité | Type de jalon | Règle |
|----------|---------------|-------|
| 1 | Streak 30 | Avant streak 7 (plus rare) |
| 2 | Badge (Webway, Vitrine) | Avant milestone « first » |
| 3 | First service/ami | Une fois par type |
| 4 | Observation (favori, ami proche) | Priorité basse, 1 par session max |

---

## 7. Mapping agrégat → capacité

| Agrégat | Capacité activée |
|---------|------------------|
| AGG_SESSION_RETURN | Retour absence, retour même jour |
| AGG_SESSION_SUMMARY.consecutive_active_days | Félicitation streak |
| AGG_TOP_SERVICES | Observation service favori |
| AGG_NEGLECTED_SERVICES | Suggestion service |
| AGG_FRIEND_REMINDERS | Rappel ami |
| AGG_TOP_FRIENDS | Observation ami proche |
| AGG_ACTIVITY_LEVEL | Encouragement retour, adaptation ton |
| AGG_NEW_MILESTONE | Félicitation jalon immédiate |
| AGG_FAVORITE_TAB | Contexte (Bibliothèque → suggestion exploration) |
| AGG_MACHINE_SPECS | Specs RAM, stockage, OS — demandes, upgrade, taquinerie |
| AGG_USER_RESPONSES | Réponses explicites (chiffrées) — personnalisation, curiosité |

---

## 8. Specs machine et connaissance utilisateur

Voir [Bot - Connaissance Utilisateur et Specs Machine](./Bot%20-%20Connaissance%20Utilisateur%20et%20Specs%20Machine.md) pour :

- **Specs** : Miou peut réclamer plus de RAM/stockage, commenter les upgrades
- **Taquinerie** : Sujets innocents (OS, heure, habitudes)
- **Curiosité** : Questions à l'utilisateur — réponses stockées localement, chiffrées (seule donnée lue et enregistrée par Miou)

---

## 9. Références

- [Bot - Connaissance Utilisateur et Specs Machine](./Bot%20-%20Connaissance%20Utilisateur%20et%20Specs%20Machine.md)
- [MiyukiniWatch - Intégration Miou et Agrégats](../../../MiyukiniWatch/MiyukiniWatch%20-%20Integration%20Miou%20et%20Agregats.md)
- [Miou - Gamification et Progression](../Miou%20-%20Gamification%20et%20Progression.md)
- [Bot - Catalogue Complet des Triggers](./Bot%20-%20Catalogue%20Complet%20des%20Triggers.md)

---

*Miou observe, comprend, s'adapte. Chaque jalon est une occasion de renforcer la relation.*
