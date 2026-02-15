# Bot Miou — Catalogue Complet des Triggers

Document exhaustif recensant **tous les déclencheurs** et **moments** qui peuvent provoquer une tentative de génération de bulle par Miou. Chaque trigger est décrit avec sa condition, sa priorité, sa fréquence et ses exclusions.

---

## 1. Vue d'ensemble

Un **trigger** est un événement ou une condition qui lance le cycle : Constructeur de contexte → Moteur de décision → Sélecteur de variante → Bulle.

| Type | Description | Exemple |
|------|-------------|---------|
| **Temps** | Timer, délai écoulé | 2–3 s après arrivée Salon, 30 min de session |
| **Événement** | Action utilisateur ou système | Connexion, ouverture onglet, badge débloqué |
| **État** | Condition dérivée du contexte | Session > 2h, événement < 1h |
| **Périodique** | Vérification à intervalle fixe | Toutes les 30 min pendant la session |

---

## 2. Triggers temporels

### 2.1 Démarrage de session

| ID | Nom | Moment | Priorité | Condition supplémentaire |
|----|-----|--------|----------|---------------------------|
| T-01 | Arrivée Salon | 2–3 secondes après affichage du Salon | Haute | `is_first_connection_of_session` |
| T-02 | Retour onglet Salon | 1–2 s après retour sur Salon (depuis un autre onglet) | Basse | Pas de bulle si une bulle affichée < 1 min |

### 2.2 Pendant la session

| ID | Nom | Fréquence | Priorité | Condition |
|----|-----|-----------|----------|-----------|
| T-10 | Vérification pause | Toutes les 30 min | Haute | `session_duration >= seuil_pause` |
| T-11 | Vérification débit | Avant toute génération | Système | `bulles_affichees < max_bulles` |
| T-12 | Délai minimum | 30 s (configurable) | Système | `now - last_bulle >= delai_min` |
| T-13 | Fin de session imminente | 30 s avant fermeture détectée | Basse | Optionnel, résumé activité |

### 2.3 Créneaux horaires

| ID | Nom | Plage | Usage |
|----|-----|-------|-------|
| T-20 | Matin | 06:00 – 11:59 | Catégorie `accueil_matin` |
| T-21 | Après-midi | 12:00 – 17:59 | Catégorie `accueil_apres_midi` |
| T-22 | Soir | 18:00 – 22:59 | Catégorie `accueil_soir` |
| T-23 | Nuit | 23:00 – 05:59 | Catégorie `accueil_nuit` (si implémenté) |

---

## 3. Triggers événementiels

### 3.1 Cycle de vie utilisateur

| ID | Nom | Événement | Priorité | Catégorie résultante |
|----|-----|-----------|----------|----------------------|
| E-01 | Connexion réussie | `user_logged_in` | P0 | Accueil ou Retour (selon jours_absent) |
| E-02 | Déconnexion | `user_logged_out` | — | Pas de bulle (session terminée) |
| E-03 | Premier lancement COG | `rite_entree_completed` | P0 | Bulle emménagement (hors Bot bulles) |

### 3.2 Navigation et usage

| ID | Nom | Événement | Priorité | Condition |
|----|-----|-----------|----------|-----------|
| E-10 | Changement d'onglet | `tab_changed` (Salon, Bibliothèque, Webway) | Basse | Max 1 bulle / 2 min par ce trigger |
| E-11 | Ouverture service | `service_opened` | Basse | Pas de bulle immédiate — données pour contexte |
| E-12 | Fermeture service | `service_closed` | — | Contexte uniquement |
| E-13 | Retour sur Salon | Après consultation d'un service | Basse | Peut déclencher suggestion ou résumé |

### 3.3 Gamification et jalons

| ID | Nom | Événement | Priorité | Catégorie |
|----|-----|-----------|----------|-----------|
| E-20 | Badge débloqué | `badge_unlocked` | P1 | `felicitation_badge` |
| E-21 | Streak atteint | `streak_milestone` (7, 30 jours) | P1 | `felicitation_streak` |
| E-22 | Premier service installé | `first_service_installed` | P1 | `felicitation_milestone` |
| E-23 | Premier ami contacté | `first_friend_contact` | P1 | `felicitation_milestone` |
| E-24 | Webway connecté | `webway_connected` | P1 | `felicitation_badge` |
| E-25 | Vitrine publiée | `vitrine_published` | P2 | `felicitation_badge` |

### 3.4 Services externes (contexte applicatif)

| ID | Nom | Événement | Priorité | Catégorie |
|----|-----|-----------|----------|-----------|
| E-30 | Événement JayKoa < 1h | `next_event_in < 1h` | P2 | `rappel_evenement` |
| E-31 | Ami connecté (Jay1Tribu) | `friend_came_online` | P2 | `notification_ami` |
| E-32 | Ami déconnecté | — | — | Pas de bulle |
| E-33 | Nouvel événement créé | `event_created` | P3 | Optionnel, « Ça se fête ! » |

---

