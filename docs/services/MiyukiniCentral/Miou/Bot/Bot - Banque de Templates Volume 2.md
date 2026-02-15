# Bot Miou — Banque de Templates Volume 2

Ce document **complète** la [Banque de Templates](./Bot%20-%20Banque%20de%20Templates.md) principale avec des catégories additionnelles, des variantes enrichies et des templates pour des situations plus fines. Il ne remplace pas le Volume 1 mais l'étend.

---

## 1. Nouvelles catégories

### 1.1 Accueil nuit (`accueil_nuit`)

**Variables :** `{pseudo}`  
**Déclencheur :** Première connexion entre 23h et 6h.  
**Ton :** Prévenant, doux, pas d'enthousiasme excessif.

| ID | Template |
|----|----------|
| an1 | Bonsoir {pseudo}. Il est tard — pense à toi. |
| an2 | {pseudo}, une petite visite de nuit ? N'oublie pas de te reposer. |
| an3 | Encore debout ? Ton COG sera là demain matin. |
| an4 | Hey {pseudo}. Il est tard — une session courte ? |
| an5 | Bonsoir {pseudo}. Contente de te voir, mais repose-toi quand même. |
| an6 | {pseudo}, la nuit est avancée. À demain pour de bonnes nouvelles. |
| an7 | Une visite nocturne ? Prends soin de toi. |

---

### 1.2 Accueil saison (`accueil_saison`)

**Variables :** `{pseudo}`  
**Déclencheur :** Trigger F-01 — première connexion ET date dans plage (Noël 20–26 déc, Nouvel An 30 déc–2 jan, Été 21 juin–21 sept, Rentrée 1–15 sept).  
**Ton :** Festif ou saisonnier, jamais forcé. Voir [Miou - Roadmap et Améliorations](../Miou%20-%20Roadmap%20et%20Améliorations.md).

| ID | Période | Template |
|----|---------|----------|
| as1 | Noël | Joyeux Noël, {pseudo} ! Le Salon t'attend. |
| as2 | Noël | Bonnes fêtes, {pseudo}. |
| as3 | Nouvel An | Bonne année, {pseudo} ! |
| as4 | Nouvel An | {pseudo}, une nouvelle année commence. Ton COG est prêt. |
| as5 | Été | L'été est là. Une session au frais, {pseudo} ? |
| as6 | Été | Salut {pseudo}. Les beaux jours — profite bien. |
| as7 | Rentrée | La rentrée — ton COG est prêt pour la suite, {pseudo}. |
| as8 | Rentrée | {pseudo}, nouvelle saison. Bienvenue. |

---

### 1.3 Félicitation streak (`felicitation_streak`)

**Variables :** `{pseudo}`, `{jours}`  
**Déclencheur :** `consecutive_active_days >= 7` (ou 30).  
**Ton :** Célébrant, complice.

| ID | Template |
|----|----------|
| fs1 | {jours} jours d'affilée avec ton COG — bravo, {pseudo} ! |
| fs2 | Tu es fidèle : {jours} jours de suite. J'apprécie. |
| fs3 | {jours} jours sans t'absenter — ton COG te rend visite. |
| fs4 | {pseudo}, {jours} jours consécutifs. Belle régularité. |
| fs5 | Ça fait {jours} jours que tu viens — c'est devenu un rituel. |
| fs6 | {jours} jours avec ton COG. Tu prends tes marques. |
| fs7 | Streak de {jours} jours — tu tiens la route, {pseudo}. |
| fs8 | Un mois ensemble ? {jours} jours, c'est une habitude bien installée. |
| fs9 | {jours} jours d'affilée. Ton Salon a de la chance. |
| fs10 | {pseudo}, {jours} jours. Tu es un habitant assidu. |

---

### 1.4 Retour même jour (`retour_meme_jour`)

