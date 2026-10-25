# Lot 2 — Identité signée : Passeports, Visas et paquets identifiables

> **Gelé v1.0 — 2026-10-24** · amendé **A-002** (v1.2 : Sceau v1.1) **A-003** (v1.3 : choix d'implémentation du lot 2) **A-004** (v1.4 : type `0x08`, natures de certificat, indice d'usage 4 des arrêtés) et **A-006** (v1.6 : registre et canal officiel du lot 4, §3, §4.1, §8, §8.3, §9). Toute modification passe par un amendement : [règles du gel](../architecture/GEL-DOC-CONCEPTUELLE-v1.md).

<!-- @id securite.lot2.identite-signee -->
<!-- @role architecture-specification -->
<!-- @layer 5-6 -->
<!-- @human Sceau v1.1, racine COG, passeports, visas, paquets identifiables, registre Merkle, fédération -->
<!-- @do specify_signed_identity -->

> Statut : **validé** le 2026-10-24 (décisions en section 12). Fait suite à
> [AUDIT-2026-10-23-auth-wan.md](AUDIT-2026-10-23-auth-wan.md) (constats C3, C8, H7).

## 1. Objectif

Donner à chaque **personne-appareil**, chaque **COG** et chaque **paquet** une identité
cryptographique **unique, infalsifiable, vérifiable hors ligne et traçable** — l'esprit d'une NFT
(unicité, provenance, historique) **sans blockchain** : pas de réseau public, pas de frais, pas de
fuite de métadonnées, révocation immédiate.

Non-objectifs de ce lot : chiffrement des canaux (lot 3, Noise), cloisonnement OS (lot 4).

## 2. Principes

1. **Une seule enveloppe signée pour tout** (passeport, visa, manifeste de paquet, entrée de
   registre) : un format, une bibliothèque, un seul endroit à auditer.
2. **Format maison, canonique et minimal** (décision 1) : Ed25519 sur un encodage binaire à
   longueurs préfixées, une seule représentation valide par titre (spécification en section 4).
   Aucune dépendance de sérialisation ; tout écart au format est un refus.
3. **Séparation de domaine** : chaque type de titre a un code `type` distinct, inclus dans les
   octets signés avec l'étiquette `miyukini/sceau/v1.1`, la version et l'algorithme. Une signature
   de paquet ne peut jamais être rejouée comme passeport, ni relue sous un autre algorithme.
4. **Vérification stricte** : `ed25519_dalek::VerifyingKey::verify_strict`, horloge avec tolérance
   bornée, tout champ inconnu critique → refus.
5. **Fail-closed partout** : absence de signature, de clé de confiance ou d'horloge = refus.
6. **Clés privées jamais exportées** : générées sur place, stockées chiffrées, effacées de la
   mémoire après usage (`zeroize`).

## 3. Hiérarchie de clés

```
Racine COG (hors ligne)           ── identité souveraine du « pays »
  │  cog_id = base32(SHA-256(clé publique racine))[0..26]
  ├── Clé d'émission COG (en ligne, rotation 90 j)   signe passeports + visas
  ├── Clé de journal COG (en ligne)                  signe les têtes du registre
  └── Certificats éditeur (si le COG publie des paquets)

Clé d'appareil (une par appareil)  ── porte le passeport, prouve la possession
Clé éditeur (développeur)          ── signe les manifestes de paquets
Racine Origin                      ── certifie les éditeurs du Market officiel
Clé de journal d'Origin (en ligne) ── signe les têtes du registre officiel (A-006)
```

Amendé A-006 : la **clé de journal d'Origin** est dédiée (distincte de la clé de signature d'Origin et de la
clé éditeur) ; ce n'est **pas** un usage `miyuracine` (Origin n'est pas une racine COG) : elle est épinglée
dans la **configuration** du canal officiel de la zone COG (`ConfigCanal`), avec Origin et l'éditeur.

