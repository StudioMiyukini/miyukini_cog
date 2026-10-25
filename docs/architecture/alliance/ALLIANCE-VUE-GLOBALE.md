# Alliances — vue globale

> **Conception v2 — hors gel, non normatif.** Voir l'[index](_index.md) pour les conventions (décidé / proposé / question).

<!-- @id arch.alliance.vue-globale -->
<!-- @role design-overview -->
<!-- @layer 5-8+zones -->
<!-- @human Trois échelles (Alliance, COG-CC, COG poste), place dans le modèle v1, confiance, réseau, propagation des politiques -->
<!-- @do describe_alliance_scales -->

## Contexte

- La v1 répond à « un propriétaire, un COG » ; une organisation a besoin d'**identités qui circulent** (changement de poste, télétravail) et de **politiques communes**
- L'Alliance réutilise le socle v1 (Sceau v1.1, registres Merkle, Portail, Douane, Vigies, Cores) au lieu de reconstruire une pile d'entreprise à côté

## Portée / Scope

- Définir les échelles, leurs responsabilités et leurs frontières ; signaler chaque tension avec la doc gelée
- Les détails sont dans [COG-CC](COG-CC.md), [HECTOR](HECTOR.md) et [Alliance-Admin](ALLIANCE-ADMIN.md)

## 1. Trois échelles

<!-- @id arch.alliance.echelles -->
<!-- @role scale-model -->
<!-- @do define_alliance_scales -->

| Échelle | Rôle | Porte |
|---|---|---|
| **Alliance** | Fédération **privée** de COG d'une organisation, sur un réseau sécurisé ; beaucoup d'utilisateurs, postes interchangeables, travail à distance | Les règles communes, l'annuaire, les comptes d'alliance |
| **COG-CC** | **Quorum administratif** de l'Alliance : inspecte, valide ou rejette les actions ; fait tourner les services `Alliance-*` | Alliance-Admin (serveur), base répliquée, HECTOR |
| **COG poste** | COG d'un ou de plusieurs utilisateurs de l'Alliance | COG-Admin (client), sessions des comptes d'alliance |

- **Décidé (propriétaire)**
  - Un COG-CC regroupe **au moins 6 COG**, chacun avec sa base et ses services de supervision et d'administration
  - Requêtes en **répartition de charge** ; **vérification d'intégrité collective quasi permanente**
  - À moitié piloté par l'IA **HECTOR** (MAIA → HECTOR sur les COG du CC) : alertes et demandes courantes traitées automatiquement
  - Un COG-CC peut contenir **plus de 6** COG
  - Alliance-Admin : solution « Active Directory maison » en Rust, côté Alliance ; COG-Admin côté poste ; compatible avec les postes Windows qui voient l'Alliance comme un **domaine AD**

## 2. Place dans le modèle v1

<!-- @id arch.alliance.place-v1 -->
<!-- @role v1-mapping -->
<!-- @do map_alliance_to_v1 -->

- **Fédération publique ≠ Alliance**
  - Fédération : COG souverains égaux, admission par confirmations de pairs, aucune autorité centrale
  - Alliance : une **organisation** et une **autorité** (le quorum du COG-CC), admission par **enrôlement**
- **Ce qui est repris tel quel (proposé)**
  - Enveloppe **Sceau v1.1** et multi-signature à seuil (≤ 8 signatures par sceau, A-002)
  - **Registre Merkle** et contresignature par témoins (A-006, A-007) → journal du COG-CC
  - Preuve d'**équivoque** `MFPV` → preuve de faute d'un membre du COG-CC
  - **Arrêtés** (certificat `0x06`) → modèle des politiques d'alliance (§5)
  - **Portail**, Douane, Vigies, CDR → frontières de chaque COG, CC compris
  - **StrongFather** décide, **TAMR** encadre l'humain, **WorrySentinel** gouverne la sécurité (D1, D2)
