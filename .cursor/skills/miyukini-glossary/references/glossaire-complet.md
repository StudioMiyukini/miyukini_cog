# Glossaire Complet Miyukini COG

Reference complete de la terminologie officielle. Ce fichier est consulte par l'agent uniquement quand une verification terminologique approfondie est necessaire.

Source de verite : `docs/reference/Miyukini Conceptual References - Glossaire.md`

---

## A

### ACTIF (ACTIVE)
Etat d'un element en usage normal. Stable, documente, supporte.

---

## B

### BondingBrother
Core de mediation (Strate 5). Traduit les intentions des Operateurs en demandes pour les Cores. Role : mediation uniquement, jamais d'autorite.

### Border Guard
Core de frontieres (Strate 4). Definit les frontieres du systeme et les niveaux de confiance. Definition conceptuelle, pas d'application directe.

### Bridge inter-COG
Canal diplomatique entre COG, extension de BondingBrother. Transporte identites, intentions et autorisations. Aucun pouvoir decisionnel, aucun etat metier. Regle : "Le bridge ne fait jamais confiance, il transporte."

### BROUILLON (DRAFT)
Etat d'un element en cours de definition. Non utilisable en production.

---

## C

### Capacite (Capability)
Pouvoir technique qu'un composant possede. Intrinseque, technique, declarative, identifiable. Distincte de la Permission (droit accorde).

### Caring Nanny
Core d'observation d'etat (Strate 4). Detecte, classe et propage les etats. Observe et rapporte sans modifier, decider ou executer.

### COG (Core-Orchestrated Governance Environment)
Definition officielle de Miyukini. C = Core, O = Orchestrated (pas "operating"), G = Governance Environment.

### COG Hebergeur (Host COG)
COG souverain qui accueille un Utilisateur Visiteur. Souverain executif de la session, unique source de verite, autorite de securite.

### COG Origine (Home COG)
COG d'appartenance d'un Utilisateur Visiteur. Autorite d'identite, garant de la conformite, emetteur du Passeport. Ne participe PAS a l'execution distante.

### Collaboration Mandatee
Cooperation entre Operateurs sous Mandat de Permission. Pas de communication directe — passage par BondingBrother obligatoire.

### Contrat d'Equipe
Regles statiques de collaboration. Operateurs membres, flux autorises, types d'echanges, conditions prealables. Valide par StrongFather une seule fois.

### Cores
Moteurs conceptuels (Strate 4). 8 cores : StrongFather, KindMother, Caring Nanny, Master Butler, Border Guard, Ever Buddy, WorrySentinel, TAMR. Regle : decident ou gouvernent, n'executent jamais.

---

## D

### Demande de Visite (Visit Intent)
Intention d'acces emise par un Visiteur vers un COG Hebergeur. C'est une intention, pas une permission.

### DEPRECIE (DEPRECATED)
Element fonctionnel mais dont l'usage est decourage. Passage obligatoire avant RETIRE.

### Divergence silencieuse
Systeme qui declare une version mais presente une empreinte differente. Signal de maintenance, pas d'erreur. Le Kernel signale mais ne corrige jamais.

---

## E

### Empreinte comportementale
Signature structurelle du systeme charge. Capture : ordre de chargement, graphe d'appel, contrats invoques, invariants. C'est une signature, pas un log.

### Environnement (COG)
Entite souveraine, versionnee, isolee et identifiee. La strate Cores est immuable.

### Equipe d'Operateurs
Collectif gouverne d'Operateurs sous regles explicites. Minimum 2, heterogenes en securite, lies par un Contrat d'Equipe. N'est PAS un nouvel Operateur ni une hierarchie libre.

### Etats de confiance (T0-T4)
T0 Normal, T1 Instable, T2 Degrade, T3 Restreint, T4 Bloque.

### Ever Buddy
Core de cycle de vie (Strate 4). Gouverne l'evolution des structures sans jamais executer de migration.

---

## F

### Facade Publique Gouvernee
Zone tampon d'exposition. Strictement unidirectionnelle. C'est le COG qui sort vers l'utilisateur, jamais l'inverse.

---

## G

### Gel local
Capacite du Kernel a marquer un composant comme gele. Decide par la gouvernance, execute par le Kernel.

---

## K

### Kernel
Substrat technique neutre (Strate 0-3). Composants : Id, Logger, Clock, Config, Lifecycle. Aucune logique metier, aucune dependance externe critique.

