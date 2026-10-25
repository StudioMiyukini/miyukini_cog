# Alliances — revue des failles et manquements

> **Conception v2 — hors gel, non normatif.** Voir l'[index](_index.md) pour les conventions (décidé / proposé / question).

<!-- @id arch.alliance.revue-failles -->
<!-- @role design-review -->
<!-- @layer 5-8+zones -->
<!-- @human Revue critique de la conception des Alliances au 2026-10-25 (décisions AL-D01..AL-D41) : failles, manquements, recommandations, questions ouvertes -->
<!-- @do review_alliance_design -->

## Contexte

- 41 décisions prises le 2026-10-25 ; avant d'aller vers un gel v2, il faut chercher ce qui casse quand on attaque la conception, et ce qui manque pour la faire tourner
- Méthode : pour chaque mécanisme, la question « que se passe-t-il si l'hypothèse tombe ? » (membres indépendants, humains disponibles, réseau joignable, matériel en sécurité, droit respecté)

## Portée / Scope

- Les documents du dossier `alliance/` et leurs points d'appui dans la [doc gelée v1](../GEL-DOC-CONCEPTUELLE-v1.md)
- Hors périmètre : défauts du code (voir [revue des constats](../../implementation/REVUE-CONSTATS.md)), conformité générale de COG (voir [synthèse conformité](../../conformite/CONFORMITE-SYNTHESE.md)), sauf quand elle bloque une Alliance

## 1. Synthèse

| Id | Gravité | Faille ou manquement | Question |
|---|---|---|---|
| AL-F01 | **critique** | Une mise à jour touche tous les membres du CC en même temps : la tolérance aux membres compromis est annulée | **traitée : AL-D42** |
| AL-F02 | **critique** | Quorum de machines ≠ quorum de personnes : un seul humain ou une seule équipe peut tenir tous les membres | **traitée : AL-D43** |
| AL-F03 | haute | Genèse du CC et garde de la racine de l'organisation non décrites | **traitée : AL-D44** |
| AL-F04 | haute | Perte définitive du quorum : lecture seule, mais aucune refondation ni procédure de bris de glace | **traitée : AL-D45** |
| AL-F05 | haute | Connexion des utilisateurs quand le CC est en lecture seule : non tranchée | **traitée : AL-D46** |
| AL-F06 | haute | Portable volé ou salarié licencié hors ligne : vérificateur local 7 jours et données sur le poste | **traitée : AL-D47** |
| AL-F07 | haute | Le coût d'entrée (6 à 7 COG au CC, modèle IA sur chacun) contredit le marché d'entrée (TPE, PME) | [AL-Q51](ALLIANCE-DECISIONS.md) |
| AL-F08 | moyenne | Surveillance des salariés : journaux de navigation, santé des postes, journal du CC, sans cadre droit du travail | [AL-Q48](ALLIANCE-DECISIONS.md) |
| AL-F09 | moyenne | Une révocation urgente la nuit peut attendre un humain TAMR | **traitée : AL-D48** |
| AL-F10 | moyenne | Source de temps de l'Alliance non définie (Kerberos, Median-Time-Past, délai de 2 ans) | [AL-Q50](ALLIANCE-DECISIONS.md) |
| AL-F11 | moyenne | Injection indirecte par Alliance-MAIA ou Alliance-Files : un initié piège un contenu que MAIA lit | proposé §3 |
| AL-F12 | moyenne | Une seule COG-Gateway : sa panne coupe télétravail et Internet | proposé §3 |
| AL-F13 | moyenne | Obligations déjà en vigueur (CRA, AI Act art. 50) non couvertes : elles bloquent tout pilote, HECTOR compris | proposé §3 |
| AL-F14 | basse | Plafond de 12 membres par CC (Sceau ≤ 8 signatures) | [AL-Q11](ALLIANCE-DECISIONS.md) |
| AL-F15 | basse | Formats nouveaux (arrêté, attestation, approbation, liaison…) sans type Sceau attribué | [synthèse §5](ALLIANCE-SYNTHESE.md) |

## 2. Failles critiques et hautes

