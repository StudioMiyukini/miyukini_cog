# Limites connues et décisions par défaut — beta 0.9.0-beta.1

<!-- @id recette.limites -->
<!-- @role known-limits -->
<!-- @layer 0-9+zones -->
<!-- @human Limites et décisions par défaut de la beta, avec leur impact pour le testeur -->
<!-- @do list_known_limits -->

- **Statut** : v1.0, 2026-10-25 · **Plan** : [PLAN-RECETTE-BETA](PLAN-RECETTE-BETA.md) · **Cas** : [CAS-DE-TEST-BETA](CAS-DE-TEST-BETA.md)
- **Règle** : un écart listé ici n'est **pas** une anomalie ; il est noté « limite attendue » au PV
- **Sources** : [amendements A-004 à A-015](../architecture/GEL-DOC-CONCEPTUELLE-v1.md), [README du déploiement](../../deploy/cog-vm/README.md), [README du banc](../../tests/intrusion-lot3/README.md)

## 1. Décisions par défaut de la beta

<!-- @id recette.limites.decisions -->
<!-- @role default-decisions -->
<!-- @do list_default_decisions -->

| Id | Limite ou décision | Origine | Impact pour le testeur |
|---|---|---|---|
| L-01 | **Pas de guichet des visas** : emplacement et détenteur de la clé d'émission non décidés (INV-H2 : aucun secret durable en Franche) | A-011 | aucun visa n'est émis ; les façades de services hybrides ne sont pas exercées |
| L-02 | **NFQUEUE non retenu** : pas de verdict en ligne, les Vigies lisent du NFLOG | A-015 (chemin critique, contournement interdit par C-12) | la détection est a posteriori : un paquet interdit est bloqué par les règles statiques de la Douane, jamais « jugé » en ligne |
| L-03 | **Clé d'émission par épinglage seulement** : pas de certificat racine → clé d'émission (nature `0x06` « émission » non définie) | A-012 | toute clé d'émission se configure à la main ; pas de chaîne de confiance racine dans la beta |
| L-04 | **`central-remote` sur liste locale** de révocations (source « registre du COG » à décider) | A-012 | hors périmètre (Central absent) ; aucun cas |
| L-05 | **Publication originale sans route dans le pont** : l'étiquette `publiable` existe, aucune route ne publie | A-013, A-015 | RB-035 et RB-036 sont des tests de bibliothèque ; aucune Fiche ne sort de la VM |
| L-06 | **Authentification des routes mémoire par clés épinglées** (Ed25519, usage `maia/memoire`), **provisoire** avant passeport et visa `miyauth` | A-011 | les clés de test sont posées par `scenarios.sh` dans un drop-in systemd ; pas d'enrôlement d'utilisateur final |
| L-07 | **Pas de `tc`** : `miyudouane` ne fait que du rejet nftables ; aucune mise en forme de débit | README déploiement, lot 3 | les plafonds de débit ne sont pas testés (couverts par les tests de l'IPS en bibliothèque) |
| L-08 | **Amendements A-010 à A-015 en attente de validation** du propriétaire ; tout point refusé sera retiré du code | GEL (statut de chaque amendement) | le comportement recetté peut changer ou disparaître ; le PV liste, par amendement, les cas qui en dépendent (voir §3) |
| L-09 | **Aucun résultat d'audit externe** : la revue de sécurité du 2026-10-25 est interne | A-013 | « recette réussie » ne vaut pas « audité » ; la beta n'est pas à exposer à Internet |

## 2. Limites de la machine et de l'outillage

<!-- @id recette.limites.outillage -->
<!-- @role tooling-limits -->
<!-- @do list_tooling_limits -->

| Id | Limite | Impact pour le testeur |
|---|---|---|
| L-10 | **MAIA sans modèle** (aucun GGUF) ; l'API LLM répond 503 | aucune génération de texte ; seuls les refus, la mémoire et le TAMR sont recettés |
| L-11 | **Défaut ouvert** : l'entrepôt mémoire de MAIA ne se reconnecte pas à KindMother après une coupure (« Broken pipe », 503) | redémarrer MAIA après KindMother ; RB-059 le constate |
| L-12 | **WOPI désactivé** (`MIYUCLOUD_WOPI_ENABLED=false`) | le refus sans jeton est un 404 ; pas d'édition en ligne |
| L-13 | **Portail web sans service raccordé** ; `portail-sas` est un processus de décodage lancé par lot, sans processus réseau ; séparation entrant / sortant non spécifiée | le banc éprouve la couche réseau (Douane, zones), non la rupture protocolaire MSCE ni la purge des files |
| L-14 | **Deux installateurs** : `deploy/cog-vm/installer.sh` refuse un hôte non virtualisé (VM jetable) ; `deploy/recette/installer.sh` accepte une machine dédiée (physique ou VM) mais refuse une machine qui ressemble à une production (heuristique : cloudflared, docker, nginx, pm2, Immich, Nextcloud…) | l'heuristique n'est pas une preuve : le testeur certifie la machine dédiée (`--confirme-machine-de-recette`) |
| L-15 | **Zone COG par comptes (UID)** : la table `miyudouane_hote` est rendue par `miyudouane hote` (description `miyudouane-hote 2`, `/etc/miyukini/douane-hote.desc`) ; boucle locale ouverte seulement aux flux déclarés (`maia → 50051`, `miyucloud → 5432`) ; arrêtés d'essai valides un jour, vérifiés à l'installation seulement | le contrôle de vigueur ne se rejoue pas après 24 h ; réinstaller (ou mettre à jour) pour regénérer |
| L-16 | **Sauvegarde et retour arrière de beta** : `sauvegarder.sh` / `restaurer.sh` / `mettre-a-jour.sh` (retour arrière automatique) / `desinstaller.sh` livrés en recette ; migrations MiyuCloud montantes seulement (le retour arrière des données passe par la sauvegarde préalable) ; sauvegarde hors site (EX-27) absente ; la clé locale ne restaure que sur la même machine | copier hors machine une sauvegarde chiffrée par phrase de passe ; ne pas compter sur la clé locale |
| L-17 | **Chiffrement MiyuCloud partiel** : vignettes, transcodages et temporaires d'envoi en clair ; blobs clairs hérités relus mais non migrés | ne pas téléverser de données réelles sensibles ; RB-014 ne prouve que les blobs |
| L-18 | **Relais NFLOG non installé en service** (`miyurelais` : essai de VM et tests de bibliothèque) ; une connexion déjà établie survit à une coupure tant que son état conntrack existe | RB-053 à RB-055 ont besoin de binaires fournis par le mainteneur ; la purge `conntrack -D` relève de WorrySentinel, hors banc |
| L-19 | **Étiquette `publiable`** : toute clé d'utilisateur épinglée peut étiqueter toute base non `nationale` ; un retrait est perdu si la fin du journal d'audit est tronquée (ancrage externe à prévoir) | à signaler au propriétaire, non corrigé en beta |
| L-20 | **Exigences contredites ou non couvertes** : EX-41 (MiyuCloud parse WebDAV lui-même), EX-30 et la fédération, canal officiel de bout en bout (transport par le Portail non lancé), mises à jour Cores, installateur d'OS (correctifs d'OS refusés), EX-24 hors ligne | hors recette ; la liste figure au [§16 des cas](CAS-DE-TEST-BETA.md) |
| L-21 | **Compte MiyuCloud non initialisé** à l'installation : le premier administrateur est créé par l'interface ou `POST /api/setup` | RB-013 le crée ; identifiants de `scenarios.sh` dans `~/.cog-scenarios/admin.env` de la VM, à détruire en fin de recette |
| L-22 | **Reproductibilité et amorçage** : paquet lié à un commit par `MANIFESTE` et **reproductible au bit près** depuis ce commit (vérifié : trois constructions identiques, voir notes de version §1 bis) ; pas d'image signée ; date du sceau fixée par contournement (`LD_PRELOAD`), indicative seulement (EX-04, EX-28) | la confiance repose sur l'empreinte transmise hors bande, vérifiable par reconstruction |
| L-25 | **Amorçage de l'installateur** : `installer.sh` (et le `tests/*.sh` lancé) est lu dans le paquet extrait **avant** toute vérification ; tout le reste (outil de sceau, `commun.sh`, fichiers installés) vient d'une copie privée root 0700 authentifiée par `openssl` et `sha256sum`. Le script est lu en entier avant exécution (bloc `{ … }`) : sa réécriture pendant l'installation est sans effet | vérifier l'archive (`sha256sum -c`, empreinte reçue hors bande) et l'extraire dans un dossier à soi avant de lancer `installer.sh` |
| L-23 | **Unité `deploy/miyucloud/miyucloud.service`** décrit une autre disposition, non utilisée | ne pas l'employer |
| L-24 | **Matrice EX** datée du 2026-10-24, antérieure aux lots 3 à 7 | ne pas lire ses statuts comme l'état de la beta |