## 4. Triggers conditionnels (états dérivés)

### 4.1 Conditions MiyukiniWatch

| ID | Nom | Condition | Priorité | Catégorie |
|----|-----|-----------|----------|-----------|
| C-01 | Session longue | `session_duration >= seuil_pause` | P0 | `pause_sante` |
| C-02 | Retour après absence | `jours_absent >= 3` + première connexion | P0 | `retour_absence` |
| C-03 | Ami délaissé | `ami_plus_delaisse.jours >= 7` | P2 | `rappel_ami` |
| C-04 | Service délaissé | `service_delaisse.jours >= 14` | P3 | `suggestion_service` |
| C-05 | Peu d'activité | `sessions_week < 2` + `jours_absent >= 7` | P4 | `encouragement_retour` (si implémenté) |
| C-06 | Streak en cours | `consecutive_active_days >= 7` | P1 | `felicitation_streak` |
| C-07 | Service favori identifié | `service_le_plus_utilise` présent | P4 | Contexte pour variantes |
| C-08 | Activité nocturne | `heure in [23, 0, 1, 2, 3, 4, 5]` | P3 | Ton nocturne |
| C-09 | RAM faible | `ram_available_mb < 512` | P5 | specs_ram_demande |
| C-17 | Stockage faible | `disk_free_gb < 1` | P5 | specs_stockage_demande |
| C-14 | Specs upgradées | `specs_upgraded_since_last_session` | P4 | specs_upgrade_commentaire |
| C-15 | Question curiosité non posée | Question du palier non posée (cooldown selon palier) | P6 | curiosite_utilisateur |
| C-16 | Contexte taquinerie | OS connu, moment léger | P6 | taquinerie_innocente |
| C-18 | Critères évolution palier | Sessions, jours, réponses, qualité, complicité — voir Registre | P5 | confirmation_relation |
| C-19 | Signalement considération | Critères à 80 % OU cooldown refus | P6 | signalement_evolution_relation |

### 4.2 Conditions profil et préférences

| ID | Nom | Condition | Effet |
|----|-----|-----------|-------|
| C-10 | Bulles désactivées | `miou_prefs.bulles_actives == false` | Aucune bulle |
| C-21 | Mode Ne pas déranger | `miou_prefs.dnd_actif == true` | Aucune bulle (priorité 0). Voir [Miou - Roadmap](../Miou%20-%20Roadmap%20et%20Améliorations.md). |
| C-11 | Fréquence discrète | `miou_prefs.frequence == Discret` | `max_bulles = 2`, `delai_min = 120s` |
| C-12 | Fréquence bavarde | `miou_prefs.frequence == Bavard` | `max_bulles = 10`, `delai_min = 15s` |
| C-13 | Rappels pause désactivés | `miou_prefs.rappels_pause_actives == false` | Exclure `pause_sante` |

### 4.3 Exclusions et cooldowns

| ID | Nom | Condition | Effet |
|----|-----|-----------|-------|
| X-01 | Bulle récemment dismissée | `now - last_dismiss < 5 min` | Pas de nouvelle bulle (sauf P0 force) |
| X-02 | Rappel ami déjà affiché | Même ami, session courante ou 3 derniers jours | Exclure |
| X-03 | Suggestion service déjà affichée | Même service, 7 derniers jours | Exclure |
| X-04 | Badge déjà annoncé | `badge_id in badge_annonces` | Exclure |
| X-05 | Débit max atteint | `bulles_affichees >= max_bulles` | Pas de bulle (sauf P0 pause/événement) |
| X-06 | Utilisateur dans Jay1Tribu | Onglet conversation ouvert | Pas de notification ami (redondant) |
| X-07 | Plaisanterie déjà affichée | `plaisanteries_session >= 1` | Pas de 2e substitution plaisanterie dans la session |
| X-08 | Demande specs récente | `specs_demande_since < 7 jours` | Cooldown 7j entre deux demandes (RAM ou stockage) |
| X-09 | Bulle curiosité récente | Question posée < cooldown (selon palier) | Ne pas reposer la même question |
| X-10 | Proposition relation refusée | Refus < 14 jours | Ne pas reproposer évolution |
| X-11 | Proposition relation cette session | Déjà proposé | Max 1 proposition / session |
| X-12 | Signalement cette session | Déjà signalé | Max 1 signalement / session |
| X-13 | Signalement récent même palier | Signalement < 7 jours pour ce palier | Cooldown signalement |

### 4.4 Trigger substitution plaisanterie (P-01)

| ID | Nom | Moment | Condition | Catégorie résultante |
|----|-----|--------|-----------|----------------------|
| P-01 | Substitution plaisanterie | Après sélection catégorie « légère » | `random() < probabilite_plaisanterie` ET catégorie ∈ {pause_sante, accueil_*, felicitation_streak, retour_*} | `plaisanterie_espiegle` |

**Catégories éligibles :** `pause_sante`, `accueil_nuit`, `accueil_matin`, `felicitation_streak`, `retour_absence`, `retour_meme_jour`.