<!-- @id arch.alliance.revue-failles-hautes -->
<!-- @role findings -->
<!-- @do list_high_findings -->

### AL-F01 · Mode commun logiciel : mise à jour simultanée du CC

- **Constat** : les membres du CC reçoivent leurs mises à jour par le même canal officiel (A-006). Une version piégée ou boguée, installée partout en même temps, compromet ou arrête `n` membres d'un coup. La tolérance n ≥ 3f+1 suppose des pannes **indépendantes** ; une mise à jour commune la réduit à zéro
- **Même risque** sur les postes : une mise à jour défectueuse arrête toute l'Alliance
- **Décidé (AL-D42)** — recommandation retenue telle quelle, consignée dans [COG-CC §5.2](COG-CC.md)
  - **Vagues** : jamais plus de `f` membres du CC mis à jour en même temps ; vague suivante seulement après une période d'observation (proposé 24 h) avec attestations d'intégrité saines (COG-CC §4)
  - Version d'un membre = partie de son attestation ; un écart de versions est normal pendant une vague, anormal au-delà
  - **Retour arrière** local possible pendant la période d'observation
  - Postes : cohortes (pilote ≈ 5 %, puis le reste), décidées par politique
  - Mise à jour du CC = **classe 2** (quorum) ; d'une version qui touche le protocole de consensus ou le format du journal = **classe 3**

### AL-F02 · Mode commun humain : un quorum de machines n'est pas un quorum de personnes

