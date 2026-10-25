# Services hybrides et durcissement du Portail interne

> **Gelé v1.0 — 2026-10-24** · amendé **A-004** (v1.4 : Portail, CDR et quarantaine du lot 3, §1, §3.1) · amendé **A-007** (v1.7 : services hybrides du lot 5, H3 refusé, anti-rejeu par auteur, §1, §3.1, §4.1). Toute modification passe par un amendement : [règles du gel](GEL-DOC-CONCEPTUELLE-v1.md).

<!-- @id arch.cog.services-hybrides -->
<!-- @role architecture-specification -->
<!-- @layer 3-8 -->
<!-- @human Services à cheval Franche / zone COG : classes d'exposition, gabarit moteur-façade-traité, durcissement du Portail interne -->
<!-- @do design_hybrid_services_and_harden_portal -->

- **Statut** : v1, 2026-10-24
- **Base** : [modèle zones et couches](MODELE-ZONES-ET-COUCHES-COG.md) (§9 services relayés, §10 contrat) · [menaces](../securite/MODELE-MENACES-COG.md)

## 1. Classes d'exposition

<!-- @id arch.hybride.classes -->
<!-- @role exposure-classification -->
<!-- @do classify_service_exposure -->

- **H0 — Interne pur** : aucune façade, aucun flux externe (MAIA, Cores, compta)
- **H1 — Hybride diffusion** : la zone COG **publie**, rien n'entre (site, partage en lecture seule)
- **H2 — Hybride collaboratif** : des données **entrent** depuis l'extérieur (bureautique coopérative, messagerie, mail entrant)
- **H3 — Hybride actif** : la façade **agit** à l'extérieur pour le compte de la zone COG (explorateur web, mail sortant, domotique)
- Règles par classe :
  - H0 : jamais de façade, même à la demande
  - H1 : voie **publication** seule
  - H2 : voies quarantaine + publication, validation à chaque entrée
  - H3 : comme H2 + destinations sortantes en liste blanche + intention StrongFather pour chaque action
- La classe est déclarée dans le **contrat** (manifeste signé) ; changer de classe = nouvelle installation
- Sens de voie ouverts (A-004) : H0 aucun · H1 sortante · H2 / H3 entrante + sortante (liste blanche sortante et intention StrongFather de H3 : hors Portail)
- **Amendé A-007** : un manifeste **H3 est refusé** tant que l'encodage de sa liste blanche sortante et de son intention StrongFather n'est pas spécifié (§4.1)

## 2. Gabarit d'un service hybride

<!-- @id arch.hybride.gabarit -->
<!-- @role service-template -->
<!-- @layer 5-7 -->
<!-- @do structure_hybrid_service -->

- **Moteur** (zone COG) : logique métier, données, prérequis
- **Façade** (Franche) : terminaison des protocoles externes, assainissement, rien d'autre
- **Traité** : schéma versionné des déclarations échangées moteur ⇄ façade, signé dans le manifeste
- Règles :
  - **INV-H1** le moteur **ne parse jamais** un protocole externe (HTTP, TLS, SMTP, formats de fichiers bruts)
  - **INV-H2** la façade n'a **aucune logique métier** ni secret durable
  - **INV-H3** seuls les types définis par le traité traversent ; tout le reste est refusé
  - **INV-H4** décodeurs générés depuis le traité, des deux côtés (aucune divergence)
  - **INV-H5** façade **jetable** : reconstruite depuis une image signée à chaque session (ou chaque jour), jamais réparée
  - **INV-H6** identifiants de façade **éphémères**, émis par la zone COG pour la session

## 3. Durcissement du Portail interne

<!-- @id arch.hybride.portail -->
<!-- @role boundary-hardening -->
<!-- @layer 3-6 -->
<!-- @do harden_internal_portal -->

**Structure**
- Instance dédiée, espace isolé puis micro-VM
- **Deux voies à sens unique séparées** (diode logicielle) :
  - voie **entrante** (quarantaine → zone COG)
  - voie **sortante** (zone COG → publication)
  - processus distincts, aucune mémoire partagée, aucun chemin retour (acquittements par compteurs seulement)
- **Une voie par service** : files, quotas et clés distincts ; une façade compromise n'atteint pas les autres voies

