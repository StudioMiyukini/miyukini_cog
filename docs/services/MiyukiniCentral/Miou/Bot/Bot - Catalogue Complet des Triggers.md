# Bot Miou â€” Catalogue Complet des Triggers

Document exhaustif recensant **tous les dÃ©clencheurs** et **moments** qui peuvent provoquer une tentative de gÃ©nÃ©ration de bulle par Miou. Chaque trigger est dÃ©crit avec sa condition, sa prioritÃ©, sa frÃ©quence et ses exclusions.

---

## 1. Vue d'ensemble

Un **trigger** est un Ã©vÃ©nement ou une condition qui lance le cycle : Constructeur de contexte â†’ Moteur de dÃ©cision â†’ SÃ©lecteur de variante â†’ Bulle.

| Type | Description | Exemple |
|------|-------------|---------|
| **Temps** | Timer, dÃ©lai Ã©coulÃ© | 2â€“3 s aprÃ¨s arrivÃ©e Salon, 30 min de session |
| **Ã‰vÃ©nement** | Action utilisateur ou systÃ¨me | Connexion, ouverture onglet, badge dÃ©bloquÃ© |
| **Ã‰tat** | Condition dÃ©rivÃ©e du contexte | Session > 2h, Ã©vÃ©nement < 1h |
| **PÃ©riodique** | VÃ©rification Ã  intervalle fixe | Toutes les 30 min pendant la session |

---

## 2. Triggers temporels

### 2.1 DÃ©marrage de session

| ID | Nom | Moment | PrioritÃ© | Condition supplÃ©mentaire |
|----|-----|--------|----------|---------------------------|
| T-01 | ArrivÃ©e Salon | 2â€“3 secondes aprÃ¨s affichage du Salon | Haute | `is_first_connection_of_session` |
| T-02 | Retour onglet Salon | 1â€“2 s aprÃ¨s retour sur Salon (depuis un autre onglet) | Basse | Pas de bulle si une bulle affichÃ©e < 1 min |

### 2.2 Pendant la session

| ID | Nom | FrÃ©quence | PrioritÃ© | Condition |
|----|-----|-----------|----------|-----------|
| T-10 | VÃ©rification pause | Toutes les 30 min | Haute | `session_duration >= seuil_pause` |
| T-11 | VÃ©rification dÃ©bit | Avant toute gÃ©nÃ©ration | SystÃ¨me | `bulles_affichees < max_bulles` |
| T-12 | DÃ©lai minimum | 30 s (configurable) | SystÃ¨me | `now - last_bulle >= delai_min` |
| T-13 | Fin de session imminente | 30 s avant fermeture dÃ©tectÃ©e | Basse | Optionnel, rÃ©sumÃ© activitÃ© |

### 2.3 CrÃ©neaux horaires

| ID | Nom | Plage | Usage |
|----|-----|-------|-------|
| T-20 | Matin | 06:00 â€“ 11:59 | CatÃ©gorie `accueil_matin` |
| T-21 | AprÃ¨s-midi | 12:00 â€“ 17:59 | CatÃ©gorie `accueil_apres_midi` |
| T-22 | Soir | 18:00 â€“ 22:59 | CatÃ©gorie `accueil_soir` |
| T-23 | Nuit | 23:00 â€“ 05:59 | CatÃ©gorie `accueil_nuit` (si implÃ©mentÃ©) |

---

## 3. Triggers Ã©vÃ©nementiels

### 3.1 Cycle de vie utilisateur

| ID | Nom | Ã‰vÃ©nement | PrioritÃ© | CatÃ©gorie rÃ©sultante |
|----|-----|-----------|----------|----------------------|
| E-01 | Connexion rÃ©ussie | `user_logged_in` | P0 | Accueil ou Retour (selon jours_absent) |
| E-02 | DÃ©connexion | `user_logged_out` | â€” | Pas de bulle (session terminÃ©e) |
| E-03 | Premier lancement COG | `rite_entree_completed` | P0 | Bulle emmÃ©nagement (hors Bot bulles) |

### 3.2 Navigation et usage

| ID | Nom | Ã‰vÃ©nement | PrioritÃ© | Condition |
|----|-----|-----------|----------|-----------|
| E-10 | Changement d'onglet | `tab_changed` (Salon, BibliothÃ¨que, Webway) | Basse | Max 1 bulle / 2 min par ce trigger |
| E-11 | Ouverture service | `service_opened` | Basse | Pas de bulle immÃ©diate â€” donnÃ©es pour contexte |
| E-12 | Fermeture service | `service_closed` | â€” | Contexte uniquement |
| E-13 | Retour sur Salon | AprÃ¨s consultation d'un service | Basse | Peut dÃ©clencher suggestion ou rÃ©sumÃ© |