- **Constat** : le quorum protège contre un membre compromis. Mais si les 7 membres sont administrés par le même compte, approuvés par la même personne TAMR, ou si une personne détient plusieurs parts de la clé à seuil, une seule compromission humaine (hameçonnage, contrainte, malveillance) suffit
- La v1 définit TAMR pour **le propriétaire** d'un COG ; l'Alliance n'a pas encore dit **qui sont les humains** de la classe 3, ni combien
- **Décidé (AL-D43)** — recommandation retenue telle quelle, consignée dans [COG-CC §5.1](COG-CC.md)
  - **Collège d'approbateurs** désigné dans l'annuaire (comptes privilégiés séparés, Alliance-Admin §2) ; une approbation de classe 3 exige **k personnes distinctes sur n** (proposé : 2 sur 3 au minimum ; 3 sur 5 recommandé)
  - L'auteur d'une demande ne peut pas l'approuver (séparation des tâches)
  - **Une personne = au plus une part** de toute clé à seuil (recouvrement MAIA, clés Kerberos, racine de l'organisation)
  - Exploitation des membres du CC répartie entre **au moins 2 personnes** ; aucun compte d'exploitation commun aux membres
  - Approbations par COG-Auth (AL-D33), sur des appareils distincts

### AL-F03 · Genèse du CC et racine de l'organisation

- **Constat** : AL-D06 donne la racine de chaque poste à l'organisation. Cette racine est donc la **clé suprême de l'Alliance** : qui la détient peut enrôler des postes et refonder le CC. Ni sa garde ni la cérémonie qui crée le premier CC ne sont décrites
- **Décidé (AL-D44)** — recommandation retenue telle quelle, consignée dans [COG-CC §4.1](COG-CC.md)
  - **Racine d'organisation hors ligne, à seuil** (proposé 3 porteurs sur 5), sur supports dédiés comme la racine USB v1 ; jamais sur un membre du CC
  - **Cérémonie de genèse** (TAMR, présence physique, procès-verbal signé) : la racine signe la liste initiale des membres, leurs clés et les paramètres (`n`, `f`, `q`) ; c'est la tête 0 du journal
  - Après la genèse, la racine ne sert qu'à : refonder le CC (AL-F04), changer de collège d'approbateurs, renouveler la racine elle-même
  - Enrôlement des postes par **délégation** : la racine signe une fois un certificat d'enrôlement pour le CC, à durée limitée ; le CC enrôle ensuite au quorum

### AL-F04 · Perte définitive du quorum et bris de glace

- **Constat** : avec moins de `q` membres sains, le CC passe en lecture seule (COG-CC §4). Rien ne dit comment en sortir si les membres perdus ne reviennent pas (incendie, saisie, compromission massive), ni comment révoquer d'urgence pendant ce temps
- **Décidé (AL-D45)** — recommandation retenue telle quelle, consignée dans [COG-CC §4.1](COG-CC.md)
  - **Refondation** par cérémonie de la racine d'organisation (AL-F03) : nouveaux membres, repartis de la **dernière tête de journal attestée par au moins `f + 1` membres** ; jamais d'une tête signée par moins
  - **Bris de glace** pendant la lecture seule : la racine d'organisation peut **seulement restreindre** (révoquer un compte, un poste, une Alliance liée ; suspendre une politique). Elle ne peut ni créer de compte ni ouvrir de droit
  - Chaque usage inscrit dans le journal refondé et notifié à tous les approbateurs
  - Exercice de refondation inclus dans le critère de réussite du pilote (AL-Q21)

### AL-F05 · Connexion quand le CC est en lecture seule

- **Constat** : la connexion d'un compte demande un **visa** au CC (Alliance-Admin §3). Si l'émission d'un visa est une écriture au journal, une perte de quorum empêche toute l'organisation de travailler — un déni de service obtenu en arrêtant `n − q + 1` membres
- **Décidé (AL-D46)** — recommandation retenue telle quelle, consignée dans [Alliance-Admin §3](ALLIANCE-ADMIN.md)
  - L'émission d'un visa **n'est pas une écriture ordonnée** : tout membre sain émet un visa court (proposé ≤ 8 h) après vérification du passeport et des révocations ; trace locale, versée au journal ensuite
  - Un membre qui émet des visas pour des comptes révoqués est détecté par l'intégrité collective (COG-CC §4) et suspendu
  - En lecture seule : connexions possibles, **aucune** écriture d'annuaire, sauf le bris de glace qui restreint (AL-F04)

### AL-F06 · Poste portable volé, salarié licencié hors ligne

- **Constat** : un portable garde un vérificateur local des derniers comptes pendant 7 jours (AL-Q17) et toutes les données de l'utilisateur (AL-D23). Un voleur peut attaquer le vérificateur hors ligne ; un salarié licencié qui ne reconnecte pas son portable garde l'accès local pendant la fenêtre
- **Décidé (AL-D47)** — recommandation retenue telle quelle
  - **Chiffrement intégral** du poste, clé scellée par le TPM et liée au compte ; sans TPM, phrase de démarrage
  - Vérificateur local à dérivation lente (Argon2id), effacé dès qu'une révocation est connue du poste
  - Fenêtre hors ligne réduite pour les comptes sensibles (proposé 24 h) ; 0 pour les comptes privilégiés
  - AL-D40 complété : la **restitution du matériel** fait partie du départ ; un poste non restitué est exclu (vue globale §6, étape 4)

### AL-F07 · Coût d'entrée et marché d'entrée

- **Constat** : un CC de 6 à 7 COG (AL-D08), chacun avec un modèle local pour HECTOR (AL-Q15), plus une Gateway, représente un investissement matériel et d'exploitation que peu de TPE ou de PME peuvent porter. Or c'est le marché d'entrée recommandé ([positionnement](../../market/POSITIONNEMENT-COG.md))
- **Recommandation (AL-Q51)**
  - Fixer le seuil de pertinence d'une Alliance (proposé : à partir de ≈ 25 postes ou de 2 sites)
  - En dessous : un **COG seul** multi-utilisateurs (v1) suffit, sans quorum ni HECTOR
  - HECTOR **facultatif** : sans lui, les procédures du catalogue restent disponibles en mode guidé, traitées par un humain
  - Membres du CC sur matériel modeste (mini-PC) quand HECTOR est désactivé ; un seul membre avec GPU ne suffit pas (il deviendrait un point unique)
  - Ne jamais réduire `n` sous 6 pour baisser le coût : c'est la sécurité même de l'Alliance

## 3. Failles moyennes et basses

<!-- @id arch.alliance.revue-failles-moyennes -->
<!-- @role findings -->
<!-- @do list_medium_findings -->

- **AL-F08 · Surveillance des salariés** (AL-Q48)
  - Collectes : domaines et volumes de navigation (AL-D15), santé des postes (CaringNanny), journal du CC (connexions, actions), demandes à HECTOR
  - En France, un dispositif de contrôle de l'activité des salariés exige au moins leur information préalable et celle du CSE, la proportionnalité et une durée de conservation (**à vérifier** par la conformité)
  - **Recommandation** : registre des collectes de l'Alliance (finalité, données, durée, accès) produit automatiquement à partir des politiques ; **AIPD de l'Alliance** avant le pilote ; résumé lisible par chaque utilisateur sur son poste
- **AL-F09 · Asymétrie restreindre / élargir** — **décidé (AL-D48)**, recommandation retenue
  - Une révocation urgente (compte compromis la nuit) ne doit pas attendre un humain
  - **Recommandation** : principe d'asymétrie — ce qui **restreint** (révoquer, suspendre, isoler un poste) est au plus classe 2 et peut être déclenché automatiquement par une alerte WorrySentinel T3 ou plus, confirmé ensuite ; ce qui **élargit** garde la classe prévue (jusqu'à 3)
  - HECTOR ne peut **pas** déclencher seul une restriction : il la propose, StrongFather et le quorum décident (sinon une injection provoquerait un déni de service par révocations)