**Contrôles sur la voie entrante**
1. Trame `MSCE` uniquement, taille bornée (IPS BorderGuard)
2. Signature de la façade (clé de session) **et** de l'auteur (participant)
3. Type autorisé par le traité du service, débit dans le contrat
4. **Désarmement et reconstruction (CDR)** : l'objet est décodé puis **réencodé** dans un format interne sûr ; aucun octet brut ne traverse
   - documents → format interne (sans macro, script, objet embarqué)
   - images → réencodées
   - archives → refusées, ou dépaquetées et contrôlées une à une
5. Inscription au registre Merkle (empreinte)

**Côté zone COG**
- **Sas de décodage** : processus séparé, seccomp strict, qui décode la déclaration en structures typées
- Le moteur ne reçoit que des **structures typées** par IPC (« parse, don't validate »)
- **Deuxième validation**, indépendante de celle du Portail (deux implémentations)
- Décision StrongFather ; TAMR pour les types sensibles
- La zone COG **relève** à son rythme (contre-pression naturelle)

**Contrôles sur la voie sortante**
- **Étiquettes de sensibilité** posées par KindMother sur chaque donnée :
  - `nationale` : ne sort jamais
  - `session` : visible des participants d'une session
  - `publiable` : diffusable
- Seules les données `session` / `publiable` peuvent sortir, **par action explicite de l'utilisateur**
- MAIA ne peut rien étiqueter `publiable` ni publier
- Types de publication déclarés dans le traité, taille bornée

**Réaction**
- WorrySentinel (IDS) peut **couper une voie** sans toucher aux autres
- Coupure = façade détruite et reconstruite, session close, incident au registre

### 3.1 Portail implémenté — amendement A-004

<!-- @id arch.hybride.portail-a004 -->
<!-- @role boundary-implementation -->
<!-- @do record_portal_implementation -->

Implémenté par `crates/miyuportail` et `crates/miyucdr` (lot 3).

**Voies**
- Une voie par service ; files, quotas (≤ 4 096 trames), débit par sens (fenêtre 60 s), compteurs et **clé de publication** distincts
- Voie déclarée **désactivée** par défaut ; ce qui n'est pas déclaré n'existe pas

**Trame `MSCE` « déclaration de voie »**
- Disposition **Sceau v1.1**, type **`0x08`**, même domaine signé (`miyukini/sceau/v1.1`) ; charge ≤ 65 536 o
- Charge TLV, tags strictement croissants, tous obligatoires :

| tag | champ | forme |
|---|---|---|
| 1 | `service` | `[a-z0-9._:-]`, 1..64 o |
| 2 | `sens` | `0x01` entrante, `0x02` sortante |
| 3 | `version du traité` | u64, **égale** à celle du traité |
| 4 | `type` | u64, déclaré par le traité pour ce sens |
| 5 | `séquence` | u64, **strictement croissante** ~~par session~~ **par auteur dans la session** (anti-rejeu ; amendé A-007, implémentation : par session, à aligner) |
| 6 | `émise le` | u64, secondes Unix |
| 7 | `champs` | TLV imbriqué, schéma du traité (≤ 32 champs ; traité ≤ 64 types) |

- Signatures : entrante = **exactement 2** (clé de session de la façade, émise par la zone COG, **et** un auteur épinglé parmi ≤ 32) ; sortante = **exactement 1** (clé de publication de la voie)
- Rupture protocolaire : la trame est décodée puis **réencodée** dans le format interne `MPI1` ; aucun octet reçu ne traverse ; en sortie, trame neuve signée par la voie

**Sas de décodage**
- Processus `portail-sas` séparé, lancé **par lot** (≤ 64 trames), délai 5 s puis mise à mort ; toute fin anormale refuse le lot entier
- Confinement **avant** toute lecture : rlimits (mémoire 256 Mio, CPU 5 s, 3 descripteurs, fichiers et processus interdits, pas de vidage mémoire), `no_new_privs`, seccomp en liste blanche (lecture fd 0, écriture fd 1–2, mémoire sans `PROT_EXEC`, fin) ; tout autre appel **tue** le processus
- **Deuxième validation** en zone COG, implémentation indépendante ; sortie du sas ≠ revalidation = refus

**CDR** (`miyucdr`, fermé par défaut)
- Type réel détecté par signature, refus si différent du type déclaré ; décodage complet sous limites (32 Mio, côté 16 384 px, 40 Mpx, 512 Mio alloués, 10 s)
- Reconstruits : PNG, JPEG, GIF (1re image), WebP, BMP → réencodés sans métadonnées ; texte, CSV (formules neutralisées), Markdown → UTF-8 NFC nettoyé ; JSON → resérialisé (profondeur ≤ 64)
- **Refusés** : PDF, bureautique, archives, exécutables, tout type inconnu
- Le Portail refuse tout document tant qu'aucun CDR n'est branché

**Quarantaine des fichiers non désarmés — MiyuCloud** (décision du propriétaire, implémentation à venir)
- Fichier non pris en charge par le CDR (PDF, vidéo, archives, bureautique…) ou trop volumineux → **quarantaine**
- Stocké **à part**, marqué « **non désarmé** »
- Téléchargeable seulement après une **action explicite** de l'utilisateur
- **Jamais ouvert** par un composant du COG

**Coupure et rétablissement**
- Coupure (WorrySentinel, EX-49) : **voie entière, deux sens** ; session close, files purgées, génération incrémentée (lots en vol rejetés) ; les autres voies inchangées
- Rétablissement : **explicite** seulement ; voie remise **désactivée** avec une **nouvelle clé de publication**

## 4. Cycle de vie d'un service hybride

<!-- @id arch.hybride.cycle-de-vie -->
<!-- @role lifecycle -->
<!-- @do manage_hybrid_service_lifecycle -->

- **Installation** : contrat et traité vérifiés (StrongFather), voies créées, espace Franche dimensionné (§10 du modèle)
- **Exposition désactivée par défaut** : l'utilisateur l'active explicitement, service par service
- **Session** : visa de session, façade neuve, DB secondaire vierge
- **Fin de session** : façade détruite, DB secondaire purgée
- **Mise à jour** : moteur et façade mis à jour ensemble ; versions de traité compatibles ou refus
- **Désactivation** : voies supprimées, aucun résidu en Franche

### 4.1 Services hybrides implémentés — amendement A-007

<!-- @id arch.hybride.lot5-a007 -->
<!-- @role hybrid-implementation -->
<!-- @do record_hybrid_service_implementation -->

Implémenté par `crates/miyuhybride`, `crates/jayrdv-traite`, `crates/jayrdv-facade` et le moteur `crates/jayrdv` (`hybride.rs`) ; Market : `miyumarket` tag 5 ; Portail : `miyuportail::msce::sceller_cosigne` (lot 5).

**Manifeste et contrat de service**
- Manifeste = `Sceau` `0x03` (manifeste de paquet), signé par l'éditeur épinglé ; tags 1–4 du Market (service, version, SHA-256 de l'archive, binaire) + **tag 5 « contrat »**, **obligatoire** pour un service hybride ; le Market le transporte **opaque** (vide refusé), `miyuhybride` le décode strictement
- Contrat = TLV imbriqué, tags strictement croissants :

