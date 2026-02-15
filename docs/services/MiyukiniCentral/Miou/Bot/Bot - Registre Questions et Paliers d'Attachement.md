# Bot Miou — Registre de Questions et Paliers d'Attachement

Ce document définit les **paliers de rapport d'attachement** entre Miou et l'utilisateur, le **registre de questions** organisé par palier (ce qu'une personne proche devrait connaître), et le **flux de confirmation** par lequel Miou propose à l'utilisateur de définir le statut de leur relation.

---

## 1. Contexte

Miou est **curieuse du monde réel et de l'utilisateur**. Elle possède un registre de questions qu'une **meilleure amie** pourrait poser pour avoir les informations qu'une personne proche devrait connaître. Les questions et les données sont organisées par **palier de rapport d'attachement**.

C'est à **Miou**, en fonction des critères, de **demander confirmation** à l'utilisateur quant au statut de leur relation. L'utilisateur valide ou ajuste.

Le statut disponible est mesuré par la **quantité**, la **qualité** et la **pertinence** des connaissances de Miou sur l'utilisateur. L'utilisateur **ne peut pas régler un statut élevé** si Miou n'a pas suffisamment d'information.

Miou mesure aussi le **degré de complicité** avec l'utilisateur (répond, sollicite, ignore, ferme, change manuellement le statut).

---

## 2. Les sept paliers d'attachement

| Niveau | Code | Nom | Ton | Ce que Miou connaît / peut demander |
|--------|------|-----|-----|-------------------------------------|
| 0 | `inconnue` | Inconnue | Neutre, accueillant | Rien de personnel. Juste le pseudo, l'heure, le contexte COG. |
| 1 | `connaissance` | Connaissance | Courtois, léger | Préférences pratiques (rappels, ton des bulles). |
| 2 | `pote` | Pote | Décontracté, complice | Hobbies, habitudes de connexion, activité préférée. |
| 3 | `amie` | Amie | Chaleureux, attentif | Ce qui fait du bien, moments de la journée, humeur préférée. |
| 4 | `amie_proche` | Amie proche | Intime, confiant | Valeurs, projets, ce qui compte vraiment. |
| 5 | `meilleure_amie` | Meilleure amie | Très proche, soutenant | Rêves, peurs légères, façon préférée d'être soutenu. |
| 6 | `grande_soeur` | Grande sœur | Protecteur, bienveillant profond | Conseils, protection, présence constante. |

### 2.1 Progression

- **Unidirectionnelle** : On ne descend jamais de palier (sauf demande explicite de l'utilisateur : « Réinitialiser notre relation »).
- **Confirmée par l'utilisateur** : Miou propose une évolution ; l'utilisateur accepte ou refuse.
- **Ni automatique ni imposée** : Miou ne force jamais. Si l'utilisateur refuse ou ignore, le palier reste inchangé.

---

## 3. Degré de complicité

Miou mesure le **degré de complicité** avec l'utilisateur à partir des interactions observées.

### 3.1 Signaux d'interaction

| Signal | Description | Impact complicité |
|--------|-------------|-------------------|
| **Répond** | L'utilisateur répond à une question de Miou (bulle curiosité) | +2 |
| **Sollicite** | L'utilisateur ouvre Paramètres > Miou, consulte « Ce que Miou sait de moi », ou interagit avec une action de bulle | +1 |
| **Ignore** | L'utilisateur ferme une bulle sans répondre (curiosité ou proposition) | -0.5 (léger) |
| **Ferme** | L'utilisateur ferme rapidement une bulle (< 3 s d'affichage) à répétition | -1 |
| **Change manuellement** | L'utilisateur modifie le statut de relation dans Paramètres (si autorisé) | +1 (engagement explicite) |

### 3.2 Calcul du score de complicité

```
complicite_score = base(0) + cumul des signaux sur fenêtre glissante (30 derniers jours)
complicite_niveau = discretisé : faible (0-5), modéré (6-15), bon (16-30), élevé (31+)
```

**Usage :** Le niveau de complicité doit être au moins **modéré** pour proposer une évolution vers Amie (palier 3+). Pour Amie proche et au-delà : **bon** minimum.

---

## 4. Métriques de connaissance (quantité, qualité, pertinence)

Le statut disponible dépend de ce que Miou **connaît** de l'utilisateur.

### 4.1 Quantité

| Métrique | Description |
|----------|-------------|
| `reponses_total` | Nombre total de réponses enregistrées |
| `reponses_palier_N` | Nombre de réponses aux questions du palier N |
| `questions_posees` | Nombre de questions posées (toutes sessions) |
| `taux_reponse` | `reponses_total / questions_posees` (excluant « Passer ») |

### 4.2 Qualité

| Métrique | Description |
|----------|-------------|
| `longueur_moyenne` | Longueur moyenne des réponses (caractères) — une réponse « oui » = faible qualité |
| `reponses_substantielles` | Nombre de réponses avec > 10 caractères (saisie libre) ou choix explicite |
| `score_qualite` | `reponses_substantielles / reponses_total` (0-1) |

### 4.3 Pertinence

| Métrique | Description |
|----------|-------------|
| `reponses_pertinentes_palier` | Réponses qui correspondent au type attendu (pas de hors-sujet manifeste) |
| `couverture_thematique` | Nombre de thèmes différents couverts (préférence, loisir, émotion, projet, etc.) |

### 4.4 Score de connaissance minimal par palier

**L'utilisateur ne peut pas régler un statut élevé si Miou n'a pas suffisamment d'information.**

| Palier cible | Quantité min | Qualité min | Pertinence |
|--------------|--------------|-------------|------------|
| Connaissance (1) | 1 réponse (palier 0 ou 1) | — | — |
| Pote (2) | 2 réponses palier 1, 1 palier 2 | Au moins 1 substantielle | — |
| Amie (3) | 4 réponses palier 1+2, 2 palier 3 | 50 % substantielles | 2 thèmes |
| Amie proche (4) | 6 réponses, 3 palier 4 | 60 % substantielles | 3 thèmes |
| Meilleure amie (5) | 10 réponses, 4 palier 5 | 70 % substantielles | 4 thèmes |
| Grande sœur (6) | 15 réponses, 5 palier 6 | 75 % substantielles | 5 thèmes |

**Règle :** Si l'utilisateur tente de changer manuellement le statut dans Paramètres vers un palier N, Miou vérifie `knowledge_score >= requis_palier_N`. Si non → message : « Je ne te connais pas encore assez pour qu'on soit [palier]. Réponds à quelques questions d'abord. » + affichage des critères manquants.

---

## 5. Critères de proposition d'évolution

Miou propose de passer au palier suivant quand **tous** les critères sont réunis. L'utilisateur confirme.

### 5.1 Critères par palier (complets)

| Évolution | Quantité | Qualité | Pertinence | Complicité | Temps / Fréquence |
|-----------|----------|---------|------------|------------|-------------------|
| Inconnue → Connaissance | 1 session complète | — | — | — | `sessions_total >= 1` |
| Connaissance → Pote | `reponses_palier_1 >= 1`, `sessions >= 3` | — | — | `complicite >= faible` | — |
| Pote → Amie | `reponses_palier_1+2 >= 4`, `reponses_palier_3 >= 2` | `score_qualite >= 0.4` | 2 thèmes | `complicite >= modéré` | `jours_distincts >= 7`, `streak >= 3` |
| Amie → Amie proche | `reponses_palier_1..3 >= 6`, `reponses_palier_4 >= 3` | `score_qualite >= 0.5` | 3 thèmes | `complicite >= modéré` | `jours >= 14`, `dismiss_rate < 0.5` |
| Amie proche → Meilleure amie | `reponses_total >= 10`, `reponses_palier_5 >= 4` | `score_qualite >= 0.6` | 4 thèmes | `complicite >= bon` | `jours >= 30` |
| Meilleure amie → Grande sœur | `reponses_total >= 15`, `reponses_palier_6 >= 5` | `score_qualite >= 0.7` | 5 thèmes | `complicite >= élevé` | `jours >= 60` |

### 5.2 Règles de proposition

| Règle | Description |
|-------|-------------|
| **Une proposition par session max** | Miou ne propose pas deux évolutions dans la même session. |
| **Cooldown après refus** | Si l'utilisateur refuse : ne pas reproposer avant 14 jours. |
| **Pas en début de session** | Proposer après au moins une bulle « normale » (accueil, rappel, etc.). |
| **Ton adapté** | La formulation reflète le palier actuel et le palier proposé. |

---

## 6. Registre de questions par palier

Chaque question est associée à un **palier minimum**. Miou ne pose une question que si `relation_level >= question_palier`.

### 6.1 Palier 1 — Connaissance

Questions qu'on pose à quelqu'un qu'on vient de rencontrer.

| ID | Question | Type réponses | Donnée stockée |
|----|----------|----------------|----------------|
| q1_1 | Tu préfères le matin ou le soir pour mes rappels ? | Matin / Soir / Peu importe | `preference_rappel` |
| q1_2 | Tu préfères que je sois discrète ou un peu bavarde ? | Discrète / Bavarde / Comme maintenant | `preference_ton` |
| q1_3 | Tu travailles plutôt du bureau ou de chez toi ? | Bureau / Maison / Les deux | `contexte_activite` |
| q1_4 | Quel moment de la journée tu préfères pour ton COG ? | Matin / Après-midi / Soir | `moment_prefere` |

### 6.2 Palier 2 — Pote

Questions pour apprendre à connaître quelqu'un avec qui on se sent à l'aise.

| ID | Question | Type réponses | Donnée stockée |
|----|----------|----------------|----------------|
| q2_1 | Tu aimes lire ? Quel genre ? | (saisie libre) | `loisir_lecture` |
| q2_2 | Tu as un hobby préféré ? | (saisie libre) | `hobby` |
| q2_3 | Tu préfères les journées chargées ou tranquilles ? | Chargées / Tranquilles / Ça dépend | `rythme_prefere` |
| q2_4 | Tu es du matin ou du soir ? | Matin / Soir / Les deux | `chronotype` |
| q2_5 | Tu as une activité qui te déconnecte ? (sport, musique…) | (saisie libre) | `activite_deconnexion` |

### 6.3 Palier 3 — Amie

Questions pour mieux comprendre ce qui fait du bien.

| ID | Question | Type réponses | Donnée stockée |
|----|----------|----------------|----------------|
| q3_1 | Qu'est-ce qui te fait du bien après une journée difficile ? | (saisie libre) | `reconfort` |
| q3_2 | Tu préfères qu'on te laisse de l'espace ou qu'on soit présente ? | Espace / Présente / Les deux selon le jour | `besoin_presence` |
| q3_3 | Tu as un endroit favori (virtuel ou réel) pour te ressourcer ? | (saisie libre) | `lieu_ressource` |
| q3_4 | Qu'est-ce qui te motive en ce moment ? | (saisie libre courte) | `motivation_actuelle` |
| q3_5 | Tu préfères les surprises ou d'être prévenu à l'avance ? | Surprises / Prévenu / Les deux | `preference_surprise` |

### 6.4 Palier 4 — Amie proche

Questions sur ce qui compte vraiment.

| ID | Question | Type réponses | Donnée stockée |
|----|----------|----------------|----------------|
| q4_1 | Qu'est-ce qui compte le plus pour toi en ce moment ? | (saisie libre) | `valeur_actuelle` |
| q4_2 | Tu as un projet qui te tient à cœur ? | (saisie libre courte) | `projet_coeur` |
| q4_3 | Qu'est-ce qui te rend fier ou fière ? | (saisie libre) | `source_fierte` |
| q4_4 | Tu préfères qu'on te pousse un peu ou qu'on te laisse avancer à ton rythme ? | Pousser / Mon rythme / Les deux | `style_accompagnement` |
| q4_5 | C'est quoi pour toi une bonne journée ? | (saisie libre courte) | `bonne_journee` |

### 6.5 Palier 5 — Meilleure amie

Questions sur les rêves et le soutien.

| ID | Question | Type réponses | Donnée stockée |
|----|----------|----------------|----------------|
| q5_1 | Tu as un rêve que tu aimerais réaliser ? | (saisie libre) | `reve` |
| q5_2 | Comment tu préfères qu'on te soutienne quand ça va mal ? | (saisie libre ou choix) | `soutien_prefere` |
| q5_3 | Qu'est-ce qui te fait peur (sans être trop intime) ? | (saisie libre) | `peur_legere` |
| q5_4 | Tu as une personne qui t'inspire ? | (saisie libre courte) | `inspiration` |
| q5_5 | C'est quoi ton petit bonheur du quotidien ? | (saisie libre) | `bonheur_quotidien` |

### 6.6 Palier 6 — Grande sœur

Questions de présence protectrice et bienveillante.

| ID | Question | Type réponses | Donnée stockée |
|----|----------|----------------|----------------|
| q6_1 | Y a-t-il des moments où tu aimerais qu'on soit plus présente ? | (saisie libre ou choix) | `moment_plus_presente` |
| q6_2 | Tu préfères un conseil direct ou qu'on te pose des questions pour réfléchir ? | Direct / Questions / Les deux | `style_conseil` |
| q6_3 | Qu'est-ce qui te rassure ? | (saisie libre) | `reassurance` |
| q6_4 | Comment on peut t'aider à prendre soin de toi ? | (saisie libre) | `aide_soin` |
| q6_5 | Tu as besoin qu'on te rappelle quelque chose en particulier ? | (saisie libre) | `rappel_personnalise` |

---

## 7. Flux de confirmation du palier

### 7.1 Déclenchement

```
1. Critères du palier N+1 sont réunis
2. relation_level == N (pas déjà au max)
3. Aucune proposition refusée dans les 14 derniers jours
4. Au moins une bulle « normale » affichée cette session
5. Pas de proposition déjà faite cette session
```

### 7.2 Bulle de proposition

Miou affiche une bulle avec :

- **Message** : variante selon palier actuel et palier proposé (voir templates)
- **Boutons** : « Oui, on est [palier_propose] » / « Pas encore » / « Rester [palier_actuel] »

### 7.3 Réponses

| Action utilisateur | Effet |
|--------------------|-------|
| « Oui, on est [palier_propose] » | `relation_level = N+1`. Enregistrement. Miou adapte ton et questions. |
| « Pas encore » | Aucun changement. Cooldown 14 jours avant reproposition. |
| « Rester [palier_actuel] » | Idem. L'utilisateur préfère garder le statut actuel. |
| Fermeture sans clic | Aucun changement. Peut reproposer à la prochaine session si critères toujours réunis. |

### 7.4 Réinitialisation

Dans Paramètres > Miou > « Ce que Miou sait de moi » :
- Option **« Réinitialiser notre relation »** → `relation_level = 0`, efface les réponses (ou propose de garder les réponses tout en revenant à « inconnue » — à définir).

### 7.5 Changement manuel du statut (par l'utilisateur)

Dans Paramètres > Miou > « Statut de notre relation » :
- L'utilisateur peut **proposer** un palier cible.
- **Vérification** : Miou vérifie si `knowledge_score >= requis_palier_cible` ET `complicite_niveau >= requis`.
- Si **non** : message « Je ne te connais pas encore assez pour qu'on soit [palier]. Réponds à quelques questions d'abord. » + affichage des critères manquants (ex. « 2 réponses palier 3 manquantes », « Qualité : 40 % — minimum 50 % »).
- Si **oui** : `relation_level = palier_cible`, enregistrement. Miou adapte.

**Règle :** On ne peut jamais monter au-delà de ce que les métriques autorisent.

---

## 8. Signalement : Miou considère un changement de relation

Avant la **proposition formelle** (bulle avec boutons Oui/Pas encore), Miou peut **informer** qu'elle considère une évolution. C'est un **signalement** discret, pas une demande.

### 8.1 Trigger « signalement considération »

**Condition :** Critères du palier N+1 **presque** réunis (80 % atteints) mais pas encore tous. Ou : critères réunis mais cooldown refus actif (attendre encore quelques jours).

**Catégorie :** `signalement_evolution_relation`

**Exemples de bulles :**
- « Je commence à mieux te connaître — on pourrait être [palier_propose] bientôt. »
- « Tu me parles de plus en plus. J'ai l'impression qu'on se rapproche. »
- « {pseudo}, je sens qu'on pourrait passer à une autre étape. Quand tu voudras. »

**Comportement :** Pas de boutons. Simple information. La proposition formelle viendra quand tous les critères seront réunis.

### 8.2 Différence signalement vs proposition

| Type | Moment | Boutons | Effet |
|------|--------|---------|-------|
| **Signalement** | Critères à 80 % ou cooldown en cours | Aucun | Information uniquement. Renforce la relation. |
| **Proposition** | Tous critères réunis | Oui / Pas encore / Rester | Confirmation et mise à jour du palier. |

### 8.3 Fréquence des signalements

- Max **1 signalement** par session.
- Cooldown **7 jours** entre deux signalements pour le même palier.
- Ne pas signaler si une proposition a été refusée dans les 7 derniers jours.

---

## 9. Adaptation du comportement selon le palier

### 9.1 Variantes selon palier

Les **variantes** des catégories (accueil, pause, retour) diffèrent selon le palier. Plus le palier est élevé, plus le ton est familier. Voir [Miou - Roadmap et Améliorations](../Miou%20-%20Roadmap%20et%20Améliorations.md).

| Palier | Accueil matin | Pause santé | Retour absence |
|--------|---------------|-------------|----------------|
| Inconnue | « Bienvenue dans Miyukini Central. » | « Ça fait 2h — une pause ? » | « Content de te revoir. » |
| Connaissance | « Bonjour {pseudo}. » | « {duree} de session — accorde-toi une pause. » | « Ça fait {jours} jours — content de te revoir. » |
| Pote | « Salut {pseudo} ! » | « 2h — pause ? Tu vas en avoir besoin. » | « Te revoilà ! {jours} jours, c'est long. » |
| Amie | « Hey {pseudo}, bien dormi ? » | « Tu t'oublies — une pause s'impose. » | « {pseudo}, tu me manquais. » |
| Amie proche+ | « Salut toi. » | « Pause. Je ne veux pas que tu t'épuises. » | « Enfin. Comment ça va ? » |

**Implémentation :** Le sélecteur de variante reçoit `relation_level` ; les templates sont tagués ou organisés par palier. Fallback : palier inférieur si pas de variante pour le palier actuel.

### 9.2 Ton et fréquence

| Palier | Ton des bulles | Fréquence questions | Exemples |
|--------|----------------|---------------------|----------|
| Inconnue | Neutre, accueillant | Aucune question perso | « Bienvenue dans Miyukini Central. » |
| Connaissance | Courtois | 1 question / 15 jours | Rappels pratiques |
| Pote | Décontracté, complice | 1 question / 10 jours | « Tu reviens souvent — j'aime ça. » |
| Amie | Chaleureux | 1 question / 7 jours | « Une pause ? Tu as l'air d'en avoir besoin. » |
| Amie proche | Intime | 1 question / 5 jours | Référence aux projets, valeurs |
| Meilleure amie | Très proche | 1 question / 3 jours | « Ton rêve, tu y penses ? » |
| Grande sœur | Protecteur | Questions plus présentes | « Je suis là. Tu te rappelles ce qui te fait du bien ? » |

---

## 10. Stockage et schéma

```rust
// Niveau de complicité (dérivé des signaux)
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
    complicite_score: i32,           // Cumul signaux (répond +2, sollicite +1, ignore -0.5, etc.)
    complicite_niveau: CompliciteNiveau,  // Faible, Modere, Bon, Eleve
    last_signalement_at: Option<DateTime>,
    // Métriques connaissance (dérivées des réponses)
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
    response_text: String,   // Chiffré
    created_at: DateTime,
}
```

---

## 11. Références

- [Bot - Connaissance Utilisateur et Specs Machine](./Bot%20-%20Connaissance%20Utilisateur%20et%20Specs%20Machine.md)
- [Bot - Intelligence et Personnalité](./Bot%20-%20Intelligence%20et%20Personnalite%20de%20Miou.md)
- [Bot - Banque de Templates Volume 2](./Bot%20-%20Banque%20de%20Templates%20Volume%202.md)

---

*Miou apprend à connaître l'utilisateur au rythme de leur relation. Les paliers respectent l'intimité et la progression naturelle.*

*Dernière mise à jour : 2026-02-15*