| Clé | Génération | Stockage | Révocation |
|---|---|---|---|
| Racine COG | Assistant MiyukiniAdmin au premier démarrage | Hors ligne : fichier chiffré Argon2id exporté sur support amovible + phrase de récupération ; jamais sur le serveur en régime normal | Rotation = nouvelle racine signée par l'ancienne (« passation ») |
| Émission / journal | Signées par la racine, validité 90 j | Disque chiffré, `0600`, utilisateur dédié ; TPM quand disponible (lot 4) | Liste de révocation dans le registre |
| Appareil | Sur l'appareil, à l'appairage | Android Keystore / trousseau OS ; à défaut fichier chiffré Argon2id (`miyucloud::crypto::keys`) | Entrée de révocation du passeport |
| Éditeur | Poste du développeur | Hors dépôt, idéalement clé matérielle | Révocation du certificat éditeur |

### 3.1 Racine, dérivation et passation — amendement A-003

<!-- @id securite.lot2.racine-derivation -->
<!-- @role key-derivation -->
<!-- @do derive_keys_per_usage -->

Implémenté par `crates/miyuracine` (7e6719ca).

- **Secret maître** : 256 bits (`OsRng`), jamais écrit en clair
- **Sauvegarde SLIP-39** : un groupe, **2 parts sur 3**, exposant d'itération 5 ; une part seule, corrompue ou d'un autre partage = refus
- **`cog_id`** : `base32(SHA-256(clé publique d'identité))`, alphabet RFC 4648 **minuscule**, tronqué à **26 caractères** (130 bits)
- **Chemins SLIP-10** (Ed25519, dérivation durcie), but `1296256839'` (`0x4D434F47`, « MCOG ») :

| Usage | Indice `u` | Chemin | Signe | Rotation |
|---|---|---|---|---|
| `identite` | 0 | `m/1296256839'/0'` | `cog_id`, passation, reconnaissances, certificats | nouvelle racine (passation) |
| `emission` | 1 | `m/1296256839'/1'/g'` | passeports **et** visas | 90 j |
| `journal` | 2 | `m/1296256839'/2'/g'` | têtes du registre (§8) | 90 j |
| `editeur` | 3 | `m/1296256839'/3'/g'` | certificats éditeur, paquets publiés par le COG | par génération |
| `arretes` (A-004) | 4 | `m/1296256839'/4'/g'` | arrêtés de frontière (BorderGuard) | 90 j |

- `g` = génération (< 2³¹), incrémentée à chaque rotation de la clé d'usage
- Indice 4 attribué aux **arrêtés** par A-004 (format : [zones §10](../architecture/MODELE-ZONES-ET-COUCHES-COG.md)) ; indices ≥ 5 **réservés** ; un indice n'est **jamais réattribué**
- **Passation** (rotation de la racine) : `Sceau` de type `0x06` (certificat) dont le premier champ `nature` (tag 1) vaut `0x01` ; champs `ancienne_racine` (2), `nouvelle_racine` (3), `emise_le` (4) ; signée par l'**ancienne et la nouvelle** clé d'identité, chacune vérifiée
- **Non implémenté, à décider ultérieurement** (A-003) : fichier racine chiffré Argon2id (tableau ci-dessus) ; certificat racine → clé d'émission / journal (chaîne vérifiable)

## 4. Enveloppe commune : le « Sceau »

Nouvelle crate **`crates/miyusceau`** (strate toolkit, aucune dépendance réseau) :

### 4.1 Format binaire « Sceau v1.1 » (big-endian) — amendement A-002

```
magic       4 o   "MSCE"
version     1 o   0x02 (Sceau v1.1 ; 0x01 = Sceau v1.0, abandonné, refusé)
alg         1 o   algorithme de signature du sceau (table ci-dessous)
type        1 o   0x01 passeport | 0x02 visa | 0x03 manifeste paquet | 0x04 entrée registre
                  0x05 tête registre | 0x06 certificat (émission, éditeur) | 0x07 reconnaissance
                  0x08 déclaration de voie (A-004, Portail)
                  0x04 : nature 1 publication · 2 révocation · 3 émission · 4 décret · 5 événement (A-006, §8.3)
len         4 o   longueur de la charge utile (≤ 65 536)
payload   len o   champs TLV canoniques (4.2)
nombre      1 o   nombre de signatures, 1 ≤ n ≤ 8
n × signature :
  kid      32 o   SHA-256(clé publique du signataire)
  alg       1 o   égal à l'`alg` de l'entête
  sig_len   2 o   longueur exacte imposée par l'algorithme
  sig  sig_len o  signature
```