| tag | champ | forme |
|---|---|---|
| 1 | `genre` | texte `miyukini/contrat-service/v1` (séparation de domaine) |
| 2 | `classe` | 1 o : 0 H0, 1 H1, 2 H2, 3 H3 |
| 3 | `exposition par défaut` | 1 o, **doit valoir 0** (EX-48) |
| 4 | `ressources` | TLV : 1 CPU % (≤ 1 600), 2 mémoire Mio (≤ 65 536), 3 disque Mio, 4 tâches (≤ 4 096), 5 sessions (≤ 1 024) ; toutes ≥ 1 |
| 5 | `voie` | TLV : 1 taille max de trame, 2 débit max / minute / sens, 3 quota de file — absent en H0 |
| 6 | `traité` | encodage canonique du traité du Portail — absent en H0 |
| 7 | `façade` | TLV : 1 SHA-256 de l'image signée, 2 DB secondaire max (Mio, ≤ disque), 3 durée max de session (≤ 24 h) — absent en H0 |
| 8 | `entrées externes` | `n` (1 o) puis `n` × (origine 1 o, protocole 1 o, port u16, débit Kio/s u32, taille max paquet u16), 1..16, strictement croissantes — absent en H0 |

**Règles de classe (fail-closed)**
- H0 : ni voie, ni traité, ni façade, ni entrée ; jamais de voie au Portail
- H1 : au moins un type de **publication** ; aucune session entrante ouverte au Portail
- H2 : au moins un type **entrant**
- **H3 : refusé** tant que son encodage n'est pas spécifié
- Entrées : origine **Fédération** seule (TCP / UDP, port ≠ 0, paquet ≥ 80 o) ; **Origin et International refusés** tant qu'aucune liste blanche de pairs n'existe (INV-Z6)
- Entrées Douane / BorderGuard **dérivées** du manifeste (fonctionnalité `douane`, génération seulement : voie et flux `Fédération → Franche`, plafonds de la sandbox) ; l'arrêté reste scellé par BorderGuard

