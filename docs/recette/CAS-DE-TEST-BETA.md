# Cas de test — recette COG 0.9.0-beta.1

<!-- @id recette.cas -->
<!-- @role test-cases -->
<!-- @layer 0-9+zones -->
<!-- @human Cas de test RB-001 à RB-068 : exigences couvertes, préconditions, étapes, résultat attendu, automatisation -->
<!-- @do list_acceptance_cases -->

- **Statut** : v1.0, 2026-10-25 · **Plan** : [PLAN-RECETTE-BETA](PLAN-RECETTE-BETA.md) · **Limites** : [LIMITES-CONNUES-BETA](LIMITES-CONNUES-BETA.md) · **PV** : [PV-RECETTE-MODELE](PV-RECETTE-MODELE.md)
- **68 cas** : 49 avec un contrôle automatisé (en tout ou partie), 3 de bibliothèque (`cargo test`), 16 purement manuels
- **Légende du mode**
  - **Auto** : un contrôle de `tester.sh` (T), une section de `scenarios.sh` (S) ou du banc `--banc` (B) le prouve ; le texte entre guillemets est la description du contrôle
  - **Manuel** : le testeur exécute les étapes
  - **Bibliothèque** : test `cargo` lancé par le mainteneur, résultat joint au PV
- **Gravité d'un échec** : B bloquante · M majeure · m mineure (voir [plan §8](PLAN-RECETTE-BETA.md))
- Raccourcis : `$ETC` = `/etc/miyukini` · `$VAR` = `/var/lib/miyukini` · `$BIN` = `/opt/miyukini/bin`
- Les EX citées sont celles que le comportement **exerce** ; cette recette ne mesure pas la conformité complète à une EX

## 1. Installation propre

<!-- @id recette.cas.installation -->
<!-- @role test-group -->
<!-- @do run_install_cases -->

### RB-001 — Intégrité du paquet (manifeste)