- **AL-F10 · Temps de l'Alliance** (AL-Q50)
  - Kerberos tolère 5 min ; le délai de 2 ans (AL-D26) et les durées des visas dépendent d'une horloge sûre
  - **Recommandation** : le CC sert le temps aux postes (NTS, RFC 8915), chaque membre comparant les autres ; dérive au-delà d'un seuil ⇒ alerte ; décisions datées par le Median-Time-Past du journal (A-006), jamais par l'horloge d'un seul membre
- **AL-F11 · Injection indirecte** (proposé, sans question)
  - Un initié écrit une Fiche d'Alliance-MAIA ou un document d'Alliance-Files contenant des consignes ; MAIA le lit
  - Atténuations déjà acquises : normalisation des Fiches (A-009), texte traité comme donnée, MAIA ne décide rien
  - **Proposé** : Fiches toujours **attribuées** à leur auteur signataire et citées par MAIA ; un contenu lu ne peut ni changer un prompt, ni déclencher une compétence, ni être recopié en mémoire sans le filtre d'AL-D38
- **AL-F12 · Gateway unique** (proposé, sans question)
  - AL-D12 fixe 1 Gateway au minimum ; sa panne coupe télétravail et Internet
  - **Proposé** : 2 Gateways requises dès que le télétravail est ouvert ; 1 tolérée pour un pilote
- **AL-F13 · Obligations en vigueur** (proposé, sans question)
  - Signalement CRA (depuis le 11/09/2026) et article 50 de l'AI Act (depuis le 02/08/2026) sont absents ([synthèse conformité](../../conformite/CONFORMITE-SYNTHESE.md) §1)
  - HECTOR parle aux utilisateurs (P-001) : il doit lui aussi se déclarer IA
  - **Proposé** : le lot 0 bis est un **préalable** au pilote d'Alliance (AL-Q21)
- **AL-F14 · Plafond de taille du CC** : `n = 12` donne `q = 8`, la limite du Sceau v1.1 ; au-delà, plusieurs CC (AL-Q11). Rien à changer, à écrire dans le gel v2
- **AL-F15 · Formats sans type attribué** : voir la [liste des formats](ALLIANCE-SYNTHESE.md) ; à attribuer par l'amendement du gel v2

## 4. Ce que la revue confirme

- Le principe « HECTOR propose, il ne décide pas » tient : aucune faille trouvée ne donne une autorité à l'IA
- Les liaisons inter-Alliances (AL-D18, AL-D19, AL-D20) résistent à une Alliance liée compromise
- La localité des données (AL-D23) et l'absence d'effacement à distance limitent fortement l'effet d'une compromission du CC
- AL-D37 / AL-D38 retirent l'essentiel du risque RGPD de la mémoire MAIA

---

Date de mise a jour : 2026-10-25