- **Tensions avec la doc gelée** (à résoudre avant tout amendement)
  - **LOI-3 « état local souverain »** et LOI-6 « fédération explicite » : un poste qui applique des politiques venues du CC n'est plus seul maître → **tranché par AL-D06** : la racine d'un poste est **toujours détenue par l'organisation** ; la souveraineté locale est celle de l'organisation, l'utilisateur n'est pas propriétaire du poste
  - **D7** : MiyukiniAdmin en zone COG uniquement, jamais exposé ; COG-Admin doit, lui, parler au CC → répartition des pouvoirs entre MiyukiniAdmin local et COG-Admin ([AL-Q06](ALLIANCE-DECISIONS.md))
  - **D8** : appareils = pairs à passeport ; un compte d'alliance se connecte sur des postes qui ne sont pas les siens → identité **centrée utilisateur**, nouvelle dans le modèle
  - **Mots de passe** : le socle v1 n'en a pas (clés, passeports, visas) ; ils n'entrent que pour la compatibilité Windows ([Alliance-Admin §4](ALLIANCE-ADMIN.md))

## 3. Confiance

<!-- @id arch.alliance.confiance -->
<!-- @role trust-mapping -->
<!-- @do map_alliance_trust -->

- **Proposé** — correspondance avec les niveaux de BorderGuard (vue globale v1 §7)

| Acteur vu depuis un COG poste | Niveau | Remarque |
|---|---|---|
| Sa propre zone COG | TRUSTED | inchangé |
| COG-CC de son Alliance | VERIFIED, périmètre `alliance:<id>` | autorité limitée aux politiques signées au quorum |
| Autre COG poste de l'Alliance | VERIFIED restreint, périmètre `alliance:<id>` | pas d'autorité, échanges selon politique |
| COG-Gateway de l'Alliance | VERIFIED restreint, périmètre `alliance:<id>` | pare-feu de l'Alliance, seul contact avec l'extérieur (AL-D09, AL-D10), aucun droit sur l'annuaire |
| Autre Alliance liée par accord | VERIFIED restreint, périmètre `inter-alliance:<id>` | par le tunnel des Gateways seulement ; inscrite dans notre base, mise à jour automatique, aucune autorité (AL-D11, AL-D17) |
| Membre de la Fédération publique | aucun lien direct | atteint seulement par une COG-Gateway (AL-D09) |
| International | UNKNOWN | inchangé |
| COG exclu de l'Alliance | HOSTILE | révocation inscrite au journal du CC |

- La reconnaissance `0x07` porte déjà un **périmètre** (`federation` en v1) : proposé d'y ajouter `alliance:<id>` plutôt qu'un nouveau type
- **Le CC n'est jamais TRUSTED** pour un poste : il propose des actes signés ; le StrongFather du poste les vérifie et décide

## 4. Réseau

<!-- @id arch.alliance.reseau -->
<!-- @role network -->
<!-- @do place_alliance_network -->

- **Proposé**
  - Réseau d'alliance = surcouche **WireGuard** dédiée, distincte de la Fédération (`10.78.0.0/16`) ; plage à attribuer
  - Nouvelle origine extérieure **« Alliance »** à la Douane de chaque COG, à côté de Fédération, Origin et International
  - Postes nomades : même surcouche par Internet ; aucun port ouvert côté International sur le poste (comme les appareils v1)
  - Le COG-CC reste derrière sa propre Douane : les postes l'atteignent par des **voies déclarées** de son Portail, jamais en direct dans sa zone COG
  - Compatibilité Windows : les protocoles hérités (Kerberos, LDAP, DNS, SMB) arrivent sur une **frontière dédiée** ([Alliance-Admin §4](ALLIANCE-ADMIN.md))
