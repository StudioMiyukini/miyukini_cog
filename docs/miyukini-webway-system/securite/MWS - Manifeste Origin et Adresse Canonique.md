# MWS — Manifeste Origin et Adresse Canonique

## Contexte

Les **distributions** (packages, installateurs, binaires officiels ou communautaires) doivent fournir aux COGs l'**adresse canonique d'Origin** pour qu'ils puissent se connecter au réseau MWS. Si cette adresse est stockée en clair dans un fichier de configuration, un attaquant peut la **falsifier** (remplacer par une fausse Origin) et isoler ou tromper le COG (attaque Eclipse, phishing). Ce document définit une solution **non falsifiable** : le **Manifeste Origin signé**.

**Référence fondatrice :** [MWS - Document Fondateur](../MWS%20-%20Document%20Fondateur.md)  
**Liens :** [Protection Eclipse](./MWS%20-%20Audit%20de%20Securite%20Complet.md) (R-004), [Chiffrement et TLS](./MWS%20-%20Chiffrement%20et%20TLS.md) (certificate pinning)

---

## 1. Problème

| Risque | Description |
|--------|-------------|
| **Fichier de config modifiable** | Si l'URL d'Origin est dans `config.toml` ou équivalent, l'utilisateur ou un malware peut la remplacer par une adresse malveillante. |
| **DNS poisoning** | Même avec un domaine fixe (ex. `origin.miyukini.com`), un attaquant peut empoisonner le DNS pour rediriger vers un faux Origin. |
| **Eclipse** | Une fausse Origin peut délivrer de faux Permis et une fausse liste de trackers, isolant le COG du réseau légitime. |

**Objectif :** Que le COG **ne fasse confiance qu'à une adresse d'Origin dont l'authenticité est prouvée cryptographiquement**, et qu'une modification du contenu par un tiers soit **détectable et rejetée**.

---

## 2. Solution : Manifeste Origin signé

### 2.1 Principe

- Les distributions **n'embarquent pas** une simple URL en clair. Elles embarquent un **manifeste signé** contenant l'adresse canonique d'Origin et les données nécessaires au **certificate pinning**.
- Le manifeste est **signé** avec une clé privée détenue par l'autorité MWS (Miyukini). La **clé publique** de cette autorité est **intégrée dans le client** (binaire ou package signé).
- Au démarrage (ou à la première connexion), le client :
  1. Charge le manifeste fourni par la distribution ;
  2. **Vérifie la signature** du manifeste avec la clé publique intégrée ;
  3. Si la signature est valide, extrait l'URL d'Origin et le pin du certificat ;
  4. Se connecte à Origin en appliquant le **certificate pinning** (R-014).