**Catégories exclues :** `rappel_evenement`, `notification_ami`, `felicitation_badge` (contenu prioritaire).

---

## 5. Ordre d'évaluation des triggers

Lorsqu'un trigger se déclenche, le Moteur de décision évalue les **conditions** dans l'ordre de priorité :

```
1. Contrôles système (débit, délai, bulles désactivées, **DND** si activé → silence)
2. Pause santé (C-01)
3. Rappel événement (E-30)
4. Notification ami (E-31)
5. Accueil / Retour (T-01 + C-02 vs heure)
6. Félicitation badge (E-20)
7. Félicitation streak (E-21, C-06)
8. Rappel ami (C-03)
9. Suggestion service (C-04)
10. Résumé activité (T-13)
11. Silence
```

---

## 6. Matrice Trigger → Catégorie

| Trigger | Catégorie(s) possible(s) | Dépendances |
|---------|---------------------------|-------------|
| T-01 (Arrivée Salon) | accueil_*, retour_absence | Heure, jours_absent |
| T-10 (Timer pause) | pause_sante | session_duration |
| E-20 (Badge) | felicitation_badge | badge_id |
| E-21 (Streak) | felicitation_streak | streak_value |
| E-30 (Événement) | rappel_evenement | event_title |
| E-31 (Ami online) | notification_ami | ami_pseudo |
| C-03 (Ami délaissé) | rappel_ami | ami, jours |
| C-04 (Service délaissé) | suggestion_service | service_name |
| P-01 (Substitution) | plaisanterie_espiegle | catégorie_principale (mapping) |
| C-09 (RAM faible) | specs_ram_demande | ram_available_mb |
| C-17 (Stockage) | specs_stockage_demande | disk_free_gb |
| C-14 (Upgrade) | specs_upgrade_commentaire | specs_previous |
| C-15 (Curiosité) | curiosite_utilisateur | user_responses, last_question |
| C-16 (Taquinerie) | taquinerie_innocente | os_type |
| C-18 (Évolution relation) | confirmation_relation | relation_level, critères palier |
| C-19 (Signalement) | signalement_evolution_relation | critères 80 %, cooldown |

---

## 7. Triggers Tutoriels (Miou-tuteur)

Miou peut **proposer** ou **répondre** à des tutoriels (Central, MWS). Voir [Miou - Moteur Tutoriels et Accompagnement](../Miou%20-%20Moteur%20Tutoriels%20et%20Accompagnement.md).

| ID | Nom | Moment | Priorité | Condition | Tutoriel déclenché |
|----|-----|--------|----------|-----------|--------------------|
| T-T01 | Premier accès Salon | 2–3 s après affichage Salon | P1 | `tutoriel_central_vu == false` | `tutoriel_central_intro` |
| T-T02 | Premier accès Webway | 2–3 s après affichage Webway | P1 | `tutoriel_mws_vu == false` | `tutoriel_mws_connexion` |
| E-T01 | Demande tutoriel Central | Message utilisateur | P0 | Intention détectée | `tutoriel_central_intro` |
| E-T02 | Demande tutoriel MWS | Message utilisateur | P0 | Intention détectée | `tutoriel_mws_connexion` |

**Intentions utilisateur (détection) :**
- Central : « Explique-moi Central », « Comment ça marche ? », « C'est quoi Central ? », « Guide-moi »
- MWS : « Comment me connecter ? », « C'est quoi le Webway ? », « Comment rejoindre le MWS ? », « Explique le réseau »

---

## 8. Triggers futurs (extension)

| ID | Nom | Condition | Catégorie | Priorité implémentation |
|----|-----|-----------|-----------|--------------------------|
| F-01 | Saison (Noël, été) | `date in plage_saison` (Noël 20–26 déc, Nouvel An 30 déc–2 jan, Été 21 juin–21 sept, Rentrée 1–15 sept) | accueil_saison | P3 |
| F-02 | Anniversaire COG | 1 an depuis Rite | felicitation_anniversaire | P3 |
| F-03 | Premier événement du mois | `events_this_month == 1` | encouragement | P3 |
| F-04 | Objectif personnel (futur) | User-defined | custom_reminder | P4 |
| F-05 | Mode Ne pas déranger | `dnd_actif == true` | Aucune bulle (contrôle priorité 0) | P2 |

**F-01 détail :** Si première connexion ET date dans plage → catégorie `accueil_saison` (priorité entre accueil standard et saison). Voir [Miou - Roadmap et Améliorations](../Miou%20-%20Roadmap%20et%20Améliorations.md).

---

## 9. Références

- [Bot - Moteur de Décision et Règles](./Bot%20-%20Moteur%20de%20Decision%20et%20Regles.md)
- [Bot - Document Fondateur et Architecture](./Bot%20-%20Document%20Fondateur%20et%20Architecture.md)

---

*Chaque bulle de Miou a une raison d'être. Ce catalogue assure qu'aucun moment pertinent n'est oublié.*
