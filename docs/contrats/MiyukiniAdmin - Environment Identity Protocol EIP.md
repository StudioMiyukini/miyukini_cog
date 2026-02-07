# MiyukiniAdmin — Environment Identity Protocol (EIP)

## 1. Contexte

Le **Environment Identity Protocol (EIP)** definit comment les **donnees d'identite de l'environnement** (COG) sont produites par les **Cores**, chiffrees et persistees. Ces donnees sont critiques pour determiner si l'environnement est vierge ou initialise et pour attester l'identite du COG (ex. LSI, version des cores, integrite).

**Principe fondamental :**

> **Les donnees d'identite du COG sont produites par les Cores, jamais par un seul acteur ; elles sont toujours stockees chiffrees.**

**Portee :** Premier boot, bootstrap, et toute regeneration d'identite gouvernée.

**References :**
- [MiyukiniAdmin - Auth and First-Boot Contract](../core/MiyukiniAdmin/contracts/security/MiyukiniAdmin%20-%20Auth%20and%20First-Boot%20Contract.md)
- [Miyukini Conceptual References - Glossaire](../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) (Environnement, LSI, COG)

---

## 2. Portee / Scope

Ce document definit :
- Le **contenu** des donnees d'identite environnement (champs, sens)
- Le **processus de production** (qui contribue, ordre)
- Le **chiffrement** et la protection du blob EIP
- Le **format** du blob persiste et les regles d'integrite
- Les **cas d'usage** (creation initiale, verification, pas de modification ad hoc)

Ce document **ne couvre pas** :
- L'auth des utilisateurs admin (voir Auth and First-Boot Contract)
- Les identites inter-COG (Passeport Utilisateur, Visa de Connexion)
- Le détail des algorithmes cotes (ex. librairies Rust) — on reste au niveau spec protocole et cryptographie.

---

## 3. Contenu des donnees d'identite (payload clair)

Les donnees d'identite du COG, **avant chiffrement**, sont un payload structure. Chaque Core (ou le Kernel) peut contribuer un ou plusieurs champs. Aucun champ ne doit contenir de donnees utilisateur ni de secrets long terme reutilisables ailleurs (les cles EIP sont dediees au blob).

### 3.1 Champs obligatoires

| Champ | Source | Description | Exemple |
|-------|--------|-------------|---------|
| **environment_id** | Kernel (Id) | Identifiant unique du COG (LSI ou equivalent). | UUID v7 ou format Kernel Id |
| **core_versions** | Cores / Ever Buddy | Version de chaque Core participant (nom + version). | `{"StrongFather":"1.0.0","KindMother":"1.0.0",...}` |
| **created_at** | Kernel (Clock) | Horodatage de creation du blob (trace only). | ISO 8601 |
| **integrity_hash** | Kernel ou Core dedie | Hash des elements structurels (ordre chargement, contrats) — pas de donnees metier. | SHA-256 en hex |
| **iteration** | Ever Buddy | Numero d'iteration / version de l'environnement. | Entier positif |
| **protocol_version** | — | Version du protocole EIP. | "EIP-1.0" |

### 3.2 Champs optionnels (selon Cores)

| Champ | Source | Description |
|-------|--------|-------------|
| **boundary_hint** | Border Guard | Indication de frontiere (ex. type LSI/VID/WID). |
| **security_baseline** | WorrySentinel | Niveau de securite initial (0-4). |
| **features_flags** | Config / Cores | Indicateurs de capacites actives au bootstrap. |

Le payload clair est serialize (ex. JSON ou CBOR) puis chiffre en bloc unique. La structure exacte peut etre fixee en implementation (schema JSON ou equivalent).

---

## 4. Processus de production (orchestration)

### 4.1 Acteurs

- **Kernel** : fournit `environment_id`, `created_at`, participe à `integrity_hash`.
- **Cores** (StrongFather, KindMother, CaringNanny, WorrySentinel, Border Guard, Ever Buddy, etc.) : fournissent leurs versions et champs optionnels.
- **MiyukiniAdmin** : n'a pas la main sur le contenu ; il **demande** la generation du blob EIP via BondingBrother. C'est un **orchestrateur** du flux, pas un producteur de champs.
- **KindMother** : recoit le blob **deja chiffre** et le persiste ; elle ne déchiffre pas le contenu en fonctionnement normal.