| `alg` | Algorithme | `sig_len` | Statut |
|---|---|---|---|
| `0x01` | Ed25519, `verify_strict` | 64 | implémenté |
| `0x02` | ML-DSA-65 (FIPS 204) | 3 309 | **réservé, refusé** à la vérification |
| `0x03` | hybride Ed25519 ‖ ML-DSA-65 (les deux valides) | 3 373 | **réservé, refusé** à la vérification |
| autre | — | — | refusé au décodage |

Compléments A-004 :

- **Type `0x08` « déclaration de voie »** : même disposition, même domaine signé ; décodé **uniquement** par `miyuportail` (le décodeur générique `miyusceau` le refuse, type inconnu) ; charge TLV et politique de signatures en [services hybrides §3.1](../architecture/SERVICES-HYBRIDES-ET-PORTAIL.md)
- **Natures de certificat** (`0x06`, premier champ `nature`, tag 1) :

| `nature` | Objet | Signé par |
|---|---|---|
| `0x01` | passation de racine (§3.1) | ancienne **et** nouvelle identité |
| `0x41` (« A ») | arrêté de frontière | clé d'usage `arretes` (indice 4) |
| autre | réservé | refusé par les décodeurs |

Octets signés (identiques pour tous les signataires) :
`"miyukini/sceau/v1.1" || 0x00 || version || alg || type || len || payload`.

- Séparation de domaine : version et algorithme sont signés → une signature ne peut pas être
  réinterprétée sous une autre version ni un autre algorithme
- Le `kid` n'est pas dans les octets signés : Ed25519 (et ML-DSA) lient déjà la clé publique au
  défi ; une signature déplacée sous un autre `kid` ne vérifie pas
- Identité d'un titre = SHA-256 des octets signés (le bloc de signatures peut varier : ajout
  d'un cosignataire)

Décodage strict (tout écart = refus) :

