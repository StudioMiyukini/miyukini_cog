# Alliances — registre des décisions et questions ouvertes

> **Conception v2 — hors gel, non normatif.** Voir l'[index](_index.md) pour les conventions.

<!-- @id arch.alliance.decisions -->
<!-- @role decision-log -->
<!-- @layer 5-8+zones -->
<!-- @human Décisions du propriétaire sur les Alliances, COG-CC, HECTOR et Alliance-Admin ; questions ouvertes avec recommandation -->
<!-- @do log_alliance_decisions -->

## Contexte

- Registre de travail de la conception des Alliances ; chaque question porte une **recommandation**, à valider ou à remplacer par le propriétaire
- Une question tranchée devient une décision `AL-Dnn` ; elle ne devient normative qu'à l'amendement de la doc gelée (AL-Q19)

## Portée / Scope

- Toutes les questions citées dans [vue globale](ALLIANCE-VUE-GLOBALE.md), [COG-CC](COG-CC.md), [COG-Gateway](COG-GATEWAY.md), [HECTOR](HECTOR.md), [Alliance-Admin](ALLIANCE-ADMIN.md)

## 1. Décisions

<!-- @id arch.alliance.decisions-prises -->
<!-- @role decisions -->
<!-- @do list_alliance_decisions -->

| Id | Date | Décision | Source |
|---|---|---|---|
| AL-D01 | 2026-10-25 | Trois échelles : Alliance (fédération privée de COG), COG-CC (quorum administratif, services `Alliance-*`), COG poste (client COG-Admin) | propriétaire |
| AL-D02 | 2026-10-25 | Un COG-CC compte **au moins 6** COG, chacun avec sa base et ses services ; peut en compter plus ; répartition de charge ; intégrité collective quasi permanente | propriétaire |
| AL-D03 | 2026-10-25 | HECTOR remplace MAIA sur les COG du CC ; rôles superviseur, relève, 4 traitants au moins ; seulement les prompts et la base du CC ; demandes prédéfinies ou proches, jugées légitimes ou malveillantes après BorderGuard | propriétaire |
| AL-D04 | 2026-10-25 | Mot de passe perdu : identification ; si elle réussit, mot de passe temporaire et changement obligatoire, automatiquement | propriétaire |
| AL-D05 | 2026-10-25 | Alliance-Admin : « AD maison » en Rust, sur le socle de sécurité COG, compatible avec les postes Windows qui voient l'Alliance comme un domaine AD | propriétaire |
| AL-D06 | 2026-10-25 | La racine d'un COG poste est **toujours détenue par l'organisation** (tranche AL-Q01) | propriétaire |
| AL-D07 | 2026-10-25 | Compatibilité Windows visée : **W3 complet, stratégies de groupe comprises** ; elle vient **après** une réussite effective dans un environnement **entièrement COG** (tranche AL-Q02) | propriétaire |
| AL-D08 | 2026-10-25 | COG-CC : **6 membres minimum, 7 recommandés** en production (tranche AL-Q09) | propriétaire |
| AL-D09 | 2026-10-25 | Un COG d'Alliance n'est **pas** membre de la Fédération publique par défaut ; des COG dédiés (**COG-Gateway**, AL-D10) assurent ce lien, notamment la **navigation sur Internet** (tranche AL-Q03) | propriétaire |
| AL-D10 | 2026-10-25 | **COG-Gateway** = **pare-feu de l'Alliance** : permet de sortir de l'Alliance sans compromission ; navigation sur Internet par un navigateur classique ou, de préférence, par un **navigateur maison** (à venir) | propriétaire |
| AL-D11 | 2026-10-25 | La COG-Gateway permet aussi la **connexion privée entre plusieurs Alliances** (IPsec / VPN) | propriétaire |
| AL-D12 | 2026-10-25 | COG-Gateway **polyvalente**, **1 au minimum** par Alliance (tranche AL-Q20) | propriétaire |
| AL-D13 | 2026-10-25 | Le **télétravail** entre par la COG-Gateway (tranche AL-Q22) | propriétaire |
| AL-D14 | 2026-10-25 | **Pas d'inspection TLS** par défaut pour le navigateur classique (recommandation retenue, tranche AL-Q23) | propriétaire |
| AL-D15 | 2026-10-25 | Navigation **journalisée** : domaines et volumes, pas les contenus ; conservation fixée par politique ; consultation en classe 3 (recommandation retenue, tranche AL-Q24) | propriétaire |
| AL-D16 | 2026-10-25 | Liaisons inter-Alliances entre COG-Gateway seulement ; **pas de partenaire non-COG pour le moment** ⇒ WireGuard, IPsec reporté (tranche AL-Q25) | propriétaire |
| AL-D17 | 2026-10-25 | Utilisateurs d'une autre Alliance : **autorisés par le COG-CC** selon ses informations ; à la **cérémonie de liaison**, les Alliances se reconnaissent et **s'inscrivent mutuellement dans leurs bases** ; **mise à jour automatique** entre COG-CC (tranche AL-Q26) | propriétaire |
| AL-D18 | 2026-10-25 | Les données venues d'un autre COG-CC sont **identifiables** et **révocables en interne** ; le COG-CC concerné en est **informé** | propriétaire |
| AL-D19 | 2026-10-25 | Une mise à jour automatique venue d'une Alliance liée ne peut que **restreindre** ou **ajouter sans ouvrir de droit** ; tout nouveau droit passe par une politique de notre CC (recommandation retenue, tranche AL-Q28) | propriétaire |
| AL-D20 | 2026-10-25 | Deux Alliances liées partagent **seulement les comptes et groupes désignés par l'accord** ; jamais les secrets ni les comptes privilégiés (recommandation retenue, tranche AL-Q27) | propriétaire |
| AL-D21 | 2026-10-25 | Politiques **élargissantes** permises seulement dans l'**enveloppe acceptée à l'enrôlement** du poste ; au-delà, **quorum et validation humaine TAMR** (classe 3) (recommandation retenue, tranche AL-Q05) | propriétaire |
| AL-D22 | 2026-10-25 | Journal du COG-CC ordonné par un protocole **tolérant aux membres compromis** (famille HotStuff), pas Raft (recommandation retenue, tranche AL-Q10) | propriétaire |
| AL-D23 | 2026-10-25 | Par conception du COG, fichiers et mémoire MAIA d'un utilisateur **restent sur le poste** ; **export par un humain** via un administrateur de l'Alliance (à distance) ou par procédure physique (clé USB) (tranche AL-Q04) | propriétaire |
| AL-D24 | 2026-10-25 | La mémoire MAIA **ne peut pas être stockée dans MiyuCloud** : trop de risque | propriétaire |
| AL-D25 | 2026-10-25 | La mémoire MAIA **peut être sauvegardée dans le COG-CC** si l'Alliance le **permet explicitement** ; elle peut alors être **réimportée** sur un autre poste ou simplement **archivée** | propriétaire |
| AL-D26 | 2026-10-25 | Une archive de mémoire MAIA est récupérable par l'Alliance : (a) avec l'**autorisation préalable de l'utilisateur** (politique globale de l'Alliance) ; ou (b) **après 2 ans d'archivage** | propriétaire |
| AL-D27 | 2026-10-25 | Récupération après 2 ans **sans consentement** (AL-D26 b) : domaines `sante`, `finances`, `tiers` **exclus** dans l'attente de la validation juridique ; l'utilisateur est **informé de la règle des 2 ans dès l'archivage** | propriétaire |
| AL-D28 | 2026-10-25 | Nouveau service **Alliance-Files** : synchronisation des fichiers entre COG de l'Alliance, droits gérés en fonction d'Alliance-Admin (tranche AL-Q31) | propriétaire |
| AL-D29 | 2026-10-25 | Alliance-Files : index et droits au CC sans contenu ; contenu de COG à COG par voies déclarées ; COG de stockage facultatifs, copies chiffrées par espace (recommandation retenue, tranche AL-Q34) | propriétaire |
| AL-D30 | 2026-10-25 | Contrôle des fichiers synchronisés **selon la provenance** : externe ⇒ CDR et quarantaine, marque à vie ; interne signé ⇒ accepté et analysé à réception ; exécutables et macros refusés sauf espace dédié (classe 3) (recommandation retenue, tranche AL-Q35) | propriétaire |
| AL-D31 | 2026-10-25 | Retrait de droits sur un espace : synchronisation coupée, nouvelle clé d'espace ; copies locales jamais effacées à distance, illisibles hors du compte (recommandation retenue, tranche AL-Q36) | propriétaire |
| AL-D32 | 2026-10-25 | Nouveau service **COG-Auth**, authentificateur à la manière de Google Authenticator ([fiche](COG-AUTH.md)) | propriétaire |
| AL-D33 | 2026-10-25 | COG-Auth : mode natif **« approbation signée »** (nombre à recopier, signature par la clé de l'appareil) **par défaut** pour tout ce qui est COG ; **TOTP** RFC 6238 seulement en **compatibilité**, avec anti-rejeu (recommandation retenue, tranche AL-Q37) | propriétaire |
| AL-D34 | 2026-10-25 | COG-Auth sur le **téléphone** de l'utilisateur (appareil pair à passeport), à partir de `apps/central-mobile` ; clé non exportable dans l'enclave sécurisée quand elle existe ; **clé FIDO2** comme second appareil (recommandation retenue, tranche AL-Q38) | propriétaire |
| AL-D35 | 2026-10-25 | COG-Auth : **aucune synchronisation dans un nuage** ; plusieurs appareils enrôlés ; codes de secours à usage unique (hachés) ; perte = révocation puis ré-enrôlement (recommandation retenue, tranche AL-Q39) | propriétaire |
| AL-D36 | 2026-10-25 | Identification de P-001 (mot de passe perdu), dans l'ordre : **approbation signée COG-Auth**, sinon **confirmation du responsable**, sinon **vérification en présence** ; jamais de questions secrètes (recommandation retenue, tranche AL-Q14) | propriétaire |
| AL-D37 | 2026-10-25 | Sur un COG membre d'une Alliance, **MAIA ne détient pas de données personnelles**. Les **données métier** relèvent d'un **service distinct**, hors du périmètre de MAIA. Une information personnelle sensible ajoutée par l'utilisateur **n'est pas écrite**, sauf si la politique de l'Alliance le **permet explicitement** ([vue globale §8](ALLIANCE-VUE-GLOBALE.md)) | propriétaire |
| AL-D38 | 2026-10-25 | MAIA en mode Alliance ne garde que des **réglages d'usage** ; domaines `sante`, `finances`, `tiers` fermés au stockage ; filtre avant écriture (règles fixes puis modèle), en cas de doute rien n'est écrit, le filtre ne fait que refuser ; permission explicite = politique de **classe 3** nommant les domaines ouverts ; « non écrit » = non conservé par MAIA (mémoire, notes, traces, journaux), utilisable pour l'échange en cours (recommandation retenue, tranche AL-Q40) | propriétaire |
| AL-D39 | 2026-10-25 | Données métier : service distinct **Alliance-MAIA** (nom retenu par le propriétaire : il ne concerne que MAIA au niveau de l'Alliance), base de connaissances en Fiches signées (`0x09`), droits par Alliance-Admin ; MAIA lit et propose, un humain signe ; aucune donnée personnelle, les données sur des personnes restent dans les applications métier, hors IA (recommandation retenue, tranche AL-Q41) | propriétaire |
| AL-D40 | 2026-10-25 | Départ d'un utilisateur : (1) compte révoqué au journal (sessions, passeports, appareils COG-Auth) ; (2) export de sa mémoire MAIA **remis à l'utilisateur** seulement si la politique avait ouvert des domaines personnels (portabilité RGPD) ; (3) effacement **local** de sa base MAIA par cérémonie TAMR sur le poste, jamais à distance (INV-Z11) ; (4) fichiers de travail et Fiches d'Alliance-MAIA restent à l'organisation, espaces Alliance-Files transmis à son responsable (recommandation retenue, tranche AL-Q30) | propriétaire |
| AL-D41 | 2026-10-25 | Sauvegarde des postes : fichiers d'un espace Alliance-Files couverts par les copies chiffrées des COG de stockage (AL-D29) ; autres fichiers du poste sauvegardés **chiffrés pour le compte** sur ces mêmes COG de stockage, illisibles par l'Alliance, restauration par la procédure d'export (classe 2) ; mémoire MAIA : CC si permis (AL-D25), sinon USB, jamais MiyuCloud ; **JayCloud écarté** (recommandation retenue, tranche AL-Q29) | propriétaire |
| AL-D42 | 2026-10-25 | Mises à jour **par vagues** : jamais plus de `f` membres du CC à la fois, observation 24 h avec attestations saines, retour arrière possible ; postes par cohortes ; mise à jour du CC en classe 2, classe 3 si elle touche consensus ou journal ([COG-CC §5.2](COG-CC.md)) (recommandation retenue, tranche AL-Q42) | propriétaire |
| AL-D43 | 2026-10-25 | **Collège d'approbateurs** de la classe 3 : `k` personnes distinctes (2 sur 3 minimum, 3 sur 5 recommandé), l'auteur n'approuve pas, une personne = au plus une part de clé à seuil, exploitation du CC par au moins 2 personnes, approbations COG-Auth sur appareils distincts ([COG-CC §5.1](COG-CC.md)) (recommandation retenue, tranche AL-Q43) | propriétaire |
| AL-D44 | 2026-10-25 | **Racine d'organisation** hors ligne à seuil (3 porteurs sur 5), jamais sur un membre du CC ; **cérémonie de genèse** qui signe membres et paramètres (tête 0 du journal) ; enrôlement des postes par délégation au CC ([COG-CC §4.1](COG-CC.md)) (recommandation retenue, tranche AL-Q44) | propriétaire |
| AL-D45 | 2026-10-25 | **Refondation** par la racine depuis la dernière tête attestée par `f + 1` membres ; **bris de glace** en lecture seule limité à restreindre, journalisé et notifié au collège ; exercice de refondation dans le critère du pilote ([COG-CC §4.1](COG-CC.md)) (recommandation retenue, tranche AL-Q45) | propriétaire |
| AL-D46 | 2026-10-25 | **Visa hors journal ordonné** : tout membre sain du CC émet un visa court (≤ 8 h) après vérification du passeport et des révocations, trace versée ensuite au journal ; abus détecté par l'intégrité collective, membre suspendu ; en lecture seule, connexions possibles, aucune écriture d'annuaire hors bris de glace ([Alliance-Admin §3](ALLIANCE-ADMIN.md)) (recommandation retenue, tranche AL-Q46) | propriétaire |
| AL-D47 | 2026-10-25 | Poste perdu ou départ hors ligne : **chiffrement intégral** scellé par TPM et lié au compte (sans TPM, phrase de démarrage) ; vérificateur local Argon2id, **effacé dès qu'une révocation est connue** ; hors ligne 24 h au plus pour les comptes sensibles, **0** pour les privilégiés ; **restitution du matériel** ajoutée au départ (AL-D40), poste non restitué exclu (recommandation retenue, tranche AL-Q47) | propriétaire |
| AL-D48 | 2026-10-25 | **Asymétrie** : restreindre (révoquer, suspendre, isoler) est au plus classe 2, déclenchable automatiquement sur alerte WorrySentinel T3 ou plus et confirmé ensuite ; élargir garde sa classe ; **HECTOR ne restreint jamais seul** ([COG-CC §5](COG-CC.md)) (recommandation retenue, tranche AL-Q49) | propriétaire |
| AL-D49 | 2026-10-26 | **Horloge** : le CC sert l'heure aux postes par NTS (RFC 8915), membres croisés, alerte sur dérive ; décisions datées par la médiane des horodatages récents du journal, jamais par un seul membre (recommandation retenue, tranche AL-Q50) | propriétaire |
| AL-D50 | 2026-10-26 | **Seuil d'Alliance** : à partir de ≈ 25 postes ou 2 sites ; en dessous, COG seul multi-utilisateurs ; HECTOR facultatif ; CC jamais sous 6 membres (recommandation retenue, tranche AL-Q51) | propriétaire |
| AL-D51 | 2026-10-26 | **Surveillance des salariés** : registre des collectes généré depuis les politiques ; AIPD de l'Alliance avant le pilote ; information des salariés et du CSE, validée par un juriste (recommandation retenue, tranche AL-Q48) | propriétaire |
| AL-D52 | 2026-10-26 | **Poste partagé** : base locale par compte, chiffrée sous sa clé ; à la réaffectation rien n'est effacé à distance, rien n'est lisible sans le compte (recommandation retenue, tranche AL-Q07) | propriétaire |
| AL-D53 | 2026-10-26 | **Sauvegarde MAIA au CC** (si l'Alliance l'autorise, AL-D37) : politique de classe 3 et accord de l'utilisateur ; clé de recouvrement à seuil des membres du CC, qui refusent leur part avant 2 ans ; domaines sensibles chiffrés pour le compte seul ; magasin séparé, hors de portée de HECTOR (recommandation retenue, tranche AL-Q32) | propriétaire |

## 2. Questions ouvertes

<!-- @id arch.alliance.questions -->
<!-- @role open-questions -->
<!-- @do track_alliance_questions -->

### Modèle d'Alliance

- ~~**AL-Q01 · Qui détient la racine d'un COG poste ?**~~ → **AL-D06** : toujours l'organisation
  - Conséquence : un appareil personnel n'est jamais un COG poste ; au mieux un **appareil pair** à passeport d'alliance (D8), sous politiques étroites
  - La LOI-3 reste vraie : l'état local est souverain **pour le propriétaire de la racine**, ici l'organisation
- ~~**AL-Q03 · Un COG d'Alliance peut-il aussi être membre de la Fédération publique ?**~~ → **AL-D09** : non par défaut, par des COG-Gateway (AL-D10)
- ~~**AL-Q20 · COG-Gateway : répartition**~~ → **AL-D12** : polyvalente, 1 au minimum
  - Reste proposé : Gateway gouvernée par le CC, **jamais membre** du CC, aucun droit sur l'annuaire ; une sandbox par usage dans la Gateway ; une deuxième Gateway recommandée en production ([COG-Gateway §6](COG-GATEWAY.md))
- ~~**AL-Q22 · Entrée du télétravail**~~ → **AL-D13** : par la Gateway
- ~~**AL-Q23 · Inspection TLS**~~ → **AL-D14** : non par défaut
- ~~**AL-Q24 · Journalisation de la navigation**~~ → **AL-D15**
- ~~**AL-Q25 · Protocole des liaisons inter-Alliances**~~ → **AL-D16** : WireGuard entre COG-Gateway ; IPsec reporté jusqu'au premier partenaire non-COG
- ~~**AL-Q26 · Utilisateurs d'une autre Alliance**~~ → **AL-D17** : inscription mutuelle à la cérémonie, mise à jour automatique, autorisation par notre CC
- ~~**AL-Q27 · Que partagent deux Alliances liées ?**~~ → **AL-D20** : identité des CC et des Gateways, plus les comptes et groupes **désignés** par l'accord ; jamais les secrets (clés Kerberos, vérificateurs), jamais les comptes privilégiés ; objets étrangers en lecture seule (minimisation RGPD)
- ~~**AL-Q28 · Alliance liée compromise au niveau de son quorum**~~ → **AL-D18** et **AL-D19**
  - Risque : elle peut déclarer de faux utilisateurs ou retirer des révocations
  - AL-D18 : tout objet venu d'elle est identifiable et révocable chez nous, et notre révocation tient face à ses mises à jour
  - AL-D19 : une mise à jour automatique ne fait que **restreindre** (révocations, retraits) ou **ajouter sans droit** ; tout nouveau droit chez nous passe par une politique de notre CC (classe 2) ; les mises à jour sont journalisées des deux côtés et contresignées par les Gateways, pour garder la preuve d'une équivoque
- ~~**AL-Q04 · Où vivent les données d'un utilisateur qui change de poste ?**~~ → **AL-D23** : sur le poste ; export par un humain (administrateur à distance ou clé USB) — ma recommandation (MiyuCloud d'alliance, mémoire qui suit le compte) est **écartée**
- ~~**AL-Q29 · Sauvegarde des postes**~~ → **AL-D41** (issue d'AL-D23) ; la recommandation ci-dessous est remplacée sur un point : JayCloud écarté au profit des COG de stockage d'Alliance-Files
  - Sans copie ailleurs, un poste perdu ou détruit emporte fichiers et mémoire MAIA
  - Options : (a) aucune sauvegarde ; (b) sauvegarde **chiffrée pour le compte**, stockée par l'Alliance (JayCloud) mais illisible par elle ; (c) sauvegarde lisible par l'organisation
  - Contrainte **AL-D24** : mémoire MAIA jamais dans MiyuCloud
  - **Recommandation**
    - **Fichiers** : (b) — chiffrés pour le compte, stockés par JayCloud, illisibles par l'Alliance ; restauration par la procédure d'export (classe 2) ; **ou**, pour les fichiers placés dans un espace Alliance-Files (AL-D28), les copies chiffrées des COG de stockage (AL-D29)
    - **Mémoire MAIA** : tranché par **AL-D24** et **AL-D25** — jamais MiyuCloud ; sauvegarde dans le **COG-CC** si l'Alliance le permet explicitement, sinon **procédure USB** ; JayCloud non retenu pour la mémoire MAIA
- ~~**AL-Q32 · Sauvegarde de la mémoire MAIA dans le COG-CC**~~ → **AL-D53** (issue d'AL-D25)
  - Depuis **AL-D37**, la question ne porte sur des données personnelles que dans une Alliance qui les a permises explicitement
  - (a) Forme de la permission explicite · (b) qui peut déchiffrer · (c) où dans le CC · (d) durée d'archivage
  - **Recommandation**
    - (a) **politique d'alliance de classe 3** (quorum et TAMR : elle ouvre un stockage de données personnelles) **et** accord de l'utilisateur, révocable par lui
    - (b) **tranché par AL-D26** : lisible par l'utilisateur ; récupérable par l'Alliance avec son autorisation préalable ou après 2 ans d'archivage — mise en œuvre proposée :
      - paquet chiffré pour le compte **et** pour une **clé de recouvrement de l'Alliance**, détenue **à seuil** par les membres du CC (quorum q) ; aucun membre seul, ni HECTOR, ni un administrateur ne lit
      - le délai de 2 ans n'est pas cryptographique : il est tenu par les membres du CC, qui refusent leur part avant l'échéance ; date de départ = inscription de l'archive au journal, temps de confiance borné par le Median-Time-Past (A-006)
      - récupération = **classe 3** (quorum et TAMR), inscrite au journal, utilisateur **notifié** s'il est joignable
      - autorisation préalable : consentement recueilli à la mise en archive, lié à la politique globale en vigueur, **retirable** par l'utilisateur pour l'avenir
    - (c) magasin **séparé** de la base du CC : hors des requêtes nommées de HECTOR, hors du journal (seule l'empreinte et la date y sont inscrites), copie sur 2 membres du CC
    - (d) durée fixée par la politique ; à l'échéance, effacement par cérémonie TAMR sur le CC, inscrit au journal
    - Réimport : même contrôle que tout import (A-009, A-010 : rien n'entre en N0 ou N1 sans validation)
- **AL-Q33 · Récupération sans consentement après 2 ans (AL-D26 b) : cadre légal**
  - La mémoire MAIA peut contenir des données de **catégories particulières** (santé, art. 9 RGPD) et des données sur des **tiers** ; un employeur qui les lit sans consentement s'expose au droit du travail et au RGPD
  - Acquis par **AL-D27** : `sante`, `finances`, `tiers` exclus en attendant ; utilisateur informé dès l'archivage
  - **Portée réduite par AL-D37** : sans permission explicite de l'Alliance, la mémoire MAIA ne contient pas de données personnelles ; le risque ne naît qu'avec cette permission
  - **Reste ouvert** : validation juridique d'AL-D26 (b) par la [conformité](../../conformite/REGLEMENTAIRE-UE-FR.md) ; recommandé **avant le gel v2**
  - **Proposé — mise en œuvre d'AL-D27** : l'archive est découpée par domaine ; les domaines sensibles sont chiffrés **pour le compte seulement**, sauf autorisation préalable (AL-D26 a) qui les ajoute à la clé de recouvrement ; ainsi l'exclusion est tenue par le chiffrement, pas seulement par la procédure
- ~~**AL-Q30 · Départ d'un utilisateur**~~ → **AL-D40**
  - Ses données restent sur le poste, dont l'organisation détient la racine (AL-D06)
  - Avec **AL-D37**, la mémoire MAIA ne contient par défaut que des réglages d'usage ; elle n'est vraiment personnelle que si la politique a ouvert des domaines
  - **Recommandation** : export de la mémoire MAIA **remis à l'utilisateur** (classe 3, droit à la portabilité RGPD), puis effacement **local** de sa base par cérémonie TAMR sur le poste (jamais à distance, INV-Z11). Les fichiers de travail relèvent d'une politique de l'organisation
- ~~**AL-Q40 · Filtre des données personnelles dans MAIA**~~ → **AL-D38** (issue d'AL-D37)
  - (a) Que garde MAIA en mode Alliance ? (b) Comment détecter une donnée personnelle ? (c) Forme de la permission explicite
  - Au sens du RGPD, presque tout ce qui est lié à un compte nommé est une donnée personnelle. Il faut donc une règle de travail
  - **Recommandation**
    - (a) seulement des **réglages d'usage** : langue, format des réponses, accessibilité, vocabulaire du poste. Aucun fait sur la vie de l'utilisateur ni sur des tiers
    - (b) domaines `sante`, `finances`, `tiers` fermés au stockage ; règles déterministes puis classement par le modèle ; en cas de doute, rien n'est écrit ; le filtre ne peut que refuser ([vue globale §8](ALLIANCE-VUE-GLOBALE.md))
    - (c) politique de **classe 3** qui nomme les domaines ouverts ; validation TAMR note par note maintenue pour les domaines sensibles
- ~~**AL-Q41 · Service des données métier**~~ → **AL-D39** (issue d'AL-D37)
  - Nom, lieu, format, droits, lien avec MAIA
  - **Recommandation**
    - Service `Alliance-*` distinct, nommé **Alliance-MAIA** par le propriétaire (proposé d'abord sous le nom « Alliance-Savoir ») : base de connaissances métier de l'organisation (procédures, références, dossiers), au format **Fiche** signée (type Sceau `0x09`, A-009) par un auteur autorisé
    - Droits par groupes d'Alliance-Admin, comme Alliance-Files ; index au CC, contenu sur des COG désignés
    - MAIA la **consulte** en lecture, comme MAIA-Lecteur pour les Fiches étrangères, et n'y écrit jamais directement : elle peut proposer une Fiche qu'un humain signe
    - Les données personnelles y sont aussi exclues ; une donnée sur une personne qui relève du métier (client, dossier RH) reste dans l'application métier qui la gère, hors IA
- ~~**AL-Q31 · Documents de travail partagés**~~ → **AL-D28** : service Alliance-Files ([fiche](ALLIANCE-FILES.md))
- ~~**AL-Q34 · Architecture d'Alliance-Files**~~ → **AL-D29**
  - Retenu : index et droits au CC (journal Merkle, sans contenu) ; contenu de COG à COG par voies déclarées ; COG de stockage facultatifs, désignés par politique, qui gardent des copies **chiffrées** par espace (et servent de sauvegarde des fichiers, AL-D41)
- ~~**AL-Q35 · Contrôle des fichiers synchronisés**~~ → **AL-D30**
  - Le CDR v1 refuse PDF et bureautique : appliqué partout, il bloquerait le travail courant
  - Retenu : contrôle **selon la provenance** — fichier **externe** (Gateway, USB, Alliance liée) : CDR et quarantaine, marque conservée à vie ; fichier **interne** signé par un auteur autorisé : accepté, analysé sur le poste qui le reçoit ; exécutables et macros refusés partout sauf espace dédié (classe 3)
- ~~**AL-Q36 · Copies locales après retrait de droits**~~ → **AL-D31**
  - Retenu : plus de synchronisation, nouvelle clé d'espace ; copies locales jamais effacées à distance (INV-Z11), illisibles hors du compte ; cérémonie locale par l'organisation si nécessaire
- ~~**AL-Q05 · Politiques élargissantes**~~ → **AL-D21** : dans l'enveloppe de l'enrôlement ; au-delà, classe 3 (quorum et TAMR)
- ~~**AL-Q07 · Poste partagé : données locales du précédent utilisateur ?**~~ → **AL-D52**
  - **Recommandation** : base locale par compte chiffrée sous sa clé ; à la réaffectation, rien n'est effacé à distance (INV-Z11) mais rien n'est lisible sans le compte
- **AL-Q11 · Un ou plusieurs COG-CC par Alliance ?** (sites, filiales)
  - **Recommandation** : un CC par Alliance au départ ; plusieurs CC = plus tard, avec une délégation d'unités signée au quorum

### COG-CC

- **AL-Q06 · Pouvoirs de MiyukiniAdmin local face à COG-Admin**
  - **Recommandation** : MiyukiniAdmin garde la constitution du poste et la sortie de l'Alliance ; COG-Admin gère comptes, sessions et politiques
- **AL-Q08 · Diversité physique des membres du CC**
  - **Recommandation** : au moins 2 sites (ou 2 alimentations et 2 accès réseau) ; aucun site ne porte plus de `q − 1` membres, pour qu'un site seul ne fasse pas quorum
- ~~**AL-Q09 · 6 ou 7 membres minimum ?**~~ → **AL-D08** : 6 minimum, 7 recommandés (6 tolèrent 1 membre compromis, 7 en tolèrent 2, [COG-CC §2](COG-CC.md))
- ~~**AL-Q10 · Protocole d'ordonnancement du journal**~~ → **AL-D22** : famille HotStuff, pas Raft ; un membre compromis ne peut pas réécrire l'annuaire
  - Reste à l'implémentation : choix de la variante, bibliothèque ou code maison, consigné par amendement

### HECTOR

- **AL-Q12 · Rotation du superviseur** — **Recommandation** : rotation quotidienne, ordre tiré au sort au quorum
- **AL-Q13 · Deux modèles différents sur les traitants ?** — **Recommandation** : oui dès que deux modèles locaux tiennent sur le matériel ; sinon un modèle, deux prompts d'analyse différents
- ~~**AL-Q14 · Facteurs d'identification de P-001**~~ → **AL-D36** : approbation signée COG-Auth, sinon responsable, sinon présence ; jamais de questions secrètes
- **AL-Q15 · Modèle local de HECTOR et matériel minimal d'un COG du CC** — à dimensionner avec [modèles locaux](../MODELES-IA-PETITES-CONFIGS.md)

### Alliance-Admin

- ~~**AL-Q02 · Niveau de compatibilité Windows visé**~~ → **AL-D07** : W3 complet, **après** la réussite d'une Alliance entièrement COG
  - Reste : critère de « réussite effective » (AL-Q21) ; ordre interne W1 → W2 → W3 recommandé pour livrer par paliers testables ; NTLM jamais exposé
- **AL-Q21 · Critère de « réussite effective » d'une Alliance entièrement COG** (porte d'entrée de la compatibilité Windows, AL-D07)
  - **Recommandation** : une Alliance pilote d'au moins 1 COG-CC de 7 membres et 20 postes, tenue 30 jours, avec : bascule du superviseur HECTOR, perte d'un membre du CC, un membre compromis simulé (équivoque détectée et suspendue), P-001 de bout en bout, une politique restrictive, une exclusion de poste et un **exercice de refondation** (AL-D45) — sans intervention hors procédure
- **AL-Q16 · Privilèges** — **Recommandation** : aucun administrateur permanent ; élévation temporaire (défaut proposé 4 h) accordée en classe 3
- **AL-Q17 · Connexion hors ligne sur un poste** — **Recommandation** : 10 derniers comptes, 7 jours au plus pour les comptes ordinaires, désactivable par politique ; déjà fixé par **AL-D47** : 24 h pour les sensibles, 0 pour les privilégiés
- **AL-Q18 · Garde des clés Kerberos et de krbtgt** — **Recommandation** : chiffrées sous une clé à seuil des membres du CC ; rotation de krbtgt tous les 30 jours

### COG-Auth

- ~~**AL-Q37 · Modes de COG-Auth**~~ → **AL-D33**
  - Un code TOTP repose sur un secret partagé et se fait hameçonner ; une approbation signée par la clé de l'appareil, non
  - Retenu : mode natif « approbation signée » (avec nombre à recopier) par défaut pour tout ce qui est COG ; TOTP RFC 6238 seulement en compatibilité (MiyuCloud existant, services tiers, Windows), avec anti-rejeu
- ~~**AL-Q38 · Plateforme**~~ → **AL-D34**
  - Retenu : téléphone de l'utilisateur, appareil pair à passeport (D8) ; partir de `apps/central-mobile` ; clé non exportable dans l'enclave sécurisée quand elle existe ; clé matérielle FIDO2 comme second appareil
- ~~**AL-Q39 · Sauvegarde et perte de l'authentificateur**~~ → **AL-D35**
  - Retenu : aucune synchronisation dans un nuage ; plusieurs appareils enrôlés ; codes de secours à usage unique (hachés) ; perte = révocation puis ré-enrôlement

### Revue des failles (2026-10-25)

- Détail, constats et recommandations complètes : [revue des failles](ALLIANCE-REVUE-FAILLES.md)
- ~~**AL-Q42 · Mises à jour par vagues**~~ → **AL-D42** (AL-F01, critique) — **Recommandation** : jamais plus de `f` membres du CC à la fois, observation 24 h avec attestations saines, retour arrière possible ; postes par cohortes ; mise à jour du CC en classe 2, classe 3 si elle touche consensus ou journal
- ~~**AL-Q43 · Collège humain de la classe 3**~~ → **AL-D43** (AL-F02, critique) — **Recommandation** : collège d'approbateurs dans l'annuaire, `k` personnes distinctes (2 sur 3 minimum, 3 sur 5 recommandé) ; l'auteur n'approuve pas ; une personne = au plus une part de clé à seuil ; exploitation du CC par au moins 2 personnes
- ~~**AL-Q44 · Genèse du CC et racine d'organisation**~~ → **AL-D44** (AL-F03) — **Recommandation** : racine hors ligne à seuil (3 sur 5), jamais sur un membre du CC ; cérémonie de genèse qui signe la tête 0 du journal ; ensuite enrôlement par délégation au CC
- ~~**AL-Q45 · Refondation et bris de glace**~~ → **AL-D45** (AL-F04) — **Recommandation** : refondation par la racine à partir de la dernière tête attestée par `f + 1` membres ; en lecture seule, la racine peut seulement restreindre
- ~~**AL-Q46 · Connexion en lecture seule**~~ → **AL-D46** (AL-F05) — **Recommandation** : visa émis par tout membre sain, court (≤ 8 h), hors journal ordonné, versé ensuite ; abus détecté par l'intégrité collective
- ~~**AL-Q47 · Poste volé, départ hors ligne**~~ → **AL-D47** (AL-F06) — **Recommandation** : chiffrement intégral scellé par TPM, vérificateur Argon2id effacé à la révocation, 24 h hors ligne pour les comptes sensibles, 0 pour les privilégiés ; restitution du matériel dans AL-D40
- ~~**AL-Q48 · Surveillance des salariés**~~ → **AL-D51** (AL-F08) — **Recommandation** : registre des collectes produit depuis les politiques, AIPD de l'Alliance avant le pilote, information des salariés et du CSE (à vérifier)
- ~~**AL-Q49 · Asymétrie restreindre / élargir**~~ → **AL-D48** (AL-F09) — **Recommandation** : restreindre au plus en classe 2, déclenchable automatiquement sur alerte T3 ou plus ; HECTOR ne restreint jamais seul
- ~~**AL-Q50 · Temps de l'Alliance**~~ → **AL-D49** (AL-F10) — **Recommandation** : NTS servi par le CC, membres croisés, décisions datées par le Median-Time-Past du journal
- ~~**AL-Q51 · Coût d'entrée et seuil d'Alliance**~~ → **AL-D50** (AL-F07) — **Recommandation** : Alliance à partir de ≈ 25 postes ou 2 sites ; en dessous, COG seul multi-utilisateurs ; HECTOR facultatif ; `n` jamais sous 6

### Chemin vers la normativité

- **AL-Q19 · Amendements successifs de la v1 ou gel v2 ?**
  - Les Alliances touchent zones, confiance, identité, fédération et IA : beaucoup de documents gelés à la fois
  - **Recommandation** : un **gel v2** dédié ; ses préalables initiaux (AL-Q01, Q02, Q05, Q09, Q10, Q20, Q27) sont **tous tranchés** depuis le 2026-10-25 ; la revue des failles en ajoute cinq, structurels : AL-Q42 à AL-Q46, **tous tranchés** (AL-D42 à AL-D46) ([synthèse §7](ALLIANCE-SYNTHESE.md)) ; plus aucun préalable de conception ; restent le lot 0 bis et l'AIPD de l'Alliance avant le pilote ; reste au propriétaire de décider du moment du gel ; les autres questions ouvertes peuvent rester ouvertes dans la doc gelée v2, comme en v1

---

Date de mise a jour : 2026-10-25