**Guichets du Portail** (séparés par les types)
- **Façade** (Franche) : déposer une déclaration entrante, lire une publication, état, compteurs — rien d'autre
- **Moteur** (zone COG) : relever les déclarations admises (structures typées), répondre — rien d'autre
- Codecs générés d'une seule déclaration (`message_traite!`), compilés des deux côtés (INV-H4) ; champ inconnu, absent ou de nature différente refusé (INV-H3)

**Signature d'auteur et réponse**
- L'**appareil** signe la charge MSCE (octets signés `0x08`) ; la façade ajoute sa clé de session (`sceller_cosigne` : cosignature vérifiée `verify_strict`, clé faible refusée, **avant** assemblage) ; la façade ne peut rien forger au nom d'un auteur
- Le moteur ne publie qu'en **réponse** à une déclaration admise : **jeton à usage unique**, périmé à la désactivation ou à une nouvelle session ; 1 à 32 messages, étiquette **`Session`**, déclencheur utilisateur
- Chaque réponse porte `demande` = SHA-256 de la trame entrante (corrélation par la façade, sans autre canal retour)

**Identité à la façade**
- Visa du COG hôte + preuve de possession (`miyauth::http`), audience = identifiant du service
- Portée **`jayrdv:reserver`** exigée (JayRDV) ; clé d'appareil du visa = **auteur épinglé** de la session, sinon refus

**Traité JayRDV v1** (H2, premier service migré)

| sens | code | message |
|---|---|---|
| entrant | 1 | `DemandeCreneaux` |
| entrant | 2 | `Reservation` |
| sortant | 101 | `CreneauDisponible` |
| sortant | 102 | `FinCreneaux` |
| sortant | 103 | `ReservationConfirmee` |
| sortant | 104 | `ReservationRefusee` (motif par code, aucun texte libre) |

**Façade jetable** (INV-H5, INV-H6, EX-43)
- Session : image conforme au manifeste, clé de session **éphémère générée en zone COG**, auteurs épinglés, DB secondaire **vierge** bornée par le contrat
- **Condamnée** à la coupure, à la désactivation ou à l'expiration de la session : tout refusé (503), DB purgée
- Le `Superviseur` la **détruit** ; reconstruction **seulement** après `retablir` **et** `activer` explicites (une voie rétablie reste désactivée)

**Multi-appareils** (décision du propriétaire, A-007)
- Anti-rejeu **par auteur** dans une session : chaque appareil épinglé a sa propre séquence strictement croissante
- Plusieurs appareils partagent **une** instance de façade
- Implémentation actuelle : séquence par session (à aligner)

**À décider** (A-007) : encodage de H3 ; liste blanche de pairs (Origin, International) ; point d'application des quotas de disque et de sessions ; emplacement du guichet des visas ; rafraîchissement des révocations d'une façade ; image de façade signée réelle

## 5. Catalogue

<!-- @id arch.hybride.catalogue -->
<!-- @role service-catalog -->
<!-- @do classify_existing_services -->

- **H0** : MAIA, Cores, JayKonta (compta), KindMother
- **H1** : site / page web publique, partage MiyuCloud en lecture seule
- **H2** : JayOffice (Docs, Sheets, Slides, Réunion), JayMessage, partage MiyuCloud en écriture, JayKoa partagé, mail entrant, accès mobile (via Fédération)
- **H3** : explorateur web, mail sortant, Alicia domotique (Franche locale)
- Hors classe : Market et mises à jour → **canal officiel** (§7 du modèle)

## 6. Exigences

<!-- @id arch.hybride.exigences -->
<!-- @role security-requirements -->
<!-- @do derive_hybrid_requirements -->

- **EX-40** Classe d'exposition déclarée dans le contrat signé ; H0 jamais exposé
- **EX-41** Moteur sans parseur de protocole externe (INV-H1)
- **EX-42** Traité versionné et décodeurs générés des deux côtés
- **EX-43** Façades jetables, reconstruites depuis une image signée
- **EX-44** Portail en deux voies à sens unique, une voie par service
- **EX-45** CDR obligatoire sur toute entrée de fichier ou document
- **EX-46** Sas de décodage seccomp en zone COG + deuxième validation indépendante
- **EX-47** Étiquettes de sensibilité KindMother ; seule une action utilisateur rend une donnée publiable
- **EX-48** Exposition désactivée par défaut, activée service par service
- **EX-49** Coupure de voie par WorrySentinel sans impact sur les autres services
