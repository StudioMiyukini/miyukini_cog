# JayEcole — Brainstorming Approfondissement

## Contexte

Ce document est la suite du brainstorming initial (`BRAINSTORM-initial.md`). Il approfondit six axes cles identifies lors de la phase de cadrage, apres validation des decisions par l'utilisateur.

## Portee / Scope

- **Applicable a :** Phase d'approfondissement, pre-Document Fondateur.
- **Audience :** Equipe projet, Denis (traduction technique), Lise (maquettes UI).
- **Statut :** Brainstorming approfondi (Phase 3 du protocole).
- **Prerequis :** BRAINSTORM-initial.md valide, decisions prises.
- **Redige par :** Maria (Chef de Projet)
- **Date :** 2026-02-27

### Decisions en vigueur

| Decision | Valeur |
|----------|--------|
| Nom du service | **JayEcole** |
| Scope MVP | Francais + Mathematiques |
| Cible geographique | France (Education Nationale) |
| Modele economique | Gratuit |
| Priorite roadmap | Moyenne |
| Miyu-sensei | Humanoide, cree de toutes pieces |
| Gardiens | Tous des animaux |
| Ton narratif | **PIVOT : artisan/construction** (remplace "brouillard de l'Oubli") |
| Source contenu | Tout from scratch, validation par une institutrice |
| Accessibilite DYS | Dans le MVP |
| Placement | Service lance separement (binaire independant) |
| Audio | Consignes audio dans le MVP |
| Tests enfants | Disponibles dans l'entourage |
| Niveau prioritaire | **CP** |

**ATTENTION :** Les decisions round 2 entrainent des pivots majeurs sur la narration (section 4) et l'architecture technique (service separe). Voir `BRAINSTORM-round3-construction.md` pour les approfondissements.

---

## 1. Parcours utilisateur detaille

### 1.1 Premier lancement (onboarding)

Le premier lancement est critique : il doit rassurer le parent ET enthousiastmer l'enfant.

**Etape 1 — Lancement de JayEcole**

L'utilisateur lance JayEcole (binaire independant, lance depuis le bureau ou depuis Central). Ecran de bienvenue avec Miyu-sensei qui salue.

> **NOTE (round 2) :** JayEcole est un service lance separement, pas un onglet de Central. Voir BRAINSTORM-round3-construction.md section F pour les implications techniques.

```
[Ecran : Bienvenue dans JayEcole !]
Miyu-sensei : "Bonjour ! Je suis Miyu-sensei, ton compagnon d'apprentissage !"
[Bouton : Commencer l'aventure]
```

**Etape 2 — Creation du profil parent (si premier enfant)**

- Saisie du code PIN parental (4-6 chiffres)
- Confirmation du PIN
- Acceptation des CGU (simplifie, langage clair)
- Le parent est informe : "Les donnees restent sur votre ordinateur. Rien n'est envoye sur internet."

**Etape 3 — Creation du profil enfant**