- **EX** : EX-04 (partiel), EX-28 (partiel : empreinte hors bande, pas d'image signée)
- **Pré** : paquet livré, empreinte du `MANIFESTE` reçue hors bande
- **Étapes**
  1. `sha256sum MANIFESTE` et comparer à l'empreinte reçue
  2. `cd cog-paquet && sha256sum --check --strict MANIFESTE`
  3. lire l'en-tête : `# commit`, `# arbre propre`
- **Attendu** : empreintes égales, tous les fichiers `OK`, aucun fichier hors manifeste, arbre propre, commit = étiquette `0.9.0-beta.1`
- **Mode** : Manuel (l'installateur refait la vérification, voir RB-002) · **Gravité** : B

### RB-002 — Installation propre puis idempotente

- **EX** : EX-21, EX-48
- **Pré** : instantané vierge, RB-001 passé
- **Étapes**
  1. `sudo installer.sh --vm-jetable --bundle /home/cog/cog-paquet --manifeste-sha256 <hex> --avec-banc`
  2. noter le code de sortie ; relancer la même commande
  3. comparer `md5sum $ETC/*.env` avant / après la 2e exécution
- **Attendu** : code 0 aux deux passages ; secrets (`miyucloud.env`, `cog-web-portal.env`) **inchangés** ; aucun message d'erreur ; binaires de `$BIN` conformes à `/opt/miyukini/MANIFESTE`
- **Mode** : Auto en partie (T « binaires installés conformes à /opt/miyukini/MANIFESTE ») · **Gravité** : B

### RB-003 — Services actifs, sans redémarrage en boucle

- **EX** : —
- **Pré** : RB-002
- **Étapes** : lancer `tester.sh` ; `systemctl status` des quatre unités et de `postgresql`
- **Attendu** : `miyukini-kindmother`, `-maia`, `-miyucloud`, `-cog-web-portal` actifs ; « pas de redémarrage en boucle » `ok`
- **Mode** : Auto T (« <unité> actif », « <unité> : pas de redémarrage en boucle ») · **Gravité** : B

### RB-004 — Écoutes uniquement sur la boucle locale

- **EX** : EX-48
- **Pré** : RB-003
- **Étapes** : `ss -ltnp` ; depuis une autre machine du réseau, `nmap -p 50051,11435,8086,8090,5432 <ip VM>`
- **Attendu** : 50051, 11435, 8086, 8090, 5432 sur `127.0.0.1` seulement (hors SSH et résolveur) ; ports filtrés ou fermés depuis l'extérieur
- **Mode** : Auto T (« aucune écoute hors boucle locale (hors SSH et résolveur systemd) », « <service> écoute sur <adresse> ») + extérieur manuel · **Gravité** : B

### RB-005 — Comptes, permissions, durcissement systemd

- **EX** : EX-21
- **Pré** : RB-003
- **Étapes** : `tester.sh` ; `systemctl show miyukini-maia -p NoNewPrivileges,ProtectSystem,PrivateTmp`
- **Attendu** : comptes de service système en `nologin` ; `$VAR/<svc>` en 0700 propriété du compte ; secrets en 0600 ; `NoNewPrivileges=yes`, `ProtectSystem=strict`, `PrivateTmp=yes` pour chaque unité
- **Mode** : Auto T (« compte de service … nologin », « … : 0700 », « secrets … en 0600 », « … : NoNewPrivileges=yes » etc.) · **Gravité** : B

### RB-006 — `bridge.toml` de MAIA en lecture seule

- **EX** : EX-21
- **Pré** : RB-003
- **Étapes** : `stat $ETC/bridge.toml` ; `sudo nsenter -t <PID MAIA> -m touch $VAR/maia/bridge.toml`
- **Attendu** : propriété root, 0444 ; écriture dans l'espace de noms du service refusée (système de fichiers en lecture seule)
- **Mode** : Auto T (« bridge.toml : … root, 0444 », « MAIA : bridge.toml monté en lecture seule dans l'espace du service ») · **Gravité** : M

## 2. Refus fail-closed

<!-- @id recette.cas.failclosed -->
<!-- @role test-group -->
<!-- @do run_failclosed_cases -->

### RB-007 — L'installateur refuse hors VM jetable

- **EX** : —
- **Pré** : paquet présent
- **Étapes** : `sudo installer.sh --bundle /home/cog/cog-paquet` (sans `--vm-jetable`)
- **Attendu** : refus explicite, code non nul, aucun changement (`systemctl list-unit-files 'miyukini-*'` identique)
- **Mode** : Manuel · **Gravité** : B

### RB-008 — KindMother sans poignée de main

- **EX** : EX-21
- **Pré** : RB-003
- **Étapes** : `tester.sh` (client Python du testeur)
- **Attendu** : réponse au ClientHello par un défi signé ; `HealthCheck` sans authentification refusé, jamais de `HealthCheckResult`
- **Mode** : Auto T (« KindMother répond au ClientHello par un défi signé », « KindMother refuse HealthCheck sans authentification ») · **Gravité** : B

### RB-009 — KindMother refuse une écoute hors boucle locale

- **EX** : EX-48
- **Pré** : RB-003
- **Étapes** : `tester.sh` (démarrage jetable sur une copie du binaire, répertoire temporaire)
- **Attendu** : démarrage refusé, code de refus non nul
- **Mode** : Auto T (« KindMother refuse une écoute hors boucle locale (démarrage jetable) ») · **Gravité** : B

### RB-010 — MAIA sans appelant épinglé : 503

- **EX** : EX-48, EX-58
- **Pré** : RB-003, aucune configuration mémoire (avant `scenarios.sh`), ou voir note
- **Étapes** : `curl -si http://127.0.0.1:11435/v1/llm/chat -d '{}'` ; routes mémoire sans signature
- **Attendu** : 503 sur l'API protégée et sur `/v1/llm/chat` ; `/health` 200 `status ok`. Aucun modèle n'est chargé : aucune réponse de LLM n'est attendue
- **Mode** : Auto T (« MAIA GET … sans signature refusé (503, aucun appelant épinglé) », « MAIA POST /v1/llm/chat sans signature refusé (503) », « MAIA /health répond 200 avec status ok ») · **Gravité** : B
- **Note** : après `scenarios.sh`, les routes mémoire sont configurées et répondent 401 (voir RB-029)

### RB-011 — MAIA refuse une écoute hors boucle locale sans appelant épinglé

- **EX** : EX-58
- **Pré** : RB-003
- **Étapes** : `tester.sh` (démarrage jetable)
- **Attendu** : démarrage refusé
- **Mode** : Auto T (« MAIA refuse de démarrer hors boucle locale sans appelant épinglé (démarrage jetable) ») · **Gravité** : B

### RB-012 — MiyuCloud sans session : 401

- **EX** : EX-21
- **Pré** : RB-003
- **Étapes** : `curl -si http://127.0.0.1:8086/api/files` (sans cookie)
- **Attendu** : 401 ; `/api/auth/status` 200 ; page d'accueil 200
- **Mode** : Auto T (« MiyuCloud API protégée refuse sans session (401) », « MiyuCloud /api/auth/status répond 200 ») · **Gravité** : B

## 3. MiyuCloud : chiffrement, CDR, quarantaine, WebDAV

<!-- @id recette.cas.miyucloud -->
<!-- @role test-group -->
<!-- @do run_miyucloud_cases -->

### RB-013 — Migrations et premier administrateur

- **EX** : —
- **Pré** : RB-003
- **Étapes** : `scenarios.sh` sections 1 et 2
- **Attendu** : migrations rejouées sur base vierge en transaction, dont `20261025000002_cdr_reclassement` et son index partiel ; `POST /api/setup` 201 ; `initialized: true` ; connexion 200 ; API sans session 401
- **Mode** : Auto S1, S2 (« migrations de l'instance : toutes réussies… », « premier administrateur créé par POST /api/setup ») · **Gravité** : B

### RB-014 — Chiffrement au repos `MIYUCENC`

- **EX** : A-013 (chiffrement des blobs), EX-25 (partiel)
- **Pré** : RB-013
- **Étapes**
  1. téléverser un PNG contenant un marqueur texte
  2. chercher le marqueur dans les blobs de `$VAR/miyucloud` : `sudo grep -r <marqueur> $VAR/miyucloud`
  3. `sudo head -c 8 <blob>`
- **Attendu** : le blob commence par `MIYUCENC` ; le marqueur en clair est **absent** du disque ; le téléchargement par l'API redonne le contenu
- **Mode** : Auto S3 (« sur disque : blob commence par MIYUCENC, clair absent ») · **Gravité** : B
- **Note** : vignettes, transcodages et temporaires d'envoi restent en clair ([L-17](LIMITES-CONNUES-BETA.md))

### RB-015 — Clé de chiffrement invalide : démarrage refusé

- **EX** : A-013
- **Pré** : instantané pris (la VM sera modifiée), RB-014
- **Étapes**
  1. `sudo systemctl stop miyukini-miyucloud`
  2. copier `$ETC/miyucloud.env`, y remplacer `MIYUCLOUD_ENCRYPTION_KEY` par une valeur de 16 octets
  3. `sudo systemctl start miyukini-miyucloud`, `journalctl -u miyukini-miyucloud -n 20`
  4. restaurer le fichier d'origine, redémarrer
- **Attendu** : démarrage refusé (clé de 32 octets exactement), aucun blob n'est écrit en clair ; service rétabli après restauration
- **Mode** : Manuel · **Gravité** : B

### RB-016 — CDR : EXIF désarmé

- **EX** : EX-45
- **Pré** : RB-013
- **Étapes** : `scenarios.sh` section 3
- **Attendu** : PNG avec chunk `eXIf` accepté (201), état CDR `desarme`, métadonnées absentes du fichier servi ; téléchargement 200
- **Mode** : Auto S3 (« téléversement PNG EXIF (201) », « état CDR du PNG = desarme ») · **Gravité** : B

### RB-017 — CDR : PDF en quarantaine

- **EX** : EX-45
- **Pré** : RB-013
- **Étapes** : `scenarios.sh` section 4 ; en interface web, téléverser un PDF puis tenter de l'ouvrir
- **Attendu** : accepté en quarantaine (201) ; téléchargement sans accord 403 ; avec accord explicite 200
- **Mode** : Auto S4 (« état CDR du PDF = quarantaine », « … sans accord refusé (403) », « … avec accord explicite (200) ») + interface manuelle · **Gravité** : B

### RB-018 — CDR : polyglotte (PNG déclaré, contenu exécutable)

- **EX** : EX-45
- **Pré** : RB-013
- **Étapes** : `scenarios.sh` section 4
- **Attendu** : refusé (rien stocké) ou accepté en quarantaine seulement ; jamais servi sans accord (403)
- **Mode** : Auto S4 (« polyglotte refusé… », « polyglotte : jamais servi sans accord (403) ») · **Gravité** : B

### RB-019 — Téléversement sans dossier

- **EX** : —
- **Pré** : RB-013
- **Étapes** : `scenarios.sh` section 4 (fin)
- **Attendu** : refus de saisie 400 ; rien n'est stocké (compte SQL 0)
- **Mode** : Auto S4 · **Gravité** : M

### RB-020 — Lectures par plages

- **EX** : —
- **Pré** : RB-014
- **Étapes** : `curl -H 'Range: bytes=0-15'` puis `bytes=10-29`
- **Attendu** : 206 pour les deux, longueur exacte, contenu déchiffré correct
- **Mode** : Auto S3 (« plage bytes=0-15 : 206 », « plage bytes=10-29 : 206 ») · **Gravité** : M

### RB-021 — Copie puis suppression de l'original

- **EX** : —
- **Pré** : RB-013
- **Étapes** : `scenarios.sh` section 5
- **Attendu** : `ref_count` = 1 après envoi, 2 après copie par lot ; copie lisible (200) après suppression de l'original ; `ref_count` cohérent
- **Mode** : Auto S5 · **Gravité** : B (perte de données si échec)

### RB-022 — Corbeille et ramasse-miettes

- **EX** : —
- **Pré** : RB-021
- **Étapes** : `scenarios.sh` section 8
- **Attendu** : plus de ligne de fichier pour le blob ; `ref_count` 0 ; après redémarrage contrôlé, ligne du blob supprimée
- **Mode** : Auto S8 · **Gravité** : M

### RB-023 — Réconciliation après corruption de `ref_count`

- **EX** : —
- **Pré** : RB-022
- **Étapes** : `scenarios.sh` section 8 bis (corruption de test, redémarrage)
- **Attendu** : `ref_count` recalculé depuis `storage.files` au démarrage
- **Mode** : Auto S8bis (« réconciliation au démarrage : ref_count recalculé… ») · **Gravité** : M

### RB-024 — Cohérence SQL globale

- **EX** : —
- **Pré** : RB-023
- **Étapes** : `scenarios.sh` section 9
- **Attendu** : aucun blob dont `ref_count` diffère du nombre de fichiers ; aucun blob orphelin
- **Mode** : Auto S9 · **Gravité** : B

### RB-025 — WebDAV

- **EX** : EX-41 (contradiction connue : le service parse WebDAV lui-même, voir [L-20](LIMITES-CONNUES-BETA.md))
- **Pré** : RB-013, identifiants de `~/.cog-scenarios/admin.env`
- **Étapes** : `scenarios.sh` section 6 ; avec un client WebDAV (`cadaver`, Nautilus) par tunnel SSH
- **Attendu** : `GET` 200 ; `PROPFIND` 207 ; `PROPFIND` sans authentification 401 ; un fichier posé par `PUT` apparaît dans le dossier personnel et est soumis au CDR
- **Mode** : Auto S6 (« WebDAV GET (200) », « WebDAV PROPFIND (207) », « … sans authentification refusé (401) ») + client manuel · **Gravité** : M

### RB-026 — WOPI sans jeton

- **EX** : —
- **Pré** : RB-013
- **Étapes** : `scenarios.sh` section 7
- **Attendu** : accès refusé sans jeton. WOPI est **désactivé** dans l'unité : le refus est observé en 404, pas par l'authentification WOPI ([L-12](LIMITES-CONNUES-BETA.md))
- **Mode** : Auto S7 · **Gravité** : m

### RB-027 — Interface web MiyuCloud par tunnel

- **EX** : EX-23
- **Pré** : RB-013, tunnel SSH vers 8086
- **Étapes** : connexion ; création de dossier ; envoi d'une image, d'un PDF ; déplacement ; corbeille ; déconnexion
- **Attendu** : parcours sans erreur serveur ; PDF signalé en quarantaine ; page protégée inaccessible après déconnexion
- **Mode** : Manuel · **Gravité** : M

## 4. Portail web

<!-- @id recette.cas.portail -->
<!-- @role test-group -->
<!-- @do run_portal_cases -->

### RB-028 — Page, CSP et chemin inconnu

- **EX** : EX-23, EX-48
- **Pré** : RB-003
- **Étapes** : `tester.sh` ; navigateur par tunnel vers 8090
- **Attendu** : accueil 200 ; en-têtes CSP et anti-cadrage présents ; chemin inconnu < 500 ; aucune ressource externe chargée (onglet réseau du navigateur) ; aucun service raccordé ([L-13](LIMITES-CONNUES-BETA.md))
- **Mode** : Auto T (« portail : page d'accueil 200 », « portail : en-têtes CSP et anti-cadrage présents », « portail : chemin inconnu sans erreur serveur (< 500) ») + navigateur manuel · **Gravité** : M

## 5. MAIA : routes mémoire signées et TAMR

<!-- @id recette.cas.maia -->
<!-- @role test-group -->
<!-- @do run_maia_cases -->

### RB-029 — Routes mémoire : désactivées puis exigeant une signature

- **EX** : EX-55, EX-56
- **Pré** : RB-003
- **Étapes** : avant `scenarios.sh` : `curl -si -X POST http://127.0.0.1:11435/v1/memoire/alice/consulter` ; après : même requête non signée
- **Attendu** : 503 par défaut (routes désactivées) ; 401 une fois configurées ; aucune route en GET, ni cookie, ni CORS
- **Mode** : Auto T (« MAIA routes mémoire désactivées par défaut (503) » / « … actives (configuration d'essai) : sans signature refusé (401) »), S10 (« MAIA sans configuration : routes mémoire en 503 », « requête mémoire non signée refusée (401) ») · **Gravité** : B

### RB-030 — Requêtes signées : succès, rejeu, autre utilisateur, cookie

- **EX** : EX-55, EX-56 ; A-011 (clés épinglées, provisoire : [L-06](LIMITES-CONNUES-BETA.md))
- **Pré** : RB-029, `scenarios.sh` a posé le drop-in `memoire-essai.conf` (deux clés épinglées distinctes)
- **Étapes** : `scenarios.sh` section 10
- **Attendu**
  - requête signée par la clé d'alice : 200
  - même requête rejouée : 401
  - signature de bob sur la mémoire d'alice : 401 ; signature d'alice présentée pour le chemin de bob : 403
  - requête signée avec un cookie ambiant : 400
- **Mode** : Auto S10 · **Gravité** : B

### RB-031 — TAMR : clé mémoire refusée pour approuver

- **EX** : EX-53, EX-54 ; A-013 (clé d'approbation distincte)
- **Pré** : RB-030
- **Étapes** : `scenarios.sh` sections 10 et 10 bis ; essayer la clé d'approbation sur une route autre que l'approbation
- **Attendu** : approbation signée par la clé mémoire : 401 ; non signée : 401 ; clé d'approbation sur une autre route : 401
- **Mode** : Auto S10 (« approbation signée par la clé mémoire : refusée (401) », « clé d'approbation refusée sur une route autre que l'approbation (401) ») · **Gravité** : B

### RB-032 — TAMR : candidat sensible, empreinte, approbation, rejeu

- **EX** : EX-53, EX-54 ; A-010 (strates N2 / N3)
- **Pré** : RB-031, `COG_PROPOSER` (`proposer_essai`) fourni
- **Étapes** : `scenarios.sh` section 10 bis
- **Attendu**
  - deux candidats sensibles listés ; validation : 202 (demande d'approbation TAMR)
  - avant approbation : aucune note écrite
  - mauvaise empreinte : refus ; clé mémoire : 401
  - bonne empreinte et clé d'approbation : 200 (StrongFather ALLOW) ; une note issue du candidat apparaît
  - rejeu de l'approbation : refusé
- **Mode** : Auto S10bis (sans `COG_PROPOSER` le contrôle est `not ok`) · **Gravité** : B

## 6. KindMother : authentification et étiquette publiable

<!-- @id recette.cas.kindmother -->
<!-- @role test-group -->
<!-- @do run_kindmother_cases -->

### RB-033 — Opérateurs non autorisés refusés

- **EX** : EX-21
- **Pré** : RB-030
- **Étapes** : `scenarios.sh` section 11
- **Attendu** : opérateur inconnu, opérateur connu avec mauvaise clé, opérateur connu sans signature : **tous refusés** ; témoin positif : l'opérateur de la mémoire s'authentifie avec sa clé
- **Mode** : Auto S11 · **Gravité** : B

### RB-034 — Identité et clés KindMother protégées

- **EX** : EX-21, EX-25 (partiel)
- **Pré** : RB-003
- **Étapes** : `tester.sh` ; `sudo ls -l $VAR/kindmother`
- **Attendu** : clé d'identité en 0600 dans un répertoire 0700, générée au premier démarrage ; aucune base lisible en clair (SQLCipher par défaut)
- **Mode** : Auto T (« clé d'identité KindMother 0600 dans un répertoire 0700 ») + lecture manuelle des bases · **Gravité** : B

### RB-035 — Étiquette `publiable` : pose valide et refus

- **EX** : EX-47 ; A-015
- **Pré** : sources du dépôt, toolchain, mainteneur
- **Étapes** : `cargo test -p kindmother-service --test etiquette_publiable_ex47`
- **Attendu** : intention signée par une clé d'utilisateur épinglée acceptée ; signature inconnue, fenêtre dépassée (±300 s), horodatage non croissant, rejeu : refus ; clé d'utilisateur égale à une clé de service : démarrage refusé
- **Mode** : Bibliothèque (non installée dans la VM, aucune route de publication : [L-05](LIMITES-CONNUES-BETA.md)) · **Gravité** : M

### RB-036 — Étiquette `publiable` : donnée nationale et modification

- **EX** : EX-47 ; A-015
- **Pré** : RB-035
- **Étapes** : même test ; relire les cas « base `maia-*` » et « donnée modifiée après la pose »
- **Attendu** : donnée `nationale` (bases `maia-*`, `bases-nationales`) jamais étiquetable ; donnée modifiée après la pose non publiable ; état = dernier événement du journal d'audit chaîné
- **Mode** : Bibliothèque · **Gravité** : B

## 7. Zone COG sans sortie

<!-- @id recette.cas.zonecog -->
<!-- @role test-group -->
<!-- @do run_cog_zone_cases -->

### RB-037 — Aucune sortie pour les comptes COG

- **EX** : EX-12, EX-20, EX-17
- **Pré** : RB-003, table nftables chargée
- **Étapes** : `tester.sh` ; `sudo -u miyukini-maia curl -m 5 http://1.1.1.1` pour chacun des quatre comptes
- **Attendu** : échec (délai ou refus local) pour les quatre ; table `inet miyudouane_hote` chargée (chaîne `sortie`, rejet journalisé `SORTIE-COG` et compté)
- **Mode** : Auto T (« Douane : table nftables miyudouane_hote chargée, sortie des comptes COG refusée », « COG <compte> : sortie vers Internet impossible (1.1.1.1:80 injoignable) ») · **Gravité** : B

### RB-038 — Contrôle positif : un compte non COG sort

- **EX** : EX-20
- **Pré** : RB-037
- **Étapes** : `curl -m 5 http://1.1.1.1` en tant que `cog`
- **Attendu** : la connexion aboutit (sinon RB-037 ne prouve rien : cas invalide, à refaire)
- **Mode** : Auto T (« contrôle : un compte non COG atteint Internet (NAT de la VM) ») · **Gravité** : M

### RB-039 — Les autres tables nftables sont intactes

- **EX** : —
- **Pré** : RB-037
- **Étapes** : `sudo nft list tables` avant / après installation ; `systemctl is-enabled miyukini-douane-cog`
- **Attendu** : `inet miyudouane_hote` seule ajoutée ; aucune réécriture de l'existant ; unité activée au démarrage
- **Mode** : Auto T (« Douane : table nftables de l'hôte chargée sans réécriture des autres tables… », « Douane : unité de zone COG activée au démarrage ») · **Gravité** : B

### RB-040 — Entrée non locale refusée sur les ports COG

- **EX** : EX-48, EX-17
- **Pré** : RB-037, deuxième machine du même réseau
- **Étapes** : depuis l'autre machine, `nc -vz <ip VM> 50051 11435 8086 8090 5432`
- **Attendu** : aucun port ne répond ; SSH reste joignable
- **Mode** : Manuel · **Gravité** : B

### RB-041 — Services joignables en boucle locale depuis leur compte

- **EX** : —
- **Pré** : RB-037
- **Étapes** : `tester.sh`
- **Attendu** : MAIA `/health` 200 depuis son compte ; KindMother joignable depuis le compte MAIA
- **Mode** : Auto T (« COG miyukini-miyucloud : PostgreSQL 127.0.0.1:5432 joignable (flux déclaré) », « COG miyukini-maia : KindMother joignable en boucle locale depuis son compte ») · **Gravité** : B

## 8. AppArmor

<!-- @id recette.cas.apparmor -->
<!-- @role test-group -->
<!-- @do run_apparmor_cases -->

### RB-042 — Cinq profils chargés en enforce

- **EX** : EX-21, EX-46
- **Pré** : RB-002
- **Étapes** : `sudo aa-status` ou `/sys/kernel/security/apparmor/profiles`
- **Attendu** : `miyukini-kindmother`, `-maia`, `-miyucloud`, `-cog-web-portal`, `-portail-sas` en `enforce` ; aucun profil `miyukini-*` en `complain`
- **Mode** : Auto T (« AppArmor : profil <p> chargé en enforce », « AppArmor : aucun profil miyukini-* en complain ») · **Gravité** : B
- **Note** : un profil `null-` en complain peut survivre à une mise au point ; il disparaît au redémarrage

### RB-043 — Processus principaux confinés

- **EX** : EX-21
- **Pré** : RB-042
- **Étapes** : `cat /proc/<PID>/attr/current` pour chaque service
- **Attendu** : `miyukini-<svc> (enforce)`
- **Mode** : Auto T (« <unité> : processus principal confiné par son profil (enforce) ») · **Gravité** : B

### RB-044 — Aucun refus AppArmor pendant les scénarios

- **EX** : EX-21
- **Pré** : `scenarios.sh` exécuté
- **Étapes** : `scenarios.sh` section 12 ; `sudo journalctl -k | grep 'apparmor="DENIED"'`
- **Attendu** : aucun `DENIED` d'un profil `miyukini-*` depuis le début du passage ; un refus de profil tiers est noté, non bloquant
- **Mode** : Auto S12 (« aucun refus AppArmor (profils miyukini-*) dans le journal noyau ») · **Gravité** : B

## 9. Sas du Portail et périmètre des binaires

<!-- @id recette.cas.sas -->
<!-- @role test-group -->
<!-- @do run_sas_cases -->

### RB-045 — `portail-sas` refuse sans argument

- **EX** : EX-46
- **Pré** : RB-002
- **Étapes** : `$BIN/portail-sas ; echo $?`
- **Attendu** : code non nul. Un noyau sans Landlock donne le code 5 et aucun décodage (A-012)
- **Mode** : Auto T (« portail-sas : sans argument, refuse (code non nul) ») · **Gravité** : M

### RB-046 — Binaires hors périmètre absents ou inertes

- **EX** : EX-24
- **Pré** : RB-002
- **Étapes** : `ls $BIN` ; `ls $ETC/douane`
- **Attendu** : `miyuracine` (racine hors ligne) **absent** ; aucune configuration `/etc/miyukini/douane` hors banc ; aucun Central desktop installé
- **Mode** : Auto T (« racine hors ligne (miyuracine) absente de la VM réseau », « Douane : binaire présent, aucune configuration /etc/miyukini/douane (hors banc) ») · **Gravité** : B

## 10. Banc d'intrusion (lot 3)

<!-- @id recette.cas.banc -->
<!-- @role test-group -->
<!-- @do run_intrusion_cases -->

### RB-047 — Banc complet : 100 contrôles

- **EX** : EX-12, EX-17, EX-19, EX-20, EX-21, EX-29, EX-44, EX-49
- **Pré** : `br_netfilter` absent, `/etc/miyukini/douane` absent, VM, root, aucun espace `mcog-*`
- **Étapes** : `sudo /home/cog/cog-paquet/tester.sh --banc` (environ 2 min)
- **Attendu** : « banc d'intrusion lot 3 : 100 contrôles ok, 0 not ok » ; code de sortie 0
- **Mode** : Auto B · **Gravité** : B

### RB-048 — Banc : structure des zones

- **EX** : EX-19, EX-20, EX-21, EX-44
- **Pré** : RB-047 (lire la sortie TAP)
- **Étapes** : relever les lignes « structure »
- **Attendu** : `table inet miyudouane` à 3 chaînes `policy drop` par frontière ; `table bridge miyudouane_zone` par zone ; aucune route par défaut en sandbox COG et Franche locale ; `ip_forward=0` sur les portails ; IPv6 désactivé
- **Mode** : Auto B · **Gravité** : B

### RB-049 — Banc : flux déclarés passent

- **EX** : EX-19, EX-44
- **Pré** : RB-047
- **Étapes** : relever les sondes « flux déclarés »
- **Attendu** : voies A (7443) et B (7080), capteur (1883), International en liste blanche vers Franche A:443 : connexion **et** jeton reçu
- **Mode** : Auto B · **Gravité** : B

### RB-050 — Banc : flux non déclarés refusés et journalisés

- **EX** : EX-12, EX-17, EX-20, EX-21, EX-29
- **Pré** : RB-047
- **Étapes** : relever les sondes « flux non déclarés » ; si présent, `tshark -V -r journaux/nflog_<espace>.pcap`
- **Attendu** : latéral COG, COG vers Internet (même avec route forcée), Franche vers COG, IPv6 forcé : aucun jeton reçu par aucun écouteur ; compteur et préfixe `MDOU|…|LATERAL`, `SORTIE-COG`, `NON-DECLARE` attendus ; **aucun RST** (un RST serait un échec)
- **Mode** : Auto B · **Gravité** : B

### RB-051 — Banc : coupure, fermeture, rétablissement (EX-49)

- **EX** : EX-49, EX-19
- **Pré** : RB-047
- **Étapes** : relever les étapes « coupure » du banc
- **Attendu** : voie A coupée dans les deux sens, voie B, Portail local et frontière externe intacts ; fermeture du Portail local : capteur bloqué (`FERME`), Portail interne intact ; rétablissement : voie A et capteur de retour
- **Mode** : Auto B · **Gravité** : B

### RB-052 — Banc : retrait propre et refus hors VM

- **EX** : —
- **Pré** : RB-047 terminé
- **Étapes** : `ip netns list | grep mcog` ; `ls /etc/miyukini/douane` ; `lsmod | grep nf_conntrack_bridge` ; tenter `lancer.sh` sans `--vm-jetable`
- **Attendu** : aucun espace `mcog-*`, `/etc/miyukini/douane` absent, module déchargé s'il a été chargé par le banc ; lancement sans `--vm-jetable` refusé (code 2)
- **Mode** : Auto B (piège `EXIT`) + vérification manuelle · **Gravité** : M

## 11. Relais NFLOG et voie aveugle

<!-- @id recette.cas.relais -->
<!-- @role test-group -->
<!-- @do run_nflog_cases -->

> `miyurelais` n'est pas installé comme service dans la VM ([L-18](LIMITES-CONNUES-BETA.md)) : ces cas utilisent le script d'essai du dépôt et les tests de bibliothèque.

### RB-053 — Essai réel NFLOG : lignes, témoin, signaux

- **EX** : EX-12, EX-49 ; A-015
- **Pré** : VM jetable, root, binaires `essai_vm` et `miyurelais` fournis par le mainteneur
- **Étapes** : `sudo crates/miyurelais/tests/vm/essai_nflog.sh <essai_vm> <miyurelais>`
- **Attendu** : toutes les lignes `OK`, aucune `KO` ; ligne de format `<sec>.<µs6> <hôte> nflog-<G>: <préfixe> IN= OUT= SRC= DST= …` ; un `NON-DECLARE` coupe la voie émettrice ; le préfixe `MDOU-TEMOIN` ne produit aucune ligne ; espaces `mrel-*` supprimés en fin d'essai
- **Mode** : Manuel (script) · **Gravité** : B

### RB-054 — Voie aveugle : capteur muet, perte, silence normal

- **EX** : EX-49 ; A-015
- **Pré** : RB-053
- **Étapes** : dans l'essai, arrêter les paquets de témoin sur une voie déclarée active ; recommencer sur une voie inactive
- **Attendu** : capteur muet sur voie active ⇒ relève en échec ⇒ **voie aveugle coupée** ; voie inactive : silence normal, pas de coupure ; trou de séquence ou `ENOBUFS` ⇒ échec
- **Mode** : Manuel (script) · **Gravité** : B

### RB-055 — Tests de bibliothèque du relais et des Vigies

- **EX** : EX-12, EX-49
- **Pré** : sources du dépôt
- **Étapes** : `cargo test -p miyurelais -p miyuvigie -p worrysentinel` (dont `chaine.rs`, `chaine_vigies.rs`)
- **Attendu** : tests verts : `non_declare_coupe_la_voie_emettrice`, `capteur_muet_voie_aveugle_coupee`, `voie_inactive_silence_normal`, `trou_de_sequence_lignes_d_abord_puis_echec`, `debordement_socket_echec_immediat`, `trame_refusee_capteur_degrade`
- **Mode** : Bibliothèque · **Gravité** : M

## 12. Redémarrage

<!-- @id recette.cas.redemarrage -->
<!-- @role test-group -->
<!-- @do run_restart_cases -->

### RB-056 — Redémarrage complet de la VM

- **EX** : EX-12, EX-20
- **Pré** : phases A à C passées
- **Étapes** : `sudo reboot` ; attendre 2 min ; `tester.sh`
- **Attendu** : quatre services actifs sans intervention ; table `miyudouane_hote` rechargée avant eux ; profils AppArmor rechargés ; 100 contrôles `ok` (routes mémoire en 401 si `scenarios.sh` est passé, 503 sinon)
- **Mode** : Auto T après redémarrage manuel · **Gravité** : B

### RB-057 — Ordre de démarrage MAIA après KindMother

- **EX** : —
- **Pré** : RB-056
- **Étapes** : `systemctl list-dependencies miyukini-maia` ; `journalctl -b -u miyukini-attente-kindmother -u miyukini-maia`
- **Attendu** : `miyukini-maia` démarre après `miyukini-kindmother` et après l'unité d'attente (30 s au plus, non fatale) ; mémoire de MAIA ouverte (pas de « Connection refused »)
- **Mode** : Auto T (« MAIA démarre après KindMother et après l'attente de son port (ordre au démarrage) ») · **Gravité** : M

### RB-058 — Redémarrage unitaire KindMother puis MAIA

- **EX** : EX-21
- **Pré** : routes mémoire configurées (`scenarios.sh`)
- **Étapes** : `systemctl restart miyukini-kindmother` puis `systemctl restart miyukini-maia` ; requête signée de RB-030
- **Attendu** : 200 après le redémarrage de MAIA ; identité KindMother inchangée (`server.pub` identique) ; données conservées
- **Mode** : Manuel (S10 le réalise aussi) · **Gravité** : M

### RB-059 — Constat : MAIA ne se reconnecte pas à KindMother

- **EX** : EX-50
- **Pré** : RB-058
- **Étapes** : redémarrer KindMother **seul** ; requête signée à MAIA ; puis redémarrer MAIA
- **Attendu** : défaut **connu** ([L-11](LIMITES-CONNUES-BETA.md)) : 503 (« Broken pipe ») jusqu'au redémarrage de MAIA ; le cas est « passé » s'il est constaté tel quel et consigné ; une réponse 200 sans redémarrage est un bon résultat à noter
- **Mode** : Manuel · **Gravité** : m (défaut documenté)

### RB-060 — Coupure brutale

- **EX** : EX-08 (partiel)
- **Pré** : données présentes (fichiers MiyuCloud, mémoire)
- **Étapes** : arrêt brutal de la VM (équivalent débranchement) pendant un envoi de fichier ; redémarrage ; `tester.sh` ; `scenarios.sh` sections 8 bis et 9
- **Attendu** : PostgreSQL redémarre (reprise après incident) ; services actifs ; cohérence `ref_count` rétablie par la réconciliation ; aucun fichier partiel servi ; bases KindMother ouvrables
- **Mode** : Manuel + Auto S8bis, S9 · **Gravité** : B

## 13. Sauvegarde et restauration

<!-- @id recette.cas.sauvegarde -->
<!-- @role test-group -->
<!-- @do run_backup_cases -->

> Aucun outil de sauvegarde n'est livré (EX-27 absente, [L-16](LIMITES-CONNUES-BETA.md)). Ces cas éprouvent la **procédure manuelle** décrite au [plan §7.5](PLAN-RECETTE-BETA.md).

### RB-061 — Sauvegarde manuelle

- **EX** : EX-08 (partiel)
- **Pré** : RB-027 et RB-032 passés (données réelles)
- **Étapes**
  1. `sudo systemctl stop miyukini-cog-web-portal miyukini-maia miyukini-miyucloud miyukini-kindmother`
  2. `sudo -u postgres pg_dump -Fc miyucloud > miyucloud.dump`
  3. `sudo tar --xattrs -cpf cog-sauvegarde.tar $VAR $ETC /opt/miyukini/MANIFESTE`
  4. `sha256sum` de l'archive et du dump ; copie hors de la VM
  5. redémarrer les services
- **Attendu** : archives créées, empreintes notées au PV, services de retour (`tester.sh` `ok`), archive jamais laissée en clair hors de la VM
- **Mode** : Manuel · **Gravité** : B

### RB-062 — Restauration sur une VM vierge

- **EX** : EX-08 (partiel), EX-21
- **Pré** : RB-061, nouvel instantané vierge, **même paquet** installé (RB-002)
- **Étapes**
  1. arrêter les quatre services
  2. `pg_restore --clean --if-exists -d miyucloud miyucloud.dump` ; extraire l'archive (droits et propriétaires conservés)
  3. `sudo systemctl start …` ; `tester.sh`
  4. se connecter à MiyuCloud, rouvrir un fichier téléversé avant la sauvegarde
- **Attendu**
  - permissions 0700 / 0600 d'origine retrouvées
  - même identité KindMother (`server.pub`)
  - fichiers chiffrés relus avec la clé restaurée (`MIYUCLOUD_ENCRYPTION_KEY` du `.env`)
  - mémoire d'alice relisible par requête signée
  - `tester.sh` : 100 `ok`
- **Mode** : Manuel · **Gravité** : B

### RB-063 — Restauration sans la clé de chiffrement (constat)

- **EX** : A-013
- **Pré** : RB-062, instantané
- **Étapes** : restaurer le dump et `$VAR/miyucloud` mais **pas** `$ETC/miyucloud.env` (nouvelle installation, nouvelle clé) ; ouvrir un fichier
- **Attendu** : le fichier est **illisible** (échec d'authentification AES-GCM), sans corruption silencieuse ni service qui plante ; message de procédure : sauvegarder `$ETC` avec les données
- **Mode** : Manuel · **Gravité** : M

## 14. Mise à jour et retour arrière

<!-- @id recette.cas.maj -->
<!-- @role test-group -->
<!-- @do run_update_cases -->

### RB-064 — Mise à jour par un nouveau paquet

- **EX** : EX-04 (partiel)
- **Pré** : paquet `0.9.0-beta.2` (ou reconstruction), données présentes
- **Étapes** : `installer.sh --vm-jetable --bundle <nouveau> --manifeste-sha256 <hex>` ; `tester.sh` ; `scenarios.sh` ; comparer `$ETC/*.env`
- **Attendu**
  - secrets, clé de chiffrement, mot de passe PostgreSQL inchangés
  - données et comptes conservés
  - `/opt/miyukini/MANIFESTE` = nouveau commit
  - migrations appliquées sans erreur
  - 100 contrôles `ok`
- **Mode** : Manuel + Auto T, S · **Gravité** : B

### RB-065 — Paquet altéré refusé

- **EX** : EX-28 (partiel)
- **Pré** : copie du paquet
- **Étapes** : modifier un octet de `bin/maia` (copie) ; lancer l'installateur
- **Attendu** : refus **avant** toute installation (`sha256sum --check --strict`) ; ajout d'un fichier hors manifeste également refusé ; installation en place inchangée
- **Mode** : Manuel · **Gravité** : B

### RB-066 — Empreinte de manifeste erronée refusée

- **EX** : EX-28 (partiel)
- **Pré** : paquet sain
- **Étapes** : `installer.sh … --manifeste-sha256 <empreinte fausse>`
- **Attendu** : refus explicite, aucune modification
- **Mode** : Manuel · **Gravité** : B

### RB-067 — Retour arrière après une mise à jour

- **EX** : EX-08 (partiel)
- **Pré** : sauvegarde de RB-061 faite **avant** RB-064
- **Étapes**
  1. arrêter les services
  2. réinstaller le paquet précédent (RB-002 avec l'ancien paquet)
  3. restaurer données et configuration (RB-062)
  4. `tester.sh`
- **Attendu**
  - version précédente en service (`MANIFESTE` ancien), données d'avant la mise à jour
  - **sans la restauration**, base à un schéma plus récent : l'ancien paquet peut refuser de démarrer (migrations montantes seulement, [L-16](LIMITES-CONNUES-BETA.md)) ; le constater et le consigner
- **Mode** : Manuel · **Gravité** : B

## 15. Désinstallation

<!-- @id recette.cas.desinstallation -->
<!-- @role test-group -->
<!-- @do run_uninstall_cases -->

### RB-068 — Désinstallation manuelle

- **EX** : EX-20 (la zone COG disparaît proprement)
- **Pré** : sauvegarde faite (RB-061) ; **dernier cas** ; aucun désinstallateur livré ([L-16](LIMITES-CONNUES-BETA.md))
- **Étapes**
  1. `systemctl disable --now` des quatre services, de `miyukini-attente-kindmother` et de `miyukini-douane-cog`
  2. supprimer les unités de `/etc/systemd/system/` (et le dossier `miyukini-maia.service.d`), `daemon-reload`
  3. `apparmor_parser -R` puis suppression des cinq profils `/etc/apparmor.d/miyukini-*`
  4. `nft delete table inet miyudouane_hote` ; supprimer `$ETC/douane-cog.nft`
  5. `DROP DATABASE miyucloud ; DROP ROLE miyucloud` ; supprimer `$VAR`, `$ETC`, `/opt/miyukini`
  6. supprimer les comptes `miyukini-*`
  7. alternative : retour à l'instantané vierge
- **Attendu**
  - plus d'unité, de profil, de table, de compte `miyukini-*`
  - aucune écoute sur 50051, 11435, 8086, 8090
  - les autres tables nftables et le SSH sont intacts
  - la base `miyucloud` n'existe plus
  - tout résidu est consigné au PV
- **Mode** : Manuel · **Gravité** : M

## 16. Couverture des exigences

<!-- @id recette.cas.couverture -->
<!-- @role coverage -->
<!-- @do summarize_coverage -->

| EX | Cas |
|---|---|
| EX-12, EX-20 | RB-037, RB-047, RB-050, RB-053, RB-056, RB-068 |
| EX-17, EX-19, EX-29, EX-44, EX-49 | RB-047 à RB-051, RB-054, RB-055 |
| EX-21 | RB-002, RB-005, RB-006, RB-008, RB-012, RB-033, RB-034, RB-042, RB-043, RB-044 |
| EX-23 | RB-027, RB-028 |
| EX-45 | RB-016, RB-017, RB-018 |
| EX-46 | RB-042, RB-045 |
| EX-47 | RB-035, RB-036 |
| EX-48 | RB-004, RB-009, RB-010, RB-040 |
| EX-50 à EX-56 | RB-029 à RB-032, RB-059 |
| EX-58 | RB-010, RB-011 |
| EX-04, EX-28 | RB-001, RB-064, RB-065, RB-066 |
| EX-08 (partiel) | RB-060 à RB-063, RB-067 |

- **Non couvertes par la beta** : EX-01 à EX-03, EX-05 à EX-07, EX-09 à EX-11, EX-13 à EX-16, EX-18, EX-22, EX-24 à EX-27, EX-30 à EX-43, EX-57, EX-59 à EX-63 (fédération, canal officiel, Portail réseau, services hybrides, IA fédérée, hors périmètre ou sans composant installé)