- **Décidé (propriétaire, AL-D09 à AL-D11)** : les COG d'Alliance ne sont **pas** membres de la Fédération publique par défaut ; la **COG-Gateway**, **pare-feu de l'Alliance**, permet d'en sortir sans compromission : navigation sur Internet (navigateur classique ou, de préférence, maison), lien avec la Fédération, **connexion privée entre Alliances** (VPN, WireGuard entre Gateways tant qu'il n'y a pas de partenaire non-COG, AL-D16), **télétravail** (AL-D13) — détail : [COG-Gateway](COG-GATEWAY.md)
- **Proposé** : un poste n'a **aucune route** vers Internet, la Fédération ou une autre Alliance autrement que par une COG-Gateway

## 5. Politiques d'alliance

<!-- @id arch.alliance.politiques -->
<!-- @role policy-propagation -->
<!-- @do propagate_alliance_policies -->

- **Proposé**
  - Une politique = **arrêté d'alliance** : certificat `0x06`, **nature à attribuer**, scellé au **quorum** du COG-CC (multi-signature à seuil)
  - Publiée au **journal** du COG-CC (Merkle, têtes contresignées par les membres du CC)
  - Le poste **tire** (la zone COG tire, D4) ; jamais de poussée dans la zone COG
  - À la réception, **COG-Admin** vérifie quorum, fraîcheur et inclusion au journal ; **StrongFather** du poste décide
  - **Constitution du poste** : une politique d'alliance ne peut pas violer les invariants locaux (INV-Z1–Z11, aucune destruction à distance INV-Z11) ; violation ⇒ refus et signal WorrySentinel
  - Deux sens :
    - **restrictive** (durcit) : appliquée dès vérification
    - **élargissante** (ouvre un droit, installe un logiciel) — **décidé (AL-D21)** : permise dans l'**enveloppe acceptée à l'enrôlement** ; au-delà, **classe 3** (quorum et TAMR)
- Ce qui se propage : comptes et groupes visibles du poste, droits locaux, logiciels du Market autorisés, réglages de sécurité, règles de Douane, révocations

## 6. Cycle de vie d'un poste

<!-- @id arch.alliance.cycle-poste -->
<!-- @role lifecycle -->
<!-- @do enroll_cog_poste -->

- **Proposé**
  1. **Enrôlement** : cérémonie locale (TAMR) ; la racine du poste est celle de l'**organisation** (AL-D06) ; le poste reçoit l'identité du CC et l'épingle ; le CC inscrit le poste au journal (quorum)
  2. **Service** : sessions des comptes d'alliance, politiques tirées, santé remontée au CC
  3. **Réaffectation** : changement d'utilisateur sans réinstallation ; les données du précédent **restent sur le poste** (AL-D23, §7)
  4. **Exclusion** : révocation au journal, le poste devient HOSTILE pour l'Alliance ; **jamais d'effacement à distance** (INV-Z11)
  5. **Sortie** : l'organisation, propriétaire de la racine, retire l'épinglage (TAMR)

## 7. Données des utilisateurs

<!-- @id arch.alliance.donnees-utilisateurs -->
<!-- @role user-data-locality -->
<!-- @do keep_user_data_local -->

- **Décidé (propriétaire, AL-D23)** — par conception du COG
  - Les fichiers et la mémoire MAIA d'un utilisateur **restent sur le poste** où ils ont été créés ; ils ne suivent pas le compte
  - Ils peuvent être **exportés par un humain** :
    - à **distance**, par un **administrateur de l'Alliance** ; ou
    - par une **procédure physique** (clé USB)