Le parent cree le profil de l'enfant :
- Prenom ou pseudo (affiche dans l'app)
- Niveau scolaire (CP, CE1, CE2, CM1, CM2) — menu deroulant
- Annee de naissance (optionnel, pour ajuster l'interface)

Puis l'enfant personnalise son avatar :
- Choix parmi 8-12 avatars pre-dessines (style Miyukini)
- Choix d'une couleur preferee (utilise pour la palette de l'interface)
- Le parent peut assister mais l'enfant manipule

**Etape 4 — Evaluation diagnostique ("Le Grand Voyage")**

L'evaluation est presentee comme le debut de l'aventure, pas comme un test.

```
Miyu-sensei : "Avant de partir a l'aventure, je vais te poser quelques questions
pour savoir quels chemins te preparer. Il n'y a pas de mauvaise reponse !"
```

Deroulement :
1. **Bloc Francais** (8-12 questions adaptatives)
   - Commence au niveau declare par le parent
   - Questions de difficulte croissante si reussite, decroissante si echec
   - Types : QCM illustres, textes a trous, association
   - Duree cible : 5-8 minutes
   - Miyu-sensei encourage entre les questions

2. **Micro-pause** (30 secondes)
   - Miyu-sensei : "Super ! On fait une petite pause. Etire tes bras !"

3. **Bloc Mathematiques** (8-12 questions adaptatives)
   - Meme logique que le francais
   - Types : calcul mental, comparaison de nombres, geometrie simple
   - Duree cible : 5-8 minutes

4. **Ecran de resultat (pour l'enfant)**
   - Pas de note. Pas de pourcentage.
   - "Ta carte du savoir est prete ! Regarde tous les endroits que tu vas explorer !"
   - La carte s'illumine avec les regions accessibles
   - Les regions ou l'enfant est fort brillent plus

5. **Ecran de resultat (pour le parent — acces PIN)**
   - Portrait scolaire detaille par competence
   - Niveau estime par domaine (graphique radar)
   - Recommandation de session : duree, frequence

**Etape 5 — Premiere session guidee**

Miyu-sensei guide l'enfant vers son premier exercice :
```
"Regarde la carte ! Je te propose de commencer par [region la plus adaptee].
Clique sur le premier village !"
```

L'enfant complete 3-5 exercices encadres avec explications enrichies. Premiere recompense : badge "Premier Pas".

### 1.2 Session type (utilisation quotidienne)

**Duree cible :** 15-25 minutes selon l'age et les parametres parentaux.

```
Minute 0     : Lancement — Ecran d'accueil
               Miyu-sensei : "Content de te revoir ! On reprend l'aventure ?"
               [Affichage du streak et des etoiles de la veille]

Minute 0-1   : Choix du parcours
               Option A : "Continuer l'aventure" (parcours recommande)
               Option B : "Choisir une region" (libre, sur la carte)
               Option C : "Reviser" (retour sur les points faibles)

Minute 1-10  : Bloc d'exercices n.1 (4-6 exercices)
               - Alternance de types (QCM, trous, association, calcul)
               - Correction immediate avec explication courte
               - Etoiles et animations a chaque reponse
               - Si 3 erreurs consecutives : indice de Miyu-sensei

Minute 10    : PAUSE ACTIVE (30-60 secondes)
               Miyu-sensei propose un exercice physique
               "Leve-toi et fais 5 sauts comme un kangourou !"
               [Bouton : "C'est fait !"]
               [L'enfant ne peut pas passer la pause les premieres semaines]

Minute 10-19 : Bloc d'exercices n.2 (4-6 exercices)
               - Peut etre une matiere differente du bloc 1
               - Ou approfondissement si le bloc 1 a revele une difficulte
               - Meme logique de correction et recompense

Minute 19    : ALERTE 5 MINUTES (si session de 20 min)
               Miyu-sensei : "Il te reste 5 minutes !"

Minute 19-24 : Derniers exercices ou recapitulatif
               - Si en cours d'exercice a la fin du temps : on le termine
               - Jamais de coupure brutale

Minute 24    : FIN DE SESSION
               Ecran de bilan de session
```

### 1.3 Fin de session

L'ecran de fin de session est concu pour etre satisfaisant et non frustrant :

```
[Ecran : Bravo pour cette session !]

Aujourd'hui tu as :
  - Complete 10 exercices
  - Gagne 24 etoiles
  - Libere 1 nouveau village : "Le Moulin des Additions"
  - Obtenu le badge : "Rapide comme l'eclair" (5 bonnes reponses d'affilee)

[Animation : Miyu-sensei danse avec des confettis]

[Bouton principal : "A demain !"]
[Bouton secondaire : "Voir mon profil"]
[Pas de bouton "Encore une session" si la limite quotidienne est atteinte]
```

Si l'enfant tente de relancer une session avant le cooldown :
```
Miyu-sensei : "Tu as bien travaille ! Repose-toi un peu, on se retrouve
dans [temps restant]. Va jouer dehors !"
```

### 1.4 Retour le lendemain

A la reconnexion le lendemain, le systeme adapte l'accueil :

**Si l'enfant a une serie en cours (streak) :**
```
Miyu-sensei : "3 jours d'affilee ! Continue comme ca, tu es formidable !"
[Affichage flamme de streak]
```

**Si l'enfant revient apres 1-2 jours d'absence :**
```
Miyu-sensei : "Te revoila ! J'ai prepare un petit rappel pour se remettre en forme."
[Mini-revision de 2-3 exercices sur les notions recentes avant de continuer]
```

**Si l'enfant revient apres 3+ jours d'absence :**
```
Miyu-sensei : "Ca me fait plaisir de te revoir ! On va reprendre en douceur."
[Revision plus longue : 5-6 exercices de rappel]
[Le parcours ne recule pas — on ne punit jamais l'absence]
```

**Si c'est une nouvelle periode scolaire / rentree :**
```
Miyu-sensei : "Nouvelle periode ! On fait un petit bilan rapide ?"
[Mini-diagnostic de 5-6 questions pour re-calibrer]
```

---

## 2. Contenu pedagogique

### 2.1 Strategie de creation du contenu

Le contenu est le **poste le plus lourd** du projet. Strategie en 3 phases :

**Phase 1 — Noyau manuel (MVP)**
- Creation manuelle des exercices par l'equipe
- Reference : programmes officiels Education Nationale (Bulletin officiel)
- Cible : 50-80 exercices par domaine et par niveau
- Estimation : ~1000-1500 exercices pour le MVP (2 matieres x 5 niveaux x 6-8 domaines x ~20 exercices)

**Phase 2 — Generation assistee (v1.0)**
- Utilisation de miou-llm-bridge pour generer des variantes d'exercices a partir de templates
- Chaque exercice genere est valide par un humain avant publication
- Objectif : multiplier par 3-5 la banque d'exercices

**Phase 3 — Templates parametriques (v1.1+)**
- Certains types d'exercices peuvent etre generes a l'infini par parametrage
- Exemple : calcul mental — generer automatiquement "34 + 27 = ?" avec des nombres aleatoires dans une fourchette de difficulte
- Exemple : conjugaison — generer "Conjugue le verbe [verbe] au [temps] a la [personne]" en piochant dans une liste

### 2.2 Format de donnees des exercices

Chaque exercice est un document JSON stocke dans KindMother. Le format est extensible pour supporter de nouveaux types.

**Schema generique d'un exercice :**

```json
{
  "id": "fr-cp-gram-001",
  "version": 1,
  "metadata": {
    "school_level": "CP",
    "subject": "francais",
    "domain": "grammaire",
    "chapter": "la_phrase",
    "skill_ids": ["fr.grammaire.phrase_simple"],
    "difficulty": 0.3,
    "estimated_time_seconds": 30,
    "tags": ["phrase", "majuscule", "point"]
  },
  "content": {
    "type": "fill_blank",
    "instruction": "Complete la phrase avec le bon mot.",
    "instruction_audio": "assets/audio/instructions/complete_phrase.ogg",
    "body": {
      "text_before": "Le chat ",
      "blank": {
        "correct_answer": "dort",
        "accept_variants": ["dort"],
        "distractors": ["dors", "dormir", "dorment"]
      },
      "text_after": " sur le canape."
    },
    "illustration": "assets/images/exercises/chat_canape.svg",
    "hint": "C'est le chat qui fait l'action. Un seul chat.",
    "explanation": {
      "on_correct": "Bravo ! 'Le chat dort' — le chat est au singulier, alors on conjugue au singulier.",
      "on_incorrect": "Le chat est un seul chat (singulier). On dit : il dort. Donc 'Le chat dort'."
    }
  }
}
```

**Types d'exercices supportes et leur schema `content.body` :**

| Type | `content.type` | Structure `body` |
|------|----------------|------------------|
| QCM | `multiple_choice` | `question`, `choices[]` (texte+id), `correct_id`, `illustration?` |
| Texte a trous | `fill_blank` | `text_before`, `blank` (reponse+variantes+distracteurs), `text_after` |
| Association | `matching` | `pairs[]` (left+right), `shuffle: true` |
| Ordre | `ordering` | `items[]` a remettre dans l'ordre, `correct_order[]` |
| Glisser-deposer | `drag_drop` | `zones[]` (id+label), `items[]` (id+content+target_zone_id) |
| Calcul libre | `numeric_input` | `question`, `correct_value`, `tolerance?`, `unit?` |
| Vrai/Faux | `true_false` | `statement`, `correct: bool` |
| Classification | `categorize` | `categories[]`, `items[]` (content+category_id) |

**Convention d'identifiant des exercices :**
```
{matiere_code}-{niveau_code}-{domaine_code}-{numero_sequentiel}

Exemples :
  fr-cp-lect-001    = Francais, CP, Lecture, exercice 1
  ma-ce2-calc-042   = Mathematiques, CE2, Calcul, exercice 42
  fr-cm1-conj-015   = Francais, CM1, Conjugaison, exercice 15
```

### 2.3 Arbre de competences (skill tree)

L'arbre de competences est la colonne vertebrale du parcours. Il definit les prerequis entre notions.

**Extrait de l'arbre Mathematiques (CP -> CE1) :**

```
math.numeration
  |-- math.numeration.0_10          [CP, P1]    Connaitre les nombres de 0 a 10
  |     |-- math.numeration.0_20    [CP, P2]    Connaitre les nombres de 0 a 20
  |     |     |-- math.numeration.0_100   [CP, P4-5]  Connaitre les nombres de 0 a 100
  |     |           |-- math.numeration.0_1000  [CE1, P1-2]  Nombres de 0 a 1000
  |
  |-- math.calcul
        |-- math.calcul.addition_simple     [CP, P2]   Additions < 10
        |     |-- math.calcul.addition_20   [CP, P3]   Additions < 20
        |     |     |-- math.calcul.addition_100  [CE1]  Additions < 100
        |     |
        |     |-- math.calcul.soustraction_simple [CP, P3]  Soustractions < 10
        |           |-- math.calcul.soustraction_20  [CP, P5]  Soustractions < 20
        |
        |-- math.calcul.mental
              |-- math.calcul.doubles       [CP, P2]   Les doubles (2+2, 3+3...)
              |-- math.calcul.complements_10 [CP, P3]  Complements a 10 (3+?=10)
```

**Extrait de l'arbre Francais (CP -> CE1) :**

```
fr.lecture
  |-- fr.lecture.lettres           [CP, P1]    Reconnaitre les lettres
  |     |-- fr.lecture.syllabes    [CP, P1-2]  Combiner lettres en syllabes
  |     |     |-- fr.lecture.mots_simples [CP, P2-3]  Lire des mots simples
  |     |           |-- fr.lecture.phrases  [CP, P3-4]  Lire des phrases
  |     |                 |-- fr.lecture.texte_court [CP, P5]  Lire un petit texte
  |
  |-- fr.grammaire
        |-- fr.grammaire.phrase_simple  [CP, P3]   La phrase (majuscule, point)
        |     |-- fr.grammaire.nom      [CE1, P1]  Le nom
        |     |-- fr.grammaire.verbe    [CE1, P2]  Le verbe
        |     |-- fr.grammaire.determinant [CE1, P2] Le determinant
```

Chaque noeud de l'arbre a :
- Un identifiant unique (`skill_id`)
- Un niveau scolaire cible
- Une periode scolaire cible (P1 a P5)
- Des prerequis (noeuds parents)
- Un seuil de maitrise (ex: 80% de reussite sur 10 exercices = maitrise)

### 2.4 Organisation du contenu embarque

Le contenu est embarque dans l'application (conformement a LOI-1 et LOI-2) :

```
assets/
  jayecole/
    curriculum/
      skill_tree.json              -- Arbre de competences complet
      chapters.json                -- Liste des chapitres par niveau/matiere
    exercises/
      fr/
        cp/
          lecture/
            fr-cp-lect-001.json
            fr-cp-lect-002.json
            ...
          grammaire/
            fr-cp-gram-001.json
            ...
        ce1/
          ...
      ma/
        cp/
          numeration/
            ma-cp-num-001.json
            ...
          calcul/
            ...
        ce1/
          ...
    images/
      exercises/                   -- Illustrations des exercices (SVG prefere)
      map/                         -- Elements de la carte du savoir
      mascot/                      -- Sprites de Miyu-sensei
      avatars/                     -- Avatars disponibles
      badges/                      -- Icones de badges
    audio/
      instructions/                -- Consignes audio (pour CP-CE1)
      mascot/                      -- Voix de Miyu-sensei
      effects/                     -- Sons de recompense, erreur, etc.
```

**Estimation de taille :**
- Exercices JSON : ~500 KB pour 1500 exercices (tres leger)
- Images SVG : ~5-10 MB (illustrations vectorielles)
- Audio (optionnel) : ~50-100 MB (compresse en OGG/Opus)
- Total estime : **60-110 MB** embarques dans l'application

---

## 3. Moteur adaptatif MVP

### 3.1 Philosophie

Pour le MVP, le moteur adaptatif est **simplifie**. On n'implemente pas un veritable algorithme IRT (Item Response Theory) qui necesite une calibration statistique complexe. On utilise un systeme de **difficulte par paliers** avec des regles simples.

Le veritable moteur adaptatif (IRT/CAT) est prevu pour la v1.0.

### 3.2 Algorithme simplifie pour le MVP

**Principe : "3 niveaux de difficulte + escalade/descente"**

Chaque exercice a un attribut `difficulty` entre 0.0 et 1.0, discretise en 3 paliers :

| Palier | Plage difficulty | Signification |
|--------|-----------------|---------------|
| Facile | 0.0 - 0.33 | Notions de base, rappels |
| Moyen | 0.34 - 0.66 | Attendu du niveau scolaire |
| Difficile | 0.67 - 1.0 | Approfondissement, defi |

**Regles d'adaptation :**

```
Etat initial :
  - L'evaluation diagnostique positionne l'enfant sur un palier par competence
  - Si pas encore evalue : on commence au palier "Moyen"

Apres chaque exercice :
  - Maintenir un compteur de reussite glissant sur les 5 derniers exercices du domaine

  SI 4 reussites sur 5 au palier actuel :
    → Monter au palier superieur
    → Miyu-sensei : "Tu progresses vite ! On essaie quelque chose de plus fort ?"

  SI 2 echecs sur 5 au palier actuel :
    → Descendre au palier inferieur
    → Miyu-sensei : "On va revoir les bases, c'est important !"

  SI 3 reussites sur 5 :
    → Rester au meme palier
    → Continuer normalement

Cas particulier — blocage :
  SI l'enfant est au palier "Facile" et echoue encore :
    → Declencher un exercice de type "lecon + exemple resolu"
    → Puis re-proposer un exercice similaire
    → Si echec persistant (3 exercices de revision echoues) :
      → Alerter le parent : "Votre enfant a des difficultes avec [competence]"
```

**Selection de l'exercice suivant :**

```
1. Determiner la competence cible :
   - Si parcours sequentiel : prochaine competence dans le chapitre en cours
   - Si mode revision : competence avec le score le plus bas
   - Si mode libre : competence choisie par l'enfant sur la carte

2. Determiner le palier de difficulte (selon les regles ci-dessus)

3. Selectionner un exercice :
   - Filtre : meme competence, meme palier de difficulte
   - Exclure les exercices deja faits dans la session en cours
   - Exclure les exercices faits dans les 3 dernieres sessions (eviter la repetition)
   - Si plusieurs candidats : choisir aleatoirement (avec graine RNG locale)

4. Si aucun exercice disponible au palier :
   - Tenter le palier adjacent
   - Si toujours rien : marquer la competence comme "epuisee" et passer a la suivante
```

### 3.3 Scoring simplifie

Chaque competence a un score entre 0.0 et 1.0 :

```
score(competence) = moyenne_ponderee des 20 derniers exercices de cette competence

  Ponderation :
    - Exercice recent (derniere session) : poids 3
    - Exercice avant-derniere session : poids 2
    - Exercice plus ancien : poids 1

  Bonus :
    - Reponse correcte rapide (< 50% du temps estime) : +0.1 bonus
    - Reponse correcte sans indice : +0.05 bonus

  Penalite :
    - Reponse apres indice : pas de penalite mais pas de bonus
    - Pas de penalite negative (le score minimum est 0.0)
```

**Seuils de maitrise :**

| Score | Signification | Affichage |
|-------|---------------|-----------|
| 0.0 - 0.3 | Non acquis | Etoile vide |
| 0.3 - 0.6 | En cours d'acquisition | Etoile a moitie |
| 0.6 - 0.8 | Acquis | Etoile pleine |
| 0.8 - 1.0 | Maitrises | Etoile doree |

### 3.4 Pseudocode Rust (esquisse)

```rust
/// Determine le prochain exercice pour un enfant dans un domaine donne
fn select_next_exercise(
    child: &ChildProfile,
    target_skill: &SkillId,
    session_history: &[ExerciseResult],
    exercise_bank: &ExerciseBank,
) -> Option<Exercise> {
    // 1. Determiner le palier de difficulte actuel
    let recent_results = child.recent_results(target_skill, 5);
    let success_count = recent_results.iter().filter(|r| r.correct).count();
    let current_tier = child.skill_tier(target_skill);

    let target_tier = match (current_tier, success_count) {
        (tier, s) if s >= 4 => tier.up(),     // Monter
        (tier, s) if s <= 2 => tier.down(),   // Descendre
        (tier, _) => tier,                     // Rester
    };

    // 2. Filtrer les exercices candidats
    let session_ids: HashSet<_> = session_history.iter().map(|r| &r.exercise_id).collect();
    let recent_ids = child.recent_exercise_ids(target_skill, 3); // 3 dernieres sessions

    let candidates: Vec<_> = exercise_bank
        .query(target_skill, target_tier)
        .filter(|ex| !session_ids.contains(&ex.id))
        .filter(|ex| !recent_ids.contains(&ex.id))
        .collect();

    // 3. Selection aleatoire
    if candidates.is_empty() {
        // Essayer un palier adjacent
        let fallback = exercise_bank.query(target_skill, target_tier.adjacent());
        fallback.choose(&mut child.rng())
    } else {
        candidates.choose(&mut child.rng())
    }
}
```

---

## 4. Mascotte et narration

> **PIVOT ROUND 2 :** Le narratif "brouillard de l'Oubli / liberation du monde" est abandonne au profit d'un ton **artisan/construction**. L'enfant ne "libere" plus un monde, il **construit** quelque chose. Ce pivot est approfondi dans `BRAINSTORM-round3-construction.md` section A. La section ci-dessous est conservee a titre d'historique mais est **OBSOLETE**.

### 4.1 L'univers narratif : ~~Le Monde du Savoir~~ OBSOLETE — voir round 3

~~JayEcole se deroule dans un monde imaginaire appele **le Monde du Savoir**. Ce monde est compose de regions thematiques que l'enfant explore au fil de son apprentissage.~~

~~**Mythe fondateur (raconte par Miyu-sensei lors du premier lancement) :**~~

> ~~"Il etait une fois un monde merveilleux ou les lettres, les chiffres et les idees
> vivaient en harmonie. Mais un jour, un brouillard appele l'Oubli a recouvert
> les regions du monde. Les villages se sont endormis et les chemins se sont effaces.
> Toi, avec mon aide, tu vas dissiper le brouillard en apprenant et en resolvant
> des enigmes. Chaque exercice reussi fait briller un peu plus le Monde du Savoir !"~~

**NOUVEAU CONCEPT :** L'enfant est un apprenti constructeur/artisan. Chaque exercice reussi lui fournit des materiaux, des plans ou des outils pour construire son projet. Voir `BRAINSTORM-round3-construction.md` pour les propositions detaillees.

### 4.2 Les regions de la Carte du Savoir (detail)

Chaque region a une identite visuelle, un gardien (PNJ secondaire), et une ambiance :

**Region : Les Collines des Nombres** (Mathematiques — Numeration)
- Ambiance : collines verdoyantes avec des chiffres geants sculptes dans la roche
- Gardien : Compti le herisson — petit, meticuleux, compte toujours tout
- Villages : "Le Village des Unites", "Le Bourg des Dizaines", "La Cite des Centaines"
- Couleur dominante : bleu

**Region : La Foret des Mots** (Francais — Vocabulaire, Orthographe)
- Ambiance : foret enchantee ou les arbres ont des lettres comme feuilles
- Gardien : Lexia la chouette — sage, aime les mots compliques
- Villages : "La Clairiere des Voyelles", "Le Sentier des Consonnes", "Le Grand Chene des Mots"
- Couleur dominante : vert

**Region : Le Lac des Histoires** (Francais — Lecture, Comprehension)
- Ambiance : lac paisible avec des iles-livres flottantes
- Gardien : Conte le poisson-lune — reveur, raconte des histoires
- Villages : "L'Ile des Syllabes", "Le Phare des Phrases", "L'Archipel des Textes"
- Couleur dominante : turquoise

**Region : La Montagne des Calculs** (Mathematiques — Operations)
- Ambiance : montagne enneigee avec des mines de cristaux-chiffres
- Gardien : Somma l'ours — grand, fort, aime les gros calculs
- Villages : "Le Camp des Additions", "La Grotte des Soustractions", "Le Pic des Multiplications"
- Couleur dominante : orange

**Region : Le Chateau de la Grammaire** (Francais — Grammaire, Conjugaison)
- Ambiance : chateau medieval avec des salles thematiques
- Gardien : ~~Verba le chevalier~~ **OBSOLETE** — remplacer par un animal (voir round 3 section C)
- Villages : "La Salle du Nom", "La Tour du Verbe", "Le Donjon de la Conjugaison"
- Couleur dominante : violet

> **NOTE ROUND 2 :** Les regions et villages sont conserves comme structure de contenu mais le framework narratif "Carte du Savoir / liberation" est remplace par la metaphore de construction. Les gardiens animaux s'integrent dans le nouveau narratif. Voir round 3.

### 4.3 Miyu-sensei : comportements detailles

**Etats de la mascotte :**

| Etat | Declencheur | Apparence | Phrase type |
|------|-------------|-----------|-------------|
| Neutre | En attente | Debout, souriant | "Prends ton temps !" |
| Encourage | Debut d'exercice | Pouce leve | "Tu peux le faire !" |
| Content | Bonne reponse | Saute de joie | "Genial !", "Super !", "Bien joue !" |
| Tres content | 3+ bonnes reponses d'affilee | Danse | "Tu es en feu !", "Incroyable !" |
| Reflechit | L'enfant hesite (10s sans reponse) | Main sur le menton | "Hmm, reflechis bien..." |
| Aide | L'enfant demande un indice | Montre un tableau | "Regarde, voila un indice..." |
| Rassure | Mauvaise reponse | Sourire doux | "Pas grave, on apprend en se trompant !" |
| Inquiet | 3+ erreurs consecutives | Sourcils fronces | "On va revoir ca ensemble." |
| Dort | Mode nuit | Couche avec bonnet | "Zzz... A demain !" |
| Pause | Pause active | Fait du sport | "Allez, on bouge !" |
| Fier | Fin de session reussie | Cape de super-heros | "Quelle session ! Tu es un champion !" |

**Banque de phrases :**
- Minimum 10 phrases differentes par etat pour eviter la repetition
- Les phrases sont adaptees au niveau scolaire (vocabulaire plus simple en CP)
- Les phrases sont stockees dans un fichier JSON separe : `assets/jayecole/mascot/dialogues.json`

### 4.4 ~~Le Jardin du Savoir~~ OBSOLETE — fusionne avec le systeme de construction

> **PIVOT ROUND 2 :** Le Jardin du Savoir est **absorbe par le systeme de construction**. Au lieu d'un jardin separatif, la construction elle-meme est la representation tangible de la perseverance. Voir `BRAINSTORM-round3-construction.md` section A pour les propositions de remplacement.

---

## 5. Mode parent : flux detaille

### 5.1 Acces au mode parent

Le mode parent est protege par un code PIN (4-6 chiffres) defini a l'onboarding.

```
[Ecran enfant — coin superieur droit : icone cadenas discret]
  |
  |-- Clic sur cadenas
  |     |
  |     |-- [Ecran : Saisie du code PIN]
  |     |     |
  |     |     |-- PIN correct → [Dashboard parent]
  |     |     |-- 3 echecs → Verrouillage 5 minutes
```

Le design du bouton d'acces parent est **volontairement discret** : petit cadenas en haut a droite, pas de texte "Mode parent" visible par l'enfant.

### 5.2 Dashboard parent : navigation

```
Dashboard Parent
  |
  |-- [Vue d'ensemble]              <- Page d'accueil du dashboard
  |     |-- Indicateurs cles
  |     |-- Derniere session
  |     |-- Alertes en cours
  |
  |-- [Progression]
  |     |-- Par matiere (Francais / Maths)
  |     |     |-- Par domaine (ex: Grammaire)
  |     |           |-- Par competence (ex: Le verbe)
  |     |-- Graphique radar des competences
  |     |-- Historique des scores (courbe temporelle)
  |
  |-- [Sessions]
  |     |-- Historique des sessions (liste chronologique)
  |     |-- Temps total cette semaine / ce mois
  |     |-- Detail par session (exercices, resultats, duree)
  |
  |-- [Profil de l'enfant]
  |     |-- Informations generales (prenom, niveau, avatar)
  |     |-- Style d'apprentissage detecte (v1.0)
  |     |-- Badges et trophees obtenus
  |     |-- Jardin du Savoir (vue)
  |
  |-- [Parametres]
  |     |-- Temps d'ecran (duree, sessions, horaires, pauses)
  |     |-- Notifications (types d'alertes)
  |     |-- Modifier le code PIN
  |     |-- Supprimer le profil de l'enfant
  |     |-- Exporter les donnees (JSON)
  |
  |-- [Retour a l'espace enfant]
```

### 5.3 Vue d'ensemble (wireframe textuel)

```
+---------------------------------------------------------------+
| JayEcole — Espace Parent                     [Retour enfant]  |
+---------------------------------------------------------------+
|                                                                |
|  [Avatar]  Prenom — CE1                      Streak : 5 jours |
|                                                                |
|  +-------------------+  +-------------------+                  |
|  | Progression       |  | Derniere session  |                  |
|  | Francais : 62%    |  | Hier, 18 min      |                  |
|  | Maths    : 71%    |  | 12 exercices      |                  |
|  | Global   : 67%    |  | 83% de reussite   |                  |
|  +-------------------+  +-------------------+                  |
|                                                                |
|  Points forts          Points a travailler                     |
|  - Calcul mental       - Conjugaison                           |
|  - Numeration          - Comprehension de texte                |
|  - Geometrie           - Orthographe                           |
|                                                                |
|  Alertes                                                       |
|  (!) Difficulte detectee en conjugaison (depuis 3 sessions)    |
|                                                                |
|  Recommandation                                                |
|  "Cette semaine, encouragez votre enfant a choisir la region   |
|   du Chateau de la Grammaire pour travailler la conjugaison."  |
|                                                                |
+---------------------------------------------------------------+
```

### 5.4 Rapport hebdomadaire automatique

Chaque dimanche soir, le systeme prepare un rapport hebdomadaire accessible dans le dashboard :

```
Rapport de la semaine du 24 fevrier au 2 mars 2026

Prenom — CE1

Sessions : 4 sessions (total : 1h12)
Exercices : 48 completes, 39 reussis (81%)

Progression par matiere :
  Francais :  +3% cette semaine (62% → 65%)
    Meilleur domaine : Vocabulaire (92%)
    A travailler : Conjugaison (45%)

  Maths :     +5% cette semaine (71% → 76%)
    Meilleur domaine : Calcul mental (88%)
    A travailler : Resolution de problemes (58%)

Recompenses obtenues : 2 badges, 1 village libere
Streak : 4 jours (record personnel : 7 jours)

Recommandation pour la semaine prochaine :
  Priorite conjugaison : les exercices sur le present de l'indicatif
  sont en dessous du seuil attendu pour le CE1 en periode 3.
```

### 5.5 Export de donnees

Le parent peut exporter toutes les donnees de son enfant :
- **Format JSON** : export complet pour portabilite (LOI-8)
- **Contenu** : profil, scores, historique des sessions, historique des exercices, recompenses
- **Pas de format PDF dans le MVP** (prevu en v1.0 pour les rapports imprimables)
- Le fichier exporte ne contient que les donnees de l'enfant selectionne

---

## 6. Roadmap MVP : jalons et phases de developpement

### 6.1 Decoupe en phases

Le MVP est decoupe en **6 phases** de developpement, avec un jalon livrable a chaque phase.

**Phase 0 — Fondations (2 semaines)**
```
Objectif : Poser l'architecture technique

Livrables :
  - Crate jayecole (structure vide, types, errors, Cargo.toml)
  - Crate miyuquiz (structure vide, types exercices, Cargo.toml)
  - Crate miyuscreenguard (structure vide, Cargo.toml)
  - Schema KindMother (tables child_profiles, skill_levels, sessions,
    exercise_results, rewards, parent_settings, exercises)
  - Arbre de competences initial (skill_tree.json) — francais + maths, CP-CM2
  - Document Fondateur valide

Responsables :
  - Denis : architecture, review Cargo.toml, doc technique
  - Francois : implementation des crates vides + schema DB
  - Maria : validation Document Fondateur

Jalon : "cargo build --workspace" compile sans erreur.
        Schema DB instancie avec donnees de test.
```

**Phase 1 — Moteur d'exercices (3 semaines)**
```
Objectif : Pouvoir creer, stocker et afficher un exercice

Livrables :
  - MiyuQuiz : types d'exercices (QCM, fill_blank, matching, numeric_input)
  - MiyuQuiz : validation des reponses
  - MiyuQuiz : rendu Dioxus basique (un exercice a l'ecran)
  - 50 exercices de test (10 par niveau, francais + maths)
  - Tests unitaires du moteur de validation

Responsables :
  - Francois : backend MiyuQuiz (types, validation)
  - Lise : rendu Dioxus des exercices
  - Arianne : verification qualite des 50 exercices de test

Jalon : Un exercice QCM et un exercice texte-a-trous s'affichent dans
        Dioxus et la reponse est validee correctement.
```

**Phase 2 — Profil et parcours (3 semaines)**
```
Objectif : Un enfant peut creer son profil et suivre un parcours lineaire

Livrables :
  - Onboarding complet (creation profil parent + enfant + avatar)
  - Evaluation diagnostique (version simplifiee : 6-8 questions par matiere)
  - Parcours lineaire : enchainement des exercices par chapitre
  - Persistance KindMother : profil, scores, resultats
  - Moteur adaptatif simplifie (3 paliers de difficulte)

Responsables :
  - Francois : moteur adaptatif, persistance DB, evaluation diagnostique
  - Lise : ecrans d'onboarding, ecran de diagnostic

Jalon : Un enfant peut creer son profil, passer le diagnostic,
        et enchainer 10 exercices avec adaptation de difficulte.
```

**Phase 3 — Gamification et carte (3 semaines)**
```
Objectif : L'experience est ludique et motivante

Livrables :
  - Carte du Savoir (affichage des 5 regions MVP avec villages)
  - Navigation carte → village → exercice
  - Systeme de recompenses (etoiles, badges)
  - Mascotte Miyu-sensei (5 etats minimum : neutre, content, rassure, aide, pause)
  - Animations de recompense (confettis, etoiles)
  - Ecrans de fin de session avec bilan

Responsables :
  - Lise : carte du savoir, mascotte, animations, ecrans de bilan
  - Francois : systeme de recompenses (backend), integration carte <-> parcours

Jalon : Un enfant navigue sur la carte, libere un village en completant
        des exercices, gagne des etoiles et voit Miyu-sensei reagir.
```

**Phase 4 — Temps d'ecran et mode parent (2 semaines)**
```
Objectif : Le parent controle l'experience et suit la progression

Livrables :
  - MiyuScreenGuard : minuteur de session, limites quotidiennes, cooldown
  - MiyuScreenGuard : pauses actives (toutes les 10 min)
  - MiyuScreenGuard : mode nuit (horaires configurables)
  - Dashboard parent : vue d'ensemble, progression par matiere, historique sessions
  - Controle parental : parametres temps d'ecran, PIN
  - Alertes basiques (fin de session, limite atteinte, difficulte detectee)

Responsables :
  - Francois : MiyuScreenGuard backend, alertes
  - Lise : dashboard parent, ecrans de parametres, ecran de pause active
  - Denis : review securite du PIN parental

Jalon : Le parent peut configurer les limites, l'enfant voit le minuteur,
        les pauses actives se declenchent, le parent voit le dashboard.
```

**Phase 5 — Contenu et polish (4 semaines)**
```
Objectif : Le service est utilisable avec un contenu suffisant

Livrables :
  - Banque d'exercices complete MVP :
    - Francais : ~400 exercices (CP-CM2, lecture + grammaire + orthographe + conjugaison + vocabulaire)
    - Maths : ~400 exercices (CP-CM2, numeration + calcul + geometrie + mesures + problemes)
    - Total : ~800 exercices minimum
  - Rapport hebdomadaire parent (automatique)
  - Jardin du Savoir (version simplifiee, visuel statique)
  - Banque de phrases Miyu-sensei (10 phrases par etat minimum)
  - Tests d'integration complets
  - Tests UX avec 2-3 enfants (si possible)
  - Correction de bugs et ajustements UX

Responsables :
  - Arianne : creation et validation des 800 exercices (poste le plus lourd)
  - Lise : polish UI, jardin du savoir, ajustements UX
  - Francois : rapport hebdomadaire, correction bugs
  - George : audit final (conformite RGPD, UX, performances)
  - Denis : tests d'integration, documentation finale

Jalon : Un enfant de CE1 peut utiliser JayEcole quotidiennement pendant
        une semaine avec du contenu adapte et suffisant. Le parent recoit
        un rapport hebdomadaire clair.
```

### 6.2 Planning recapitulatif

| Phase | Nom | Duree | Semaines | Jalon |
|-------|-----|-------|----------|-------|
| 0 | Fondations | 2 sem | S1-S2 | Architecture compilee, schema DB |
| 1 | Moteur d'exercices | 3 sem | S3-S5 | Exercice affiche et valide |
| 2 | Profil et parcours | 3 sem | S6-S8 | Profil + diagnostic + parcours adaptatif |
| 3 | Gamification et carte | 3 sem | S9-S11 | Carte navigable + recompenses + mascotte |
| 4 | Temps d'ecran + parent | 2 sem | S12-S13 | Dashboard parent + limites fonctionnelles |
| 5 | Contenu et polish | 4 sem | S14-S17 | 800 exercices + tests + audit |
| **TOTAL MVP** | | **17 semaines** | | **Service utilisable** |

**Fourchette realiste :** 17 semaines (optimiste) a 24 semaines (pessimiste, incluant les imprevus et les iterations UX).

### 6.3 Dependances entre phases

```
Phase 0 (Fondations)
  |
  +-- Phase 1 (Moteur d'exercices)
  |     |
  |     +-- Phase 2 (Profil et parcours)
  |     |     |
  |     |     +-- Phase 3 (Gamification et carte)
  |     |     |     |
  |     |     |     +-- Phase 5 (Contenu et polish)
  |     |     |
  |     |     +-- Phase 4 (Temps d'ecran et parent)
  |     |           |
  |     |           +-- Phase 5 (Contenu et polish)
```

Les phases 3 et 4 peuvent etre developpees en parallele (Lise sur la Phase 3, Francois sur la Phase 4). La Phase 5 necessite que les phases 3 et 4 soient terminees.

### 6.4 Criteres de succes du MVP

Le MVP sera considere comme reussi si :

1. Un enfant de chaque niveau (CP a CM2) peut completer 5 sessions sans bug bloquant
2. Le diagnostic positionne correctement l'enfant (validation par un adulte)
3. Le moteur adaptatif ajuste visiblement la difficulte
4. Le parent peut voir la progression et configurer les limites de temps
5. Les pauses actives se declenchent toutes les 10 minutes
6. Le temps de chargement est inferieur a 3 secondes
7. La memoire utilisee est inferieure a 200 MB
8. Aucun crash sur une session de 30 minutes
9. Le contenu de 800 exercices est pedagogiquement valide

---

## 7. Prochaines etapes immediates

1. **Maria** : finaliser le brainstorming round 3 (pivot construction, Miyu-sensei, CP, DYS, audio, service separe) -- voir `BRAINSTORM-round3-construction.md`
2. **Maria** : collecter les decisions round 3 aupres de l'utilisateur (concept construction, apparence Miyu-sensei, voix audio)
3. **Maria** : rediger le Document Fondateur une fois le round 3 valide
4. **Denis** : recevoir le Document Fondateur et produire la documentation technique (architecture des crates, API internes, contrats entre operateurs, **architecture binaire separe**)
5. **Lise** : creer des maquettes du **systeme de construction** et d'un ecran d'exercice **CP** (prioritaire)
6. **Francois** : initialiser les crates `jayecole`, `miyuquiz`, `miyuscreenguard` (structure vide, **Cargo.toml independant ou workspace member**)
7. **Arianne** : archiver ce document, commencer a inventorier les competences du programme EN **avec l'institutrice**
8. **George** : preparer la checklist d'audit RGPD pour les services destines aux mineurs + audit accessibilite DYS

---

*Document redige par Maria, Chef de Projet Miyukini AI Studio*
*Brainstorming approfondissement — JayEcole*
*Date : 2026-02-27*
*Mis a jour : 2026-02-27 (integration decisions round 2, marquage sections obsoletes)*
