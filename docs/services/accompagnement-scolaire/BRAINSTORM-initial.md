# JayEcole — Brainstorming Initial

## Contexte

Ce document est le brainstorming structure pour un nouveau service d'accompagnement scolaire destine aux enfants de CP a CM2 (6-11 ans), integre dans l'ecosysteme Miyukini COG. Le service suivrait le programme officiel de l'Education Nationale francaise, avec une interface adaptee aux enfants, un systeme de profiling cognitif, de la gamification et une gestion responsable du temps d'ecran.

## Portee / Scope

- **Applicable a :** Phase exploratoire, brainstorming initial, collecte d'idees.
- **Audience :** Equipe projet, parties prenantes, utilisateur fondateur.
- **Statut :** Document de brainstorming (pre-fondateur) — VALIDE par l'utilisateur le 2026-02-27.
- **Redige par :** Maria (Chef de Projet)
- **Date :** 2026-02-27
- **Mis a jour :** 2026-02-27 (integration decisions round 1 + round 2)

---

## Decisions validees par l'utilisateur

### Round 1 (2026-02-27)

| Question | Decision |
|----------|----------|
| **Nom du service** | **JayEcole** (definitif) |
| **Scope MVP** | Francais + Mathematiques uniquement |
| **Cible geographique** | France uniquement (programme Education Nationale) |
| **Modele economique** | Gratuit (aucune monetisation) |
| **Priorite roadmap** | Moyenne dans la roadmap globale Miyukini |

### Round 2 (2026-02-27)