- **Conséquences**
  - Un utilisateur qui change de poste retrouve son **compte** et ses droits, pas ses fichiers ni la mémoire de MAIA, sauf export
  - Aucune copie centrale des données personnelles : une compromission du CC ne les expose pas (cohérent avec LOI-3)
  - **Portée limitée par AL-D37** (§8) : sans permission explicite de l'Alliance, la mémoire MAIA d'un poste ne contient pas de données personnelles
  - **Décidé (propriétaire, AL-D24)** : la mémoire MAIA **ne peut pas être stockée dans MiyuCloud** (trop de risque)
  - **Décidé (propriétaire, AL-D25)** : elle **peut être sauvegardée dans le COG-CC** si l'Alliance le **permet explicitement**, pour être **réimportée** sur un autre poste ou simplement **archivée**
  - **Décidé (propriétaire, AL-D26)** : récupération par l'Alliance avec l'**autorisation préalable de l'utilisateur** (politique globale) ou **après 2 ans d'archivage**
  - **Décidé (propriétaire, AL-D27)** : sans consentement, les domaines `sante`, `finances`, `tiers` sont **exclus** dans l'attente de la validation juridique ; l'utilisateur est **informé de la règle des 2 ans dès l'archivage**
  - **Proposé** ([AL-Q32](ALLIANCE-DECISIONS.md), [AL-Q33](ALLIANCE-DECISIONS.md)) : permission = politique de classe 3 et accord de l'utilisateur ; paquet chiffré pour le compte et pour une clé de recouvrement **à seuil** du CC ; récupération en classe 3 ; magasin séparé de la base du CC, hors de portée de HECTOR ; cadre légal du cas « 2 ans sans consentement » à valider
  - **Sauvegarde — décidé (AL-D41)** : fichiers d'un espace Alliance-Files couverts par les copies chiffrées des COG de stockage ; autres fichiers du poste sauvegardés **chiffrés pour le compte** sur ces mêmes COG, illisibles par l'Alliance, restauration par la procédure d'export (classe 2) ; mémoire MAIA au CC si permis (AL-D25), sinon USB, jamais MiyuCloud