- Taille totale exacte, aucun octet après la dernière signature
- Taille maximale : 92 812 o (charge maximale + 8 signatures de l'algorithme le plus long)
- `alg` connu ; `nombre` entre 1 et 8 ; `alg` de chaque entrée = `alg` de l'entête ;
  `sig_len` = longueur imposée
- `kid` strictement croissants : ni désordre ni doublon (un signataire compte une seule fois)

Vérification à seuil (politique = seuil `m ≥ 1` + jeu d'ancres) :

1. Format strict, type attendu
2. Algorithme implémenté (sinon refus, même si une autre entrée serait valable)
3. Chaque entrée dont le `kid` est épinglé : `verify_strict` ; **une seule signature invalide
   d'un signataire épinglé = refus du sceau entier** (pas d'entrée ignorée)
4. Entrées non épinglées : bien formées, non comptées
5. Valide si au moins `m` signataires épinglés distincts ont signé ; aucun épinglé → « signataire
   inconnu » ; sinon « seuil non atteint »
6. Niveau de confiance = **le plus faible** des signataires comptés (`Reconnu` < `Souverain`)
7. Puis décodage canonique de la charge et fenêtre de validité

Signature simple = seuil 1. EX-01 (mise à jour des Cores) = seuil 2 sur le jeu {Origin, éditeur}.
Amendé A-006 : ce seuil 2 porte sur l'**annonce** (entrée `0x04` de nature publication), pas sur la tête
du registre officiel (clé de journal d'Origin, seuil 1, §8.1).

### 4.2 Charge utile TLV canonique

Suite de champs `tag (1 o) || longueur (4 o) || valeur`.

- Tags **strictement croissants** (ni doublon ni désordre) → une seule encodage possible.
- Tag inconnu pour le type → **refus** (pas de champ ignoré silencieusement en v1).
- Champs obligatoires du type tous présents.
- Types de valeur : entier `u64` (8 o exactement), octets bruts, texte UTF-8 valide,
  clé publique (32 o), empreinte (32 o), liste = valeur concaténée de sous-TLV canoniques.
- Horodatages : secondes Unix `u64`.

### 4.3 API

API :

- `sceller<T: Titre>(titre, &CleSignature) -> Sceau` (un signataire)
- `sceller_multi<T: Titre>(titre, &[&CleSignature]) -> Sceau` (plusieurs signataires)
- `cosigner<T: Titre>(&Sceau, &CleSignature) -> Sceau` (ajoute une signature sur les mêmes octets signés)
- `verifier<T: Titre>(&Sceau, &AncresDeConfiance, maintenant) -> Result<Verifie<T>, RefusSceau>` (seuil 1)
- `verifier_seuil<T: Titre>(&Sceau, &AncresDeConfiance, m, maintenant)` (seuil `m`)
- `Verifie<T>` : seul moyen d'obtenir un titre exploitable (constructeur privé, déjà introduit
  dans `miyauth` au lot 1).

`AncresDeConfiance` = clés racine épinglées (son propre COG, COG reconnus, Origin), chacune avec
son niveau BorderGuard (`Trusted` / `Verified`).

## 5. Passeport (identité personne + appareil)

Émis par la clé d'émission du COG d'origine.

| Champ | Rôle |
|---|---|
| `serial` | 128 bits aléatoires, unique — c'est « l'identifiant NFT » du passeport |
| `issuer` | `cog_id` émetteur + `kid` de la clé d'émission |
| `subject` | identifiant stable de la personne dans le COG (pas l'email) |
| `device_pk` | clé publique de l'appareil titulaire |
| `role` | `Citizen` |
| `scopes` | droits de base (ex. `services:*:read`) |
| `nbf` / `exp` | validité (proposé : 30 j, renouvellement silencieux) |

Un passeport sans preuve de possession ne vaut rien : voler le fichier ne suffit pas.

**Implémentation (A-003)** — `crates/miyauth` (4277203c, e34f1f08), `Sceau` type `0x01`, TLV :

| Tag | Champ | Valeur |
|---|---|---|
| 1 | `serie` | 16 o aléatoires |
| 2 | `emetteur_cog` | `cog_id` du COG d'origine |
| 3 | `emetteur_kid` | `kid` de la clé d'émission |
| 4 | `sujet` | identifiant par paire pour `destination` (32 o) |
| 5 | `destination` | `cog_id` du COG visité (le COG d'origine pour un usage local) |
| 6 | `cle_appareil` | clé publique Ed25519 de l'appareil (`device_pk`) |
| 7 | `role` | `1` = `Citizen` (seule valeur acceptée) |
| 8 | `portees` | ≤ 32 portées, triées, sans doublon, ≤ 128 caractères |
| 9 / 10 | `pas_avant` / `pas_apres` | durée ≤ **30 j** |

- **Un passeport par COG visité** : `destination` = COG local, sinon refus (EX-36)
- **Identifiant par paire** : `HKDF-SHA256(sel = "miyukini/miyauth/pairwise/v1", ikm = secret du titulaire (32 o, détenu par le COG d'origine), info = "cog:" || cog_id visité)`, 32 o
- **Signataire** : exactement un signataire compté, égal à `emetteur_kid`, épinglé sous le `cog_id` émetteur ; niveau `Souverain` ⇔ COG local
- **À décider ultérieurement** : une clé d'appareil par COG visité — aujourd'hui `cle_appareil` est commune à tous les passeports d'un appareil, ce qui permet de relier deux COG visités

## 6. Visa (droit d'entrée ciblé)

Émis par le COG **hôte** (le sien ou un COG étranger reconnu), sur présentation d'un passeport.

| Champ | Rôle |
|---|---|
| `serial` | unique |
| `passport` | SHA-256 du passeport présenté (lien indissociable) |
| `audience` | service visé (`jaykoa`, `miyucloud`…) |
| `scopes` | droits restreints à ce service |
| `nbf` / `exp` | court (proposé : 15 min) |
| `role` | `Visitor` si COG étranger, `Citizen` si COG d'origine |

Remplace à terme les JWT HS256 et jetons au porteur de CentralRemote, Alicia API, MiyuCloud.

**Implémentation (A-003)** — `Sceau` type `0x02`, TLV : `serie` (1), `passeport` (2, SHA-256 des octets signés du passeport), `emetteur_cog` (3, COG hôte), `sujet` (4) et `cle_appareil` (5) **recopiés du passeport**, `audience` (6, `[a-z0-9._-]{1,64}`), `role` (7, `1` Citizen / `2` Visitor), `portees` (8), `pas_avant` / `pas_apres` (9 / 10).

- Durée ≤ **15 min**, contenue dans la fenêtre du passeport
- Émis après preuve de possession (§7), signé par une ancre **souveraine du COG hôte** ; exactement un signataire compté
- Liens contrôlés séparément : empreinte du passeport, `sujet`, `cle_appareil`, rôle, audience attendue
- **Portées du visa ⊆ portées du passeport**, à l'émission et à la vérification ; comparaison exacte, **aucun joker**

## 7. Preuve de possession

À chaque ouverture de session, le service envoie un défi ; l'appareil signe avec `device_pk` :

```
signature = Ed25519(device_sk, "miyukini/pop/v1" || defi(32 o) || audience || liaison_canal)
```

`liaison_canal` = exporteur TLS ou hash de transcription Noise (lot 3) : la signature ne peut pas
être relayée vers un autre canal (anti-MITM, anti-rejeu).

**Implémentation (A-003)**

- `defi` et `liaison_canal` de **32 o** chacun (longueurs fixes : l'audience, au milieu, est non ambiguë) ; audience au format du visa
- Un rôle n'est accordé que sur `AccesProuve` = visa vérifié **et** possession prouvée pour la session : un visa intercepté seul n'accorde rien (`identity_resolve`)
- Identifiant opaque exposé aux services : `pw:<cog d'origine>:<hex(sujet)>` (deux COG ne produisent jamais le même) ; référence de session client jamais reprise telle quelle : `sess:` + SHA-256
- `attest` reste **refusé** : aucun format d'attestation défini (à décider ultérieurement)

## 8. Registre public chaîné (révocations + publications)

Un journal **append-only** par COG, et un pour le Market Origin, au format **arbre de Merkle**
(modèle Certificate Transparency, RFC 9162) :

- **Entrées** : `Publication(paquet)`, `Revocation(serial | package_id | kid)`, `Emission(cert éditeur)` ;
  amendé A-006 : `Decret`, `Evenement`, révocation d'une annonce ; encodage au §8.3.
- **Tête signée** : `{taille, racine Merkle, horodatage}` signée par la clé de journal (registre
  officiel : clé de journal **d'Origin**, A-006).
- **Preuve d'inclusion** : livrée avec chaque paquet ; Central la vérifie hors ligne.
- **Preuve de cohérence** : Central mémorise la dernière tête vue et refuse une tête qui
  réécrit l'histoire → un Origin compromis ne peut pas substituer une version déjà publiée
  sans que ce soit détecté.

Remplace la chaîne d'audit actuelle de `miyukini-connect` (SHA-256 sans clé, en mémoire).

### 8.1 Tête, fraîcheur et monotonie — amendement A-003

<!-- @id securite.lot2.registre-tete -->
<!-- @role registry-head -->
<!-- @do verify_registry_head -->

Implémenté par `crates/miyusceau` (`registre.rs`, `merkle.rs` ; 7ed62645, 6774aaa6).

**Tête** = `Sceau` type `0x05`, TLV (tous obligatoires) :

| Tag | Champ | Valeur |
|---|---|---|
| 1 | `registre` | identifiant du registre (32 o) |
| 2 | `taille` | nombre de feuilles (`u64`) |
| 3 | `racine` | racine Merkle ; `SHA-256("")` si `taille = 0` |
| 4 | `sequence` | numéro de séquence (`u64`), croissant |
| 5 | `emise_a` | horodatage d'émission (secondes Unix) |
| 6 | `validite_max_secs` | validité annoncée, 1 s à 7 j |

- **Fraîcheur** (EX-03) : âge accepté = `min(validite_max_secs, âge max de la politique)` ; défaut **24 h** (`AGE_MAX_TETE_DEFAUT_SECS`), plafond **7 j** ; tolérance d'horloge **120 s** ; tête trop ancienne = « registre gelé », refus
- **Politiques** : `journal_cog` = 1 signataire (clé de journal) ; `canal_officiel` (Market Origin, mises à jour) = **2 sur 2** ; politique explicite : seuil 1 à 8
  - **Amendé A-006** (décision du propriétaire) : les têtes du **registre officiel** sont signées par la **clé de journal d'Origin**, **seuil 1** (âge 24 h) ; le 2 sur 2 des mises à jour est porté par l'annonce (§8.3) ; politique plus stricte admise seulement en **N sur N** d'un ensemble contenant la clé de journal d'Origin (par exemple `canal_officiel` sur {journal d'Origin, Origin})
- **Registre attendu** : toute tête est vérifiée contre l'identifiant de registre attendu (`RegistreInattendu` sinon)
- **Monotonie** (dernière tête acceptée) :
  - séquence en recul ou horodatage en recul ⇒ retour en arrière, refus
  - même séquence et contenu différent, taille en recul, ou même taille et racine différente ⇒ **équivoque** ; preuve conservée = les deux sceaux signés
  - taille en hausse depuis un état non vide ⇒ **preuve de cohérence exigée**, liée aux tailles des deux têtes
  - preuve d'inclusion vérifiée seulement contre une tête **fraîche**, de même taille
- **Encodage des preuves** (big-endian, taille exacte, chemin ≤ 64 empreintes) :
  - inclusion : `index (8) | taille (8) | nombre (1) | nombre × empreinte (32)`
  - cohérence : `taille1 (8) | taille2 (8) | nombre (1) | nombre × empreinte (32)`
- **État persistant** : `"MSRG" | version 0x01 | longueur (4) | sceau de la dernière tête` ; au chargement, signatures revérifiées et **registre attendu exigé** (un état d'un autre registre est refusé)
- **Fuzzing** (EX-10) : `crates/miyusceau/fuzz`, 4 cibles (sceau, titres, preuves Merkle, vérification à seuil)

### 8.2 Révocation — amendement A-003

- Cibles : série d'un titre, ou clé (appareil, émission) ; vérification passeport et visa contre série, appareil et clé d'émission
- Fail-closed : liste absente, **plus ancienne que 15 min** (défaut, durée d'un visa) ou datée dans le futur au-delà de la tolérance d'horloge ⇒ refus
- Interface indépendante du stockage (`SourceRevocation`) ; adaptateur ← tête de registre vérifiée : reste à faire (plan, lot 2)
- **Canal officiel (amendé A-006)** : révocation = entrée `0x04` de nature 2, signée par **Origin ou l'éditeur** (**seuil 1** sur l'ensemble exact {Origin, éditeur} ; la clé de journal d'Origin n'est pas comptée) ; cibles série, **annonce** (identité du sceau), **artefact** (SHA-256), **clé** (tout ce qu'elle a signé) ; une révocation ne peut qu'**empêcher** une installation ; vue par le miroir intégral du registre officiel

### 8.3 Entrées, stockage et canal officiel — amendement A-006

<!-- @id securite.lot2.registre-entrees -->
<!-- @role registry-entries -->
<!-- @do encode_registry_entries -->

Implémenté par `crates/miyuregistre` et `crates/miyucanal` (lot 4).

**Entrée** = `Sceau` type `0x04`, TLV (tags strictement croissants, tous obligatoires pour la nature, aucun en trop) :

| Tag | Champ | Natures | Valeur |
|---|---|---|---|
| 1 | `nature` | toutes | `u64` : 1 publication, 2 révocation, 3 émission, 4 décret, 5 événement |
| 2 | `inscrite_a` | toutes | secondes Unix |
| 3 | `categorie` | publication | `u64` : 1 Cores, 2 OS, 3 modèle IA |
| 4 | `composant` | publication | texte `[a-z0-9._:-]`, 1..64 o |
| 5 | `version` | publication | `u64`, strictement croissante par composant |
| 6 | `empreinte` | publication | SHA-256 de l'artefact (32 o) |
| 7 | `taille` | publication | taille exacte de l'artefact (`u64`, ≥ 1) |
| 3 | `cible_nature` | révocation | `u64` : 1 série, 2 annonce, 3 artefact, 4 clé |
| 4 | `cible` | révocation | 32 o |
| 5 | `motif` | révocation | texte sans caractère de contrôle, 1..256 o |
| 3 | `certificat` | émission | identité du certificat éditeur (32 o) |
| 3 | `objet` | décret, événement | texte `[a-z0-9._:-]`, 1..64 o |
| 4 | `empreinte` | décret, événement | 32 o |

- **Feuille** = `SHA-256(0x00 || octets du sceau)` ; **identité** d'une annonce = SHA-256 des octets signés
- **Annonce** = entrée de nature publication, **2 sur 2** sur l'ensemble exact {Origin, éditeur} ; inscription datée de plus de 120 s dans le futur = refus
- **Stockage** `MRGJ` : `"MRGJ" | 0x01` puis `longueur (4) | octets | contrôle (8)`, contrôle = `SHA-256(longueur || octets)[0..8]`, ≤ 1 Mio par enregistrement ; `fsync` à chaque ajout ; fin tronquée retirée à l'ouverture, corruption au milieu refusée
- **Median-Time-Past** : 11 dernières têtes, médiane haute si la fenêtre est paire, égal accepté ; un MTP par registre + horloge de confiance globale ([zones §17](../architecture/MODELE-ZONES-ET-COUCHES-COG.md)) ; état `MSMT`
- **Miroir** du registre officiel : copie intégrale, racine reconstruite, cohérence recalculée ; ≤ 4 096 entrées nouvelles par relevé
- **Registre fédéral** (EX-34) : tête d'un COG = 2 sur {clé de journal du COG, Origin}, Origin cosignant le même sceau ; amorçage au premier usage
- Relevé, quarantaine et persistance : [zones §7](../architecture/MODELE-ZONES-ET-COUCHES-COG.md) ; détail et points ouverts : [A-006](../architecture/GEL-DOC-CONCEPTUELLE-v1.md)

## 9. Paquets identifiables (Market)

### 9.1 Identité d'un paquet

```
package_id = "msp:sha256:" + hex(SHA-256(octets du .msp))
```

Deux paquets différents ne peuvent pas avoir le même identifiant ; un octet modifié change
l'identifiant.

### 9.2 Manifeste signé (`Sceau<ManifestePaquet>`)

Reprend `miyumarket::ServiceManifest` et ajoute :

| Champ | Rôle |
|---|---|
| `package_id` | empreinte ci-dessus |
| `publisher` | `kid` de la clé éditeur + certificat éditeur (signé par Origin ou un COG) |
| `permissions` | permissions demandées (déjà dans le manifeste) — couvertes par la signature |
| `files` | liste `(chemin relatif, SHA-256)` de chaque fichier de l'archive |
| `published_at` | horodatage |

### 9.3 Vérification par Central avant extraction

1. `Sceau` du manifeste valide, `verify_strict`.
2. Certificat éditeur valide et chaîné à une ancre de confiance (Origin ou COG reconnu), non révoqué.
3. `SHA-256(.msp) == package_id`.
4. Preuve d'inclusion dans le registre + cohérence avec la dernière tête connue.
5. `manifest.id` == service demandé, version strictement supérieure (anti-retour en arrière).
6. Extraction (règles Zip Slip du lot 1), puis contrôle `files` fichier par fichier.
7. Enregistrement local de `package_id` → **revérification périodique** des binaires installés
   (détection de modification sur disque).

Le point d'extension `verify_package_signature` posé au lot 1 devient bloquant.

Amendé A-006 — mises à jour du **canal officiel** (Cores, OS, modèles IA) : l'annonce 2 sur 2 (§8.3) tient lieu
de manifeste ; inclusion vérifiée contre une tête fraîche du miroir du registre officiel ; quarantaine de 48 h ;
artefact relevé par empreinte, taille et SHA-256 exacts ; version strictement supérieure à l'installée ; remise
au chemin d'installation existant (Central, StrongFather).

## 10. Fédération (« diplomatie »)

- `cog_id` = empreinte de la clé racine (fin du « premier arrivé » sur Origin Relay, constat H7).
- **Reconnaissance** entre deux COG = chacun signe la racine de l'autre (`Sceau<Reconnaissance>`
  avec niveau et périmètre). BorderGuard : racine propre → `Trusted`, racine reconnue →
  `Verified`, sinon `Unknown`.
- `REGISTER` sur Origin Relay = signature d'un défi serveur par la racine (ou clé d'émission) du
  COG ; permis délivré = `Sceau` lié au `cog_id`.
- Liste des trackers signée par la racine Origin (remplace `tracker_signature` à zéro).

## 11. Intégration dans le code existant

| Composant | Changement |
|---|---|
| `crates/miyusceau` (nouvelle) | Format Sceau v1.1, ancres, `Verifie<T>`, Merkle log, preuves |
| `crates/miyuracine` (nouvelle, A-003) | Racine hors ligne, SLIP-39, SLIP-10 par usage, passation, CLI de cérémonie (§3.1) |
| `crates/miyauth` | `verify`/`resolve`/`attest` s'appuient sur `miyusceau` (fin du stub fail-closed du lot 1) |
| `crates/miyukini-admin` | Création de la racine COG, émission des clés d'émission/journal |
| `crates/borderguard` | Niveaux de confiance dérivés des ancres |
| `crates/miyumarket` | `ManifestePaquet`, `package_id`, outil CLI de signature éditeur |
| `apps/origin` | Registre Merkle du Market, certificats éditeurs, Relay `REGISTER` signé |
| `apps/central` | Vérification 9.3, mémoire des têtes, revérification périodique |
| `crates/central-remote` + mobile | Appairage = émission d'un passeport pour la clé d'appareil (QR) ; fin des `device_id` au porteur |
| `crates/miyuwebway_*` | Déclarations = `Sceau<Declaration>` |
| `crates/miyukini-connect` | Audit → entrées du registre |

## 12. Décisions (validées le 2026-10-24)

1. **Format** : maison « Sceau v1 » (section 4), sans dépendance de sérialisation ; **v1.1**
   (algorithme + multi-signature) depuis l'amendement A-002.
2. **Racine COG** : hors ligne sur support amovible ; sauvegarde **SLIP-39 2 sur 3** ; clés
   d'émission/journal **dérivées en SLIP-10** (durcie), 90 j, seules présentes sur le serveur ;
   chemins et passation en §3.1 (A-003).
3. **Durées** : passeport 30 j, visa 15 min, clés d'émission 90 j.
4. **Market** : paquets **officiels uniquement** dans ce lot — un seul éditeur (Studio Miyukini),
   certificat éditeur signé par la racine Origin. Les éditeurs tiers sont reportés.
5. **Registre** : arbre de Merkle (RFC 9162 : feuilles `SHA-256(0x00 || entrée)`, nœuds
   `SHA-256(0x01 || g || d)`).
6. **Crate** : `miyusceau`.

## 13. Plan de mise en œuvre (incréments testables)

1. `miyusceau` : enveloppe, ancres, `Verifie<T>` + vecteurs de test (y compris signatures
   malléables rejetées par `verify_strict`).
2. Racine COG + clé d'émission dans MiyukiniAdmin ; `miyauth` branché.
3. Paquets : `package_id`, manifeste signé, CLI éditeur, vérification bloquante dans Central.
4. Registre Merkle (Origin Market puis COG) + preuves d'inclusion/cohérence.
5. Passeport d'appareil à l'appairage CentralRemote/mobile + preuve de possession.
6. Visas et remplacement progressif des JWT/jetons au porteur.
7. Fédération : `cog_id` par clé, reconnaissance croisée, Relay `REGISTER` signé, trackers signés.

Chaque incrément : tests unitaires, vecteurs de test croisés, revue sécurité avant commit.

## 14. Menaces

**Couvertes** : paquet modifié ou substitué (même par un Origin compromis, détecté par le
registre) ; passeport/visa forgé ; vol d'un fichier de passeport sans la clé d'appareil ;
usurpation de `cog_id` ; rejeu d'une preuve de possession ; retour à une version vulnérable.

**Non couvertes par ce lot** : appareil entièrement compromis (la clé d'appareil est utilisable
par le malware — atténuation : Keystore matériel, lot 4) ; vol de la racine COG si elle reste sur
le serveur (décision 2) ; canaux non chiffrés (lot 3) ; confinement des services (lot 4).