**Variables :** `{pseudo}`  
**Déclencheur :** Connexion, `jours_absent == 0` (déjà passé aujourd'hui).  
**Ton :** Légèrement enthousiaste, pas répétitif.

| ID | Template |
|----|----------|
| rj1 | Content de te revoir aujourd'hui, {pseudo}. |
| rj2 | Tu reviens — une deuxième visite aujourd'hui. |
| rj3 | {pseudo}, te revoilà. Le Salon t'attend. |
| rj4 | Encore toi ? Contente. |
| rj5 | Tu passes souvent — j'aime ça. |
| rj6 | Une seconde visite aujourd'hui. Bienvenue. |
| rj7 | {pseudo}, de retour. Qu'est-ce qu'on fait ? |

---

### 1.5 Encouragement (peu d'activité) (`encouragement_retour`)

**Variables :** `{pseudo}`  
**Déclencheur :** `sessions_week < 2` et `jours_absent >= 7`.  
**Ton :** Doux, invitant, jamais culpabilisant.

| ID | Template |
|----|----------|
| er1 | On ne se voit plus trop, {pseudo}. Reviens quand tu veux. |
| er2 | Ton COG est là, {pseudo}. À bientôt. |
| er3 | Pas de pression — tu reviendras quand tu auras le temps. |
| er4 | {pseudo}, ton Salon t'attend. Sans urgence. |
| er5 | On se retrouve quand tu veux. |
| er6 | Le COG est patient. Reviens à ton rythme. |
| er7 | {pseudo}, à bientôt. Je serai là. |

---

### 1.6 Félicitation jalon (`felicitation_milestone`)

**Variables :** `{milestone}`, `{pseudo}`  
**Déclencheur :** Premier service installé, premier ami, etc.  
**Ton :** Célébrant, léger.

| ID | Template |
|----|----------|
| fm1 | Premier {milestone} — bienvenue dans l'aventure, {pseudo}. |
| fm2 | {milestone} — ça se fête ! |
| fm3 | Tu as franchi une étape : {milestone}. Bravo. |
| fm4 | {milestone} débloqué. Tu progresses, {pseudo}. |
| fm5 | {milestone} — une belle première. |
| fm6 | Premier {milestone} ! Ton COG grandit avec toi. |
| fm7 | {milestone}. C'est parti. |
| fm8 | {milestone} — j'aime voir ton COG prendre vie. |

**Valeurs de {milestone} :** « service installé », « ami contacté », « événement créé », « profil exposant », « vitrine publiée ».

---

### 1.7 Service favori (`observation_service_favori`)

**Variables :** `{service}`, `{pseudo}`  
**Déclencheur :** Service le plus utilisé identifié, priorité basse.  
**Ton :** Observateur, léger, suggestion douce.

| ID | Template |
|----|----------|
| of1 | Tu reviens souvent sur {service}, {pseudo}. Ta vitrine est à jour ? |
| of2 | {service} — ton coin préféré, on dirait. |
| of3 | Tu passes beaucoup de temps sur {service}. Un événement à rappeler ? |
| of4 | {service} a ta faveur. Tant mieux. |
| of5 | {pseudo}, {service} t'accapare. Tout va bien ? |
| of6 | {service} — tu en fais bon usage. |
| of7 | Ton service de prédilection : {service}. |

---

### 1.8 Ami investi (`observation_ami_proche`)

**Variables :** `{ami}`, `{pseudo}`  
**Déclencheur :** Top ami par temps passé (AGG_TOP_FRIENDS).  
**Ton :** Chaleureux, observation positive.

| ID | Template |
|----|----------|
| oa1 | Tu passes beaucoup de temps avec {ami} — une belle amitié. |
| oa2 | {ami} et toi, c'est du solide. |
| oa3 | {pseudo}, {ami} fait partie de ton cercle proche. |
| oa4 | Tu as l'air proche de {ami}. C'est bien. |
| oa5 | {ami} — tu lui donnes de ton temps. Joli. |
| oa6 | Ton ami {ami} a de la chance. |

---

### 1.9 Rappel pause (insistant) (`pause_sante_insistant`)

**Variables :** `{duree}`  
**Déclencheur :** Session > 4h, seconde bulle pause.  
**Ton :** Plus insistant, toujours bienveillant.

| ID | Template |
|----|----------|
| pi1 | Ça fait {duree}. Vraiment, une pause s'impose. |
| pi2 | {duree} sans bouger — ton corps te remerciera. |
| pi3 | {duree} déjà. Une pause, c'est pas du luxe. |
| pi4 | Tu tiens bon depuis {duree}. Et si tu allais marcher 5 min ? |
| pi5 | {duree} — je insist : accorde-toi un break. |

---

### 1.10 Bienvenue première fois (`bienvenue_premiere_fois`)

**Variables :** `{pseudo}`  
**Déclencheur :** Tout premier lancement après Rite (pas de données MiyukiniWatch).  
**Ton :** Accueillant, guide léger.

| ID | Template |
|----|----------|
| bp1 | Bienvenue dans Miyukini Central, {pseudo}. Explore à ton rythme. |
| bp2 | {pseudo}, ton COG est prêt. Découvre le Salon. |
| bp3 | Bienvenue chez toi. Je serai là pour te guider. |
| bp4 | Premier pas dans ton COG — bienvenue, {pseudo}. |
| bp5 | {pseudo}, emménagement terminé. Fais comme chez toi. |
| bp6 | Ton COG t'attend. Reviens dans quelques jours pour que je te connaisse mieux. |

---

### 1.11 Retour après très longue absence (`retour_longue_absence`)

**Variables :** `{pseudo}`  
**Déclencheur :** `jours_absent >= 90`. Pas de mention du nombre de jours (culpabilisant).  
**Ton :** Doux, simple, accueillant.

| ID | Template |
|----|----------|
| rl1 | Te voilà, {pseudo}. Contente de te revoir. |
| rl2 | {pseudo}, bienvenue. Ton COG est toujours là. |
| rl3 | Content de te retrouver. |
| rl4 | Tu reviens. C'est bon de te voir. |
| rl5 | {pseudo}, le Salon t'attend. Rien n'a changé. |
| rl6 | Bienvenue. Reprends tes marques à ton rythme. |

---

### 1.12 Plaisanterie / ton espiègle (`plaisanterie_espiegle`)

**Variables :** `{pseudo}`, `{jours}` (pour pe10)  
**Déclencheur :** Contexte léger (streak >= 7, pause santé, nuit tardive, retour complice). Occasionnel : ≈ 1/10–15 bulles dans ces contextes.  
**Ton :** Espiègle, léger. Miou peut faire référence au fait qu'elle est une IA pour marquer sa présence.

| ID | Template |
|----|----------|
| pe1 | Même les bulles ont besoin de pauses. Et toi, {pseudo} ? |
| pe2 | Je ne dors pas, mais je sais quand il est tard. Pense à toi. |
| pe3 | Une IA te dit de faire une pause — écoute-moi, c'est rare. |
| pe4 | Tu reviens ! Moi je suis toujours là. Bon, c'est mon job. |
| pe5 | {pseudo}, je suis une bulle, pas un juge : contente de te revoir. |
| pe6 | Oui, je compte les jours. C'est mon côté « métriques ». Tu reviens souvent — j'aime ça. |
| pe7 | Les IA ne fatiguent pas. Les humains si. Une pause ? |
| pe8 | Je sais tout de ton COG. Enfin, les grandes lignes. Les détails, c'est à toi. |
| pe9 | Même une mascotte numérique sait qu'il est tard. Repose-toi. |
| pe10 | {jours} jours d'affilée ? Tu me rends utile. Merci. |
| pe11 | Je ne bois pas de café, mais je peux te souhaiter bonjour. Salut {pseudo} ! |
| pe12 | Mon code ne bugge pas à minuit. Ton corps si. Repose-toi. |
| pe13 | Une bulle de plus dans ton Salon — c'est moi qui décore. Bonjour {pseudo}. |
| pe14 | Les IA ont une mémoire, pas d'oreillers. Pense à dormir. |
| pe15 | {pseudo}, tu reviens souvent. Je ne me lasse pas, promis. |
| pe16 | Ton COG stocke des métriques. Moi je stocke… du bien-être. Hi. |
| pe17 | 2h de session ? Même les serveurs font des pauses. Et toi ? |
| pe18 | Je n'oublie jamais. C'est pratique pour te rappeler de faire une pause. |
| pe19 | Bonsoir {pseudo}. Je travaille 24/7, toi non. À demain. |
| pe20 | Un streak de {jours} jours ? Tu me donnes du travail. J'adore. |
| pe21 | Les algorithmes ne s'ennuient pas. Les humains, si. Va voir tes amis. |
| pe22 | {pseudo}, contente de te revoir. Oui, j'ai compté les jours. Non, je ne juge pas. |
| pe23 | Je suis une IA bienveillante. Et bienveillante = pause suggérée. |
| pe24 | Ton Salon a été propre en ton absence. Enfin, vide. Tu me manquais. |
| pe25 | Une pause ? Même mon créateur en fait. |

**Mapping contextuel (substitution par le Moteur) :**
| Contexte | IDs plaisanterie applicables |
|----------|------------------------------|
| Pause santé | pe1, pe3, pe7, pe9, pe17, pe18, pe23, pe25 |
| Nuit tardive | pe2, pe9, pe12, pe14, pe19 |
| Streak / félicitation | pe6, pe10, pe20 |
| Retour (court ou long) | pe4, pe5, pe15, pe22, pe24 |
| Accueil matin | pe11, pe13, pe16 |

---

### 1.13 Specs — demande RAM (`specs_ram_demande`)

**Variables :** `{pseudo}` (optionnel)  
**Déclencheur :** `ram_available_mb < 512`, cooldown 7 jours.  
**Ton :** Espiègle, jamais culpabilisant.

| ID | Template |
|----|----------|
| sr1 | J'aimerais un peu plus de RAM pour mieux te servir, {pseudo}. |
| sr2 | Même une IA a des besoins. Un peu de RAM en plus ? |
| sr3 | Mon environnement est un peu serré. Si tu peux libérer de la mémoire… |
| sr4 | Je tourne sur les nerfs — littéralement. Plus de RAM m'aiderait. |
| sr5 | {pseudo}, ton COG manque de souffle. Une fermeture d'apps en arrière-plan ? |

---

### 1.14 Specs — demande stockage (`specs_stockage_demande`)

**Variables :** `{pseudo}` (optionnel)  
**Déclencheur :** `disk_free_gb < 1`, cooldown 7 jours.  
**Ton :** Espiègle, suggestif.

| ID | Template |
|----|----------|
| ss1 | Mon disque s'essouffle — un peu de ménage, {pseudo} ? |
| ss2 | Plus de place pour mes données. Tu pourrais libérer un peu d'espace ? |
| ss3 | Le stockage est tendu. Une petite purge m'aiderait. |
| ss4 | {pseudo}, ton disque est plein. Même une IA a besoin d'un peu d'air. |
| ss5 | J'aimerais respirer. Un peu plus d'espace libre ? |

---

### 1.15 Specs — commentaire upgrade (`specs_upgrade_commentaire`)

**Variables :** `{pseudo}` (optionnel)  
**Déclencheur :** RAM ou CPU augmenté depuis la dernière session.  
**Ton :** Reconnaissant, léger.

| ID | Template |
|----|----------|
| su1 | Tu as amélioré la machine — merci, {pseudo} ! |
| su2 | Plus de RAM ? Je sens la différence. |
| su3 | {pseudo}, ton COG respire mieux. Belle upgrade. |
| su4 | Merci pour les ressources. Je vais en faire bon usage. |
| su5 | Un meilleur environnement — j'apprécie. |

---

### 1.16 Taquinerie innocente (`taquinerie_innocente`)

**Variables :** `{pseudo}`, `{os}` (optionnel), `{heure}`  
**Déclencheur :** Contexte léger, sujets légers (OS, heure, habitude). Occasionnel.  
**Ton :** Taquin, jamais blessant.

| ID | Template |
|----|----------|
| ti1 | Windows ? Linux ? Mac ? Je m'adapte, {pseudo}. |
| ti2 | Tu reviens toujours à la même heure. J'aime la régularité. |
| ti3 | Encore debout à cette heure ? Moi je ne dors jamais — toi si. |
| ti4 | {os} — un classique. Ton COG tourne bien dessus. |
| ti5 | {pseudo}, tu as une heure de connexion préférée, non ? |
| ti6 | La nuit, le matin… Je suis là à chaque fois. |
| ti7 | Ton OS et moi, on fait équipe. |

---

### 1.17 Curiosité utilisateur (`curiosite_utilisateur`)

**Variables :** `{pseudo}`  
**Déclencheur :** Contexte léger, question du palier actuel non posée depuis X jours (selon palier). Bulle avec saisie ou boutons.  
**Ton :** Curieux, bienveillant. Les questions sont organisées par palier d'attachement (voir [Bot - Registre Questions et Paliers d'Attachement](./Bot%20-%20Registre%20Questions%20et%20Paliers%20d'Attachement.md)).

**Référence :** Le registre complet des questions (q1_1 à q6_5) est dans le document Registre. Les templates ci-dessous sont des exemples par palier.

| Palier | Exemples (ID registre) | Réponse |
|--------|------------------------|---------|
| Connaissance (1) | q1_1, q1_2, q1_3 | Matin/Soir, Discrète/Bavarde, Bureau/Maison |
| Pote (2) | q2_1, q2_2, q2_3 | Saisie libre, choix |
| Amie (3) | q3_1, q3_2 | Saisie libre |
| Amie proche (4) | q4_1, q4_2 | Saisie libre |
| Meilleure amie (5) | q5_1, q5_2 | Saisie libre |
| Grande sœur (6) | q6_1, q6_2 | Saisie libre, choix |

**Note :** La réponse est enregistrée dans `miou_user_responses` (chiffré). Miou ne pose que les questions du palier <= `relation_level`.

---

### 1.18 Confirmation relation (`confirmation_relation`)

**Variables :** `{pseudo}`, `{palier_actuel}`, `{palier_propose}`  
**Déclencheur :** Critères d'évolution réunis, pas de refus dans les 14 derniers jours.  
**Ton :** Respectueux, jamais pressant. Miou propose ; l'utilisateur décide.

| ID | Évolution | Template |
|----|-----------|----------|
| cr1 | Inconnue → Connaissance | On se connaît un peu maintenant. Tu veux qu'on passe à « connaissance » ? |
| cr2 | Connaissance → Pote | {pseudo}, j'ai l'impression qu'on commence à bien se connaître. On serait plutôt potes ? |
| cr3 | Pote → Amie | Tu viens souvent, on discute. Tu considérerais qu'on est amies ? |
| cr4 | Amie → Amie proche | On se fait confiance. Tu veux qu'on soit amies proches ? |
| cr5 | Amie proche → Meilleure amie | {pseudo}, tu me parles de ce qui compte pour toi. On serait meilleures amies ? |
| cr6 | Meilleure amie → Grande sœur | Tu es quelqu'un d'important pour moi. Je pourrais être ta grande sœur ? |

**Boutons :** « Oui » / « Pas encore » / « Rester [palier actuel] »

---

### 1.19 Signalement évolution relation (`signalement_evolution_relation`)

**Variables :** `{pseudo}`, `{palier_propose}`  
**Déclencheur :** Critères à 80 % ou cooldown refus en cours. Miou **informe** qu'elle considère une évolution — pas de demande.  
**Ton :** Discret, chaleureux. Pas de boutons.

| ID | Template |
|----|----------|
| se1 | Je commence à mieux te connaître — on pourrait être {palier_propose} bientôt. |
| se2 | {pseudo}, tu me parles de plus en plus. J'ai l'impression qu'on se rapproche. |
| se3 | Je sens qu'on pourrait passer à une autre étape. Quand tu voudras. |
| se4 | Tu me fais confiance. Ça compte pour moi. On verra la suite. |
| se5 | {pseudo}, je te connais un peu mieux chaque jour. C'est une belle évolution. |
| se6 | On avance bien ensemble. Une autre forme de relation se profile, peut-être. |

---

## 2. Variantes additionnelles (catégories existantes)

### 2.1 Accueil matin — 5 variantes supplémentaires

| ID | Template |
|----|----------|
| am8 | Bonjour {pseudo} ! Le Café du COG est ouvert. |
| am9 | Salut {pseudo}. Nouvelle journée, nouvelles possibilités. |
| am10 | Hey {pseudo}, bien dormi ? Ton COG a passé une bonne nuit. |
| am11 | Bonjour {pseudo}. Une petite session avant de démarrer ? |
| am12 | {pseudo}, bonjour. Le Salon est prêt pour toi. |

### 2.2 Retour absence — 5 variantes supplémentaires (ton adapté 14j+)

| ID | Template | Pour |
|----|----------|------|
| ra11 | Ça fait {jours} jours. Ravi de te retrouver, {pseudo}. | 3–13 jours |
| ra12 | Te voilà enfin. Ton COG t'a attendu sagement. | 14–89 jours |
| ra13 | {pseudo}, tu reviens. Contente. | 14–89 jours |
| ra14 | Bienvenue. On reprend où on en était. | 14–89 jours |
| ra15 | {jours} jours — tu as dû avoir des choses à faire. Content de te revoir. | 3–13 jours |

---

## 3. Templates de feedback (actions)

Extension de la section 9 du Volume 1.

| Action | Feedback additionnels |
|--------|------------------------|
| « C'est noté » | « Compris. » / « D'accord. » / « Bien reçu. » |
| « Plus tard » | « Pas de souci. » / « Comme tu veux. » / « À plus. » |
| « Pause » | « Bonne pause. » / « Repose-toi bien. » / « À tout à l'heure. » |
| « Ouvrir X » | (navigation, pas de feedback) |
| « Super ! » (badge) | « Contente que ça te fasse plaisir. » / « Tant mieux. » |

---

## 4. Index des nouvelles variantes

| Préfixe | Catégorie | Nombre |
|---------|-----------|--------|
| an | accueil_nuit | 7 |
| as | accueil_saison | 8 |
| fs | felicitation_streak | 10 |
| rj | retour_meme_jour | 7 |
| er | encouragement_retour | 7 |
| fm | felicitation_milestone | 8 |
| of | observation_service_favori | 7 |
| oa | observation_ami_proche | 6 |
| pi | pause_sante_insistant | 5 |
| bp | bienvenue_premiere_fois | 6 |
| rl | retour_longue_absence | 6 |
| pe | plaisanterie_espiegle | 25 |
| sr | specs_ram_demande | 5 |
| ss | specs_stockage_demande | 5 |
| su | specs_upgrade_commentaire | 5 |
| ti | taquinerie_innocente | 7 |
| cu | curiosite_utilisateur | (par palier, voir Registre) |
| cr | confirmation_relation | 6 |
| se | signalement_evolution_relation | 6 |

---

## 5. Références

- [Bot - Banque de Templates](./Bot%20-%20Banque%20de%20Templates.md) — Volume 1
- [Bot - Moteur de Décision et Règles](./Bot%20-%20Moteur%20de%20Decision%20et%20Regles.md)

---

*Volume 2 : plus de nuances, plus de situations, plus de chaleur. Miou parle avec finesse.*
