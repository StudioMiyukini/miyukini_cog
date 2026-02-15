# Bot Miou — Banque de Templates

Document exhaustif recensant tous les templates du Bot Miou : structure, syntaxe, catégories, variantes, variables, règles de sélection et exemples de sortie.

---

## 1. Structure et syntaxe des templates

### 1.1 Format général

Un template est une chaîne de caractères contenant des **placeholders** au format `{nom_variable}`. À la génération, chaque placeholder est remplacé par la valeur correspondante du contexte.

**Exemple :**
```
Bonjour {pseudo} ! Ça fait {jours} jours que tu n'es pas passé — contente de te revoir.
```

Avec `pseudo = "Kaito"` et `jours = 5` → « Bonjour Kaito ! Ça fait 5 jours que tu n'es pas passé — contente de te revoir. »

### 1.2 Règles de syntaxe

| Règle | Description |
|-------|-------------|
| **Placeholder** | `{nom}` — accolades obligatoires, nom en minuscules, underscore autorisé. |
| **Pas d'espaces** | `{ pseudo }` est invalide. |
| **Pas de nesting** | `{pseudo_{suffixe}}` non supporté. |
| **Échappement** | Pour afficher `{` littéral, utiliser `{{` (ou convention à définir). |
| **Longueur** | Template final recommandé : 10–120 caractères. Au-delà, risque de bulle trop longue. |

### 1.3 Variables disponibles

Voir section 3 pour la liste complète par catégorie. Variables globales (disponibles dans tous les templates) :

| Variable | Type | Description | Exemple |
|----------|------|-------------|---------|
| `{pseudo}` | string | Pseudo ou prénom de l'utilisateur | "Kaito" |
| `{heure}` | string | Heure actuelle (format court) | "14h30" |
| `{jour_semaine}` | string | Jour de la semaine | "samedi" |
| `{jours}` | number | Nombre de jours (absence, ami, etc.) | "5" |
| `{duree}` | string | Durée formatée (session, etc.) | "2h15" |
| `{service}` | string | Nom du service | "JayXpose" |
| `{ami}` | string | Pseudo de l'ami | "Luna" |
| `{badge}` | string | Nom du badge | "Webway connecté" |
| `{evenement}` | string | Titre de l'événement | "Réunion équipe" |
| `{temps_total}` | string | Temps total formaté | "1h30" |
| `{service_top}` | string | Service le plus utilisé | "JayKoa" |

### 1.4 Valeurs par défaut

Si une variable est absente du contexte :

| Variable | Défaut |
|----------|--------|
| `{pseudo}` | "toi" ou "habitant" |
| `{jours}` | "quelques" ou "plusieurs" |
| `{duree}` | "un moment" |
| `{service}` | "(service)" — éviter d'afficher si inconnu |
| `{ami}` | "(ami)" — éviter d'afficher si inconnu |
| `{badge}` | "(badge)" |
| `{evenement}` | "un événement" |
| `{temps_total}` | "du temps" |
| `{service_top}` | "tes services" |

---

## 2. Catégories de templates

### 2.1 Liste des catégories

| ID | Catégorie | Priorité | Déclencheur principal |
|----|-----------|----------|------------------------|
| `accueil_matin` | Accueil matin | 3 | Première connexion, 6h–12h |
| `accueil_apres_midi` | Accueil après-midi | 3 | Première connexion, 12h–18h |
| `accueil_soir` | Accueil soir | 3 | Première connexion, 18h–6h |
| `retour_absence` | Retour après absence | 4 | jours_absent > 3 |
| `pause_sante` | Pause santé | 1 | durée_session > seuil |
| `rappel_evenement` | Rappel événement | 2 | Événement JayKoa < 1h |
| `rappel_ami` | Rappel ami | 6 | ami non contacté > 7 jours |
| `suggestion_service` | Suggestion service | 7 | service délaissé > 14 jours |
| `felicitation_badge` | Félicitation badge | 5 | Badge débloqué non annoncé |
| `resume_activite` | Résumé activité | 8 | Optionnel, fin de session |
| `notification_ami` | Notification ami connecté | 2b | Ami en ligne (Jay1Tribu) |
| `proposition_llm` | Proposition LLM | Spécial | Scan specs + consentement |

