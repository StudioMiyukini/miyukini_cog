# COG-CC — centre de commandement d'une Alliance

> **Conception v2 — hors gel, non normatif.** Voir l'[index](_index.md) pour les conventions (décidé / proposé / question).

<!-- @id arch.alliance.cog-cc -->
<!-- @role design-command-center -->
<!-- @layer 5-8 -->
<!-- @human Groupe d'au moins 6 COG formant le quorum administratif d'une Alliance : rôles, quorum, base répliquée, intégrité collective, classes d'actions -->
<!-- @do govern_alliance_quorum -->

## Contexte

- L'Alliance a besoin d'une autorité administrative **sans point unique de compromission** : aucun COG seul ne doit pouvoir modifier l'annuaire, émettre une politique ou réinitialiser un compte privilégié
- Le COG-CC répartit cette autorité sur plusieurs COG qui se surveillent mutuellement

## Portée / Scope

- Composition, rôles, quorum, base de données, intégrité collective, classes d'actions
- L'IA est décrite dans [HECTOR](HECTOR.md) ; l'annuaire dans [Alliance-Admin](ALLIANCE-ADMIN.md)

## 1. Composition

<!-- @id arch.alliance.cc-composition -->
<!-- @role composition -->
<!-- @do compose_command_center -->

- **Décidé (propriétaire)**
  - **≥ 6 COG**, chacun avec **sa** base et ses services de supervision et d'administration
  - Répartition de charge des requêtes ; vérification d'intégrité collective quasi permanente
  - Un COG-CC peut contenir **plus de 6** COG
  - Rôles HECTOR : **1 superviseur**, **1 relève** (fail-over), **4 traitants** (sous-agents de traitement) au minimum
- **Proposé**
  - Chaque membre du CC est un COG complet (racine propre, Cores, Douane, Portail) : **aucune clé partagée** entre membres
  - Les membres sont inscrits au journal du CC à leur entrée (quorum) ; leur ensemble forme l'**ensemble des signataires**
  - Diversité exigée : au moins deux sites physiques ou deux alimentations ([AL-Q08](ALLIANCE-DECISIONS.md))

## 2. Tolérance aux pannes et aux compromissions

<!-- @id arch.alliance.cc-tolerance -->
<!-- @role fault-tolerance -->
<!-- @do size_quorum -->

- Pour résister à `f` membres **compromis** (et pas seulement en panne), il faut `n ≥ 3f + 1` et un quorum `q = ⌈(n + f + 1) / 2⌉`

| n (membres) | f compromis tolérés | quorum q | Remarque |
|---|---|---|---|
| 6 | 1 | 4 | minimum décidé |
| 7 | 2 | 5 | **un membre de plus double la tolérance** |
| 10 | 3 | 7 | |
| 12 | 3 | 8 | plafond avec un seul sceau (≤ 8 signatures, A-002) |