### 4.2 Sequence de generation (first-boot)

1. MiyukiniAdmin (mode installation) envoie une requete « generer identite environnement » via BondingBrother.
2. StrongFather valide que l'environnement est bien vierge et que le verrou bootstrap est actif.
3. BondingBrother relaie vers les Cores et le Kernel pour collecter les contributions :
   - Kernel : `environment_id`, `created_at`, contribution à `integrity_hash`.
   - Chaque Core : sa version (et champs optionnels).
   - Ever Buddy (ou Kernel) : assemble `core_versions`, `iteration`, `protocol_version`.
   - Calcul final de `integrity_hash` sur la structure (ordre, contrats, pas de donnees metier).
4. Un composant designe (ex. Kernel ou Core « EIP Producer ») assemble le payload clair, le serialize.
5. **Chiffrement** (voir section 5) : le payload est chiffre avec une cle dediee EIP ; le blob chiffre + métadonnées (non sensibles) sont produits.
6. Le blob chiffre est envoye à KindMother pour persistance (table ou stockage protege).
7. MiyukiniAdmin recoit un accusé de réception (succes/echec) ; en cas de succes, l'etape « identite environnement » du parcours d'installation est marquee complete.

**Regle :** Aucun acteur unique ne possede à la fois le payload clair et la cle de chiffrement long terme ; la cle peut etre derivee au moment de la creation et ne pas etre stockee en clair (voir 5.2).

---

## 5. Cryptographie

### 5.1 Objectifs

- **Confidentialite** : le contenu du blob EIP n'est pas lisible sans autorisation (cle).
- **Integrite** : toute modification du blob est detectable (AEAD ou HMAC).
- **Reproducibilite limitee** : on ne « re-chiffre » pas le meme payload avec des cles differentes en production ; une seule generation par environnement initial.

### 5.2 Cle de chiffrement

- **Option A (recommandee pour first-boot) :** Cle derivee (KDF) à partir d'un secret bootstrap :
  - Entree : secret bootstrap (genere par le Kernel au premier demarrage, fort, aleatoire).
  - KDF : ex. Argon2id ou HKDF avec sel et contexte "EIP-1.0-env-identity".
  - Sortie : cle de chiffrement (ex. 256 bits pour AES-256-GCM).
  - Le **secret bootstrap** est stocke une seule fois dans un stockage protege (ex. fichier ou slot KindMother dedie), jamais en clair en log. La cle derivee peut etre recalculée au besoin (ex. verification) puis oubliée de la memoire.

- **Option B :** Cle generee par un HSM ou module securise si disponible (hors scope spec minimal).

**Regle :** La cle (ou le secret bootstrap) ne doit jamais etre exposee hors du processus bootstrap / verification gouvernée.

### 5.3 Algorithme de chiffrement

- **Chiffrement authentifié** (AEAD) pour confidentialite + integrite.
- **Algorithme recommandé :** AES-256-GCM (IV 96 bits, aleatoire par chiffrement ; tag 128 bits).
- **Format du blob persiste (exemple) :**
  - `version_eip` : 1 octet ou string "EIP-1.0"
  - `iv` : 12 octets
  - `ciphertext` : payload clair chiffre
  - `tag` : 16 octets (GCM tag)
  - Optionnel : `key_id` ou `key_hint` si rotation future (pour EIP v2).

La concatenation `iv || ciphertext || tag` (ou equivalent) est ce que KindMother persiste. Le déchiffrement n'est autorise que dans des contextes explicites (ex. verification d'integrite par un processus admin, recovery).

### 5.4 Derivee de cle (KDF) — recommandation

- **Argon2id** (ou scrypt si Argon2 indisponible) pour le secret bootstrap → cle symetrique.
  - Parametres : memory cost, time cost, parallelism selon recommandations (ex. OWASP / NIST).
- **Contexte/sel** : unique par environnement (ex. `environment_id` une fois connu, ou un sel aleatoire stocke à cote du blob).
- **Sortie** : 32 octets (256 bits) pour AES-256.