- **Proposé — export**
  - Paquet d'export **chiffré pour le compte** de l'utilisateur (clé de son passeport d'alliance) et **scellé par le poste d'origine** : l'administrateur ou la clé USB **transportent** sans pouvoir lire
  - Mémoire MAIA au format existant `maia-memoire/v1` (A-009) ; fichiers dans une archive à format interne, sans exécution
  - **Export à distance** : demande de l'utilisateur ou de l'administrateur ⇒ **classe 2** (quorum) avec accord de l'utilisateur ; sans son accord (départ, absence) ⇒ **classe 3** (quorum et TAMR)
  - **Procédure USB** : cérémonie locale devant le poste (TAMR), support dédié ; à l'import, poste cible : sceau d'origine vérifié, déchiffrement par le compte, mémoire MAIA revérifiée comme toute importation (règles d'A-009 et A-010 : rien n'entre en N0 ou N1 sans validation), fichiers passés au **CDR**, non pris en charge ⇒ quarantaine
  - Chaque export et import inscrit au journal du CC ; HECTOR ne fait que **préparer la demande**, jamais l'export lui-même
- **Fichiers d'équipe** — décidé (propriétaire, AL-D28) : service **[Alliance-Files](ALLIANCE-FILES.md)**, synchronisation entre COG, droits par Alliance-Admin ; proposé : seul ce qui est placé **explicitement** dans un espace synchronisé quitte le poste (AL-D23 intact) ; la mémoire MAIA n'est jamais synchronisée
- **Départ d'un utilisateur** — décidé (AL-D40)
  1. Compte révoqué au journal : sessions, passeports et appareils COG-Auth coupés
  2. Export de sa mémoire MAIA **remis à l'utilisateur**, seulement si la politique avait ouvert des domaines personnels (portabilité RGPD) ; sinon rien à remettre (AL-D38)
  3. Effacement **local** de sa base MAIA par cérémonie TAMR devant le poste, jamais à distance (INV-Z11)
  4. Fichiers de travail et Fiches d'Alliance-MAIA restent à l'organisation ; ses espaces Alliance-Files passent à son responsable
  5. **Restitution du matériel** (AL-D47) ; un poste non restitué est exclu (§6, étape 4) ; chiffré intégralement, il reste illisible
- Poste partagé ([AL-Q07](ALLIANCE-DECISIONS.md)) : base locale par compte, chiffrée sous la clé du compte ; rien n'est lisible par l'utilisateur suivant

## 8. MAIA dans une Alliance : pas de données personnelles

<!-- @id arch.alliance.maia-sans-donnees-personnelles -->
<!-- @role privacy-boundary -->
<!-- @do refuse_personal_data_in_maia -->

- **Décidé (propriétaire, AL-D37)**
  - Sur un COG membre d'une Alliance, **MAIA ne détient pas de données personnelles** : un agent IA d'entreprise n'a pas vocation à les détenir
  - Exception : les **données métier**. Elles relèvent d'un **service distinct, hors du périmètre de MAIA** : **Alliance-MAIA** (AL-D39), service `Alliance-*` qui ne concerne que MAIA au niveau de l'Alliance, distinct de la mémoire MAIA du poste ; base de connaissances en Fiches signées, droits par Alliance-Admin ; MAIA la lit et propose des Fiches qu'un humain signe ; aucune donnée personnelle, les données sur des personnes restent dans les applications métier, hors IA
  - Une information personnelle sensible que l'utilisateur ajoute sur son COG **n'est pas écrite**, sauf si la politique de l'Alliance le **permet explicitement**
  - Un COG seul, hors Alliance, garde les règles v1 de la mémoire MAIA (A-009, A-010) sans changement
- **Lecture confirmée (AL-D38)** : « écrite » veut dire persistée par MAIA, c'est-à-dire sa base `maia-<utilisateur>`, ses notes, ses traces et ses journaux. MAIA peut s'en servir pour **l'échange en cours**, puis l'oublie. Les fichiers que l'utilisateur crée lui-même ne sont pas concernés
- **Conséquences sur AL-D23 à AL-D27**
  - Par défaut, la mémoire MAIA d'un poste d'alliance ne garde que des réglages d'usage (AL-D38) : langue, format des réponses, accessibilité, vocabulaire du poste : l'export, la sauvegarde au CC et la récupération après 2 ans portent sur peu de chose
  - AL-D25 à AL-D27 ne gardent toute leur portée que dans une Alliance dont la politique permet explicitement des données personnelles
  - Le risque juridique d'AL-D26 (b) ([AL-Q33](ALLIANCE-DECISIONS.md)) devient résiduel : il n'existe que si cette permission est donnée
- **Décidé (AL-D38) — mise en œuvre**, avec précisions proposées
  - **Mode Alliance** de `maia-memoire`, fixé à l'enrôlement du poste. Ce n'est pas un réglage de l'utilisateur
  - Domaines `sante`, `finances`, `tiers` **fermés au niveau du stockage** : KindMother refuse l'écriture quel que soit l'auteur (utilisateur, MAIA, import). En cas de doute, refus (fail-closed)
  - Toute note candidate passe un **filtre** avant écriture. D'abord des règles déterministes (IBAN, numéro de sécurité sociale, date de naissance, adresse, téléphone personnel…), puis un classement par le modèle. **En cas de doute, rien n'est écrit.** Le filtre peut seulement refuser, jamais autoriser
  - L'utilisateur est **informé** : « non mémorisé : la politique de l'Alliance ne le permet pas ». Rien n'est gardé en attente
  - Un import `maia-memoire/v1` vers un poste d'alliance passe les mêmes filtres. Les notes refusées sont écartées et un compte rendu est remis à l'utilisateur
  - La **permission explicite** prend la forme d'une politique de **classe 3** (quorum et TAMR) qui nomme les domaines ouverts. Même ouverts, les domaines sensibles gardent la validation TAMR note par note de la v1
  - Un COG qui rejoint une Alliance avec une mémoire existante ne la convertit pas : l'utilisateur l'exporte pour lui-même avant l'enrôlement

## 9. Points à trancher

- Voir le [registre](ALLIANCE-DECISIONS.md) : AL-Q06 (MiyukiniAdmin et COG-Admin), AL-Q07 (données sur un poste partagé), AL-Q32 (sauvegarde de la mémoire MAIA dans le CC)

---

Date de mise a jour : 2026-10-25