### Kernel Maintenance Observability
Capacites bas niveau : empreinte comportementale, detection de divergence, carte de complexite, gel local, detection d'ambiguite, maintenance explicable. Le Kernel observe et atteste, ne corrige jamais.

### KindMother
Core de donnees (Strate 4). Autorite absolue de la persistance et synchronisation.

---

## L

### Local Sovereign ID (LSI)
Niveau 1 d'identite. Generee par le kernel local. Confiance : souveraine.

### LOI-1 a LOI-8
8 lois d'autonomie non negociables (voir SKILL miyukini-architecture).

---

## M

### Maintenance explicable
Diagnostic qui explique le chemin de gouvernance, jamais l'implementation. Pas de stacktrace, pas de dump memoire, pas de donnees utilisateur.

### Mandat de Permission
Autorisation deleguee, temporaire et encadree, emise par StrongFather. Ce n'est PAS un token, une session, un cache de decision, ou un droit implicite.

### Mandat Public d'Acces
Autorisation attachee a un service public pour les utilisateurs externes. Attache au service, pas a l'utilisateur.

### Master Butler
Core de capacites (Strate 4). Registre des capacites et permissions. Declare quels Outils existent mais ne les implemente ni execute.

### Migration
Processus formel d'echange de donnees entre environnements. Contrat explicite, frontiere controlee, traduction (pas copie brute).

### MiyukiniAdmin
Operateur Souverain (Strate 9). Exception a la logique Operateur standard. Autorite quasi institutionnelle.

---

## N

### Niveaux de securite (0-4)
0 Public, 1 Standard, 2 Sensitive, 3 Critical, 4 Highest.

---

## O

### Operateur
Entite fonctionnelle gouvernee (Strate 7). Types : Service, Interface, Automatisation, Domaine, Souverain. N'est PAS un produit, une app, autonome, ou souverain.

### Outil (Tool)
Capacite executable gouvernee (Strate 6). Sans autorite, sans logique metier, sans connaissance du contexte. Fait mais ne decide jamais.

---

## P

### Passeport Utilisateur
Attestation d'identite emise par un COG Origine. Non falsifiable, non transferable. Ne donne aucun droit, prouve seulement l'identite.

### Permission
Droit accorde pour acceder a une capacite. Definie, associee, attribuable, revocable, tracable.

### Pyramide Miyukini
Architecture en strates 0-9 (voir SKILL miyukini-architecture).

---

## R

### RETIRE (RETIRED)
Element retire. Non disponible. Transition depuis DEPRECIE uniquement, pas de retour.

---

## S

### Securite Heterogene
Un Operateur = un seul niveau. Une Equipe peut combiner plusieurs. Risque segmente, pas securite uniforme.

### Service
Capacite percue par l'utilisateur. Peut etre porte par un Operateur ou une Equipe.

### Souverainete
Un COG est souverain, versionne, isole. La strate Cores est immuable. Pas de patch, que des environnements complets.

### StrongFather
Core de decision (Strate 4). Decide si une action devrait etre faite. Emetteur des Mandats de Permission. Decision != Execution.

---

## T

### TAMR (Trust & Authority Mediation Resolver)
Core d'intervention humaine (Strate 4). Definit quand l'humain a le droit d'intervenir.

---

## U

### Utilisateur Externe
Consommateur non certifie. Sans identite souveraine, sans COG d'origine. Acces via Facade Publique Gouvernee uniquement. N'entre jamais dans un COG.

### Utilisateur Visiteur
Utilisateur accedant temporairement a un COG etranger. Citoyen chez lui, visiteur ailleurs. Perd toute souverainete d'execution hors de son COG.

---

## V

### Verified ID (VID)
Niveau 2 d'identite. LSI verifiee par un registre global.

### Visa de Connexion
Autorisation temporaire emise par un COG Hebergeur. Temporaire, revocable, non transferable, auditee. Niveaux : S1 Observation → S5 Critique.

### Visite gouvernee inter-COG
Modele d'acces temporaire. Aucun core n'est partage, aucun etat migre en direct, aucun pouvoir delegue.

### Witnessed ID (WID)
Niveau 3 d'identite. Verifiee par echange indirect.

---

## W

### WorrySentinel
Core de gouvernance de securite (Strate 4). Gouverne les niveaux de securite et etats de confiance sans executer de controle technique.

### WriteIntent (Intention d'Ecriture)
Intention d'ecriture soumise a KindMother. Peut etre acceptee, refusee ou differee.