---

## 6. Stockage et acces

### 6.1 Rôle de KindMother

- KindMother stocke le **blob chiffre** (et eventuellement les métadonnées non sensibles : `protocol_version`, `created_at` si duplique en clair pour indexation).
- KindMother **ne déchiffre pas** le blob en fonctionnement normal. Elle fournit le blob à un demandeur autorise (ex. MiyukiniAdmin en mode verification) ; le déchiffrement a lieu cote demandeur avec la cle derivee.

### 6.2 Acces au blob

- **Lecture (blob brut)** : uniquement par MiyukiniAdmin (ou processus designe) pour verification / recovery.
- **Déchiffrement** : uniquement dans un contexte explicite (verification integrite, diagnostic) avec la cle derivee ; jamais expose en API ni en log.

### 6.3 Modification

- Le blob EIP **n'est pas modifie** après creation. Toute evolution de l'identite (ex. nouvelle iteration) peut donner lieu à un **nouveau blob** dans un processus gouverné (migration, nouveau COG), pas à un patch du blob existant.

---

## 7. Verification d'integrite

- Lors du demarrage (environnement suppose initialise), MiyukiniAdmin peut demander à KindMother le blob EIP.
- Avec la cle derivee (secret bootstrap), déchiffrement puis verification :
  - **Tag AEAD** : si invalide → blob tronque ou altere → environnement **compromis** (pas vierge).
  - **integrity_hash** : coherent avec la configuration courante (ordre, contrats). Si incoherent → environnement **compromis** (alteration post-creation).
  - **protocol_version** : supporte.
- **Si incohérence ou tag invalide** : l'environnement est classe **compromis** (attaque, troncature, alteration). Il ne doit **pas** etre traite comme vierge. La **reponse securitaire** s'applique (mode degrade, blocage login normal, alerte, procedure de recovery gouvernée). Voir [MiyukiniAdmin - Auth and First-Boot Contract](../core/MiyukiniAdmin/contracts/security/MiyukiniAdmin%20-%20Auth%20and%20First-Boot%20Contract.md) section 3.5.

---

## 8. Invariants EIP

| Code | Invariant |
|------|-----------|
| **INV-EIP-1** | Les donnees d'identite sont produites par les Cores (et Kernel), jamais par MiyukiniAdmin seul. |
| **INV-EIP-2** | Le blob EIP est toujours stocke chiffre (AEAD). |
| **INV-EIP-3** | La cle de chiffrement est derivee (KDF) et le secret bootstrap n'est pas stocke en clair en log. |
| **INV-EIP-4** | KindMother ne déchiffre pas le blob en fonctionnement normal. |
| **INV-EIP-5** | Le blob EIP n'est pas modifie après creation ; evolution = processus gouverné (nouveau blob si besoin). |

---

## 9. Résumé

- **EIP** = protocole de production et de stockage **chiffre** des donnees d'identite du COG.
- **Contenu** : environment_id, core_versions, created_at, integrity_hash, iteration, protocol_version (+ optionnels).
- **Production** : orchestration par MiyukiniAdmin, contributions Kernel + Cores via BondingBrother ; assemblage puis chiffrement.
- **Cryptographie** : AES-256-GCM, cle derivee (Argon2id/HKDF) depuis secret bootstrap ; stockage blob = iv + ciphertext + tag.
- **Stockage** : KindMother persiste le blob ; pas de déchiffrement en fonctionnement normal.
- **Usage** : first-boot (creation), demarrage suivant (verification presence/integrite), pas de modification ad hoc.

---

## 10. Documents associes

- [MiyukiniAdmin - Auth and First-Boot Contract](../core/MiyukiniAdmin/contracts/security/MiyukiniAdmin%20-%20Auth%20and%20First-Boot%20Contract.md)
- [MiyukiniAdmin - Installation & Bootstrap Guide](../core/MiyukiniAdmin/foundation/MiyukiniAdmin%20-%20Installation%20&%20Bootstrap%20Guide.md)
- [Miyukini Conceptual References - Glossaire](../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

**Date de creation :** 2026-01-29  
**Version :** 1.0.0  
**Statut :** Protocole normatif — EIP v1
