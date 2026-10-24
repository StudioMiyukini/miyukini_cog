# Lot 2 — Identité signée : Passeports, Visas et paquets identifiables

> **Gelé v1.0 — 2026-10-24.** Toute modification passe par un amendement : [règles du gel](../architecture/GEL-DOC-CONCEPTUELLE-v1.md).

<!-- @id securite.lot2.identite-signee -->
<!-- @role architecture-specification -->
<!-- @layer 5-6 -->
<!-- @human Sceau v1, racine COG, passeports, visas, paquets identifiables, registre Merkle, fédération -->
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
   octets signés avec l'étiquette `miyukini/sceau/v1`. Une signature de paquet ne peut jamais être
   rejouée comme passeport.
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
```

| Clé | Génération | Stockage | Révocation |
|---|---|---|---|
| Racine COG | Assistant MiyukiniAdmin au premier démarrage | Hors ligne : fichier chiffré Argon2id exporté sur support amovible + phrase de récupération ; jamais sur le serveur en régime normal | Rotation = nouvelle racine signée par l'ancienne (« passation ») |
| Émission / journal | Signées par la racine, validité 90 j | Disque chiffré, `0600`, utilisateur dédié ; TPM quand disponible (lot 4) | Liste de révocation dans le registre |
| Appareil | Sur l'appareil, à l'appairage | Android Keystore / trousseau OS ; à défaut fichier chiffré Argon2id (`miyucloud::crypto::keys`) | Entrée de révocation du passeport |
| Éditeur | Poste du développeur | Hors dépôt, idéalement clé matérielle | Révocation du certificat éditeur |

## 4. Enveloppe commune : le « Sceau »

Nouvelle crate **`crates/miyusceau`** (strate toolkit, aucune dépendance réseau) :

### 4.1 Format binaire « Sceau v1 » (big-endian)

```
magic      4 o   "MSCE"
version    1 o   0x01
type       1 o   0x01 passeport | 0x02 visa | 0x03 manifeste paquet | 0x04 entrée registre
                 0x05 tête registre | 0x06 certificat (émission, éditeur) | 0x07 reconnaissance
kid       32 o   SHA-256(clé publique Ed25519 du signataire)
len        4 o   longueur de la charge utile (≤ 65 536)
payload  len o   champs TLV canoniques (4.2)
signature 64 o   Ed25519
```

Octets signés :
`"miyukini/sceau/v1" || 0x00 || version || type || kid || len || payload`.
Aucun octet après la signature ; taille totale exacte ; sinon refus.

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

- `sceller<T: Titre>(titre, &CleSignature) -> Sceau`
- `verifier<T: Titre>(&Sceau, &AncresDeConfiance, maintenant) -> Result<Verifie<T>, RefusSceau>`
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

## 7. Preuve de possession

À chaque ouverture de session, le service envoie un défi ; l'appareil signe avec `device_pk` :

```
signature = Ed25519(device_sk, "miyukini/pop/v1" || defi(32 o) || audience || liaison_canal)
```

`liaison_canal` = exporteur TLS ou hash de transcription Noise (lot 3) : la signature ne peut pas
être relayée vers un autre canal (anti-MITM, anti-rejeu).

## 8. Registre public chaîné (révocations + publications)

Un journal **append-only** par COG, et un pour le Market Origin, au format **arbre de Merkle**
(modèle Certificate Transparency, RFC 9162) :

- **Entrées** : `Publication(paquet)`, `Revocation(serial | package_id | kid)`, `Emission(cert éditeur)`.
- **Tête signée** : `{taille, racine Merkle, horodatage}` signée par la clé de journal.
- **Preuve d'inclusion** : livrée avec chaque paquet ; Central la vérifie hors ligne.
- **Preuve de cohérence** : Central mémorise la dernière tête vue et refuse une tête qui
  réécrit l'histoire → un Origin compromis ne peut pas substituer une version déjà publiée
  sans que ce soit détecté.

Remplace la chaîne d'audit actuelle de `miyukini-connect` (SHA-256 sans clé, en mémoire).

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
| `crates/miyusceau` (nouvelle) | Format Sceau v1, ancres, `Verifie<T>`, Merkle log, preuves |
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

1. **Format** : maison « Sceau v1 » (section 4), sans dépendance de sérialisation.
2. **Racine COG** : hors ligne sur support amovible ; sauvegarde **SLIP-39 2 sur 3** ; clés
   d'émission/journal **dérivées en SLIP-10** (durcie), 90 j, seules présentes sur le serveur.
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
