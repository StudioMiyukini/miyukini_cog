# MiyukiniTerminal — Spécification Notifications

## Contexte

Ce document décrit les **types de notifications** (rappels JayKoa, seuils JayKonta), le **canal** (push FCM vs pull vs local), les **permissions Android** et le **design** des notifications.

**Références :**

- [Spec Services Consultatifs](./MiyukiniTerminal%20-%20Spec%20Services%20Consultatifs.md)
- [Spec Mode Offline](./MiyukiniTerminal%20-%20Spec%20Mode%20Offline.md)

---

## Portée / Scope

- Types : rappels JayKoa, seuils JayKonta
- Canal : push / pull / local
- Permissions Android
- Design (style, actions)

---

## 1. Types de notifications

### 1.1 Rappels JayKoa

| Déclencheur | Message exemple |
|-------------|-----------------|
| X min avant événement | "RDV médecin dans 30 min" |
| Événement du jour | "Aujourd'hui : 3 événements" |

### 1.2 Seuils JayKonta

| Déclencheur | Message exemple |
|-------------|-----------------|
| Solde < seuil | "Portefeuille Principal : solde bas (25 €)" |
| Dépense importante | "Dépense de 100 € enregistrée" (optionnel) |

### 1.3 Autres

| Type | Description |
|------|-------------|
| Sync terminée | "Données synchronisées" (optionnel, si en arrière-plan) |
| Actions en attente | "3 actions en attente de synchronisation" |

---

## 2. Canal : push vs pull vs local

### 2.1 Push (FCM)

| Avantage | Inconvénient |
|----------|--------------|
| Temps réel | Dépendance Google ; config serveur |
| Peu de batterie | Permission utilisateur |

**Flow :** Parent ou Origin envoie à FCM → FCM push au device → app affiche notification.

### 2.2 Pull (polling)

| Avantage | Inconvénient |
|----------|--------------|
| Simple | Consommation batterie ; pas temps réel |
| Pas de FCM | Délai |

**Flow :** App interroge périodiquement le parent ; si nouveaux rappels/seuils, afficher notification locale.

### 2.3 Local (alarmes)

| Avantage | Inconvénient |
|----------|--------------|
| Autonome | Nécessite sync préalable (données agenda) |
| Pas de serveur push | Rappels uniquement si données en cache |

**Flow :** À la sync, calculer alarmes pour événements ; planifier `AlarmManager` Android. Au déclenchement : afficher notification.

### 2.4 Recommandation

- **MVP :** Local (alarmes) pour rappels JayKoa ; seuils JayKonta au prochain sync (affichage in-app).
- **Phase 2 :** FCM si besoin de notifications push temps réel.

---

## 3. Permissions Android

| Permission | Usage |
|------------|-------|
| POST_NOTIFICATIONS (API 33+) | Afficher notifications |
| VIBRATE | Vibrer (optionnel) |
| RECEIVE_BOOT_COMPLETED | Replanifier alarmes après redémarrage |

---

## 4. Design notifications

### 4.1 Style

| Élément | Valeur |
|---------|--------|
| Icône | App ou service (JayKoa, JayKonta) |
| Titre | Court (ex. "Rappel JayKoa") |
| Contenu | Message détaillé |
| Canal | Canal dédié (priorité selon type) |

### 4.2 Actions

| Action | Description |
|--------|-------------|
| Ouvrir | Ouvre l'app sur l'écran concerné (event, purse) |
| Dissiper | Ferme |

### 4.3 Priorité

| Type | Priorité |
|------|----------|
| Rappel imminent | Haute |
| Seuil solde | Moyenne |
| Sync | Basse |

---

## 5. Références

- [Spec Services Consultatifs](./MiyukiniTerminal%20-%20Spec%20Services%20Consultatifs.md)
- [Android Notifications](https://developer.android.com/develop/ui/views/notifications)