### 3.3 Gamification et jalons

| ID | Nom | Ã‰vÃ©nement | PrioritÃ© | CatÃ©gorie |
|----|-----|-----------|----------|-----------|
| E-20 | Badge dÃ©bloquÃ© | `badge_unlocked` | P1 | `felicitation_badge` |
| E-21 | Streak atteint | `streak_milestone` (7, 30 jours) | P1 | `felicitation_streak` |
| E-22 | Premier service installÃ© | `first_service_installed` | P1 | `felicitation_milestone` |
| E-23 | Premier ami contactÃ© | `first_friend_contact` | P1 | `felicitation_milestone` |
| E-24 | Webway connectÃ© | `webway_connected` | P1 | `felicitation_badge` |
| E-25 | Vitrine publiÃ©e | `vitrine_published` | P2 | `felicitation_badge` |

### 3.4 Services externes (contexte applicatif)

| ID | Nom | Ã‰vÃ©nement | PrioritÃ© | CatÃ©gorie |
|----|-----|-----------|----------|-----------|
| E-30 | Ã‰vÃ©nement JayKoa < 1h | `next_event_in < 1h` | P2 | `rappel_evenement` |
| E-31 | Ami connectÃ© (Jay1Tribu) | `friend_came_online` | P2 | `notification_ami` |
| E-32 | Ami dÃ©connectÃ© | â€” | â€” | Pas de bulle |
| E-33 | Nouvel Ã©vÃ©nement crÃ©Ã© | `event_created` | P3 | Optionnel, Â« Ã‡a se fÃªte ! Â» |

---

## 4. Triggers conditionnels (Ã©tats dÃ©rivÃ©s)

### 4.1 Conditions MiyukiniWatch

| ID | Nom | Condition | PrioritÃ© | CatÃ©gorie |
|----|-----|-----------|----------|-----------|
| C-01 | Session longue | `session_duration >= seuil_pause` | P0 | `pause_sante` |
| C-02 | Retour aprÃ¨s absence | `jours_absent >= 3` + premiÃ¨re connexion | P0 | `retour_absence` |
| C-03 | Ami dÃ©laissÃ© | `ami_plus_delaisse.jours >= 7` | P2 | `rappel_ami` |
| C-04 | Service dÃ©laissÃ© | `service_delaisse.jours >= 14` | P3 | `suggestion_service` |
| C-05 | Peu d'activitÃ© | `sessions_week < 2` + `jours_absent >= 7` | P4 | `encouragement_retour` (si implÃ©mentÃ©) |
| C-06 | Streak en cours | `consecutive_active_days >= 7` | P1 | `felicitation_streak` |
| C-07 | Service favori identifiÃ© | `service_le_plus_utilise` prÃ©sent | P4 | Contexte pour variantes |
| C-08 | ActivitÃ© nocturne | `heure in [23, 0, 1, 2, 3, 4, 5]` | P3 | Ton nocturne |
| C-09 | RAM faible | `ram_available_mb < 512` | P5 | specs_ram_demande |
| C-17 | Stockage faible | `disk_free_gb < 1` | P5 | specs_stockage_demande |
| C-14 | Specs upgradÃ©es | `specs_upgraded_since_last_session` | P4 | specs_upgrade_commentaire |
| C-15 | Question curiositÃ© non posÃ©e | Question du palier non posÃ©e (cooldown selon palier) | P6 | curiosite_utilisateur |
| C-16 | Contexte taquinerie | OS connu, moment lÃ©ger | P6 | taquinerie_innocente |
| C-18 | CritÃ¨res Ã©volution palier | Sessions, jours, rÃ©ponses, qualitÃ©, complicitÃ© â€” voir Registre | P5 | confirmation_relation |
| C-19 | Signalement considÃ©ration | CritÃ¨res Ã  80 % OU cooldown refus | P6 | signalement_evolution_relation |

### 4.2 Conditions profil et prÃ©fÃ©rences

