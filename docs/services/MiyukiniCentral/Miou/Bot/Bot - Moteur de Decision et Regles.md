# Bot Miou — Moteur de Décision et Règles

Document exhaustif décrivant l'algorithme de décision du Bot, les règles de priorité, les conditions d'évaluation, l'anti-répétition, la gestion des états et les edge cases.

---

## 1. Vue d'ensemble

Le **Moteur de Décision** est le composant central du Bot. Il reçoit un `BotContext` et détermine **quelle catégorie de bulle** afficher (ou s'il faut rester silencieux). Il ne génère pas le texte ; il sélectionne la catégorie. Le Sélecteur de variante et l'Injecteur produisent ensuite le texte final.

### 1.1 Principe de priorité

Les conditions sont évaluées **dans un ordre fixe**. La **première** condition satisfaite détermine la catégorie. Les conditions suivantes ne sont pas évaluées. Si **aucune** condition n'est satisfaite → **silence** (pas de bulle).

### 1.2 Ordre de priorité (décroissant)

| Priorité | Catégorie | Justification |
|----------|-----------|---------------|
| 1 | Pause santé | Bien-être physique immédiat. |
| 2 | Rappel événement | Timing critique (événement dans < 1h). |
| 2b | Notification ami | Interaction sociale en temps réel. |
| 3 | Accueil (matin/après-midi/soir) | Première impression de la session. |
| 4 | Retour après absence | Reconnaissance de l'absence. |
| 5 | Félicitation badge | Célébration, gamification. |
| 6 | Rappel ami | Bien-être relationnel. |
| 7 | Suggestion service | Découverte, réengagement. |
| 8 | Résumé activité | Optionnel, fin de session. |
| 9 | Specs upgrade | Commentaire après upgrade détecté. |
| 10 | Specs demande (RAM/stockage) | Environnement limité. |
| 11 | Taquinerie innocente | Contexte léger, sujet léger (OS, heure). |
| 12 | Curiosité utilisateur | Question pour mieux connaître (réponses stockées, chiffrées). Questions par palier. |
| 13 | Confirmation relation | Proposition d'évolution du palier d'attachement (inconnue → … → grande sœur). |
| 14 | Signalement évolution | Miou informe qu'elle considère un changement (critères à 80 %). Pas de boutons. |
| — | Silence | Aucune condition satisfaite. |

---

## 2. Conditions d'évaluation détaillées

### 2.1 Condition 1 : Pause santé

**Catégorie :** `pause_sante`

**Condition :**
```
session_duration_minutes >= seuil_pause_minutes
```

**Paramètres :**
- `seuil_pause_minutes` : configurable (défaut 120). Valeurs typiques : 60, 120, 180.
- Source : Paramètres Miou > Rappels de pause.

**Exclusions :**
- Si l'utilisateur a déjà vu une bulle pause dans cette session et l'a dismissée depuis moins de 2h → ne pas répéter.
- Si `bulles_deja_affichees >= max_bulles_par_session` → ne pas afficher (respect du débit).

**Variables requises :** `{duree}` (formatée depuis `session_duration_minutes`).

**Exemple :** Session de 2h15, seuil 120 → condition vraie → bulle pause.

---

### 2.2 Condition 2 : Rappel événement

**Catégorie :** `rappel_evenement`

**Condition :**
```
evenement_prochain.is_some() 
AND evenement_prochain.début - now < 1 heure
AND evenement_prochain.début > now
```

**Paramètres :**
- Fenêtre : 1 heure avant l'événement.
- Pas de rappel si l'événement est passé.

**Exclusions :**
- Si une bulle rappel événement pour ce même événement a déjà été affichée dans la session → ne pas répéter.
- Si l'utilisateur a dismissé ce rappel → ne pas réafficher avant 30 min.

**Variables requises :** `{evenement}` (titre), optionnellement `{heure}` (heure de l'événement).

**Source données :** JayKoa (prochain événement du calendrier de l'utilisateur).

---

### 2.3 Condition 2b : Notification ami connecté

**Catégorie :** `notification_ami`

**Condition :**
```
ami_connecte_recemment.is_some()
AND ami_connecte_recemment != ami_de_derniere_notification
```

**Paramètres :**
- Un ami vient de se connecter (événement temps réel ou polling).
- Ne pas notifier pour le même ami deux fois dans la même session.

**Exclusions :**
- Si l'utilisateur est déjà dans Jay1Tribu (conversation ouverte) → pas de notification (éviter redondance).
- Si `bulles_deja_affichees >= max_bulles_par_session` → ne pas afficher.

**Variables requises :** `{ami}` (pseudo de l'ami connecté).

**Source données :** Jay1Tribu (présence, événement « ami en ligne »).

---

### 2.4 Condition 3 : Accueil

**Catégorie :** `accueil_matin` | `accueil_apres_midi` | `accueil_soir`

**Condition :**
```
is_first_connection_of_session == true
AND bulles_deja_affichees == 0
```

**Sous-condition (choix de la sous-catégorie) :**
- Heure locale 6h–12h → `accueil_matin`
- Heure locale 12h–18h → `accueil_apres_midi`
- Heure locale 18h–6h → `accueil_soir`

**Exclusions :**
- Si une bulle a déjà été affichée dans cette session → pas d'accueil (éviter doublon avec une bulle prioritaire qui serait passée avant).

**Variables requises :** `{pseudo}`.

**Exemple :** Première connexion à 14h30 → `accueil_apres_midi`.

---

### 2.5 Condition 4 : Retour après absence

**Catégorie :** `retour_absence`

**Condition :**
```
jours_depuis_derniere_visite.is_some()
AND jours_depuis_derniere_visite >= 3
AND is_first_connection_of_session == true
```

**Paramètres :**
- Seuil minimal : 3 jours. En dessous, pas de bulle « retour » (l'utilisateur revient souvent).
- Ne s'affiche qu'à la première connexion de la session (pas en milieu de session).

**Exclusions :**
- Si une bulle accueil a déjà été affichée (même session) → la bulle retour peut être fusionnée ou prioritaire selon le design. Règle actuelle : accueil et retour sont mutuellement exclusifs par « première connexion ». Si `jours_depuis_derniere_visite >= 3`, on préfère `retour_absence` à `accueil` (priorité 4 > 3). Donc : si les deux conditions sont vraies, c'est `retour_absence` qui gagne.

**Variables requises :** `{pseudo}`, `{jours}`.

**Source données :** MiyukiniWatch (dernière session, date de fin).

---

### 2.6 Condition 5 : Félicitation badge

**Catégorie :** `felicitation_badge`

**Condition :**
```
badge_recent_non_annonce.is_some()
```

**Paramètres :**
- Un badge a été débloqué et n'a pas encore été annoncé dans une bulle.
- Le badge est « consommé » après annonce (marqué comme annoncé).

**Exclusions :**
- Si une bulle de priorité plus haute est en attente → la félicitation peut être différée (file d'attente).
- Si `bulles_deja_affichees >= max_bulles_par_session` → reporter à la prochaine session (le badge restera « non annoncé »).

**Variables requises :** `{badge}`.

**Source données :** Module gamification (détection de déblocage, état « annoncé »).

---

### 2.7 Condition 6 : Rappel ami

**Catégorie :** `rappel_ami`

**Condition :**
```
ami_plus_delaisse.is_some()
AND ami_plus_delaisse.jours >= 7
```

**Paramètres :**
- Seuil : 7 jours sans contact.
- `ami_plus_delaisse` : l'ami avec le plus grand délai depuis dernière discussion.

**Exclusions :**
- Si une bulle rappel ami pour ce même ami a été affichée dans les 7 derniers jours (persisté) → ne pas répéter.
- Si l'utilisateur a dismissé ce rappel récemment → attendre 3 jours avant de reproposer (configurable).
- Si `bulles_deja_affichees >= max_bulles_par_session` → ne pas afficher.

**Variables requises :** `{ami}`, `{jours}`.

**Source données :** MiyukiniWatch (amis, dernière discussion).

---

### 2.8 Condition 7 : Suggestion service

**Catégorie :** `suggestion_service`

**Condition :**
```
service_delaisse.is_some()
AND service_delaisse.jours >= 14
```

**Paramètres :**
- Seuil : 14 jours sans ouvrir le service.
- `service_delaisse` : le service avec le plus grand délai depuis dernière ouverture (parmi ceux installés).

**Exclusions :**
- Si une bulle suggestion service pour ce même service a été affichée dans les 14 derniers jours → ne pas répéter.
- Si l'utilisateur a dismissé ce rappel → attendre 7 jours avant de reproposer.
- Si `bulles_deja_affichees >= max_bulles_par_session` → ne pas afficher.

**Variables requises :** `{service}` (nom affiché du service, ex. « JayKoa »).

**Source données :** MiyukiniWatch (services, dernière ouverture).

---

### 2.9 Condition 8 : Résumé activité

**Catégorie :** `resume_activite`

**Condition :**
```
session_duration_minutes >= 30
AND service_le_plus_utilise.is_some()
AND (optionnel) fin_de_session_imminente OU timer_spécifique
```

**Paramètres :**
- Cette catégorie est **optionnelle** et **basse priorité**. Elle peut être désactivée par défaut.
- Déclencheur : plutôt en fin de session (avant fermeture) ou après 30 min de session.
- Nécessite des données de session (temps total, service top).

**Exclusions :**
- Si déjà une bulle résumé dans la session → ne pas répéter.
- Souvent désactivée pour éviter la surcharge.

**Variables requises :** `{temps_total}`, `{service_top}`.

**Source données :** MiyukiniWatch (session courante, services utilisés).

---

### 2.10 Silence

**Condition :** Aucune des conditions 1 à 8 n'est satisfaite.

**Comportement :** Le moteur retourne `None` (pas de bulle). Le composant UI ne fait rien.

---

## 3. Graphe de décision (diagramme)

```
                    ┌─────────────────┐
                    │  BotContext     │
                    └────────┬────────┘
                             │
                             ▼
                    ┌─────────────────┐
                    │ durée_session   │
                    │ >= seuil_pause? │
                    └────────┬────────┘
                             │
              ┌──────────────┼──────────────┐
              │ Oui          │              │ Non
              ▼              │              │
       ┌──────────────┐      │              │
       │ pause_sante  │      │              │
       └──────────────┘      │              │
                             │              ▼
                             │     ┌─────────────────┐
                             │     │ événement < 1h? │
                             │     └────────┬────────┘
                             │              │
                             │   ┌──────────┼──────────┐
                             │   │ Oui      │          │ Non
                             │   ▼          │          │
                             │ ┌────────────┐          │
                             │ │rappel_evt  │          │
                             │ └────────────┘          │
                             │              │          ▼
                             │              │   ┌─────────────────┐
                             │              │   │ 1ère connexion? │
                             │              │   └────────┬────────┘
                             │              │            │
                             │              │  ┌────────┼────────┐
                             │              │  │ Oui   │        │ Non
                             │              │  ▼       │        │
                             │              │ ┌────────┴──┐      │
                             │              │ │ accueil   │      │
                             │              │ │ (heure)   │      │
                             │              │ └───────────┘      │
                             │              │            │      ▼
                             │              │            │  ... (suite)
                             │              │            │
                             │              │            ▼
                             │              │     ┌─────────────┐
                             │              │     │  silence    │
                             │              │     └─────────────┘
```

*(Le graphe complet serait très long ; l'ordre séquentiel des conditions suffit pour l'implémentation.)*

---

## 4. Anti-répétition

### 4.1 Objectif

Éviter qu'un utilisateur voie la **même variante** trop souvent. Varier les phrases pour maintenir l'impression que Miou est vivante et attentive.

### 4.2 Niveaux d'anti-répétition

| Niveau | Périmètre | Règle |
|--------|-----------|-------|
| **Session** | Session courante | Une variante affichée ne peut pas être réutilisée dans la même session. |
| **Sessions récentes** | 3 dernières sessions | Les variantes affichées dans les 3 dernières sessions sont évitées (si possible). |
| **Catégorie** | Par catégorie | L'anti-répétition est indépendante par catégorie. Une variante accueil_matin n'affecte pas accueil_soir. |

### 4.3 Algorithme du Sélecteur de variante

```
Entrée : categorie, liste_variantes, historique_session, historique_3_sessions
Sortie : variante_id

1. variantes_disponibles = liste_variantes
2. Exclure de variantes_disponibles celles dans historique_session pour cette catégorie
3. Exclure de variantes_disponibles celles dans historique_3_sessions pour cette catégorie
4. Si variantes_disponibles est vide :
     4a. Si LLM activé et disponible : déléguer au LLM
     4b. Sinon : variantes_disponibles = [première variante de la catégorie]
5. Choisir aléatoirement une variante dans variantes_disponibles
6. Retourner variante_id
```

### 4.4 Persistance de l'historique

| Donnée | Stockage | Durée |
|--------|----------|-------|
| Historique session | Mémoire (AppState ou équivalent) | Durée de la session |
| Historique 3 sessions | KindMother ou fichier local | 3 sessions (ou 7 jours max) |

Structure :
```json
{
  "session_id": "uuid",
  "timestamp": "ISO8601",
  "entries": [
    { "categorie": "accueil_matin", "variante_id": "am3" },
    { "categorie": "pause_sante", "variante_id": "ps2" }
  ]
}
```

### 4.5 Cas limites

| Cas | Comportement |
|-----|--------------|
| Une seule variante dans la catégorie | Toujours utiliser cette variante. Pas d'anti-répétition possible. |
| Toutes les variantes épuisées | Réutiliser la première (ou déléguer au LLM). |
| Historique corrompu ou vide | Considérer toutes les variantes comme disponibles. |
| Nouvelle variante ajoutée | Elle est immédiatement disponible (pas dans l'historique). |

### 4.6 Substitution plaisanterie (catégorie `plaisanterie_espiegle`)

Après avoir déterminé la catégorie principale, le moteur peut **substituer** avec une plaisanterie espiègle pour marquer la présence de Miou.

**Condition de substitution :**
```
categorie_principale IN {pause_sante, accueil_nuit, felicitation_streak, retour_absence, retour_meme_jour, accueil_matin}
AND random() < 0.10   // 10 % de chances
AND plaisanteries_session < 1   // Max 1 plaisanterie par session (éviter surcharge)
```

**Mapping catégorie → IDs plaisanterie :** Voir [Banque de Templates Volume 2 - Mapping contextuel](./Bot%20-%20Banque%20de%20Templates%20Volume%202.md). Ex. : pour `pause_sante` → pe1, pe3, pe7, pe9, pe17, pe18, pe23, pe25.

**Règles :**
- Ne **jamais** substituer pour : `rappel_evenement`, `notification_ami`, `felicitation_badge` (contenu critique).
- Si substitution activée : ignorer la variante de la catégorie principale, piocher dans `plaisanterie_espiegle` selon le mapping.
- Anti-répétition : même logique que les autres catégories (éviter peX déjà affiché cette session).

**Paramètre :** `probabilite_plaisanterie` (défaut 0.10, configurable 0–0.25).

### 4.7 Conditions specs et connaissance utilisateur

Voir [Bot - Connaissance Utilisateur et Specs Machine](./Bot%20-%20Connaissance%20Utilisateur%20et%20Specs%20Machine.md) pour les conditions détaillées :
- **Specs RAM/stockage** : Priorité P5, cooldown 7 jours
- **Specs upgrade** : Priorité P4
- **Taquinerie innocente** : Priorité P6
- **Curiosité utilisateur** : Priorité P6, cooldown par palier (voir Registre)
- **Confirmation relation** : Priorité P5, critères par palier (quantité, qualité, pertinence, complicité), cooldown 14 jours après refus, max 1 proposition / session
- **Signalement évolution** : Priorité P6, critères à 80 % ou cooldown refus, max 1 signalement / session, cooldown 7 jours par palier

---

## 5. Gestion des conflits

### 5.1 Conflit accueil vs retour

**Conflit :** Première connexion de la session ET `jours_depuis_derniere_visite >= 3`.

**Résolution :** Priorité à `retour_absence` (priorité 4 > 3). L'utilisateur revient après une absence ; c'est plus pertinent que un simple « bonjour ».

### 5.2 Conflit pause vs rappel événement

**Conflit :** Session > 2h ET événement dans < 1h.

**Résolution :** Priorité à `rappel_evenement` (priorité 2 > 1). L'événement est plus urgent (timing critique). La pause peut attendre la prochaine évaluation (30 min plus tard).

### 5.3 Conflit multiple amis délaissés

**Conflit :** Plusieurs amis non contactés depuis > 7 jours.

**Résolution :** Choisir `ami_plus_delaisse` (celui avec le plus grand délai). Une seule bulle à la fois. Les autres seront proposés dans les sessions suivantes (rotation).

### 5.4 Conflit débit (trop de bulles)

**Conflit :** Plusieurs conditions vraies, mais `bulles_deja_affichees >= max_bulles_par_session`.

**Résolution :** Ne pas afficher de nouvelle bulle. Les conditions restent vraies ; à la prochaine session, elles seront réévaluées. Exception : `pause_sante` et `rappel_evenement` peuvent avoir une priorité « force » qui ignore le débit (à définir : peut-être 1 bulle pause par session autorisée même au-delà du max).

---

## 6. Seuils et paramètres configurables

### 6.1 Liste des paramètres

| Paramètre | Défaut | Min | Max | Description |
|-----------|--------|-----|-----|-------------|
| `seuil_pause_minutes` | 120 | 30 | 480 | Durée de session avant suggestion de pause. |
| `seuil_jours_absence` | 3 | 1 | 30 | Jours sans connexion pour bulle « retour ». |
| `seuil_jours_ami` | 7 | 3 | 60 | Jours sans contact ami pour rappel. |
| `seuil_jours_service` | 14 | 7 | 90 | Jours sans ouvrir service pour suggestion. |
| `fenetre_rappel_evenement_minutes` | 60 | 15 | 120 | Fenêtre avant événement pour rappel. |
| `max_bulles_par_session` | 5 | 1 | 15 | Nombre max de bulles par session. |
| `delai_min_entre_bulles_secondes` | 30 | 10 | 300 | Délai minimum entre deux bulles. |
| `historique_sessions_anti_repetition` | 3 | 1 | 10 | Nombre de sessions pour anti-répétition. |
| `probabilite_plaisanterie` | 0.10 | 0 | 0.25 | Chance de substituer par une plaisanterie espiègle (catégories légères). |

### 6.2 Profils de fréquence

| Profil | max_bulles_par_session | delai_min_entre_bulles |
|--------|------------------------|------------------------|
| Discrète | 2 | 120 |
| Normale | 5 | 30 |
| Bavarde | 10 | 15 |

---

## 7. Edge cases et comportements

### 7.1 Contexte vide (première installation)

**Situation :** Aucune donnée MiyukiniWatch, profil minimal (pseudo peut-être vide).

**Comportement :** 
- `is_first_connection_of_session` = true.
- Heure disponible → accueil (matin/après-midi/soir).
- `{pseudo}` = "toi" ou "habitant" si vide.
- Toutes les autres conditions sont fausses (pas de jours, pas de services, etc.).

### 7.2 MiyukiniWatch désactivé

**Situation :** L'utilisateur a désactivé la collecte MiyukiniWatch.

**Comportement :**
- `jours_depuis_derniere_visite` = None.
- `service_delaisse` = None.
- `ami_plus_delaisse` = None.
- `session_duration_minutes` = peut être estimé par Central (timer local) ou = 0.
- Seules les conditions accueil, rappel événement (JayKoa), notification ami (Jay1Tribu), félicitation badge restent évaluables.
- Dégradation gracieuse : moins de bulles, mais pas de crash.

### 7.3 Session très courte (< 1 min)

**Situation :** L'utilisateur ouvre Central et ferme immédiatement.

**Comportement :**
- Une bulle accueil peut être générée (priorité 3).
- Pas de bulle pause (session trop courte).
- Si l'utilisateur ferme avant l'affichage de la bulle, pas de problème (la bulle n'est jamais rendue).

### 7.4 Session très longue (> 8h)

**Situation :** L'utilisateur reste connecté toute la journée.

**Comportement :**
- Bulle pause à 2h, 4h, 6h... selon `seuil_pause_minutes` et règle « pas de répétition pause avant 2h ».
- Si max_bulles_par_session atteint, plus de bulles pause (sauf si règle « force » pour pause).
- Recommandation : autoriser 1 bulle pause supplémentaire par tranche de 2h au-delà du max (santé prioritaire).

### 7.5 Changement d'heure (été/hiver, voyage)

**Situation :** L'utilisateur change de fuseau ou l'heure système change.

**Comportement :**
- Utiliser l'heure locale actuelle pour accueil (matin/après-midi/soir).
- Pas de persistance de l'heure ; toujours recalculer.
- Pas d'edge case particulier.

### 7.6 Données incohérentes

**Situation :** `jours_depuis_derniere_visite` = 0 mais `is_first_connection_of_session` = true.

**Comportement :**
- Incohérence possible si MiyukiniWatch n'a pas encore enregistré la session précédente.
- Utiliser les données telles quelles. Si `jours` = 0, la condition `retour_absence` (>= 3) est fausse.
- Accueil s'affichera.

### 7.7 Service ou ami supprimé

**Situation :** Un service a été désinstallé ou un ami retiré entre deux sessions.

**Comportement :**
- Le Constructeur de contexte ne doit pas inclure les services désinstallés ni les amis retirés.
- Filtrer en amont. Si `service_delaisse` pointe vers un service inexistant, ignorer la condition.

---

## 8. Diagramme d'états (session)

```
                    ┌─────────────┐
                    │  Début      │
                    │  session    │
                    └──────┬──────┘
                           │
                           ▼
                    ┌─────────────┐
                    │ Attente     │
                    │ déclencheur │
                    └──────┬──────┘
                           │
         ┌─────────────────┼─────────────────┐
         │                 │                 │
         ▼                 ▼                 ▼
  ┌─────────────┐   ┌─────────────┐   ┌─────────────┐
  │ Timer 2-3s │   │ Événement   │   │ Timer 30min │
  │ (accueil)  │   │ externe     │   │ (pause)     │
  └──────┬──────┘   └──────┬──────┘   └──────┬──────┘
         │                 │                 │
         └─────────────────┴─────────────────┘
                           │
                           ▼
                    ┌─────────────┐
                    │ Génération │
                    │ bulle      │
                    └──────┬──────┘
                           │
              ┌────────────┼────────────┐
              │            │            │
              ▼            ▼            ▼
       ┌──────────┐  ┌──────────┐  ┌──────────┐
       │ Bulle    │  │ Silence  │  │ File     │
       │ affichée│  │ (aucune  │  │ d'attente│
       │          │  │ condition)│  │          │
       └────┬─────┘  └────┬─────┘  └────┬─────┘
            │              │              │
            │              │              │
            └──────────────┴──────────────┘
                           │
                           ▼
                    ┌─────────────┐
                    │ Historique │
                    │ mis à jour │
                    └──────┬──────┘
                           │
                           ▼
                    ┌─────────────┐
                    │ Attente    │
                    │ prochain   │
                    │ déclencheur│
                    └─────────────┘
```

---

## 9. Tests et validation

### 9.1 Tests unitaires du moteur

| Test | Entrée | Sortie attendue |
|------|--------|-----------------|
| Pause prioritaire | session 3h, reste vide | pause_sante |
| Événement prioritaire | session 3h, événement 30min | rappel_evenement |
| Accueil première connexion | première connexion, 10h | accueil_matin |
| Retour après 5 jours | première connexion, jours 5 | retour_absence |
| Silence | session 15min, pas d'événement, pas d'absence | silence |
| Débit max atteint | 5 bulles déjà affichées, condition pause vraie | silence (ou pause si force) |

### 9.2 Tests d'intégration

| Scénario | Étapes | Vérification |
|----------|--------|--------------|
| Session type | Connexion → 2 min → 2h | Accueil puis pause. |
| Retour après absence | Connexion (5 jours absent) | Retour avec « 5 jours ». |
| Anti-répétition | 5 connexions rapides (même catégorie) | 5 variantes différentes. |
| MiyukiniWatch désactivé | Connexion, Watch off | Accueil uniquement. |

### 9.3 Tests de non-régression

- Snapshot des seuils par défaut.
- Snapshot de l'ordre de priorité.
- Vérifier qu'aucune condition ne bloque (timeout, boucle infinie).

---

## 11. Pseudo-code de l'algorithme principal

```
Fonction DECIDER_BULLE(context: BotContext) -> Option<Categorie>
    // Vérification débit
    si context.bulles_deja_affichees >= context.max_bulles_par_session:
        retourner None  // ou exception pour pause/événement si force
    
    // Vérification délai minimum
    si maintenant - context.last_bulle_timestamp < context.delai_min_entre_bulles:
        retourner None  // mettre en file d'attente ou ignorer
    
    // Évaluation séquentielle
    si context.session_duration_minutes >= context.seuil_pause_minutes:
        et pas pause_deja_affichee_cette_session:
        retourner Some(pause_sante)
    
    si context.evenement_prochain existe et dans moins d'1h:
        et pas rappel_evt_deja_affiche:
        retourner Some(rappel_evenement)
    
    si context.ami_connecte_recemment existe:
        retourner Some(notification_ami)
    
    si context.is_first_connection_of_session et context.bulles_deja_affichees == 0:
        heure = maintenant.heure
        si 6 <= heure < 12: retourner Some(accueil_matin)
        si 12 <= heure < 18: retourner Some(accueil_apres_midi)
        sinon: retourner Some(accueil_soir)
    
    si context.is_first_connection_of_session et context.jours_depuis_derniere_visite >= 3:
        retourner Some(retour_absence)
    
    si context.badge_recent_non_annonce existe:
        retourner Some(felicitation_badge)
    
    si context.ami_plus_delaisse existe et context.ami_plus_delaisse.jours >= 7:
        retourner Some(rappel_ami)
    
    si context.service_delaisse existe et context.service_delaisse.jours >= 14:
        retourner Some(suggestion_service)
    
    si context.session_duration_minutes >= 30 et resume_activite_autorise:
        retourner Some(resume_activite)
    
    retourner None  // silence
```

---

## 12. File d'attente des bulles (optionnel)

Si plusieurs conditions sont vraies simultanément mais le débit limite l'affichage, une **file d'attente** peut être utilisée :

| Priorité | Catégorie | En attente |
|----------|-----------|------------|
| 1 | Pause, Rappel événement | Affichage prioritaire (peut dépasser max_bulles) |
| 2 | Notification ami | File |
| 3 | Accueil, Retour | File |
| 4 | Félicitation, Rappel ami, Suggestion | File |
| 5 | Résumé | File (basse priorité) |

**Comportement :** Quand une bulle est dismissée ou disparaît, la prochaine bulle de la file (priorité la plus haute) est affichée après `delai_min_entre_bulles`. La file est vidée à la fin de la session.

---

## 13. Références

- [Bot - Document Fondateur et Architecture](./Bot%20-%20Document%20Fondateur%20et%20Architecture.md)
- [Bot - Banque de Templates](./Bot%20-%20Banque%20de%20Templates.md)
- [Bot - Intégration et Flux de Données](./Bot%20-%20Integration%20et%20Flux%20de%20Donnees.md)

---

*Moteur de décision : priorité, conditions, anti-répétition. Chaque bulle est le fruit d'une évaluation rigoureuse et bienveillante.*