| Question | Decision |
|----------|----------|
| **Miyu-sensei** | Personnage **humanoide**, cree de toutes pieces pour JayEcole |
| **Gardiens de regions** | Tous des **animaux** (Verba le chevalier remplace par un animal) |
| **Ton narratif** | PIVOT : abandonner "brouillard de l'Oubli" pour un ton **artisan/construction**. L'enfant construit quelque chose au fil de l'apprentissage. |
| **Source du contenu** | Tout a creer from scratch, pas de base existante |
| **Validation pedagogique** | Acces a une ancienne institutrice pour valider l'alignement programme EN |
| **Accessibilite DYS** | Integrer **des le MVP** (OpenDyslexic, interlignage, representations visuelles dyscalculie) |
| **Placement dans Central** | Service **lance separement** (binaire independant, pas un onglet de Central) |
| **Audio** | Consignes audio **dans le MVP** (essentiel pour CP) |
| **Tests enfants** | Enfants disponibles dans l'entourage pour tester |
| **Niveau prioritaire prototype** | **CP** (le plus contraint car l'enfant ne lit pas encore bien) |

---

## Nom du service : JayEcole

**Justification :** Le prefixe "Jay" s'inscrit dans la nomenclature des services Miyukini orientes utilisateur (JayKoa, JayFestival, JayKonta, JayXpose, JayShop, JayRDV, JayFaim). "Ecole" est le terme le plus direct et comprehensible pour les parents comme pour les enfants. Choix valide par l'utilisateur.

---

## 1. Analyse du besoin

### 1.1 Public cible

**Public primaire : les enfants**
- Age : 6 a 11 ans (CP, CE1, CE2, CM1, CM2)
- Niveaux de maturite numerique tres differents entre 6 ans et 11 ans
- Attention variable : 15-25 minutes de concentration soutenue selon l'age
- Besoin de retour immediat et de gratification visuelle

**Public secondaire : les parents/tuteurs**
- Souhaitent suivre la progression scolaire de leur enfant
- Veulent controle du temps d'ecran et des contenus
- Recherchent un complement au travail scolaire, pas un remplacement
- Besoin de rapports clairs et actionnables

**Public tertiaire (futur) : les enseignants**
- Possibilite de recommander le service aux familles
- Integration potentielle avec les progressions de classe (hors MVP)

### 1.2 Contexte et problemes a resoudre

| Probleme | Description |
|----------|-------------|
| **Decrochage scolaire precoce** | Certains enfants accumulent des lacunes des le primaire sans que ce soit detecte a temps |
| **Manque de personnalisation** | L'ecole gere 25-30 eleves par classe ; le rythme individuel est rarement respecte |
| **Cours particuliers couteux** | 25-50 EUR/h pour du soutien scolaire prive ; inaccessible pour beaucoup de familles |
| **Applications existantes inadaptees** | Soit trop simplistes (exercices repetitifs sans sens), soit trop complexes, soit bourrees de publicite |
| **Temps d'ecran non maitrise** | Les enfants sur tablette/PC sans cadre : risque de surexposition |
| **Difficulte de suivi parental** | Les parents manquent d'outils pour comprendre ou en est leur enfant concretement |

### 1.3 Positionnement

JayEcole n'est **pas** une ecole en ligne. C'est un **compagnon scolaire intelligent** qui :
- S'adapte au rythme de chaque enfant
- Rend l'apprentissage ludique sans sacrifier la rigueur pedagogique
- Fournit aux parents une vision claire de la progression
- Respecte le bien-etre numerique de l'enfant

---

## 2. Fonctionnalites cles

### 2.1 MVP (Version 0.1)

| Fonctionnalite | Description | Priorite |
|----------------|-------------|----------|
| **Exercices par niveau et matiere** | Banque d'exercices CP-CM2 en francais et mathematiques | Critique |
| **Profil enfant basique** | Creation de profil avec niveau scolaire, avatar | Critique |
| **Evaluation diagnostique initiale** | Test de positionnement au premier lancement pour situer l'enfant | Critique |
| **Progression lineaire** | Parcours structure par chapitres suivant le programme EN | Critique |
| **Retour immediat** | Correction instantanee avec explication courte | Critique |
| **Tableau de bord parent simple** | Vue des exercices faits, score global, temps passe | Critique |
| **Minuteur de session** | Limite de temps configurable par le parent (defaut : 20 min) | Critique |
| **Mascotte d'accompagnement** | Personnage guide qui encourage et reagit | Important |
| **Systeme de recompenses basique** | Etoiles, badges pour les exercices completes | Important |

### 2.2 Version 1.0

| Fonctionnalite | Description | Priorite |
|----------------|-------------|----------|
| **Toutes les matieres** | Ajout sciences, histoire-geo, education civique, anglais | Eleve |
| **Profiling cognitif complet** | Detection du style d'apprentissage (visuel, auditif, kinesthesique) | Eleve |
| **Parcours adaptatif** | L'IA ajuste la difficulte et le type d'exercice selon le profil | Eleve |
| **Mini-jeux educatifs** | Jeux integres pour chaque notion (puzzle, quiz interactif, memory) | Eleve |
| **Rapports detailles parent** | Graphiques de progression, forces/faiblesses par competence | Eleve |
| **Mode revision** | Sessions ciblees sur les points faibles detectes | Moyen |
| **Calendrier scolaire** | Synchronisation avec les periodes scolaires et vacances | Moyen |
| **Multi-enfants** | Plusieurs profils enfants dans un meme compte parent | Moyen |

### 2.3 Extensions futures

| Fonctionnalite | Description |
|----------------|-------------|
| **Assistant Miou pour l'enfant** | Chatbot adapte qui repond aux questions avec un langage enfant (via miou-llm-bridge) |
| **Mode dictee** | Dictees vocales avec reconnaissance de l'ecriture |
| **Mode lecture** | Textes adaptes au niveau avec comprehension de lecture |
| **Defis entre amis** | Challenges entre enfants (via Jay1Tribu) |
| **Integration enseignant** | Dashboard enseignant pour une classe entiere |
| **Contenu multimedia** | Videos courtes explicatives, animations pedagogiques |
| **Mode hors-ligne** | Exercices telecharges pour travailler sans connexion (LOI-2) |
| **Recompenses physiques** | Systeme de bons d'achat ou recompenses reelles (via JayShop) |

---

## 3. Profiling enfant

### 3.1 Evaluation diagnostique initiale

Au premier lancement, l'enfant passe un **test de positionnement** ludique (presente comme un jeu, jamais comme un examen) :

**Structure du test :**
1. **Niveau scolaire declare** : le parent indique CP, CE1, etc.
2. **Test adaptatif** : 15-20 questions par matiere principale (francais, maths)
   - Si l'enfant reussit facilement, la difficulte monte
   - Si l'enfant echoue, la difficulte descend
   - Arret automatique quand le niveau est cerne (algorithme de type CAT - Computerized Adaptive Testing)
3. **Resultat** : un "portrait scolaire" avec niveaux par competence

**Competences evaluees en francais :**
- Lecture / decodage
- Comprehension de texte
- Orthographe
- Grammaire
- Conjugaison
- Vocabulaire
- Expression ecrite (CM1-CM2)

**Competences evaluees en mathematiques :**
- Numeration / connaissance des nombres
- Calcul mental
- Operations (addition, soustraction, multiplication, division)
- Geometrie
- Mesures et grandeurs
- Resolution de problemes

### 3.2 Profil cognitif

Au-dela des competences scolaires, le systeme detecte progressivement le **style d'apprentissage** de l'enfant :

| Dimension | Comment la detecter | Impact sur le parcours |
|-----------|--------------------|-----------------------|
| **Visuel vs textuel** | L'enfant reussit-il mieux les exercices avec images ou avec texte seul ? | Adapter la proportion images/texte |
| **Rapide vs methodique** | Temps de reponse moyen, taux d'erreur par precipitation | Proposer plus/moins de temps, encourager la relecture |
| **Autonome vs guide** | L'enfant demande-t-il souvent l'aide de la mascotte ? | Ajuster le niveau d'accompagnement |
| **Competitif vs collaboratif** | L'enfant est-il motive par les classements ou les defis de groupe ? | Adapter la gamification (solo vs social) |
| **Regulier vs sporadique** | Frequence et regularite des sessions | Adapter les rappels et la structure des sessions |

**Methode de detection :** Pas de questionnaire explicite (inutile a cet age). Le systeme observe les patterns d'utilisation sur 2-4 semaines et ajuste silencieusement le profil. Le parent peut voir le profil detecte et le corriger.

### 3.3 Suivi longitudinal

Le profil evolue dans le temps :
- **Graphique d'evolution par competence** (progression sur l'annee)
- **Detection des regressions** : alerte si une competence acquise se degrade
- **Prediction de difficulte** : anticiper les notions qui seront problematiques selon le profil
- **Historique complet** : chaque exercice, chaque resultat, chaque temps de reponse est conserve localement (KindMother)

### 3.4 Modele de donnees du profil (esquisse)

```
ChildProfile {
    id: UUID v4
    display_name: String
    avatar_id: AvatarId
    school_level: SchoolLevel  // CP, CE1, CE2, CM1, CM2
    birth_year: u16            // pour ajuster l'age
    created_at: ISO 8601

    // Profil scolaire
    skill_levels: Map<SkillId, SkillLevel>  // niveau par competence
    diagnostic_results: Vec<DiagnosticResult>

    // Profil cognitif
    learning_style: LearningStyleProfile {
        visual_preference: f32      // 0.0 = textuel, 1.0 = visuel
        pace: f32                   // 0.0 = methodique, 1.0 = rapide
        autonomy: f32               // 0.0 = tres guide, 1.0 = autonome
        social_motivation: f32      // 0.0 = solo, 1.0 = social
    }

    // Historique
    session_history: Vec<SessionRecord>
    total_time_spent: Duration
    streak_days: u32               // jours consecutifs
}
```

---

## 4. Gamification

### 4.1 Principes fondamentaux

La gamification pour des enfants de 6-11 ans doit respecter des regles strictes :

