# Bot Miou â€” Registre de Questions et Paliers d'Attachement

Ce document dÃ©finit les **paliers de rapport d'attachement** entre Miou et l'utilisateur, le **registre de questions** organisÃ© par palier (ce qu'une personne proche devrait connaÃ®tre), et le **flux de confirmation** par lequel Miou propose Ã  l'utilisateur de dÃ©finir le statut de leur relation.

---

## 1. Contexte

Miou est **curieuse du monde rÃ©el et de l'utilisateur**. Elle possÃ¨de un registre de questions qu'une **meilleure amie** pourrait poser pour avoir les informations qu'une personne proche devrait connaÃ®tre. Les questions et les donnÃ©es sont organisÃ©es par **palier de rapport d'attachement**.

C'est Ã  **Miou**, en fonction des critÃ¨res, de **demander confirmation** Ã  l'utilisateur quant au statut de leur relation. L'utilisateur valide ou ajuste.

Le statut disponible est mesurÃ© par la **quantitÃ©**, la **qualitÃ©** et la **pertinence** des connaissances de Miou sur l'utilisateur. L'utilisateur **ne peut pas rÃ©gler un statut Ã©levÃ©** si Miou n'a pas suffisamment d'information.

Miou mesure aussi le **degrÃ© de complicitÃ©** avec l'utilisateur (rÃ©pond, sollicite, ignore, ferme, change manuellement le statut).

---

## 2. Les sept paliers d'attachement

| Niveau | Code | Nom | Ton | Ce que Miou connaÃ®t / peut demander |
|--------|------|-----|-----|-------------------------------------|
| 0 | `inconnue` | Inconnue | Neutre, accueillant | Rien de personnel. Juste le pseudo, l'heure, le contexte COG. |
| 1 | `connaissance` | Connaissance | Courtois, lÃ©ger | PrÃ©fÃ©rences pratiques (rappels, ton des bulles). |
| 2 | `pote` | Pote | DÃ©contractÃ©, complice | Hobbies, habitudes de connexion, activitÃ© prÃ©fÃ©rÃ©e. |
| 3 | `amie` | Amie | Chaleureux, attentif | Ce qui fait du bien, moments de la journÃ©e, humeur prÃ©fÃ©rÃ©e. |
| 4 | `amie_proche` | Amie proche | Intime, confiant | Valeurs, projets, ce qui compte vraiment. |
| 5 | `meilleure_amie` | Meilleure amie | TrÃ¨s proche, soutenant | RÃªves, peurs lÃ©gÃ¨res, faÃ§on prÃ©fÃ©rÃ©e d'Ãªtre soutenu. |
| 6 | `grande_soeur` | Grande sÅ“ur | Protecteur, bienveillant profond | Conseils, protection, prÃ©sence constante. |

### 2.1 Progression

- **Unidirectionnelle** : On ne descend jamais de palier (sauf demande explicite de l'utilisateur : Â« RÃ©initialiser notre relation Â»).
- **ConfirmÃ©e par l'utilisateur** : Miou propose une Ã©volution ; l'utilisateur accepte ou refuse.
- **Ni automatique ni imposÃ©e** : Miou ne force jamais. Si l'utilisateur refuse ou ignore, le palier reste inchangÃ©.

---

## 3. DegrÃ© de complicitÃ©

Miou mesure le **degrÃ© de complicitÃ©** avec l'utilisateur Ã  partir des interactions observÃ©es.

### 3.1 Signaux d'interaction

| Signal | Description | Impact complicitÃ© |
|--------|-------------|-------------------|
| **RÃ©pond** | L'utilisateur rÃ©pond Ã  une question de Miou (bulle curiositÃ©) | +2 |
| **Sollicite** | L'utilisateur ouvre ParamÃ¨tres > Miou, consulte Â« Ce que Miou sait de moi Â», ou interagit avec une action de bulle | +1 |
| **Ignore** | L'utilisateur ferme une bulle sans rÃ©pondre (curiositÃ© ou proposition) | -0.5 (lÃ©ger) |
| **Ferme** | L'utilisateur ferme rapidement une bulle (< 3 s d'affichage) Ã  rÃ©pÃ©tition | -1 |
| **Change manuellement** | L'utilisateur modifie le statut de relation dans ParamÃ¨tres (si autorisÃ©) | +1 (engagement explicite) |

### 3.2 Calcul du score de complicitÃ©

```
complicite_score = base(0) + cumul des signaux sur fenÃªtre glissante (30 derniers jours)
complicite_niveau = discretisÃ© : faible (0-5), modÃ©rÃ© (6-15), bon (16-30), Ã©levÃ© (31+)
```

**Usage :** Le niveau de complicitÃ© doit Ãªtre au moins **modÃ©rÃ©** pour proposer une Ã©volution vers Amie (palier 3+). Pour Amie proche et au-delÃ  : **bon** minimum.

---

## 4. MÃ©triques de connaissance (quantitÃ©, qualitÃ©, pertinence)

Le statut disponible dÃ©pend de ce que Miou **connaÃ®t** de l'utilisateur.

### 4.1 QuantitÃ©

| MÃ©trique | Description |
|----------|-------------|
| `reponses_total` | Nombre total de rÃ©ponses enregistrÃ©es |
| `reponses_palier_N` | Nombre de rÃ©ponses aux questions du palier N |
| `questions_posees` | Nombre de questions posÃ©es (toutes sessions) |
| `taux_reponse` | `reponses_total / questions_posees` (excluant Â« Passer Â») |

### 4.2 QualitÃ©

| MÃ©trique | Description |
|----------|-------------|
| `longueur_moyenne` | Longueur moyenne des rÃ©ponses (caractÃ¨res) â€” une rÃ©ponse Â« oui Â» = faible qualitÃ© |
| `reponses_substantielles` | Nombre de rÃ©ponses avec > 10 caractÃ¨res (saisie libre) ou choix explicite |
| `score_qualite` | `reponses_substantielles / reponses_total` (0-1) |

### 4.3 Pertinence

| MÃ©trique | Description |
|----------|-------------|
| `reponses_pertinentes_palier` | RÃ©ponses qui correspondent au type attendu (pas de hors-sujet manifeste) |
| `couverture_thematique` | Nombre de thÃ¨mes diffÃ©rents couverts (prÃ©fÃ©rence, loisir, Ã©motion, projet, etc.) |

### 4.4 Score de connaissance minimal par palier

**L'utilisateur ne peut pas rÃ©gler un statut Ã©levÃ© si Miou n'a pas suffisamment d'information.**

| Palier cible | QuantitÃ© min | QualitÃ© min | Pertinence |
|--------------|--------------|-------------|------------|
| Connaissance (1) | 1 rÃ©ponse (palier 0 ou 1) | â€” | â€” |
| Pote (2) | 2 rÃ©ponses palier 1, 1 palier 2 | Au moins 1 substantielle | â€” |
| Amie (3) | 4 rÃ©ponses palier 1+2, 2 palier 3 | 50 % substantielles | 2 thÃ¨mes |
| Amie proche (4) | 6 rÃ©ponses, 3 palier 4 | 60 % substantielles | 3 thÃ¨mes |
| Meilleure amie (5) | 10 rÃ©ponses, 4 palier 5 | 70 % substantielles | 4 thÃ¨mes |
| Grande sÅ“ur (6) | 15 rÃ©ponses, 5 palier 6 | 75 % substantielles | 5 thÃ¨mes |

**RÃ¨gle :** Si l'utilisateur tente de changer manuellement le statut dans ParamÃ¨tres vers un palier N, Miou vÃ©rifie `knowledge_score >= requis_palier_N`. Si non â†’ message : Â« Je ne te connais pas encore assez pour qu'on soit [palier]. RÃ©ponds Ã  quelques questions d'abord. Â» + affichage des critÃ¨res manquants.

---

## 5. CritÃ¨res de proposition d'Ã©volution

Miou propose de passer au palier suivant quand **tous** les critÃ¨res sont rÃ©unis. L'utilisateur confirme.

### 5.1 CritÃ¨res par palier (complets)

| Ã‰volution | QuantitÃ© | QualitÃ© | Pertinence | ComplicitÃ© | Temps / FrÃ©quence |
|-----------|----------|---------|------------|------------|-------------------|
| Inconnue â†’ Connaissance | 1 session complÃ¨te | â€” | â€” | â€” | `sessions_total >= 1` |
| Connaissance â†’ Pote | `reponses_palier_1 >= 1`, `sessions >= 3` | â€” | â€” | `complicite >= faible` | â€” |
| Pote â†’ Amie | `reponses_palier_1+2 >= 4`, `reponses_palier_3 >= 2` | `score_qualite >= 0.4` | 2 thÃ¨mes | `complicite >= modÃ©rÃ©` | `jours_distincts >= 7`, `streak >= 3` |
| Amie â†’ Amie proche | `reponses_palier_1..3 >= 6`, `reponses_palier_4 >= 3` | `score_qualite >= 0.5` | 3 thÃ¨mes | `complicite >= modÃ©rÃ©` | `jours >= 14`, `dismiss_rate < 0.5` |
| Amie proche â†’ Meilleure amie | `reponses_total >= 10`, `reponses_palier_5 >= 4` | `score_qualite >= 0.6` | 4 thÃ¨mes | `complicite >= bon` | `jours >= 30` |
| Meilleure amie â†’ Grande sÅ“ur | `reponses_total >= 15`, `reponses_palier_6 >= 5` | `score_qualite >= 0.7` | 5 thÃ¨mes | `complicite >= Ã©levÃ©` | `jours >= 60` |

### 5.2 RÃ¨gles de proposition

| RÃ¨gle | Description |
|-------|-------------|
| **Une proposition par session max** | Miou ne propose pas deux Ã©volutions dans la mÃªme session. |
| **Cooldown aprÃ¨s refus** | Si l'utilisateur refuse : ne pas reproposer avant 14 jours. |
| **Pas en dÃ©but de session** | Proposer aprÃ¨s au moins une bulle Â« normale Â» (accueil, rappel, etc.). |
| **Ton adaptÃ©** | La formulation reflÃ¨te le palier actuel et le palier proposÃ©. |

---

## 6. Registre de questions par palier

Chaque question est associÃ©e Ã  un **palier minimum**. Miou ne pose une question que si `relation_level >= question_palier`.

### 6.1 Palier 1 â€” Connaissance

Questions qu'on pose Ã  quelqu'un qu'on vient de rencontrer.

| ID | Question | Type rÃ©ponses | DonnÃ©e stockÃ©e |
|----|----------|----------------|----------------|
| q1_1 | Tu prÃ©fÃ¨res le matin ou le soir pour mes rappels ? | Matin / Soir / Peu importe | `preference_rappel` |
| q1_2 | Tu prÃ©fÃ¨res que je sois discrÃ¨te ou un peu bavarde ? | DiscrÃ¨te / Bavarde / Comme maintenant | `preference_ton` |
| q1_3 | Tu travailles plutÃ´t du bureau ou de chez toi ? | Bureau / Maison / Les deux | `contexte_activite` |
| q1_4 | Quel moment de la journÃ©e tu prÃ©fÃ¨res pour ton COG ? | Matin / AprÃ¨s-midi / Soir | `moment_prefere` |

### 6.2 Palier 2 â€” Pote

Questions pour apprendre Ã  connaÃ®tre quelqu'un avec qui on se sent Ã  l'aise.

| ID | Question | Type rÃ©ponses | DonnÃ©e stockÃ©e |
|----|----------|----------------|----------------|
| q2_1 | Tu aimes lire ? Quel genre ? | (saisie libre) | `loisir_lecture` |
| q2_2 | Tu as un hobby prÃ©fÃ©rÃ© ? | (saisie libre) | `hobby` |
| q2_3 | Tu prÃ©fÃ¨res les journÃ©es chargÃ©es ou tranquilles ? | ChargÃ©es / Tranquilles / Ã‡a dÃ©pend | `rythme_prefere` |
| q2_4 | Tu es du matin ou du soir ? | Matin / Soir / Les deux | `chronotype` |
| q2_5 | Tu as une activitÃ© qui te dÃ©connecte ? (sport, musiqueâ€¦) | (saisie libre) | `activite_deconnexion` |

### 6.3 Palier 3 â€” Amie

Questions pour mieux comprendre ce qui fait du bien.

| ID | Question | Type rÃ©ponses | DonnÃ©e stockÃ©e |
|----|----------|----------------|----------------|
| q3_1 | Qu'est-ce qui te fait du bien aprÃ¨s une journÃ©e difficile ? | (saisie libre) | `reconfort` |
| q3_2 | Tu prÃ©fÃ¨res qu'on te laisse de l'espace ou qu'on soit prÃ©sente ? | Espace / PrÃ©sente / Les deux selon le jour | `besoin_presence` |
| q3_3 | Tu as un endroit favori (virtuel ou rÃ©el) pour te ressourcer ? | (saisie libre) | `lieu_ressource` |
| q3_4 | Qu'est-ce qui te motive en ce moment ? | (saisie libre courte) | `motivation_actuelle` |
| q3_5 | Tu prÃ©fÃ¨res les surprises ou d'Ãªtre prÃ©venu Ã  l'avance ? | Surprises / PrÃ©venu / Les deux | `preference_surprise` |

### 6.4 Palier 4 â€” Amie proche

Questions sur ce qui compte vraiment.

| ID | Question | Type rÃ©ponses | DonnÃ©e stockÃ©e |
|----|----------|----------------|----------------|
| q4_1 | Qu'est-ce qui compte le plus pour toi en ce moment ? | (saisie libre) | `valeur_actuelle` |
| q4_2 | Tu as un projet qui te tient Ã  cÅ“ur ? | (saisie libre courte) | `projet_coeur` |
| q4_3 | Qu'est-ce qui te rend fier ou fiÃ¨re ? | (saisie libre) | `source_fierte` |
| q4_4 | Tu prÃ©fÃ¨res qu'on te pousse un peu ou qu'on te laisse avancer Ã  ton rythme ? | Pousser / Mon rythme / Les deux | `style_accompagnement` |
| q4_5 | C'est quoi pour toi une bonne journÃ©e ? | (saisie libre courte) | `bonne_journee` |

### 6.5 Palier 5 â€” Meilleure amie

Questions sur les rÃªves et le soutien.

| ID | Question | Type rÃ©ponses | DonnÃ©e stockÃ©e |
|----|----------|----------------|----------------|
| q5_1 | Tu as un rÃªve que tu aimerais rÃ©aliser ? | (saisie libre) | `reve` |
| q5_2 | Comment tu prÃ©fÃ¨res qu'on te soutienne quand Ã§a va mal ? | (saisie libre ou choix) | `soutien_prefere` |
| q5_3 | Qu'est-ce qui te fait peur (sans Ãªtre trop intime) ? | (saisie libre) | `peur_legere` |
| q5_4 | Tu as une personne qui t'inspire ? | (saisie libre courte) | `inspiration` |
| q5_5 | C'est quoi ton petit bonheur du quotidien ? | (saisie libre) | `bonheur_quotidien` |

### 6.6 Palier 6 â€” Grande sÅ“ur

Questions de prÃ©sence protectrice et bienveillante.

| ID | Question | Type rÃ©ponses | DonnÃ©e stockÃ©e |
|----|----------|----------------|----------------|
| q6_1 | Y a-t-il des moments oÃ¹ tu aimerais qu'on soit plus prÃ©sente ? | (saisie libre ou choix) | `moment_plus_presente` |
| q6_2 | Tu prÃ©fÃ¨res un conseil direct ou qu'on te pose des questions pour rÃ©flÃ©chir ? | Direct / Questions / Les deux | `style_conseil` |
| q6_3 | Qu'est-ce qui te rassure ? | (saisie libre) | `reassurance` |
| q6_4 | Comment on peut t'aider Ã  prendre soin de toi ? | (saisie libre) | `aide_soin` |
| q6_5 | Tu as besoin qu'on te rappelle quelque chose en particulier ? | (saisie libre) | `rappel_personnalise` |

---

## 7. Flux de confirmation du palier

### 7.1 DÃ©clenchement

```
1. CritÃ¨res du palier N+1 sont rÃ©unis
2. relation_level == N (pas dÃ©jÃ  au max)
3. Aucune proposition refusÃ©e dans les 14 derniers jours
4. Au moins une bulle Â« normale Â» affichÃ©e cette session
5. Pas de proposition dÃ©jÃ  faite cette session
```

### 7.2 Bulle de proposition

Miou affiche une bulle avec :

- **Message** : variante selon palier actuel et palier proposÃ© (voir templates)
- **Boutons** : Â« Oui, on est [palier_propose] Â» / Â« Pas encore Â» / Â« Rester [palier_actuel] Â»

### 7.3 RÃ©ponses

| Action utilisateur | Effet |
|--------------------|-------|
| Â« Oui, on est [palier_propose] Â» | `relation_level = N+1`. Enregistrement. Miou adapte ton et questions. |
| Â« Pas encore Â» | Aucun changement. Cooldown 14 jours avant reproposition. |
| Â« Rester [palier_actuel] Â» | Idem. L'utilisateur prÃ©fÃ¨re garder le statut actuel. |
| Fermeture sans clic | Aucun changement. Peut reproposer Ã  la prochaine session si critÃ¨res toujours rÃ©unis. |

### 7.4 RÃ©initialisation

Dans ParamÃ¨tres > Miou > Â« Ce que Miou sait de moi Â» :
- Option **Â« RÃ©initialiser notre relation Â»** â†’ `relation_level = 0`, efface les rÃ©ponses (ou propose de garder les rÃ©ponses tout en revenant Ã  Â« inconnue Â» â€” Ã  dÃ©finir).

### 7.5 Changement manuel du statut (par l'utilisateur)

Dans ParamÃ¨tres > Miou > Â« Statut de notre relation Â» :
- L'utilisateur peut **proposer** un palier cible.
- **VÃ©rification** : Miou vÃ©rifie si `knowledge_score >= requis_palier_cible` ET `complicite_niveau >= requis`.
- Si **non** : message Â« Je ne te connais pas encore assez pour qu'on soit [palier]. RÃ©ponds Ã  quelques questions d'abord. Â» + affichage des critÃ¨res manquants (ex. Â« 2 rÃ©ponses palier 3 manquantes Â», Â« QualitÃ© : 40 % â€” minimum 50 % Â»).
- Si **oui** : `relation_level = palier_cible`, enregistrement. Miou adapte.

**RÃ¨gle :** On ne peut jamais monter au-delÃ  de ce que les mÃ©triques autorisent.

---

## 8. Signalement : Miou considÃ¨re un changement de relation

Avant la **proposition formelle** (bulle avec boutons Oui/Pas encore), Miou peut **informer** qu'elle considÃ¨re une Ã©volution. C'est un **signalement** discret, pas une demande.

### 8.1 Trigger Â« signalement considÃ©ration Â»

**Condition :** CritÃ¨res du palier N+1 **presque** rÃ©unis (80 % atteints) mais pas encore tous. Ou : critÃ¨res rÃ©unis mais cooldown refus actif (attendre encore quelques jours).

**CatÃ©gorie :** `signalement_evolution_relation`

**Exemples de bulles :**
- Â« Je commence Ã  mieux te connaÃ®tre â€” on pourrait Ãªtre [palier_propose] bientÃ´t. Â»
- Â« Tu me parles de plus en plus. J'ai l'impression qu'on se rapproche. Â»
- Â« {pseudo}, je sens qu'on pourrait passer Ã  une autre Ã©tape. Quand tu voudras. Â»

**Comportement :** Pas de boutons. Simple information. La proposition formelle viendra quand tous les critÃ¨res seront rÃ©unis.

### 8.2 DiffÃ©rence signalement vs proposition

| Type | Moment | Boutons | Effet |
|------|--------|---------|-------|
| **Signalement** | CritÃ¨res Ã  80 % ou cooldown en cours | Aucun | Information uniquement. Renforce la relation. |
| **Proposition** | Tous critÃ¨res rÃ©unis | Oui / Pas encore / Rester | Confirmation et mise Ã  jour du palier. |

### 8.3 FrÃ©quence des signalements

- Max **1 signalement** par session.
- Cooldown **7 jours** entre deux signalements pour le mÃªme palier.
- Ne pas signaler si une proposition a Ã©tÃ© refusÃ©e dans les 7 derniers jours.

---

## 9. Adaptation du comportement selon le palier

### 9.1 Variantes selon palier

Les **variantes** des catÃ©gories (accueil, pause, retour) diffÃ¨rent selon le palier. Plus le palier est Ã©levÃ©, plus le ton est familier. Voir [Miou - Roadmap et AmÃ©liorations](..//_index.md).

| Palier | Accueil matin | Pause santÃ© | Retour absence |
|--------|---------------|-------------|----------------|
| Inconnue | Â« Bienvenue dans Miyukini Central. Â» | Â« Ã‡a fait 2h â€” une pause ? Â» | Â« Content de te revoir. Â» |
| Connaissance | Â« Bonjour {pseudo}. Â» | Â« {duree} de session â€” accorde-toi une pause. Â» | Â« Ã‡a fait {jours} jours â€” content de te revoir. Â» |
| Pote | Â« Salut {pseudo} ! Â» | Â« 2h â€” pause ? Tu vas en avoir besoin. Â» | Â« Te revoilÃ  ! {jours} jours, c'est long. Â» |
| Amie | Â« Hey {pseudo}, bien dormi ? Â» | Â« Tu t'oublies â€” une pause s'impose. Â» | Â« {pseudo}, tu me manquais. Â» |
| Amie proche+ | Â« Salut toi. Â» | Â« Pause. Je ne veux pas que tu t'Ã©puises. Â» | Â« Enfin. Comment Ã§a va ? Â» |

**ImplÃ©mentation :** Le sÃ©lecteur de variante reÃ§oit `relation_level` ; les templates sont taguÃ©s ou organisÃ©s par palier. Fallback : palier infÃ©rieur si pas de variante pour le palier actuel.

### 9.2 Ton et frÃ©quence

| Palier | Ton des bulles | FrÃ©quence questions | Exemples |
|--------|----------------|---------------------|----------|
| Inconnue | Neutre, accueillant | Aucune question perso | Â« Bienvenue dans Miyukini Central. Â» |
| Connaissance | Courtois | 1 question / 15 jours | Rappels pratiques |
| Pote | DÃ©contractÃ©, complice | 1 question / 10 jours | Â« Tu reviens souvent â€” j'aime Ã§a. Â» |
| Amie | Chaleureux | 1 question / 7 jours | Â« Une pause ? Tu as l'air d'en avoir besoin. Â» |
| Amie proche | Intime | 1 question / 5 jours | RÃ©fÃ©rence aux projets, valeurs |
| Meilleure amie | TrÃ¨s proche | 1 question / 3 jours | Â« Ton rÃªve, tu y penses ? Â» |
| Grande sÅ“ur | Protecteur | Questions plus prÃ©sentes | Â« Je suis lÃ . Tu te rappelles ce qui te fait du bien ? Â» |

---

## 10. Stockage et schÃ©ma

```rust
// Niveau de complicitÃ© (dÃ©rivÃ© des signaux)
enum CompliciteNiveau {
    Faible,    // 0-5
    Modere,    // 6-15
    Bon,       // 16-30
    Eleve,     // 31+
}

// Niveau de relation (palier)
enum RelationLevel {
    Inconnue = 0,
    Connaissance = 1,
    Pote = 2,
    Amie = 3,
    AmieProche = 4,
    MeilleureAmie = 5,
    GrandeSoeur = 6,
}

struct MiouUserProfile {
    profile_id: String,
    relation_level: RelationLevel,
    relation_level_confirmed_at: Option<DateTime>,
    last_level_proposal_at: Option<DateTime>,
    last_level_proposal_refused: bool,
    complicite_score: i32,           // Cumul signaux (rÃ©pond +2, sollicite +1, ignore -0.5, etc.)
    complicite_niveau: CompliciteNiveau,  // Faible, Modere, Bon, Eleve
    last_signalement_at: Option<DateTime>,
    // MÃ©triques connaissance (dÃ©rivÃ©es des rÃ©ponses)
    reponses_total: u32,
    reponses_par_palier: [u32; 7],  // [p0, p1, p2, ...]
    score_qualite: f32,              // 0-1
    couverture_thematique: u32,
}

struct MiouUserResponse {
    id: Uuid,
    profile_id: String,
    question_id: String,      // "q1_1", "q3_2", etc.
    palier: u8,
    response_text: String,   // ChiffrÃ©
    created_at: DateTime,
}
```

---

## 11. RÃ©fÃ©rences

- [Bot - Connaissance Utilisateur et Specs Machine](./Bot%20-%20Connaissance%20Utilisateur%20et%20Specs%20Machine.md)
- [Bot - Intelligence et PersonnalitÃ©](./Bot%20-%20Intelligence%20et%20Personnalite%20de%20Miou.md)
- [Bot - Banque de Templates Volume 2](./Bot%20-%20Banque%20de%20Templates%20Volume%202.md)

---

*Miou apprend Ã  connaÃ®tre l'utilisateur au rythme de leur relation. Les paliers respectent l'intimitÃ© et la progression naturelle.*

*DerniÃ¨re mise Ã  jour : 2026-02-15*