Toute **altération** du manifeste (changement d'URL, de pin, etc.) invalide la signature : le client **refuse** d'utiliser le manifeste et **ne se connecte pas** à une Origin non authentifiée (ou utilise un mode dégradé explicite, voir § 5).

### 2.2 Bilan

| Aspect | Effet |
|--------|--------|
| **Non falsifiable** | Modifier l'URL ou le pin dans le manifeste casse la signature → rejet. |
| **Distributions** | Chaque distribution embarque le **même** manifeste signé (ou une copie). Pas besoin de faire confiance au distributeur pour le contenu : seule la signature compte. |
| **Mise à jour d'Origin** | Pour changer l'adresse ou le certificat, on publie un **nouveau** manifeste signé ; les mises à jour logicielles ou un mécanisme de rafraîchissement (voir § 4) peuvent le déployer. |

---

## 3. Format du Manifeste Origin

### 3.1 Structure (JSON)

Le manifeste est un fichier JSON (ou équivalent binaire) dont le **contenu canonique** (avant signature) est signé. Exemple de structure :

```json
{
  "manifest_version": 1,
  "origin": {
    "canonical_url": "https://origin.miyukini.com:7000",
    "canonical_domain": "origin.miyukini.com",
    "fallback_urls": [],
    "ip": "46.202.129.65",
    "ports": {
      "relay": 7000,
      "tracker": 21000,
      "web": 443
    },
    "tls_pin": {
      "type": "sha256",
      "fingerprint": "base64_encoded_sha256_of_spki_or_cert"
    }
  },
  "valid_after": "2026-02-13T00:00:00Z",
  "valid_until": "2027-02-13T00:00:00Z",
  "issuer": "Miyukini MWS Authority",
  "manifest_id": "origin-manifest-2026-001"
}
```

### 3.2 Champs

| Champ | Description |
|-------|-------------|
| `manifest_version` | Version du format du manifeste (entier). Permet d'évoluer le schéma. |
| `origin.canonical_url` | **Adresse canonique** d'Origin (URL principale, ex. `https://origin.miyukini.com:7000`). |
| `origin.canonical_domain` | Domaine DNS canonique d'Origin (ex. `origin.miyukini.com`). Utilisé quand le DNS est en place. |
| `origin.ip` | Adresse IP publique du VPS Origin (Hostinger). |
| `origin.ports` | Ports des services Origin : `relay` (7000), `tracker` (21000), `web` (443). |
| `origin.fallback_urls` | (Optionnel) URLs de secours (même pin ou pins associés). |
| `origin.tls_pin` | Donnée pour le **certificate pinning** : type (ex. `sha256` du SPKI ou du certificat) et empreinte en base64. |
| `valid_after` / `valid_until` | Fenêtre de validité du manifeste (pas de confiance en dehors de cette fenêtre). |
| `issuer` | Identifiant de l'émetteur (lisible). |
| `manifest_id` | Identifiant unique du manifeste (ex. UUID). |

Les champs optionnels (ex. `fallback_urls`, métadonnées) peuvent être étendus sans casser la vérification tant que le client ignore ce qu'il ne connaît pas.

### 3.3 Signature

- **Algorithme recommandé :** Ed25519 (signature compacte, rapide).
- **Contenu signé :** sérialisation **canonique** du JSON (clés triées, encodage fixe) pour reproductibilité.
- **Emplacement de la signature :** soit dans le même fichier (ex. champ `signature` en base64), soit dans un fichier séparé (`.sig`).

Exemple avec signature dans le manifeste :

```json
{
  "manifest_version": 1,
  "origin": { ... },
  "valid_after": "...",
  "valid_until": "...",
  "issuer": "...",
  "manifest_id": "...",
  "signature": "base64_ed25519_signature",
  "signature_key_id": "mws-origin-authority-1"
}
```

Le client doit :
1. Reconstruire le blob canonique **sans** le champ `signature` (et sans `signature_key_id` si celui-ci n'est pas dans le blob signé).
2. Vérifier `signature` avec la clé publique correspondant à `signature_key_id` (voir § 3.4).

### 3.4 Clé publique racine (Root of Trust)

- Une ou plusieurs **clés publiques** sont considérées comme racine de confiance pour les manifestes Origin.
- Elles sont **intégrées dans le binaire du client** (ou dans un bloc read-only du package) et **ne dépendent pas** d'un fichier externe modifiable par l'utilisateur.
- En pratique : tableau de bytes ou fichier embarqué dans l'image du programme, livré avec la distribution **signée** (signature du binaire par le projet ou par l'OS). Ainsi, modifier le binaire pour changer la clé publique invalide la signature du binaire.
- `signature_key_id` permet de sélectionner quelle clé racine utiliser (rotation future possible en ajoutant une nouvelle clé et en publiant des manifestes signés par celle-ci).

---

## 4. Vérification côté client

### 4.1 Algorithme

```text
1. Charger le fichier manifeste (depuis l'emplacement fourni par la distribution).
2. Parser le JSON et vérifier le schéma (champs obligatoires, types).
3. Vérifier valid_after ≤ now ≤ valid_until (avec marge de tolérance optionnelle).
4. Extraire le blob canonique à signer (sans signature / signature_key_id).
5. Récupérer la clé publique correspondant à signature_key_id (depuis le stockage intégré).
6. Vérifier Ed25519(public_key, blob_canonical, signature).
7. Si invalide : refuser le manifeste, ne pas connecter à Origin (ou mode dégradé).
8. Si valide : utiliser origin.canonical_url et origin.tls_pin pour les connexions à Origin, avec certificate pinning obligatoire.
```

### 4.2 Emplacement du manifeste dans la distribution

- **Recommandation :** chemin fixe dans l'arborescence du package, par exemple :
  - `share/miyukini/origin_manifest.json` (Linux),
  - ou équivalent dans les ressources du binaire (Windows/macOS).
- Le **programme d'installation** ou le **package** doit placer le fichier **sans que l'utilisateur ait à le modifier**. Les droits du fichier peuvent être en lecture seule pour les utilisateurs normaux.
- Le client **ne doit pas** utiliser en priorité un fichier de configuration utilisateur pour l'URL d'Origin ; il utilise d'abord le manifeste signé. Une surcharge explicite « mode dev » peut exister (voir § 5).

---

## 5. Comportement en cas d'échec ou d'absence de manifeste

| Situation | Comportement recommandé |
|----------|--------------------------|
| **Manifeste absent** | Refuser de se connecter à Origin ; afficher un message clair (ex. « Installation incomplète ou manifeste Origin manquant »). |
| **Signature invalide** | Refuser le manifeste ; ne pas utiliser l'URL qu'il contient ; alerter (log / message) qu'une falsification est possible. |
| **Manifeste expiré** | Refuser d'utiliser le manifeste pour une connexion normale ; inviter à mettre à jour le logiciel ou le manifeste. |
| **Override manuel (dev/test)** | Certains builds (ex. « dev ») peuvent autoriser une URL d'Origin configurée manuellement, **désactivée par défaut** et avec avertissement explicite (ex. « Mode développement : l'adresse Origin n'est pas authentifiée »). En production, ce chemin ne doit pas être utilisé. |

---

## 6. Certificate pinning (couche complémentaire)

Même avec un manifeste signé, la **première** connexion à l'URL canonique doit appliquer le **certificate pinning** (R-014) :

- Le client utilise l'empreinte (ou la clé publique) fournie dans `origin.tls_pin` et refuse toute connexion TLS dont le certificat du serveur ne correspond pas.
- Ainsi, même si un attaquant parvenait à modifier le **réseau** (DNS, BGP) pour rediriger vers un autre serveur, le handshake TLS échouerait car le certificat ne matcherait pas le pin.

Cela renforce la garantie que l'adresse livrée par la distribution **n'est pas falsifiable** et que la **connexion effective** est bien celle attendue.

---

## 7. Rôle des distributions

| Rôle | Action |
|------|--------|
| **Éditeur officiel (Miyukini)** | Génère le manifeste, le signe avec la clé privée de l'autorité, publie le fichier (et sa signature si séparée). Intègre la clé publique racine dans le client. Les packages officiels embarquent le manifeste signé au chemin défini. |
| **Repackagers / tierces parties** | Doivent **inclure le même manifeste signé** (sans le modifier). Ils ne signent pas eux-mêmes le manifeste sauf si une clé tierce est acceptée par le client (politique hors scope ici). En pratique : ne pas remplacer ni éditer `origin_manifest.json`. |
| **Mise à jour** | Lors d'un changement d'URL ou de certificat d'Origin, publication d'un nouveau manifeste signé. Mise à jour du logiciel ou mécanisme de mise à jour du manifeste (téléchargement depuis une URL de confiance + vérification signature) peut délivrer le nouveau fichier. |

---

## 8. Résumé

| Élément | Rôle |
|--------|------|
| **Manifeste signé** | Contient l'URL canonique d'Origin (VPS Hostinger) et le pin TLS ; non falsifiable sans casser la signature. |
| **Clé publique racine** | Intégrée dans le client (binaire/package) ; permet de vérifier le manifeste. |
| **Vérification au chargement** | Signature + validité temporelle ; rejet si invalide ou expiré. |
| **Certificate pinning** | Une fois l'URL tirée du manifeste, connexion uniquement si le certificat serveur correspond au pin. |
| **Distributions** | Embarquent le manifeste tel quel, sans le modifier ; le client n'accepte aucune URL d'Origin non authentifiée par ce mécanisme (sauf mode dev explicite). |

Ainsi, les distributions peuvent **livrer l'adresse d'Origin** de manière **non falsifiable** : toute modification du contenu par un tiers est détectée et le client refuse de l'utiliser.

---

## Références

- [MWS - Document Fondateur](../MWS%20-%20Document%20Fondateur.md)
- [MWS - Chiffrement et TLS](./MWS%20-%20Chiffrement%20et%20TLS.md) — certificate pinning (R-014)
- [MWS - Audit de Sécurité Complet](./MWS%20-%20Audit%20de%20Securite%20Complet.md) — Eclipse (R-004)
- [MWS - Contre-Mesures de Sécurité](./MWS%20-%20Contre-Mesures%20de%20Securite.md)
- [MWS - Implémentation Origin Hostinger](../deploiement/MWS%20-%20Implementation%20Origin%20Hostinger.md) — guide complet de déploiement

---

**Version :** 2.0  
**Mise à jour :** Hébergement Hostinger VPS (Debian 13), champs étendus (ip, ports, canonical_domain)  
**Classification :** Documentation MWS — Sécurité