1. **Jamais de punition** : l'echec est toujours une occasion d'apprendre, jamais une perte
2. **Gratification immediate** : chaque effort est recompense, meme en cas d'erreur partielle
3. **Pas de monetisation** : aucun achat in-app, aucune loot box, aucun mecanisme addictif
4. **Pas de classement global** : pas de pression sociale entre enfants
5. **Progression visible** : l'enfant voit toujours ou il en est et le chemin parcouru

### 4.2 Mascotte : Miyu-sensei

**Concept :** Une mascotte animee qui accompagne l'enfant tout au long de son parcours.

**Nom propose :** Miyu-sensei (coherent avec l'univers Miyukini)

**Caracteristiques :**
- Petit personnage stylise (style Ghibli/kawaii, coherent avec l'identite Miyukini)
- Exprime des emotions : content quand l'enfant reussit, encourageant quand il se trompe, surpris quand il va tres vite
- Donne des explications courtes et claires
- Felicite avec des phrases variees (pas toujours "Bravo !")
- Peut etre personnalise (couleur, accessoires gagnes)

**Variantes selon le niveau :**
- CP-CE1 : Miyu-sensei est plus expressif, plus de gestes, phrases courtes
- CE2 : equilibre
- CM1-CM2 : Miyu-sensei est plus "prof cool", humour leger, explications plus detaillees

### 4.3 Systeme de recompenses

**Recompenses immediates (par exercice) :**
- Etoiles (1 a 3) selon la qualite de la reponse
- Animation de celebration (confettis, feux d'artifice, danse de Miyu-sensei)
- Piece de "savoir" (monnaie virtuelle du jeu)

**Recompenses a moyen terme (par chapitre/session) :**
- Badges de maitrise ("Expert en multiplication", "As de la conjugaison")
- Pieces de puzzle : chaque chapitre complete donne une piece d'un puzzle qui revele une illustration
- Accessoires pour Miyu-sensei ou pour l'avatar de l'enfant

**Recompenses a long terme (par trimestre/annee) :**
- Trophees de niveau ("Chevalier du savoir", "Mage des maths", "Druide des lettres")
- Deblocage de mini-jeux bonus
- Diplome imprimable a la fin de chaque niveau scolaire
- "Jardin du savoir" : espace virtuel que l'enfant fait pousser avec ses efforts (chaque exercice arrose une plante)

### 4.4 Systeme de progression : La Carte du Savoir

**Concept :** L'enfant se deplace sur une carte du monde imaginaire, ou chaque region represente un domaine scolaire.

**Regions de la carte :**
- **Les Collines des Nombres** : mathematiques / numeration
- **La Foret des Mots** : francais / vocabulaire / orthographe
- **Le Lac des Histoires** : lecture / comprehension
- **La Montagne des Calculs** : operations / resolution de problemes
- **Le Chateau de la Grammaire** : grammaire / conjugaison
- **L'Archipel des Decouvertes** : sciences (v1.0)
- **Les Plaines du Passe** : histoire-geo (v1.0)

Chaque region contient des "villages" (chapitres) et chaque village des "maisons" (exercices). L'enfant "libere" les villages en completant les exercices. Le chemin parcouru reste colore et anime.

### 4.5 Mecaniques anti-frustration

| Situation | Reponse du systeme |
|-----------|-------------------|
| 3 erreurs consecutives | Miyu-sensei propose un indice ou une explication |
| 5 erreurs sur le meme type | Exercice simplifie automatiquement, retour aux bases |
| L'enfant quitte en cours d'exercice | Sauvegarde automatique, reprise la ou il en etait |
| L'enfant ne revient pas depuis 3 jours | Notification douce au parent (pas a l'enfant) |
| L'enfant revient apres une absence | Miyu-sensei fait un "rappel express" des notions precedentes |

---

## 5. Gestion du temps d'ecran

### 5.1 Philosophie

Le temps d'ecran est un sujet de sante publique pour les enfants. JayEcole doit etre un modele de respect du bien-etre numerique. Le service doit **s'autolimiter** meme si cela signifie moins de temps d'utilisation.

### 5.2 Mecanismes de protection

**Minuteur de session :**
- Configurable par le parent (defaut : 20 minutes par session)
- Fourchettes recommandees :
  - CP (6 ans) : 10-15 minutes
  - CE1-CE2 (7-8 ans) : 15-20 minutes
  - CM1-CM2 (9-11 ans) : 20-30 minutes
- A 5 minutes de la fin : notification douce ("Il te reste 5 minutes !")
- A la fin : ecran de fin de session avec resume des accomplissements
- **Pas de coupure brutale** : le systeme laisse toujours finir l'exercice en cours

**Limites quotidiennes :**
- Maximum configurable par jour (defaut : 2 sessions / 40 minutes)
- Cooldown minimum entre deux sessions : 30 minutes (configurable)
- Compteur visible pour l'enfant : "Tu as utilise 1 session sur 2 aujourd'hui"

**Pauses actives :**
- Toutes les 10 minutes : micro-pause de 30 secondes
- Miyu-sensei propose un mini-exercice physique : "Leve-toi et touche tes orteils !", "Fais 5 sauts sur place !"
- L'enfant peut passer la pause mais le systeme les encourage

**Mode nuit :**
- Horaire configurable par le parent (defaut : pas d'acces entre 20h et 7h)
- Le service est simplement indisponible en dehors des horaires autorises
- Ecran de nuit : Miyu-sensei dort avec un message "Reviens demain matin !"

### 5.3 Dashboard temps d'ecran pour le parent

- Temps passe par jour / semaine / mois (graphique)
- Nombre de sessions et duree moyenne
- Respect des limites : combien de fois l'enfant a atteint la limite
- Recommendations : "Votre enfant utilise le service regulierement, les sessions sont bien dosees" ou "Les sessions semblent longues, pensez a reduire la limite"

### 5.4 Alertes parent

| Alerte | Condition | Canal |
|--------|-----------|-------|
| **Session terminee** | Fin d'une session | Notification in-app |
| **Limite quotidienne atteinte** | Toutes les sessions du jour utilisees | Notification in-app |
| **Inactivite prolongee** | Pas de session depuis 3+ jours | Notification in-app |
| **Progression bloquee** | Meme exercice echoue 5+ fois | Notification in-app + detail |
| **Anomalie de temps** | Session anormalement longue (parent a oublie de configurer) | Notification in-app |

---

## 6. Programme scolaire

### 6.1 Reference officielle

Le contenu suit les **programmes officiels de l'Education Nationale francaise** (Bulletin officiel). Les cycles concernes :

- **Cycle 2** (CP, CE1, CE2) : apprentissages fondamentaux
- **Cycle 3** (CM1, CM2, 6e) : consolidation (on s'arrete a CM2)

### 6.2 Structure du contenu

```
Programme
  |-- Niveau (CP, CE1, CE2, CM1, CM2)
       |-- Matiere (Francais, Mathematiques, Sciences, etc.)
            |-- Domaine (ex: Numeration, Grammaire)
                 |-- Chapitre (ex: Les nombres de 0 a 100)
                      |-- Lecon (explication courte + visuel)
                      |-- Exercices (5-10 par chapitre)
                           |-- Type (QCM, texte a trous, drag-drop, calcul, etc.)
                           |-- Difficulte (3 niveaux)
                      |-- Evaluation de chapitre
```

### 6.3 Matieres par cycle

**Cycle 2 (CP, CE1, CE2) :**

| Matiere | Domaines principaux |
|---------|-------------------|
| **Francais** | Lecture et comprehension, Ecriture, Etude de la langue (grammaire, orthographe, lexique), Langage oral |
| **Mathematiques** | Nombres et calculs, Grandeurs et mesures, Espace et geometrie, Resolution de problemes |
| **Questionner le monde** | Le vivant, La matiere, Les objets techniques, Le temps, L'espace |
| **Enseignement moral et civique** | Respect, regles, vivre ensemble |
| **Langues vivantes** | Anglais (vocabulaire de base, comptines, salutations) |

**Cycle 3 (CM1, CM2) :**

| Matiere | Domaines principaux |
|---------|-------------------|
| **Francais** | Langage oral, Lecture et comprehension, Ecriture, Etude de la langue |
| **Mathematiques** | Nombres et calculs (fractions, decimaux), Grandeurs et mesures, Espace et geometrie, Resolution de problemes |
| **Sciences et technologie** | Le vivant, La Terre, La matiere, L'energie, Les objets techniques |
| **Histoire et geographie** | Prehistoire a aujourd'hui, Decouverte du monde, La France et l'Europe |
| **Enseignement moral et civique** | Citoyennete, laicite, droits et devoirs |
| **Langues vivantes** | Anglais (conversations simples, culture) |

### 6.4 Types d'exercices par matiere

**Francais :**
- QCM de comprehension de texte
- Textes a trous (orthographe, conjugaison)
- Remettre des mots dans l'ordre (construction de phrase)
- Associer mot et definition (vocabulaire)
- Dictee interactive (l'enfant ecrit, correction en temps reel)
- Classer des mots (noms, verbes, adjectifs)

**Mathematiques :**
- Calcul mental (reponse libre ou QCM)
- Glisser-deposer pour ordonner des nombres
- Tracer des figures geometriques (interface tactile)
- Problemes a etapes avec schema interactif
- Tableaux de conversion (mesures)
- Jeu de la monnaie (rendre la monnaie, compter)

**Sciences / Histoire-geo :**
- QCM illustres
- Legendes a completer sur un schema
- Frise chronologique interactive (glisser les evenements)
- Tri d'images (vivant/non-vivant, liquide/solide)
- Mini-experiences virtuelles (observer, hypothese, conclusion)

### 6.5 Alignement avec le calendrier scolaire

- Le parcours est decoupe en **5 periodes** (correspondant aux 5 periodes scolaires)
- Chaque periode a des objectifs clairs
- Pendant les vacances : mode "revision ludique" (pas de nouvelles notions, jeux sur l'acquis)
- Rentree : test de positionnement rapide pour ajuster le parcours

---

## 7. Interface enfant

### 7.1 Principes UX/UI pour enfants

**Navigation :**
- Maximum 2 niveaux de profondeur
- Boutons grands (minimum 48px, idealement 64px)
- Icones + texte (jamais d'icone seule pour les jeunes enfants)
- Navigation par la carte du monde (pas de menu classique)
- Retour a la carte toujours visible

**Typographie :**
- Police lisible et ronde (type "Gentium Book Plus" deja disponible dans les assets, ou "OpenDyslexic" en option pour les enfants dyslexiques)
- Taille minimum : 18px pour le corps de texte, 24px pour les titres
- Interlignage genereux (1.5 minimum)
- CP-CE1 : lettres en script (pas de cursive sur ecran)

**Couleurs :**
- Palette vive mais pas agressive
- Contraste eleve (WCAG AA minimum, idealement AAA)
- Code couleur par matiere : bleu = maths, vert = francais, orange = sciences, violet = histoire
- Fond clair (pas de dark mode pour les enfants : lisibilite et regulation du sommeil)

**Animations :**
- Transitions douces (pas de clignotements)
- Animations de recompense celebratoires mais courtes (2-3 secondes)
- Option pour reduire les animations (enfants sensibles)
- Pas d'auto-play video

**Accessibilite :**
- Support lecteur d'ecran pour les enfants malvoyants
- Taille de police ajustable
- Option OpenDyslexic
- Consignes audio en option (l'enfant peut ecouter la consigne)
- Daltonisme : ne jamais coder l'information par la couleur seule

### 7.2 Ecrans principaux (enfant)

1. **Ecran d'accueil** : carte du monde, avatar, Miyu-sensei, bouton "Jouer"
2. **Carte du Savoir** : vue zoomable avec les regions, villages liberes/verrouilles
3. **Ecran d'exercice** : consigne en haut, zone d'interaction au centre, Miyu-sensei en bas a droite
4. **Ecran de resultat** : animation de recompense, etoiles, bouton "Suivant" ou "Revoir"
5. **Mon profil** : avatar, badges, trophees, jardin du savoir
6. **Ecran de pause active** : mini-exercice physique avec Miyu-sensei

### 7.3 Adaptation par age

| Element | CP (6 ans) | CE1-CE2 (7-8 ans) | CM1-CM2 (9-11 ans) |
|---------|-----------|-------------------|-------------------|
| Taille texte | 22px+ | 20px | 18px |
| Consignes | Audio + texte | Texte + audio optionnel | Texte seul |
| Navigation | Tres simplifiee (3 boutons max) | Standard | Standard + raccourcis |
| Animations | Tres presentes | Presentes | Moderees |
| Mascotte | Tres expressive | Equilibree | Discrete mais presente |
| Complexite ecran | 1 element a la fois | 2-3 elements | Interface complete |

---

## 8. Interface parent/tuteur

### 8.1 Dashboard principal

Le parent accede a un **espace dedie** distinct de l'espace enfant (switch de profil avec code PIN).

**Vue d'ensemble :**
- Photo/avatar de l'enfant + niveau scolaire
- Indicateur global de progression (% du programme complete)
- Derniere session : date, duree, matiere, resultat
- Points forts (top 3 competences)
- Points a travailler (top 3 faiblesses)
- Streak (jours consecutifs)

### 8.2 Rapports detailles

**Rapport hebdomadaire :**
- Nombre de sessions et temps total
- Exercices completes / taux de reussite
- Competences travaillees
- Evolution vs semaine precedente
- Recommandation : "Cette semaine, concentrez-vous sur les tables de multiplication"

**Rapport mensuel :**
- Graphique de progression par competence (radar chart)
- Comparaison avec les attendus du programme EN pour le niveau
- Points forts consolides / points faibles persistants
- Suggestion de focus pour le mois suivant

**Rapport trimestriel :**
- Bilan complet avec positionnement par rapport au programme
- Diplome / certificat de progression
- Recommendations pour les vacances

### 8.3 Controle parental

| Parametre | Description | Defaut |
|-----------|-------------|--------|
| Duree max par session | En minutes | 20 min |
| Sessions max par jour | Nombre | 2 |
| Cooldown entre sessions | En minutes | 30 min |
| Horaires autorises | Plage horaire | 7h-20h |
| Pauses actives | Activer/desactiver | Active |
| Frequence des pauses | En minutes | 10 min |
| Notifications parent | Types d'alertes activees | Toutes |
| Acces aux resultats | L'enfant voit-il ses stats detaillees ? | Non (CP-CE2), Oui (CM1-CM2) |

### 8.4 Multi-enfants

- Le parent peut creer plusieurs profils enfants
- Dashboard comparatif (optionnel, a activer) : attention a ne pas creer de competition entre freres et soeurs
- Parametres individuels par enfant
- Notifications regroupees ou separees (configurable)

---

## 9. Considerations techniques

### 9.1 Positionnement dans l'architecture COG

**Type de Service : Service interne COG (Type 1)**
- Acces via Miyukini Central
- Donnees stockees localement (KindMother)
- Pas de surface web externe (dans un premier temps)
- Conformite totale avec les Lois d'Autonomie

**Strate 7 — Service**
```
JayEcole
|-- Interface enfant (Dioxus)
|-- Interface parent (Dioxus)
|-- Moteur d'exercices
|-- Moteur adaptatif
```

**Strate 7 — Operateurs**
```
JayEcole.Curriculum      -> Gestion du programme scolaire, contenu, progression
JayEcole.Assessment      -> Evaluation, diagnostic, scoring
JayEcole.Adaptive        -> Moteur adaptatif, recommandation d'exercices
JayEcole.Gamification    -> Recompenses, badges, carte du savoir
JayEcole.ScreenTime      -> Minuteur, limites, pauses, horaires
JayEcole.ParentDashboard -> Rapports, controle parental, alertes
```

**Strate 6 — Toolkits requis**
```
Toolkits existants a reutiliser :
  MiyuProfile       -> Gestion profils (enfant + parent)
  MiyuNotify        -> Notifications parent
  MiyuMedia         -> Assets multimedia (images, sons, animations)
  MiyuExport        -> Export rapports PDF
  MiyuLocale        -> i18n (francais prioritaire, future extension)
  MiyuValidate      -> Validation des reponses

Toolkits a creer :
  MiyuQuiz          -> Moteur d'exercices generique (QCM, texte a trous, drag-drop, etc.)
  MiyuAdaptive      -> Algorithme adaptatif (IRT, CAT) pour ajuster la difficulte
  MiyuCurriculum    -> Structure du programme scolaire (niveaux, matieres, competences, chapitres)
  MiyuScreenGuard   -> Gestion temps d'ecran (minuteur, limites, pauses, horaires)
  MiyuReward        -> Systeme de recompenses (etoiles, badges, trophees, monnaie virtuelle)
  MiyuMascot        -> Systeme de mascotte animee (etats, dialogues, reactions)
```

**Strate 4 — Cores utilises**
```
KindMother         -> Persistance locale (profils, progression, exercices, resultats)
StrongFather       -> Autorisations (acces parent vs enfant, PIN parental)
CaringNanny        -> Metriques et observation (temps d'ecran, taux de reussite)
MasterButler       -> Orchestration des Toolkits
WorrySentinel      -> Limites (temps d'ecran, age minimum, alertes)
EverBuddy          -> Versioning des profils, migration entre appareils
```

### 9.2 Structure des crates

```
crates/jayecole/
  Cargo.toml
  src/
    lib.rs
    data/
      mod.rs
      types.rs
      kindmother_db.rs        -- Schema DB local (profils, progression, resultats)
    services/
      curriculum.rs           -- Gestion programme scolaire
      assessment.rs           -- Evaluation et diagnostic
      adaptive.rs             -- Moteur adaptatif
      gamification.rs         -- Recompenses, progression carte
      screen_time.rs          -- Gestion temps d'ecran
    auth/
      mod.rs
      parent_pin.rs           -- Code PIN parental
    export/
      mod.rs
      report_pdf.rs           -- Generateur de rapports

crates/miyuquiz/              -- Toolkit moteur d'exercices (Strate 6)
  Cargo.toml
  src/
    lib.rs
    admin_cell.rs
    context.rs
    errors.rs
    exercise.rs               -- Types d'exercices
    renderer.rs               -- Rendu des exercices dans Dioxus
    validator.rs              -- Validation des reponses

crates/miyuadaptive/          -- Toolkit algorithme adaptatif (Strate 6)
  Cargo.toml
  src/
    lib.rs
    admin_cell.rs
    context.rs
    errors.rs
    irt.rs                    -- Item Response Theory
    recommender.rs            -- Recommandation d'exercice suivant

crates/miyucurriculum/        -- Toolkit programme scolaire (Strate 6)
  Cargo.toml
  src/
    lib.rs
    admin_cell.rs
    context.rs
    errors.rs
    curriculum.rs             -- Structure programme EN
    skill_tree.rs             -- Arbre de competences

crates/miyuscreenguard/       -- Toolkit temps d'ecran (Strate 6)
  Cargo.toml
  src/
    lib.rs
    admin_cell.rs
    context.rs
    errors.rs
    timer.rs                  -- Minuteur de session
    schedule.rs               -- Horaires autorises
    breaks.rs                 -- Pauses actives
```

### 9.3 Schema de base de donnees (KindMother, esquisse)

```sql
-- Profil enfant
CREATE TABLE child_profiles (
    id TEXT PRIMARY KEY,          -- UUID v4
    parent_profile_id TEXT NOT NULL,
    display_name TEXT NOT NULL,
    avatar_id TEXT,
    school_level TEXT NOT NULL,   -- 'CP', 'CE1', 'CE2', 'CM1', 'CM2'
    birth_year INTEGER,
    created_at TEXT NOT NULL,     -- ISO 8601
    updated_at TEXT NOT NULL
);

-- Competences et niveaux
CREATE TABLE skill_levels (
    id TEXT PRIMARY KEY,
    child_id TEXT NOT NULL REFERENCES child_profiles(id),
    skill_id TEXT NOT NULL,       -- ex: 'math.numeration', 'fr.grammaire'
    level REAL NOT NULL,          -- 0.0 a 1.0
    confidence REAL NOT NULL,     -- confiance dans l'estimation
    last_assessed TEXT NOT NULL,  -- ISO 8601
    UNIQUE(child_id, skill_id)
);

-- Historique des sessions
CREATE TABLE sessions (
    id TEXT PRIMARY KEY,
    child_id TEXT NOT NULL REFERENCES child_profiles(id),
    started_at TEXT NOT NULL,
    ended_at TEXT,
    duration_seconds INTEGER,
    exercises_completed INTEGER DEFAULT 0,
    exercises_correct INTEGER DEFAULT 0
);

-- Historique des exercices
CREATE TABLE exercise_results (
    id TEXT PRIMARY KEY,
    session_id TEXT NOT NULL REFERENCES sessions(id),
    child_id TEXT NOT NULL REFERENCES child_profiles(id),
    exercise_id TEXT NOT NULL,
    skill_id TEXT NOT NULL,
    difficulty REAL NOT NULL,
    answer_correct INTEGER NOT NULL,  -- 0 ou 1
    response_time_ms INTEGER,
    attempted_at TEXT NOT NULL
);

-- Recompenses obtenues
CREATE TABLE rewards (
    id TEXT PRIMARY KEY,
    child_id TEXT NOT NULL REFERENCES child_profiles(id),
    reward_type TEXT NOT NULL,    -- 'star', 'badge', 'trophy', 'puzzle_piece'
    reward_id TEXT NOT NULL,
    obtained_at TEXT NOT NULL
);

-- Parametres parentaux
CREATE TABLE parent_settings (
    child_id TEXT PRIMARY KEY REFERENCES child_profiles(id),
    session_max_minutes INTEGER DEFAULT 20,
    daily_max_sessions INTEGER DEFAULT 2,
    cooldown_minutes INTEGER DEFAULT 30,
    allowed_start_hour INTEGER DEFAULT 7,
    allowed_end_hour INTEGER DEFAULT 20,
    breaks_enabled INTEGER DEFAULT 1,
    break_interval_minutes INTEGER DEFAULT 10,
    parent_pin_hash TEXT NOT NULL
);

-- Contenu : banque d'exercices
CREATE TABLE exercises (
    id TEXT PRIMARY KEY,
    school_level TEXT NOT NULL,
    subject TEXT NOT NULL,        -- 'francais', 'mathematiques', etc.
    domain TEXT NOT NULL,         -- 'numeration', 'grammaire', etc.
    chapter TEXT NOT NULL,
    exercise_type TEXT NOT NULL,  -- 'qcm', 'fill_blank', 'drag_drop', etc.
    difficulty REAL NOT NULL,     -- 0.0 a 1.0
    content_json TEXT NOT NULL,   -- Contenu de l'exercice serialise
    created_at TEXT NOT NULL
);
```

### 9.4 Integration avec miou-llm-bridge (futur)

Pour la version avec assistant IA (post-MVP), le service pourrait utiliser miou-llm-bridge pour :
- **Generation d'exercices** : creer des exercices varies a partir de modeles
- **Explication adaptee** : quand l'enfant se trompe, l'IA genere une explication avec le vocabulaire adapte a son age
- **Assistant Miyu-sensei** : chatbot limite et filtre pour repondre aux questions scolaires
- **Analyse de redaction** : pour les exercices d'expression ecrite (CM1-CM2)

**Contrainte critique :** Tout contenu genere par LLM doit etre **valide et filtre** avant d'etre montre a un enfant. Pas de LLM en direct avec l'enfant sans filet de securite.

### 9.5 Conformite avec les Lois d'Autonomie

| Loi | Application a JayEcole |
|-----|------------------------|
| **LOI-1** (pas de dependance externe) | Tous les exercices et le moteur adaptatif fonctionnent hors-ligne. Le LLM est optionnel. |
| **LOI-2** (isolement = normal) | Le service fonctionne entierement en local. Pas besoin d'internet pour les exercices. |
| **LOI-3** (etat local souverain) | Les donnees de l'enfant sont stockees localement dans KindMother. Aucun serveur distant. |
| **LOI-4** (pas de temps global) | Pas de synchronisation temps reel. Le calendrier scolaire est local. |
| **LOI-5** (cout proportionnel au hardware) | Le moteur adaptatif utilise des algorithmes legers (IRT). Pas de GPU requis. |
| **LOI-6** (federation possible) | Futur : partage de progression entre appareils via MWS. Defis entre amis via Jay1Tribu. |
| **LOI-7** (Cores immuables) | Les Cores (KindMother, StrongFather, etc.) ne sont pas modifies. |
| **LOI-8** (migration = diplomatie) | Export/import du profil enfant entre installations COG. |

---

## 10. Risques et contraintes

### 10.1 RGPD et protection des donnees des mineurs

**Cadre legal :**
- **RGPD Article 8** : le consentement parental est requis pour le traitement des donnees d'enfants de moins de 16 ans (15 ans en France avec la loi Informatique et Libertes)
- **CNIL** : la CNIL a des recommandations specifiques pour les services destines aux mineurs
- **Code de l'education** : utilisation de donnees dans un cadre educatif

**Mesures de conformite :**

| Mesure | Description |
|--------|-------------|
| **Consentement parental explicite** | Le parent doit creer le compte et accepter les CGU avant toute utilisation par l'enfant |
| **Minimisation des donnees** | On ne collecte que ce qui est strictement necessaire (pas d'email enfant, pas de photo, pas de geolocalisation) |
| **Stockage local uniquement** | Conformement aux Lois d'Autonomie, toutes les donnees restent sur la machine de l'utilisateur (KindMother). Aucun envoi a un serveur. Cela simplifie enormement la conformite RGPD. |
| **Pas de tracking** | Aucun cookie tiers, aucune analytics externe, aucune publicite |
| **Droit a l'effacement** | Le parent peut supprimer toutes les donnees de l'enfant a tout moment |
| **Pas de reseau social** | Pas de chat entre enfants dans le MVP. Les futurs "defis entre amis" seront supervises par le parent. |
| **Pseudonyme uniquement** | L'enfant n'est identifie que par un prenom/pseudo et un avatar. Pas de nom de famille, pas de photo reelle. |

**Avantage majeur de l'architecture COG :** Le fait que TOUTES les donnees soient stockees localement (LOI-3) rend JayEcole bien plus simple a mettre en conformite RGPD que les solutions cloud concurrentes. Il n'y a pas de transfert de donnees, pas de serveur a securiser, pas de fuite possible depuis un serveur central.

### 10.2 Ethique de l'IA avec les enfants

| Risque | Mitigation |
|--------|-----------|
| **Hallucination du LLM** | Tout contenu genere est pre-valide par un filtre de coherence. Banque d'exercices prioritaire sur la generation. |
| **Contenu inapproprie** | Filtrage strict. Pas de LLM en acces direct pour l'enfant dans le MVP. |
| **Biais dans l'evaluation** | Algorithme adaptatif transparent et auditable. Pas de boite noire. |
| **Dependance excessive** | Limites de temps d'ecran. Le service ne se substitue pas a l'ecole. |
| **Pression psychologique** | Pas de classement, pas de punition, pas de mecanisme addictif. |

### 10.3 Risques projet

| Risque | Probabilite | Impact | Mitigation |
|--------|-------------|--------|-----------|
| **Volume de contenu enorme** | Elevee | Eleve | Commencer par 2 matieres (francais, maths). Generer du contenu incrementalement. Envisager la generation assistee par LLM. |
| **Complexite du moteur adaptatif** | Moyenne | Eleve | MVP avec parcours lineaire simple. Moteur adaptatif en v1.0 seulement. |
| **Validation pedagogique** | Elevee | Eleve | Consulter un enseignant ou conseiller pedagogique pour valider le contenu et l'alignement avec le programme EN. |
| **UX enfant mal calibree** | Moyenne | Eleve | Tests utilisateurs avec de vrais enfants (avec consentement parental). Iterations rapides. |
| **Scope creep** | Elevee | Moyen | Respecter strictement le perimetre MVP. Documenter les idees pour les versions futures sans les implementer. |
| **Performance des animations** | Faible | Moyen | Animations simples en CSS/SVG. Pas de moteur 3D. Tester sur du hardware modeste. |
| **Maintenance du contenu** | Moyenne | Moyen | Reforme des programmes EN environ tous les 5 ans. Structurer le contenu pour faciliter les mises a jour. |

### 10.4 Contraintes specifiques

1. **Programme EN vivant** : les programmes scolaires evoluent. Le contenu doit etre structure de maniere a pouvoir etre mis a jour sans refactoring massif (d'ou le toolkit MiyuCurriculum separe).

2. **Diversite des enfants** : un enfant de CP qui ne sait pas encore lire n'utilise pas l'interface comme un CM2. L'adaptation par age est critique et non triviale.

3. **Absence de connexion internet** : conformement a LOI-1 et LOI-2, le service doit fonctionner entierement hors-ligne. Toute la banque d'exercices doit etre embarquee.

4. **Taille de la banque d'exercices** : pour 5 niveaux x 5+ matieres x 10+ domaines x 10+ exercices = des milliers d'exercices a creer. C'est le plus gros poste de travail du projet.

---

## 11. Estimation des couts et ressources

### 11.1 Couts de developpement (en jours-homme estimes)

| Poste | Optimiste | Pessimiste | Notes |
|-------|-----------|------------|-------|
| **Architecture et crates** | 5 j | 10 j | Structure des crates, schemas DB, types |
| **Moteur d'exercices (MiyuQuiz)** | 15 j | 25 j | Differents types d'exercices, rendu, validation |
| **Interface enfant** | 20 j | 35 j | Carte du savoir, ecrans d'exercice, animations, mascotte |
| **Interface parent** | 10 j | 18 j | Dashboard, rapports, parametres |
| **Gestion temps d'ecran** | 5 j | 8 j | Minuteur, limites, pauses |
| **Systeme de gamification** | 8 j | 15 j | Etoiles, badges, carte, jardin |
| **Evaluation diagnostique** | 8 j | 12 j | Test adaptatif initial |
| **Contenu pedagogique (MVP)** | 30 j | 60 j | Exercices francais + maths, CP a CM2 |
| **Tests et QA** | 10 j | 20 j | Tests unitaires, tests UX enfants |
| **Documentation** | 5 j | 8 j | Doc fondateur, doc technique, guide parent |
| **TOTAL MVP** | **116 j** | **211 j** | Soit 6-10 mois a 1 dev |

### 11.2 Repartition par agent

| Agent | Responsabilite | Charge estimee |
|-------|---------------|---------------|
| **Maria** | Plan projet, suivi, coordination avec un pedagogues | 5-10 j |
| **Denis** | Architecture, doc technique, review code, securite RGPD | 15-25 j |
| **Francois** | Back-end : crates, moteur adaptatif, DB, services | 40-70 j |
| **Lise** | Front-end : UI enfant, UI parent, animations, mascotte | 35-60 j |
| **George** | Audit conformite RGPD/CNIL, audit UX, tests finaux | 8-15 j |
| **Arianne** | Qualite contenu pedagogique, archivage, memoire projet | 10-20 j |

### 11.3 Ressources externes potentielles

| Ressource | Pourquoi | Cout estime |
|-----------|----------|-------------|
| **Enseignant/conseiller pedagogique** | Validation du contenu par rapport au programme EN | Benevolat ou 500-2000 EUR |
| **Illustrateur mascotte** | Design de Miyu-sensei et des elements visuels de la carte | 500-3000 EUR |
| **Tests utilisateurs** | Tester avec 5-10 enfants reels (avec parents) | 0 EUR (reseau personnel) a 500 EUR |

---

## 12. Questions ouvertes (a clarifier avec l'utilisateur)

**Questions resolues — Round 1 (2026-02-27) :**
1. ~~Nom du service~~ -- **RESOLU : JayEcole**
2. ~~Scope MVP~~ -- **RESOLU : Francais + Mathematiques uniquement**
3. ~~Cible geographique~~ -- **RESOLU : France uniquement (Education Nationale)**
4. ~~Monetisation~~ -- **RESOLU : Gratuit**
8. ~~Priorite projet~~ -- **RESOLU : Priorite moyenne dans la roadmap Miyukini**

**Questions resolues — Round 2 (2026-02-27) :**
5. ~~Mascotte~~ -- **RESOLU : Humanoide, cree de toutes pieces pour JayEcole**
6. ~~Integration Central~~ -- **RESOLU : Service lance separement (binaire independant)**
7. ~~Public reel~~ -- **RESOLU : Oui, enfants disponibles pour tester**
9. ~~Contenu existant~~ -- **RESOLU : Tout a creer from scratch**
10. ~~Accessibilite~~ -- **RESOLU : Support DYS des le MVP**

**Nouvelles questions ouvertes — Round 3 :**
11. **Concept de construction** : Quel mecanisme concret ? Village, navire, atelier, cabane dans l'arbre, autre ? -- EN ATTENTE
12. **Apparence Miyu-sensei** : Quel style visuel pour le personnage humanoide ? -- EN ATTENTE
13. **Voix audio** : TTS locale ou voix humaine enregistree pour Miyu-sensei et les consignes ? -- EN ATTENTE
14. **Implication institutrice** : Quel niveau d'implication ? Validation ponctuelle ou co-creation du contenu ? -- EN ATTENTE
15. **Format de distribution** : Installeur Windows, portable, ou les deux ? -- EN ATTENTE

Voir `BRAINSTORM-round3-construction.md` pour l'approfondissement de ces questions.

---

## 13. Prochaines etapes

1. ~~Validation de ce brainstorming par l'utilisateur~~ -- FAIT (2026-02-27)
2. ~~Brainstorming approfondissement~~ -- FAIT, voir `BRAINSTORM-approfondissement.md`
3. ~~Decisions round 2~~ -- FAIT (2026-02-27), voir section "Decisions validees"
4. **Brainstorming round 3 — construction** : pivot narratif, Miyu-sensei humanoide, CP prioritaire, DYS, audio, service separe -- voir `BRAINSTORM-round3-construction.md`
5. **Decisions round 3** : concept de construction, apparence Miyu-sensei, voix audio, implication institutrice, format distribution
6. **Redaction du Document Fondateur** (format norme Miyukini) par Maria
7. **Transmission a Denis** pour la documentation technique et l'architecture detaillee des crates
8. **Creation d'un prototype UI** par Lise (maquettes du systeme de construction et d'un ecran d'exercice CP)
9. **Recherche de contenu pedagogique** : programme EN, avec validation par l'institutrice
10. **Archivage** par Arianne de ce brainstorming et des decisions prises

---

*Document redige par Maria, Chef de Projet Miyukini AI Studio*
*Brainstorming initial — Valide par l'utilisateur le 2026-02-27*
*Decisions round 1 + round 2 integrees*
*Nom de service definitif : JayEcole*