| ID | Nom | Condition | Effet |
|----|-----|-----------|-------|
| C-10 | Bulles dÃ©sactivÃ©es | `miou_prefs.bulles_actives == false` | Aucune bulle |
| C-21 | Mode Ne pas dÃ©ranger | `miou_prefs.dnd_actif == true` | Aucune bulle (prioritÃ© 0). Voir [Miou - Roadmap](..//_index.md). |
| C-11 | FrÃ©quence discrÃ¨te | `miou_prefs.frequence == Discret` | `max_bulles = 2`, `delai_min = 120s` |
| C-12 | FrÃ©quence bavarde | `miou_prefs.frequence == Bavard` | `max_bulles = 10`, `delai_min = 15s` |
| C-13 | Rappels pause dÃ©sactivÃ©s | `miou_prefs.rappels_pause_actives == false` | Exclure `pause_sante` |

### 4.3 Exclusions et cooldowns

| ID | Nom | Condition | Effet |
|----|-----|-----------|-------|
| X-01 | Bulle rÃ©cemment dismissÃ©e | `now - last_dismiss < 5 min` | Pas de nouvelle bulle (sauf P0 force) |
| X-02 | Rappel ami dÃ©jÃ  affichÃ© | MÃªme ami, session courante ou 3 derniers jours | Exclure |
| X-03 | Suggestion service dÃ©jÃ  affichÃ©e | MÃªme service, 7 derniers jours | Exclure |
| X-04 | Badge dÃ©jÃ  annoncÃ© | `badge_id in badge_annonces` | Exclure |
| X-05 | DÃ©bit max atteint | `bulles_affichees >= max_bulles` | Pas de bulle (sauf P0 pause/Ã©vÃ©nement) |
| X-06 | Utilisateur dans Jay1Tribu | Onglet conversation ouvert | Pas de notification ami (redondant) |
| X-07 | Plaisanterie dÃ©jÃ  affichÃ©e | `plaisanteries_session >= 1` | Pas de 2e substitution plaisanterie dans la session |
| X-08 | Demande specs rÃ©cente | `specs_demande_since < 7 jours` | Cooldown 7j entre deux demandes (RAM ou stockage) |
| X-09 | Bulle curiositÃ© rÃ©cente | Question posÃ©e < cooldown (selon palier) | Ne pas reposer la mÃªme question |
| X-10 | Proposition relation refusÃ©e | Refus < 14 jours | Ne pas reproposer Ã©volution |
| X-11 | Proposition relation cette session | DÃ©jÃ  proposÃ© | Max 1 proposition / session |
| X-12 | Signalement cette session | DÃ©jÃ  signalÃ© | Max 1 signalement / session |
| X-13 | Signalement rÃ©cent mÃªme palier | Signalement < 7 jours pour ce palier | Cooldown signalement |

### 4.4 Trigger substitution plaisanterie (P-01)

| ID | Nom | Moment | Condition | CatÃ©gorie rÃ©sultante |
|----|-----|--------|-----------|----------------------|
| P-01 | Substitution plaisanterie | AprÃ¨s sÃ©lection catÃ©gorie Â« lÃ©gÃ¨re Â» | `random() < probabilite_plaisanterie` ET catÃ©gorie âˆˆ {pause_sante, accueil_*, felicitation_streak, retour_*} | `plaisanterie_espiegle` |

**CatÃ©gories Ã©ligibles :** `pause_sante`, `accueil_nuit`, `accueil_matin`, `felicitation_streak`, `retour_absence`, `retour_meme_jour`.

**CatÃ©gories exclues :** `rappel_evenement`, `notification_ami`, `felicitation_badge` (contenu prioritaire).

---

## 5. Ordre d'Ã©valuation des triggers

Lorsqu'un trigger se dÃ©clenche, le Moteur de dÃ©cision Ã©value les **conditions** dans l'ordre de prioritÃ© :

```
1. ContrÃ´les systÃ¨me (dÃ©bit, dÃ©lai, bulles dÃ©sactivÃ©es, **DND** si activÃ© â†’ silence)
2. Pause santÃ© (C-01)
3. Rappel Ã©vÃ©nement (E-30)
4. Notification ami (E-31)
5. Accueil / Retour (T-01 + C-02 vs heure)
6. FÃ©licitation badge (E-20)
7. FÃ©licitation streak (E-21, C-06)
8. Rappel ami (C-03)
9. Suggestion service (C-04)
10. RÃ©sumÃ© activitÃ© (T-13)
11. Silence
```

---

## 6. Matrice Trigger â†’ CatÃ©gorie

| Trigger | CatÃ©gorie(s) possible(s) | DÃ©pendances |
|---------|---------------------------|-------------|
| T-01 (ArrivÃ©e Salon) | accueil_*, retour_absence | Heure, jours_absent |
| T-10 (Timer pause) | pause_sante | session_duration |
| E-20 (Badge) | felicitation_badge | badge_id |
| E-21 (Streak) | felicitation_streak | streak_value |
| E-30 (Ã‰vÃ©nement) | rappel_evenement | event_title |
| E-31 (Ami online) | notification_ami | ami_pseudo |
| C-03 (Ami dÃ©laissÃ©) | rappel_ami | ami, jours |
| C-04 (Service dÃ©laissÃ©) | suggestion_service | service_name |
| P-01 (Substitution) | plaisanterie_espiegle | catÃ©gorie_principale (mapping) |
| C-09 (RAM faible) | specs_ram_demande | ram_available_mb |
| C-17 (Stockage) | specs_stockage_demande | disk_free_gb |
| C-14 (Upgrade) | specs_upgrade_commentaire | specs_previous |
| C-15 (CuriositÃ©) | curiosite_utilisateur | user_responses, last_question |
| C-16 (Taquinerie) | taquinerie_innocente | os_type |
| C-18 (Ã‰volution relation) | confirmation_relation | relation_level, critÃ¨res palier |
| C-19 (Signalement) | signalement_evolution_relation | critÃ¨res 80 %, cooldown |

---

## 7. Triggers Tutoriels (Miou-tuteur)

Miou peut **proposer** ou **rÃ©pondre** Ã  des tutoriels (Central, MWS). Voir [Miou - Moteur Tutoriels et Accompagnement](../Miou%20-%20Moteur%20Tutoriels%20et%20Accompagnement.md).

| ID | Nom | Moment | PrioritÃ© | Condition | Tutoriel dÃ©clenchÃ© |
|----|-----|--------|----------|-----------|--------------------|
| T-T01 | Premier accÃ¨s Salon | 2â€“3 s aprÃ¨s affichage Salon | P1 | `tutoriel_central_vu == false` | `tutoriel_central_intro` |
| T-T02 | Premier accÃ¨s Webway | 2â€“3 s aprÃ¨s affichage Webway | P1 | `tutoriel_mws_vu == false` | `tutoriel_mws_connexion` |
| E-T01 | Demande tutoriel Central | Message utilisateur | P0 | Intention dÃ©tectÃ©e | `tutoriel_central_intro` |
| E-T02 | Demande tutoriel MWS | Message utilisateur | P0 | Intention dÃ©tectÃ©e | `tutoriel_mws_connexion` |

**Intentions utilisateur (dÃ©tection) :**
- Central : Â« Explique-moi Central Â», Â« Comment Ã§a marche ? Â», Â« C'est quoi Central ? Â», Â« Guide-moi Â»
- MWS : Â« Comment me connecter ? Â», Â« C'est quoi le Webway ? Â», Â« Comment rejoindre le MWS ? Â», Â« Explique le rÃ©seau Â»

---

## 8. Triggers futurs (extension)

| ID | Nom | Condition | CatÃ©gorie | PrioritÃ© implÃ©mentation |
|----|-----|-----------|-----------|--------------------------|
| F-01 | Saison (NoÃ«l, Ã©tÃ©) | `date in plage_saison` (NoÃ«l 20â€“26 dÃ©c, Nouvel An 30 dÃ©câ€“2 jan, Ã‰tÃ© 21 juinâ€“21 sept, RentrÃ©e 1â€“15 sept) | accueil_saison | P3 |
| F-02 | Anniversaire COG | 1 an depuis Rite | felicitation_anniversaire | P3 |
| F-03 | Premier Ã©vÃ©nement du mois | `events_this_month == 1` | encouragement | P3 |
| F-04 | Objectif personnel (futur) | User-defined | custom_reminder | P4 |
| F-05 | Mode Ne pas dÃ©ranger | `dnd_actif == true` | Aucune bulle (contrÃ´le prioritÃ© 0) | P2 |

**F-01 dÃ©tail :** Si premiÃ¨re connexion ET date dans plage â†’ catÃ©gorie `accueil_saison` (prioritÃ© entre accueil standard et saison). Voir [Miou - Roadmap et AmÃ©liorations](..//_index.md).

---

## 9. RÃ©fÃ©rences

- [Bot - Moteur de DÃ©cision et RÃ¨gles](./Bot%20-%20Moteur%20de%20Decision%20et%20Regles.md)
- [Bot - Document Fondateur et Architecture](./Bot%20-%20Document%20Fondateur%20et%20Architecture.md)

---

*Chaque bulle de Miou a une raison d'Ãªtre. Ce catalogue assure qu'aucun moment pertinent n'est oubliÃ©.*