### 2.2 Catégories sans variantes (messages fixes)

Certaines catégories ont un message unique (écrans Rite d'Entrée, Connexion) — ils ne font pas partie du Bot des bulles mais sont listés pour exhaustivité :

| Contexte | Message | Fichier |
|----------|---------|---------|
| Rite — étape Nom | « Bienvenue à toi dans ton nouveau Miyukini COG. Avant d'emménager, peux-tu me dire quel est ton nom ? » | rite_entree.rs |
| Rite — étape Email | « Pour pouvoir t'envoyer du courrier, peux-tu entrer ton adresse e-mail, s'il te plaît ? » | rite_entree.rs |
| Rite — étape Clé | « Pour finir, peux-tu me donner une clé pour protéger l'entrée ? » | rite_entree.rs |
| Connexion — variante a | « Quelle bonne surprise. Entre donc avec ta clé et rejoins moi à l'intérieur. » | connexion.rs |
| Connexion — variante b | « Te voilà de retour. Entre donc avec ta clé et rejoins moi à l'intérieur. » | connexion.rs |
| Connexion — variante c | « J'étais si impatiente de te revoir. Entre donc avec ta clé et rejoins moi à l'intérieur. » | connexion.rs |

---

## 3. Templates par catégorie (exhaustif)

### 3.1 Accueil matin (`accueil_matin`)

**Variables :** `{pseudo}`

**Variantes :**

| ID | Template |
|----|----------|
| am1 | Bonjour {pseudo} ! Prêt pour une bonne journée ? |
| am2 | Salut {pseudo}, le soleil se lève sur ton COG. |
| am3 | Bonjour {pseudo} ! Une nouvelle journée commence. |
| am4 | Hey {pseudo}, bien dormi ? Ton COG t'attend. |
| am5 | Bonjour {pseudo}. Belle matinée à toi. |
| am6 | Salut {pseudo} ! Le Salon est prêt pour toi. |
| am7 | Bonjour {pseudo}. Une petite visite ou tu restes un moment ? |

**Exemple de sortie :** « Bonjour Kaito ! Prêt pour une bonne journée ? »

---

### 3.2 Accueil après-midi (`accueil_apres_midi`)

**Variables :** `{pseudo}`

**Variantes :**

| ID | Template |
|----|----------|
| ap1 | Salut {pseudo} ! L'après-midi est bien avancée. |
| ap2 | Hey {pseudo}, contente de te voir en ce milieu de journée. |
| ap3 | Bonjour {pseudo}. Tu passes en coup de vent ? |
| ap4 | Salut {pseudo} ! Ton COG est là si tu as besoin. |
| ap5 | Hey {pseudo}, bienvenue au Salon. |
| ap6 | Bonjour {pseudo}. L'après-midi, c'est souvent un bon moment pour avancer. |
| ap7 | Salut {pseudo} ! Tu as des projets pour cet après-midi ? |

**Exemple de sortie :** « Salut Kaito ! L'après-midi est bien avancée. »

---

### 3.3 Accueil soir (`accueil_soir`)

**Variables :** `{pseudo}`

**Variantes :**

| ID | Template |
|----|----------|
| as1 | Bonsoir {pseudo}. Tu passes en coup de vent ou tu restes un moment ? |
| as2 | Bonsoir {pseudo}. La soirée s'annonce bien. |
| as3 | Hey {pseudo}, bonsoir. Ton COG veille. |
| as4 | Bonsoir {pseudo}. Une petite session avant de te reposer ? |
| as5 | Salut {pseudo}, bonsoir. Le Salon est calme à cette heure. |
| as6 | Bonsoir {pseudo}. Pense à toi si tu restes tard. |
| as7 | Hey {pseudo}, bonsoir. Contente de te voir ce soir. |

**Exemple de sortie :** « Bonsoir Kaito. Tu passes en coup de vent ou tu restes un moment ? »

---

### 3.4 Retour après absence (`retour_absence`)

**Variables :** `{pseudo}`, `{jours}`

**Variantes :**

| ID | Template |
|----|----------|
| ra1 | Ça fait {jours} jours — contente de te revoir, {pseudo}. |
| ra2 | Te voilà de retour ! {jours} jours sans toi, c'était long. |
| ra3 | {pseudo}, tu reviens après {jours} jours. Bienvenue. |
| ra4 | Ça fait {jours} jours. Contente que tu sois là, {pseudo}. |
| ra5 | Te voilà ! {jours} jours d'absence — ton COG t'a attendu. |
| ra6 | {pseudo}, revoilà ! {jours} jours, c'est une belle pause. |
| ra7 | Contente de te revoir après {jours} jours, {pseudo}. |
| ra8 | Tu reviens après {jours} jours — ton Salon n'a pas bougé. |
| ra9 | {jours} jours plus tard, te voilà. Bienvenue, {pseudo}. |
| ra10 | Ça fait {jours} jours, {pseudo}. Ravi de te retrouver. |

**Exemple de sortie :** « Ça fait 5 jours — contente de te revoir, Kaito. »

---

### 3.5 Pause santé (`pause_sante`)

**Variables :** `{duree}`

**Variantes :**

| ID | Template |
|----|----------|
| ps1 | Ça fait {duree} que tu es connecté — accorde-toi une petite pause. |
| ps2 | {duree} de session, c'est bien. Et si tu prenais l'air ? |
| ps3 | Tu as passé {duree} sur ton COG. Une pause te ferait du bien. |
| ps4 | {duree} déjà. Pense à te dégourdir les jambes. |
| ps5 | Ça fait {duree} — une petite pause, ça ne fait pas de mal. |
| ps6 | Tu tiens bien la route depuis {duree}. Une pause ? |
| ps7 | {duree} de concentration, bravo. Accordons-toi un break. |
| ps8 | Ça fait {duree} que tu es là. Et si tu allais prendre l'air ? |
| ps9 | {duree} sans pause — ton corps te remerciera. |
| ps10 | Tu as bien bossé pendant {duree}. Une petite pause ? |

**Exemple de sortie :** « Ça fait 2h15 que tu es connecté — accorde-toi une petite pause. »

---

### 3.6 Rappel événement (`rappel_evenement`)

**Variables :** `{evenement}`, `{heure}` (optionnel)

**Variantes :**

| ID | Template |
|----|----------|
| re1 | N'oublie pas : {evenement} bientôt. |
| re2 | Rappel : {evenement} approche. |
| re3 | {evenement} — c'est dans peu de temps. |
| re4 | Pense à {evenement}, ça approche. |
| re5 | Un petit rappel : {evenement} est à venir. |
| re6 | {evenement} — n'oublie pas. |
| re7 | Ça approche : {evenement}. |
| re8 | {evenement} arrive. Tu es prêt ? |
| re9 | Rappel doux : {evenement} bientôt. |
| re10 | {evenement} — je te le rappelle au cas où. |

**Exemple de sortie :** « N'oublie pas : Réunion équipe bientôt. »

---

### 3.7 Rappel ami (`rappel_ami`)

**Variables :** `{ami}`, `{jours}`

**Variantes :**

| ID | Template |
|----|----------|
| rm1 | Ça fait {jours} jours que tu n'as pas échangé avec {ami}. Un petit message ? |
| rm2 | {ami} serait peut-être content d'avoir de tes nouvelles — ça fait {jours} jours. |
| rm3 | Pense à {ami}. {jours} jours sans contact, c'est long. |
| rm4 | Tu n'as pas parlé à {ami} depuis {jours} jours. Un petit coucou ? |
| rm5 | {ami} — {jours} jours. Il ou elle apprécierait peut-être un message. |
| rm6 | Ça fait {jours} jours avec {ami}. Un petit signe ? |
| rm7 | {ami} n'a pas eu de tes nouvelles depuis {jours} jours. |
| rm8 | Pense à reprendre contact avec {ami}. {jours} jours, c'est beaucoup. |
| rm9 | {ami} — {jours} jours sans échange. Un message lui ferait plaisir. |
| rm10 | Tu pourrais envoyer un petit message à {ami}. Ça fait {jours} jours. |

**Exemple de sortie :** « Ça fait 12 jours que tu n'as pas échangé avec Luna. Un petit message ? »

---

### 3.8 Suggestion service (`suggestion_service`)

**Variables :** `{service}`

**Variantes :**

| ID | Template |
|----|----------|
| ss1 | Tu n'as pas ouvert {service} depuis un moment. Envie d'y jeter un œil ? |
| ss2 | {service} te manque peut-être ? |
| ss3 | Tu n'es pas passé par {service} depuis longtemps. Une visite ? |
| ss4 | {service} pourrait avoir des nouveautés pour toi. |
| ss5 | Un petit tour dans {service} ? Tu l'as délaissé. |
| ss6 | {service} — ça fait un moment. Envie d'y retourner ? |
| ss7 | Tu as oublié {service} ? Il t'attend. |
| ss8 | {service} n'a pas eu de tes nouvelles. Une petite visite ? |
| ss9 | Tu pourrais faire un tour dans {service}. |
| ss10 | {service} — tu l'as un peu négligé. Une idée ? |

**Exemple de sortie :** « Tu n'as pas ouvert JayKoa depuis un moment. Envie d'y jeter un œil ? »

---

### 3.9 Félicitation badge (`felicitation_badge`)

**Variables :** `{badge}`

**Variantes :**

| ID | Template |
|----|----------|
| fb1 | Tu as débloqué le badge « {badge} » — bravo ! |
| fb2 | Bravo ! Le badge « {badge} » est à toi. |
| fb3 | « {badge} » — tu l'as ! Félicitations. |
| fb4 | Tu as gagné le badge « {badge} ». Bien joué. |
| fb5 | « {badge} » débloqué. Tu progresses. |
| fb6 | Félicitations pour le badge « {badge} ». |
| fb7 | Tu as débloqué « {badge} ». C'est une belle étape. |
| fb8 | « {badge} » — c'est à toi maintenant. Bravo. |
| fb9 | Bien joué ! Le badge « {badge} » est débloqué. |
| fb10 | « {badge} » — tu l'as mérité. Félicitations. |

**Exemple de sortie :** « Tu as débloqué le badge « Webway connecté » — bravo ! »

---

### 3.10 Résumé activité (`resume_activite`)

**Variables :** `{temps_total}`, `{service_top}`

**Variantes :**

| ID | Template |
|----|----------|
| rs1 | Aujourd'hui : {temps_total} dans Central, surtout sur {service_top}. Beau boulot. |
| rs2 | {temps_total} aujourd'hui, principalement sur {service_top}. Pas mal. |
| rs3 | Tu as passé {temps_total} sur ton COG, surtout dans {service_top}. |
| rs4 | Résumé du jour : {temps_total}, {service_top} en tête. |
| rs5 | {temps_total} de session, avec {service_top} en vedette. Bien. |
| rs6 | Aujourd'hui : {temps_total}, surtout {service_top}. |
| rs7 | Belle session : {temps_total} dans Central, {service_top} en priorité. |
| rs8 | {temps_total} aujourd'hui — {service_top} a eu la part du lion. |
| rs9 | Tu as bien utilisé ton COG : {temps_total}, {service_top}. |
| rs10 | Résumé : {temps_total}, principalement {service_top}. Beau travail. |

**Exemple de sortie :** « Aujourd'hui : 1h30 dans Central, surtout sur JayXpose. Beau boulot. »

---

### 3.11 Notification ami connecté (`notification_ami`)

**Variables :** `{ami}`

**Variantes :**

| ID | Template |
|----|----------|
| na1 | {ami} est en ligne. Envie de lui dire bonjour ? |
| na2 | {ami} vient de se connecter. |
| na3 | {ami} est là. Un petit message ? |
| na4 | {ami} est en ligne sur le Webway. |
| na5 | {ami} est connecté. Tu veux lui parler ? |
| na6 | {ami} est là. Bonne occasion pour échanger. |
| na7 | {ami} est en ligne. |
| na8 | {ami} vient d'arriver. Une idée ? |
| na9 | {ami} est connecté. Tu peux lui envoyer un message. |
| na10 | {ami} est en ligne. Envie de discuter ? |

**Exemple de sortie :** « Luna est en ligne. Envie de lui dire bonjour ? »

---

### 3.12 Proposition LLM (`proposition_llm`)

**Variables :** `{pseudo}`

**Variantes :**

| ID | Template |
|----|----------|
| pl1 | {pseudo}, mon environnement me permet d'être un peu plus vivante. Si tu le souhaites, je peux utiliser une partie de la puissance de calcul de ton COG pour varier mes messages et mieux m'adapter à toi. C'est optionnel — je fonctionne très bien sans. Tu préfères que je reste légère ou que j'essaie ? |
| pl2 | {pseudo}, ma machine me permet d'être plus réactive. Je pourrais utiliser un peu de puissance pour enrichir mes messages. C'est facultatif. Tu veux que j'essaie ? |
| pl3 | Hey {pseudo}. Ton COG a assez de ressources pour que je sois un peu plus intelligente. Je peux utiliser un petit modèle local pour varier mes réponses. Tu préfères que je reste simple ou que je tente ? |

**Note :** Cette catégorie est gérée par le flux de consentement (Proto-IA Scan). Une seule variante affichée selon le contexte (première proposition vs relance).

---

## 4. Actions associées aux bulles

Chaque catégorie peut définir des **actions** (boutons) affichées dans la bulle.

| Catégorie | Actions possibles |
|-----------|-------------------|
| Accueil (tous) | Aucune, ou « Voir les suggestions » |
| Retour absence | Aucune |
| Pause santé | « Pause » (ferme + log), « Plus tard » |
| Rappel événement | « Voir le calendrier », « C'est noté » |
| Rappel ami | « Ouvrir Jay1Tribu », « Plus tard » |
| Suggestion service | « Ouvrir {service} », « C'est noté » |
| Félicitation badge | « Super ! », « Voir mes badges » |
| Résumé activité | Aucune |
| Notification ami | « Ouvrir Jay1Tribu », « Plus tard » |
| Proposition LLM | « Oui, vas-y », « Pas pour l'instant », « Plus d'infos » |

---

## 5. Règles de sélection des variantes

### 5.1 Anti-répétition

Pour chaque catégorie, le Sélecteur de variante :
1. Exclut les variantes utilisées dans la session courante.
2. Exclut les variantes utilisées dans les 3 dernières sessions (si historique persisté).
3. Parmi les variantes restantes, choisit aléatoirement (ou round-robin si configuré).
4. Si aucune variante disponible → utilise la variante 1 (ou délègue au LLM).

### 5.2 Variantes contextuelles (optionnel)

Certaines catégories peuvent avoir des **sous-variantes** selon un critère :

| Catégorie | Critère | Effet |
|-----------|---------|-------|
| Accueil | Heure (matin/après-midi/soir) | Catégorie différente (accueil_matin vs accueil_soir). |
| Retour absence | jours > 14 | Variantes plus « douces » (éviter « c'était long » si absence très longue). |
| Pause santé | duree > 4h | Variantes plus insistantes (reste bienveillant). |

### 5.3 Règles de cohérence narrative

- **Tutoiement** : Tous les templates utilisent « tu », « ton », « toi ».
- **Miou parle** : Les templates sont à la première personne de Miou (« contente », « je te rappelle »).
- **Pas de vous** : Jamais de vouvoiement.
- **Métaphore maison** : Préférer « Salon », « COG », « emménager », « coins » quand pertinent.
- **Positif** : Pas de « tu as oublié » culpabilisant. Préférer « tu n'as pas ouvert » ou « ça fait X jours ».

---

## 6. Localisation (préparation)

### 6.1 Structure pour multi-langue

Les templates sont organisés par fichier ou clé de langue :

```
templates/
├── fr/
│   ├── accueil_matin.json
│   ├── accueil_soir.json
│   ├── retour_absence.json
│   └── ...
├── en/
│   ├── accueil_matin.json
│   └── ...
└── ...
```

Ou en base de données avec colonne `langue`.

### 6.2 Format JSON (exemple)

```json
{
  "categorie": "accueil_matin",
  "variantes": [
    {
      "id": "am1",
      "template": "Bonjour {pseudo} ! Prêt pour une bonne journée ?",
      "actions": []
    },
    {
      "id": "am2",
      "template": "Salut {pseudo}, le soleil se lève sur ton COG.",
      "actions": []
    }
  ]
}
```

### 6.3 Langue par défaut

Français (`fr`) par défaut. La langue est lue depuis le profil utilisateur ou les paramètres système.

---

## 7. Validation des templates

### 7.1 Checklist par template

Avant d'ajouter un template :
- [ ] Toutes les variables sont documentées.
- [ ] Le template respecte le ton Miou (bienveillant, tutoiement).
- [ ] Longueur < 120 caractères (hors variables).
- [ ] Pas de contenu culpabilisant.
- [ ] Les variables ont des valeurs par défaut définies.
- [ ] Le template a du sens avec des valeurs vides (dégradation gracieuse).

### 7.2 Tests de rendu

Pour chaque template, vérifier le rendu avec :
- Valeurs normales.
- Valeurs vides (pseudo = "", jours = 0).
- Valeurs extrêmes (jours = 365, duree = "12h").
- Caractères spéciaux dans pseudo (accents, espaces).

---

## 8. Templates spéciaux (Rite d'Entrée, Connexion)

Ces templates ne sont **pas** gérés par le Bot des bulles mais par les écrans dédiés. Ils sont listés pour exhaustivité et cohérence.

### 8.1 Rite d'Entrée

| Étape | Template | Variables |
|-------|----------|-----------|
| Nom | Bienvenue à toi dans ton nouveau Miyukini COG. Avant d'emménager, peux-tu me dire quel est ton nom ? | Aucune |
| Email | Pour pouvoir t'envoyer du courrier, peux-tu entrer ton adresse e-mail, s'il te plaît ? | Aucune |
| Clé | Pour finir, peux-tu me donner une clé pour protéger l'entrée ? Suis les instructions ci-dessous. | Aucune |

### 8.2 Connexion (retour habitant connu)

| Variante | Template |
|----------|----------|
| a | Quelle bonne surprise. Entre donc avec ta clé et rejoins moi à l'intérieur. |
| b | Te voilà de retour. Entre donc avec ta clé et rejoins moi à l'intérieur. |
| c | J'étais si impatiente de te revoir. Entre donc avec ta clé et rejoins moi à l'intérieur. |

### 8.3 Connexion — étape clé (sans profil sauvegardé)

| Contexte | Template |
|----------|----------|
| Premier passage | Bienvenue, habitant. Identifie-toi pour entrer. |
| Après accueil | Bienvenue à toi. Peux-tu entrer la clé ici, s'il te plaît ? |

---

## 9. Templates de réponses aux actions (feedback)

Quand l'utilisateur clique sur une action, Miou peut afficher un court feedback. Ces templates sont très courts (5–15 caractères ou une phrase).

| Action | Feedback possible |
|--------|-------------------|
| « C'est noté » | « Parfait. » / « D'accord. » / « C'est noté. » |
| « Plus tard » | « Pas de souci. » / « À plus tard. » |
| « Pause » | « Bonne pause. » / « Prends soin de toi. » |
| « Ouvrir X » | (Pas de feedback — navigation directe) |
| « Oui, vas-y » (LLM) | « Merci. Je vais me préparer en douceur. » |
| « Pas pour l'instant » (LLM) | « Très bien, je reste discrète. Tu peux changer d'avis dans Paramètres > Miou. » |

---

## 10. Statistiques et évolution des templates

### 10.1 Métriques utiles (interne, non exposé)

Pour améliorer les templates au fil du temps (analytics locaux uniquement) :
- Nombre d'affichages par catégorie.
- Nombre de dismiss par catégorie.
- Taux de clic sur les actions par catégorie.
- Variantes les plus/moins utilisées (si rotation).

Ces données restent **locales** et ne sont pas envoyées à l'extérieur. Elles peuvent servir à désactiver ou modifier des templates peu pertinents.

### 10.2 Processus d'ajout de templates

1. Rédiger le template selon les règles (section 7).
2. Assigner un ID unique dans la catégorie.
3. Documenter les variables utilisées.
4. Ajouter au fichier JSON ou à la source de données.
5. Tester avec valeurs normales et edge cases.
6. Valider la cohérence narrative (checklist).

### 10.3 Processus de suppression

Un template ne doit être supprimé que s'il :
- Contient une erreur (typo, ton inapproprié).
- Est redondant avec un autre (après consolidation).
- A un taux de dismiss très élevé (utilisateur ferme systématiquement).

La suppression doit être documentée (changelog interne).

---

## 11. Index des variantes par ID

Référence rapide pour l'implémentation et les tests :

| ID | Catégorie | Template (extrait) |
|----|-----------|---------------------|
| am1–am7 | accueil_matin | Bonjour {pseudo} ! ... |
| ap1–ap7 | accueil_apres_midi | Salut {pseudo} ! ... |
| as1–as7 | accueil_soir | Bonsoir {pseudo}. ... |
| ra1–ra10 | retour_absence | Ça fait {jours} jours — ... |
| ps1–ps10 | pause_sante | Ça fait {duree} que tu es connecté — ... |
| re1–re10 | rappel_evenement | N'oublie pas : {evenement} ... |
| rm1–rm10 | rappel_ami | Ça fait {jours} jours que tu n'as pas échangé avec {ami} ... |
| ss1–ss10 | suggestion_service | Tu n'as pas ouvert {service} depuis un moment ... |
| fb1–fb10 | felicitation_badge | Tu as débloqué le badge « {badge} » ... |
| rs1–rs10 | resume_activite | Aujourd'hui : {temps_total} ... |
| na1–na10 | notification_ami | {ami} est en ligne. ... |
| pl1–pl3 | proposition_llm | {pseudo}, mon environnement me permet ... |

---

## 12. Références

- [Bot - Document Fondateur et Architecture](./Bot%20-%20Document%20Fondateur%20et%20Architecture.md)
- [Bot - Moteur de Décision et Règles](./Bot%20-%20Moteur%20de%20Decision%20et%20Regles.md)
- [Bot - Intégration et Flux de Données](./Bot%20-%20Integration%20et%20Flux%20de%20Donnees.md)

---

*Banque de templates : exhaustivité, cohérence narrative, anti-répétition. Chaque mot de Miou est pensé pour la relation.*