## 3. Amendements en attente et cas dépendants

<!-- @id recette.limites.amendements -->
<!-- @role pending-amendments -->
<!-- @do map_amendments_to_cases -->

| Amendement | Objet (court) | Cas touchés |
|---|---|---|
| A-010 | notes écrites par MAIA : strates N2 / N3 seulement | RB-032 |
| A-011 | clé racine `MYRC`, état fédéral, façade, chemin TAMR, routes mémoire, clés épinglées | RB-029 à RB-032 |
| A-012 | clé d'appareil par COG, révocations relayées, fichiers `inconnu`, sas Landlock, modèles | RB-045 |
| A-013 | `MIYUCENC`, clé d'approbation TAMR distincte, provenance `Inconnue`, liaison v2 | RB-014, RB-015, RB-031, RB-063 |
| A-014 | enveloppe signée de la synchro des révocations | aucun (Origin hors périmètre) |
| A-015 | étiquette `publiable`, relais NFLOG, éditeur de révocations | RB-035, RB-036, RB-053 à RB-055 |

- Si le propriétaire **refuse** un point : les cas listés passent « sans objet » au PV et le mainteneur retire le code correspondant
- A-004 à A-009 ne figurent pas ici : leur statut de validation n'est pas signalé « en attente » dans le registre du gel

## 4. Ce que la beta ne prouve pas

<!-- @id recette.limites.non-prouve -->
<!-- @role non-claims -->
<!-- @do state_non_claims -->

- La **sécurité du COG** face à un adversaire réel (aucun audit externe, aucun test d'intrusion hors banc local)
- La **fédération**, le **canal officiel**, les **services hybrides** et l'**IA fédérée** de bout en bout
- Le **comportement sous charge** et les **performances** (aucune mesure de référence)
- L'**utilisation avec un modèle LLM** : le pipeline LLM n'est exercé que par l'outil d'essai `proposer_essai`
- Le **matériel physique** : seules les VM sont recettées