- **Décidé (propriétaire, AL-D08)** : **6 membres minimum, 7 recommandés** en production (6 tolèrent un seul membre compromis, 7 en tolèrent deux)
- **Proposé** : les [COG-Gateway](COG-GATEWAY.md) ne sont jamais membres du CC ; le CC fixe leurs règles (arrêtés de frontière d'alliance) et signe les accords inter-Alliances
- **Constat** : au-delà de 12 membres, le quorum dépasse 8 signatures ; il faudra soit un **comité tiré au sort** parmi les membres, soit un bloc de signatures plus large (amendement du Sceau)

## 3. Base de données et journal

<!-- @id arch.alliance.cc-base -->
<!-- @role replicated-state -->
<!-- @do replicate_command_center_state -->

- **Proposé**
  - Chaque membre tient une **instance Mère KindMother** de la base du CC (annuaire, politiques, demandes, décisions)
  - Les écritures passent par un **journal ordonné** et répliqué : une écriture n'est appliquée qu'une fois **inscrite au quorum**
  - Lectures servies par n'importe quel membre (répartition de charge) ; une lecture sensible (secret, compte privilégié) exige une réponse concordante de **2 membres**
  - Journal = **registre Merkle** (format `MRGJ`, A-006) ; têtes contresignées par ≥ q membres, comme le registre fédéral (A-007) mais avec le quorum du CC
  - **Décidé (AL-D22)** : ordonnancement par un protocole **tolérant aux membres compromis** (famille HotStuff), pas un protocole « pannes seules » de type Raft

## 4. Intégrité collective

<!-- @id arch.alliance.cc-integrite -->
<!-- @role collective-integrity -->
<!-- @do cross_check_members -->

- **Proposé** — chaque membre, à chaque période (défaut proposé : 30 s)
  - publie une **attestation signée** : tête du journal, empreinte de l'état de la base, empreintes des binaires des services `Alliance-*`, du modèle et du **catalogue de prompts** de HECTOR, niveau WorrySentinel
  - compare celles des autres
- Écart détecté
  - deux têtes signées incompatibles d'un même membre ⇒ preuve d'**équivoque** (type `MFPV`) ⇒ membre **suspendu** automatiquement, signal **critique** à WorrySentinel, TAMR saisi
  - binaire, modèle ou catalogue différent ⇒ membre **mis à l'écart** de HECTOR et du quorum jusqu'à réalignement validé
  - attestation absente au-delà de 3 périodes ⇒ membre **muet** : retiré de la répartition de charge, compté absent pour le quorum
- Si le nombre de membres sains passe sous `q` ⇒ le CC passe en **lecture seule** (aucune écriture, aucune politique), état T3 côté WorrySentinel, décisions à TAMR
- Fail-closed (C-12) : doute sur une attestation ⇒ membre traité comme muet

### 4.1 Racine d'organisation, genèse et refondation (décidé, AL-D44, AL-D45)

- **Racine d'organisation** (AL-D44) : clé suprême de l'Alliance, puisqu'elle détient la racine de chaque poste (AL-D06)
  - **hors ligne, à seuil** : 3 porteurs sur 5, sur supports dédiés comme la racine USB v1 ; jamais sur un membre du CC ; une part par personne (AL-D43)
  - sert seulement à : la genèse, la refondation, le changement du collège d'approbateurs, son propre renouvellement, et la délégation d'enrôlement
- **Genèse** (AL-D44) : cérémonie TAMR en présence physique, procès-verbal signé ; la racine signe la liste initiale des membres, leurs clés et les paramètres `n`, `f`, `q` ; c'est la **tête 0** du journal
- **Enrôlement par délégation** (AL-D44) : la racine signe un certificat d'enrôlement pour le CC, à durée limitée ; le CC enrôle ensuite les postes au quorum (classe 2)
- **Refondation** (AL-D45), quand le quorum est perdu définitivement : cérémonie de la racine ; nouveaux membres repartis de la **dernière tête de journal attestée par au moins `f + 1` membres**, jamais d'une tête signée par moins
- **Bris de glace** (AL-D45), pendant la lecture seule : la racine peut **seulement restreindre** (révoquer un compte, un poste, une Alliance liée ; suspendre une politique) ; jamais créer un compte ni ouvrir un droit ; chaque usage inscrit au journal refondé et notifié au collège
- Un **exercice de refondation** fait partie du critère de réussite du pilote ([AL-Q21](ALLIANCE-DECISIONS.md))

## 5. Classes d'actions

<!-- @id arch.alliance.cc-classes -->
<!-- @role action-classes -->
<!-- @do classify_alliance_actions -->

- **Proposé** — toute action dans l'Alliance tombe dans une classe, fixée par le catalogue et jamais par HECTOR

| Classe | Exemples | Décision | Humain |
|---|---|---|---|
| **0 · consultation** | état d'un poste, appartenance à un groupe | un membre | non |
| **1 · courante** | mot de passe perdu d'un compte ordinaire, déverrouillage, ajout à un groupe ordinaire | HECTOR propose, **StrongFather** du CC décide, double analyse (HECTOR §4) | non, journalisé |
| **2 · sensible** | création de compte, politique restrictive, politique élargissante **dans l'enveloppe** d'enrôlement, enrôlement de poste | **quorum q** du CC | revue a posteriori |
| **3 · critique** | compte privilégié, politique élargissante **hors enveloppe** (AL-D21), exclusion de poste, entrée ou sortie d'un membre du CC, rotation de clés | quorum q **et** APPROVAL TAMR (D2) par le **collège** (AL-D43) | oui, avant |

- Une action ne peut **jamais** descendre de classe par le contenu d'une demande ; seule une politique de classe 3 modifie le classement
- **Asymétrie (décidé, AL-D48)** : ce qui **restreint** (révoquer, suspendre, isoler un poste) est au plus classe 2 et peut être déclenché **automatiquement** par une alerte WorrySentinel T3 ou plus, puis confirmé ; ce qui **élargit** garde sa classe ; **HECTOR ne restreint jamais seul** (une injection ne doit pas provoquer de révocations en masse)

### 5.1 Collège d'approbateurs (décidé, AL-D43)

- Les humains de la classe 3 forment un **collège** désigné dans l'annuaire, sur des comptes privilégiés séparés des comptes de travail
- Une approbation de classe 3 exige **k personnes distinctes sur n** : **2 sur 3 au minimum, 3 sur 5 recommandé**
- L'auteur d'une demande ne peut pas l'approuver (séparation des tâches)
- **Une personne détient au plus une part** de toute clé à seuil (recouvrement MAIA, clés Kerberos, racine de l'organisation)
- L'exploitation des membres du CC est répartie entre **au moins 2 personnes** ; aucun compte d'exploitation n'est commun à plusieurs membres
- Approbations signées par COG-Auth (AL-D33), sur des appareils distincts

### 5.2 Mises à jour par vagues (décidé, AL-D42)

- **Jamais plus de `f` membres** du CC mis à jour en même temps : une mise à jour commune ne doit pas pouvoir annuler la tolérance aux membres compromis
- Vague suivante seulement après une **observation de 24 h** avec attestations d'intégrité saines (§4) ; la version installée fait partie de l'attestation
- **Retour arrière** local possible pendant l'observation
- Postes : par **cohortes** (pilote ≈ 5 %, puis le reste), fixées par politique
- Mise à jour du CC = **classe 2** ; si elle touche le protocole de consensus ou le format du journal = **classe 3**

## 6. Points à trancher

- [AL-Q08](ALLIANCE-DECISIONS.md) diversité physique · [AL-Q11](ALLIANCE-DECISIONS.md) un ou plusieurs CC par Alliance

---

Date de mise a jour : 2026-10-25
