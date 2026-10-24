# Miyukini COG — Référentiels ANSSI et doctrines de souveraineté

<!-- @id conformite.anssi -->
<!-- @role compliance-research -->
<!-- @layer 0-9+zones -->
<!-- @human Cadres ANSSI (CSPN, EUCC, qualification, SecNumCloud), guides techniques et doctrine de souveraineté appliqués à COG -->
<!-- @do research_compliance -->

- **Recherche du 2026-10-24** — sources et dates dans le document ; points incertains marqués « à vérifier » ; synthèse : [CONFORMITE-SYNTHESE](CONFORMITE-SYNTHESE.md)

- **Date** : 2026-10-24 · **Périmètre** : certification/qualification ANSSI, guides techniques ANSSI, doctrine « cloud de confiance », loi SREN, droit extraterritorial
- **Base projet lue** : `docs/architecture/COG-VUE-GLOBALE.md`, `docs/securite/MODELE-MENACES-COG.md` (EX-01..63), `docs/implementation/CONTRAINTES-IMPLEMENTATION.md` (C-01..82), `Cargo.toml` du workspace (inventaire crypto)
- **Légende applicabilité** : **Obligatoire** · **Recommandé** · **Visé optionnel** · **Non applicable**
- **Convention** : « à vérifier » = pas confirmé sur source primaire lors de cette recherche

---

## 0. Constat préalable (code vs contraintes)

- **C-24** n'autorise que `miyusceau`, `ed25519-dalek`, `sha2`, `OsRng`. Le workspace utilise **aussi** `x25519-dalek`, `chacha20poly1305` (XChaCha20), `hkdf`, `aes-gcm`, `argon2`, `blake3`, `rustls` (fournisseur `ring`, versions 0.22 **et** 0.23), `tokio-rustls` 0.25 **et** 0.26, `rcgen`, `webpki-roots`
  - C-24 est donc déjà en écart avec le code. Un évaluateur CSPN le relèverait en premier (cohérence cible de sécurité / produit)
- **C-05** (chaîne Rust épinglée) : pas de `rust-toolchain.toml` dans le dépôt
- **FFI C/C++** : `whisper-rs` (whisper.cpp), MAIA via llama.cpp. `unsafe_code = "forbid"` (C-02) ne couvre que le code du workspace, pas les dépendances
- **Infrastructure tierce citée dans les docs** : Origin prévu sur un VPS Hostinger ; anti-DDoS « ex. Cloudflare, AWS Shield, Akamai » (doc MWS) ; CI sur GitHub Actions (`ubuntu-latest`)

---

## 1. Certification et qualification ANSSI

### 1.1 CSPN — Certification de sécurité de premier niveau

- **Ce que c'est** : certification nationale ANSSI créée en 2008. Évaluation « boîte noire » en temps et charge contraints par un CESTI agréé, sur une **cible de sécurité** écrite par le commanditaire
- **Applicabilité** : **Visé optionnel**, mais c'est **la cible réaliste n°1**. Elle n'est obligatoire que pour vendre à l'État/OIV certains types de produits, ou comme base d'une qualification élémentaire
- **Processus** (procédure ANSSI-CSPN-CER-P-01 v5.0, 2023-01-12 ; critères CER-P-02 v5.0) :
  1. Le commanditaire choisit un CESTI agréé CSPN et rédige la cible de sécurité : périmètre, hypothèses, menaces, fonctions de sécurité
  2. L'ANSSI valide la recevabilité
  3. Évaluation :
     - installation et documentation
     - **conformité** des fonctions de sécurité à la cible
     - **résistance** des mécanismes, recherche de vulnérabilités et tests d'intrusion (fuzzing de protocoles, scans, etc.)
     - **analyse cryptographique** contre le référentiel crypto ANSSI (ex-RGS B1/B2/B3, aujourd'hui ANSSI-PG-083 v3.00)
  4. Rapport technique d'évaluation (ETR), puis certificat ANSSI
- **Charge et délai** :
  - méthodologie de base : **25 hommes-jours**, **+10 h.j** si la cible contient de la cryptographie, soit environ **8 semaines** d'évaluation
  - notes d'application spécifiques :
    - API : 50 + 10 h.j (25 h.j si le code source ou les spécifications complètes sont fournis)
    - application mobile : au moins 15 + 10 h.j
- **Coût** : aucun barème officiel ; devis CESTI. Ordre de grandeur marché **≈ 30–60 k€** pour l'évaluation (**à vérifier** par devis), **+ 1–3 mois** de préparation interne (cible, guides, version figée). Pas de frais ANSSI connus (**à vérifier**)
- **Validité** : certificats émis en 2025 valables **3 ans**. Maintenance et réévaluation : procédure ANSSI-CSPN-MAI-P-01 v1.1 (2025-09-23)
- **Pertinence COG — composants candidats (périmètre étroit = coût maîtrisé)** :
  - **A — BorderGuard/Douane (pare-feu + IPS) en Franche**. Catégorie « pare-feu / communication sécurisée ». Expose la surface Internet (M1, M2)
  - **B — Pont mobile E2E (`miyukini-cog-bridge` + app)**. Catégorie « communication sécurisée » ; méthodologie appli mobile (NOTE-08). Crypto X25519/XChaCha20-Poly1305/HKDF à l'analyse
  - **C — Canal de mise à jour + vérification Sceau v1 (`miyusceau`)**. Catégorie « signature / intégrité ». Petit, déterministe, déjà fuzzé (EX-10), avec des vecteurs (C-61) : **meilleur rapport preuve/coût**
- **Exigences qui alimentent la cible de sécurité** : EX-01/02/03/09/10/14/19/22/24/25, C-12/13/22/23/24/25/26
- **Manques avant de candidater** :
  - **NC-ANSSI-01** Cible de sécurité CSPN rédigée pour un composant (périmètre, hypothèses d'environnement, menaces, fonctions de sécurité, utilisateurs), version figée et taguée
  - **NC-ANSSI-02** Guides d'installation et d'exploitation sécurisée (« guide AGD »), réservés au périmètre évalué
- **Sources** :
  - Présentation CSPN : https://cyber.gouv.fr/offre-de-service/solutions-certifiees-et-qualifiees/comprendre-levaluation-de-securite/certification-de-produits/presentation-certification-cspn/
  - CER-P-01 v5.0 (2023-01-12) : https://cyber.gouv.fr/sites/default/files/document/ANSSI-CSPN-CER-P-01%20Certification_de_securite_de_premier_niveau_v5.0.pdf
  - Note méthodologie API (charge 50+10) : https://cyber.gouv.fr/sites/default/files/2022-08/ANSSI-CSPN-NOTE-03-Methodologie-pour-%C3%A9valuation-CSPN-dAPI_v1.0%5B1%5D.pdf
  - Note applications mobiles NOTE-08 v1.0 : https://cyber.gouv.fr/sites/default/files/document/ANSSI-CSPN-NOTE-08%20M%C3%A9thodologie%20pour%20l%27%C3%A9valuation%20d%27applications%20mobiles_v1.0.pdf
  - MAI-P-01 v1.1 (2025-09-23) : https://cyber.gouv.fr/sites/default/files/document/ANSSI-CSPN-MAI-P-01%20Maintien%20de%20la%20confiance%2C%20continuit%C3%A9%20de%20l%27assurance_v1.1.pdf
  - Ordre de grandeur 25 + 10 h.j, 8 semaines (laboratoire) : https://cryptoexperts.com/services/certification/

### 1.2 Critères Communs → EUCC

- **Ce que c'est** : schéma européen fondé sur les Critères Communs (ISO 15408 / CEM). Règlement d'exécution (UE) 2024/482, amendé par 2024/3144 et 2025/2462 (contenu de ce dernier **à vérifier**). Niveaux « substantiel » et « élevé »
- **Calendrier** :
  - applicable depuis le **27/02/2025**
  - plus d'enregistrement SOG-IS depuis cette date ; fin des certificats CC nationaux le **27/02/2026**
  - cibles fondées sur l'ancienne version CC acceptées jusqu'au 31/12/2027 (considérants de 2024/3144, **à vérifier** au JO)
  - **la CSPN n'est pas remplacée par l'EUCC** et reste en vigueur
- **Applicabilité** : **Non applicable à court terme**. EAL3/EAL4 : 6–18 mois, 150–500 k€+ (**à vérifier**). Ne devient pertinent que pour viser une qualification « standard/renforcée » ou de grands comptes publics
- **Exigences anticipables dès maintenant** : gestion de configuration (ALC_CMC), livraison sécurisée (ALC_DEL, couverte par EX-01/04/28), correction des défauts (ALC_FLR)
  - **NC-ANSSI-03** Processus documenté de gestion des vulnérabilités : réception, triage, correctif, publication d'avis signés. Commun à l'EUCC (ALC_FLR) et au CRA
- **Sources** :
  - Premiers certificats EUCC ANSSI : https://cyber.gouv.fr/actualites/le-centre-de-certification-de-lanssi-%C3%A9met-ses-premiers-certificats-eucc/
  - Accords CC et fin SOG-IS : https://cyber.gouv.fr/offre-de-service/solutions-certifiees-et-qualifiees/comprendre-levaluation-de-securite/certification-de-produits/accords-de-reconnaissance-criteres-communs/
  - Règlement 2024/482 : https://eur-lex.europa.eu/FR/legal-content/summary/european-common-criteria-based-cybersecurity-certification-scheme-eucc.html
  - Analyse Almond/Amossys : https://almond.eu/cybersecurity-insights/eucc-renouveau-evaluation-criteres-communs-en-europe/

### 1.3 Qualification de produit et « Visa de sécurité »

- **Ce que c'est** :
  - « Visa de sécurité » = label chapeau de l'ANSSI pour ses certifications (CSPN, CC/EUCC) et qualifications
  - la **qualification** ajoute à la certification une recommandation d'emploi par l'État : conformité au référentiel crypto, engagements de l'éditeur, suivi
  - niveaux élémentaire (sur base CSPN), standard, renforcé (**à vérifier** : correspondance exacte avec l'EUCC depuis 2025)
- **Applicabilité** : **Visé optionnel**, à moyen terme (après une CSPN). Utile seulement pour vendre à des administrations ou des OIV/OSE
- **Point bloquant** : la qualification exige que la crypto soit **conforme au référentiel ANSSI**. Or Ed25519/X25519 ne sont pas cités nommément dans ANSSI-PG-083 v3.00 (voir §2.1). Il faut une justification ou un avis préalable de l'ANSSI
- **Post-quantique** : l'ANSSI a annoncé qu'à partir de **2027** les produits candidats à une certification devront intégrer un mécanisme post-quantique (Assises 2025-10-08, réaffirmé à France Quantum en juin 2026). Statut réglementaire formel **à vérifier**
  - **NC-ANSSI-04** Feuille de route post-quantique datée pour tout composant candidat à une certification après 2026 (voir §2.1)
- **Sources** :
  - Annonce PQ 2027 (presse) : https://www.clubic.com/actualite-617680-face-a-la-menace-quantique-l-anssi-renforce-ses-exigences-de-certification.html
  - Annonce PQ 2025 (presse) : https://www.clubic.com/actualite-582496-la-menace-quantique-est-la-l-anssi-alerte-et-fixe-une-date-limite-pour-se-preparer.html

### 1.4 SecNumCloud (v3.2)

- **Ce que c'est** :
  - référentiel de qualification ANSSI des prestataires cloud (IaaS/PaaS/SaaS)
  - v3.2 publiée le **08/03/2022** (date **à vérifier**). Elle ajoute des critères d'**immunité aux droits extra-européens** : siège et contrôle capitalistique UE, pas d'accès par une autorité d'un État tiers. Elle introduit la composition (un SaaS qualifiable sur un IaaS qualifié)
  - environ 10 offres qualifiées mi-2026 (S3NS PREMI3NS visée en décembre 2025)
  - aucune v3.3/v4 officielle trouvée (**à vérifier**)
- **Applicabilité** :
  - **Non applicable** au COG lui-même (auto-hébergé chez l'utilisateur, pas un prestataire)
  - **Recommandé** pour l'**hébergement d'Origin** et de tout service fédéral opéré par l'éditeur (Market, registre Merkle, relais)
  - **Visé optionnel** pour une future offre « COG hébergé » (SaaS) : il faudrait alors qualifier l'offre ou la composer sur un IaaS qualifié
- **Analyse** :
  - Origin sur VPS **Hostinger** : société de droit européen, mais pas d'offre SecNumCloud, et le capital/contrôle n'est pas audité. Le risque est limité tant qu'Origin ne détient **aucune donnée utilisateur** et que la confiance repose sur des signatures 2 sur 2 hors ligne (EX-01, EX-30)
  - Un anti-DDoS américain (Cloudflare, AWS Shield, Akamai) cité dans la doc MWS mettrait un acteur soumis au CLOUD Act / FISA 702 en coupure TLS devant Origin : métadonnées de tous les COG et capacité de censure du canal officiel. Contraire à l'esprit d'EX-03/EX-30
  - **NC-ANSSI-05** Origin et services fédéraux de l'éditeur hébergés chez un fournisseur sous contrôle UE, de préférence qualifié SecNumCloud (IaaS) ; **aucun proxy TLS-terminant non européen** devant Origin ; anti-DDoS européen ou sans terminaison TLS
  - **NC-ANSSI-06** Fonctionnement nominal de la Fédération si Origin ou son hébergeur est indisponible ou contraint (complète EX-30) : miroirs de la tête de registre chez les membres et test de bascule
- **Sources** :
  - Visa S3NS (2025-12-18) : https://www.businesswire.com/news/home/20251218398028/fr
  - Synthèse v3.2 : https://www.cloud-temple.com/en/secnumcloud-3-2-strengthening-security-and-legal-sovereignty-in-the-cloud/
  - Composition et immunité (presse) : https://www.clubic.com/actualite-554403-un-acteur-du-cloud-securise-decroche-la-plus-haute-qualification-de-securite-de-l-autorite-cyber-francaise.html

### 1.5 PASSI / PRIS / PDIS (prestataires qualifiés)

- **Ce que c'est** : qualifications ANSSI de **prestataires** :
  - **PASSI** : audit de sécurité (architecture, code, configuration, tests d'intrusion)
  - **PRIS** : réponse aux incidents
  - **PDIS** : détection d'incidents
- **Applicabilité** :
  - **Recommandé** de **faire appel** à un **PASSI** pour un audit de code (`miyusceau`, Portail, BorderGuard) et un test d'intrusion d'Origin/Franche avant toute CSPN. Coût **≈ 10–30 k€** (**à vérifier**). C'est le signal de crédibilité le moins cher
  - **Non applicable** de se faire qualifier soi-même
  - PDIS : inspiration pour WorrySentinel/Vigies (règles de détection, journalisation) ; pas d'obligation
- **Exigences** :
  - **NC-ANSSI-07** Audit PASSI (code + intrusion) avant chaque lot exposé au réseau ; constats suivis dans le registre des écarts
  - **NC-ANSSI-08** Contrat ou procédure de réponse à incident (PRIS ou équivalent) pour Origin
- **Sources** :
  - Catalogue des prestataires qualifiés : https://cyber.gouv.fr/offre-de-service/solutions-certifiees-et-qualifiees/
  - Versions des référentiels PASSI/PRIS/PDIS : **à vérifier**

---

## 2. Guides techniques ANSSI

### 2.1 Cryptographie — ANSSI-PG-083 v3.00 (2026-03-20) + guide de sélection v1.0 (2021-03-08)

- **Ce que c'est** :
  - **PG-083 v3.00** « Règles et recommandations concernant le choix et le dimensionnement des mécanismes cryptographiques ». Successeur de l'annexe B1 du RGS (v2.04, 2020). La v3.00 intègre la menace quantique
  - le « Guide de sélection d'algorithmes cryptographiques » v1.0 liste des mécanismes Recommandés (R) / Obsolescents (O)
  - les annexes B2 (gestion des clés) et B3 (authentification) du RGS restent la référence CSPN pour ces sujets (versions **à vérifier**)
- **Applicabilité** : **Obligatoire** de fait pour toute CSPN ou qualification (analyse crypto du CESTI) ; **Recommandé** sinon

**Statut des primitives COG**

- **ChaCha20 / ChaCha20-Poly1305**
  - PG-083 v3.00 : conforme à RègleChiffFlot et **RèglePQChiffFlot** ; l'intégrité doit être assurée (Poly1305)
  - guide de sélection 2021 : R (R12)
  - verdict : **OK**
- **XChaCha20-Poly1305 (nonce 192 bits)**
  - pas cité nommément ; c'est une construction HChaCha20 + ChaCha20 (draft IRTF, pas RFC)
  - verdict : acceptable par analyse (**à vérifier** avec le CESTI) ; justification à documenter
- **AES-GCM**
  - R (2021) ; AES-128 conforme mais **non conforme à RecoPQTailleCléSym** (≥ 192 bits si une sécurité post-quantique est visée)
  - verdict : imposer **AES-256-GCM** et des nonces uniques par clé
- **SHA-256**
  - conforme (RègleHachage)
  - **RecoPQHachage** : empreintes ≥ 384 bits si une sécurité post-quantique est visée
  - verdict : OK aujourd'hui. Pour le registre Merkle (long terme), étudier SHA-384/SHA-512 ou SHA3
- **BLAKE3**
  - absent des deux documents
  - verdict : **hors référentiel**. Le cantonner aux usages non sécuritaires (déduplication, cache)
- **HKDF**
  - R (2021, RFC 5869)
  - verdict : **OK**
- **Argon2id**
  - absent ; seul PBKDF2 est R (2021). PG-083 v3.00 pose **RègleSecretFaibleEntropie** : aucune recherche exhaustive hors ligne possible
  - verdict : défendable (RFC 9106) mais **à justifier**. Documenter les paramètres et prévoir PBKDF2-HMAC-SHA-256 en repli si le CESTI l'exige (**à vérifier**)
- **Curve25519 (X25519)**
  - 2021 : Curve25519 **R** (R18, RFC 7748, note 5.3.b)
  - v3.00 : **non cité nommément** (sont cités FRP256v1, P-256/384/521, Brainpool). Conforme par analyse à RègleCourbeElliptiqueGFp (sous-groupe d'ordre premier ≈ 2^252 ≥ 250 bits)
  - verdict : acceptable, **à justifier** ; **non post-quantique**
- **Ed25519 (EdDSA)**
  - **absent** des listes de signatures conformes, en 2021 (R21) comme en v3.00 (seuls RSA-PSS, ECDSA, ECKCDSA, ML-DSA hybride, SLH-DSA)
  - EdDSA figure pourtant dans FIPS 186-5 (2023)
  - verdict : **risque d'écart en CSPN/qualification**. Justification à produire, ou avis ANSSI à demander (**à vérifier**). Non post-quantique
- **ML-KEM-768 / ML-DSA / SLH-DSA**
  - ML-KEM et ML-DSA conformes **uniquement hybridés** avec un mécanisme classique
  - **SLH-DSA (FIPS 205) conforme seul, « utilisable tel quel après 2030 »**
  - verdict : cible post-quantique
- **RNG (`OsRng`/getrandom)**
  - RègleArchiGénAléa : générateur à retraitement cryptographique
  - verdict : OK (noyau Linux). Le documenter

**Règles transverses de la v3.00**

- **RecoSécuLongTerme** : usage au-delà du 01/01/2030 ou risque d'attaque rétroactive ⇒ viser une sécurité post-quantique
  - concerne le **registre Merkle**, les **signatures des Cores et du Market**, les **identités racines**, le **pont mobile** (enregistrer maintenant, déchiffrer plus tard)
- **RecoPQConfidentialitéPersistante** : PFS préservée face à un adversaire quantique. Une hybridation par clé pré-partagée seule n'y suffit pas
- **Attention (signatures)** : certains protocoles exigent la non-malléabilité, la sécurité forte en contrefaçon et la non-resignabilité
  - concerne Sceau v1, les opérations collaboratives signées (EX-06) et les blacklists (EX-32)

**Exigences nouvelles**

- **NC-ANSSI-09** Inventaire cryptographique (CBOM) versionné. Pour chaque usage : primitive, taille de clé, crate, règle ANSSI couverte, justification si hors liste. **Réécrire C-24** pour lister toutes les crates crypto autorisées
- **NC-ANSSI-10** Note de justification Ed25519/X25519/XChaCha20/Argon2id face à PG-083 v3.00, ou demande d'avis à conseil.technique@ssi.gouv.fr. Repli possible : Sceau en ECDSA P-256/FRP256v1 pour les composants à qualifier
- **NC-ANSSI-11** Vérification Ed25519 **stricte** (`verify_strict` : rejet des points de petit ordre et des encodages non canoniques). Signatures liées au contexte : séparation de domaine par usage et étiquette de version dans Sceau
- **NC-ANSSI-12** **Agilité crypto** dans Sceau : identifiant d'algorithme signé, liste d'algorithmes autorisés par version, aucune négociation descendante (anti-downgrade, complète EX-09)
- **NC-ANSSI-13** **Sceau v2 hybride** avant 2030, avec deux options :
  - Ed25519 ‖ ML-DSA-65 (valide ssi les deux sont valides)
  - **SLH-DSA** seul pour la **clé éditeur hors ligne** (EX-01), car rarement utilisée et sans contrainte de taille
- **NC-ANSSI-14** Pont mobile : échange hybride **X25519 + ML-KEM-768**, combiné par HKDF sur les deux secrets, chiffrés et clés publiques. Clés éphémères par session (PFS)
- **NC-ANSSI-15** Clés symétriques ≥ 256 bits partout (XChaCha20 ok, AES-256-GCM). Interdire AES-128 et les nonces aléatoires de 96 bits au-delà de 2^32 messages par clé
- **NC-ANSSI-16** Hachage du registre Merkle et des empreintes de paquets : évaluer SHA-384/SHA-512 (RecoPQHachage). BLAKE3 exclu de tout usage de sécurité
- **NC-ANSSI-17** Hachage des secrets de faible entropie (PIN, mots de passe) : Argon2id avec paramètres documentés (mémoire ≥ 64 MiB, par exemple) et sel unique, ou PBKDF2 ; limitation des essais en ligne (RègleSecretFaibleEntropie)

**Sources**

- PG-083 v3.00 (2026-03-20) : https://messervices.cyber.gouv.fr/documents-guides/anssi-guide-mecanismes-crypto-3.00.pdf
- Guide de sélection v1.0 (2021-03-08) : https://messervices.cyber.gouv.fr/documents-guides/anssi-guide-selection_crypto-1.0.pdf
- Documents RGS (B1 v2.04 2020, B2, B3) : https://cyber.gouv.fr/reglementation/reglementation-identite-confiance-numerique/securite-echanges-voie-electronique/referentiel-general-de-securite/documents-referentiel-general-de-securite/
- RFC 10024 (TLS hybride ML-KEM, 2026, via S. Bortzmeyer) : https://www.bortzmeyer.org/10024.pdf

### 2.2 Authentification et mots de passe — ANSSI-PG-078 v2.0 (2021-10-08)

- **Ce que c'est** : « Recommandations relatives à l'authentification multifacteur et aux mots de passe ». 42 recommandations, publiées avec la CNIL
- **Applicabilité** : **Recommandé** (passeport, visa, accès distant, MiyukiniAdmin, services H2)
- **Correspondances** :
  - multifacteur et facteur de possession matériel : EX-14 (révocation), passeport + preuve de possession par une clé en Keystore matériel (§4 du modèle de menaces)
  - comparaisons en temps constant : C-22
  - identité jamais tirée d'un champ de requête : C-20
- **Manques** :
  - **NC-ANSSI-18** Politique d'authentification écrite :
    - MFA obligatoire pour tout accès hors zone COG et pour MiyukiniAdmin
    - pas de SMS comme facteur
    - limitation et temporisation des essais
    - mots de passe : longueur minimale ≥ 12 et pas de règles de composition imposées (**à vérifier** contre le guide)
    - stockage selon NC-ANSSI-17
  - **NC-ANSSI-19** Procédure de recouvrement de compte sans affaiblir le second facteur : SLIP-39 pour la racine (EX-24), ré-enrôlement d'appareil signé
- **Source** : https://www.it-connect.fr/nouveau-guide-anssi-sur-les-mots-de-passe-et-lauthentification-multifacteur/ (PDF ANSSI-PG-078 sur cyber.gouv.fr)

### 2.3 TLS — « Recommandations de sécurité relatives à TLS »

- **Ce que c'est** : guide ANSSI de configuration TLS. v1.1 du 19/08/2016 confirmée ; une v1.2 (2020) existe probablement (**à vérifier**)
- **Applicabilité** : **Recommandé** (Franche, Origin, DAV, SMTP, MQTT)
- **Principes** : TLS 1.3 de préférence (1.2 toléré avec ECDHE et AEAD uniquement), suites AEAD, PFS, certificats ≥ P-256/RSA-3072, pas de renégociation ni de compression
- **Constat code** : `rustls` 0.23 avec le fournisseur **`ring`**, qui n'offre pas ML-KEM ; le groupe hybride X25519MLKEM768 de rustls requiert `aws-lc-rs` (**à vérifier** pour la version en usage). Doublons rustls 0.22/0.23
- **Manques** :
  - **NC-ANSSI-20** Profil TLS unique du COG :
    - TLS 1.3 uniquement en Franche et pour Origin
    - groupes X25519MLKEM768 puis X25519
    - suites AES-256-GCM et ChaCha20-Poly1305
    - épinglage des clés Origin et des pairs fédérés (pas de dépendance aux AC publiques pour la Fédération)
    - une seule version de rustls (C-04 / `cargo deny` doublons)
- **Source** : https://cyber.gouv.fr/sites/default/files/2016/09/guide_tls_v1.1.pdf

### 2.4 Rust — « Règles de programmation pour le développement d'applications sécurisées en Rust »

- **Ce que c'est** :
  - guide v1.0 (PDF du 09/06/2020), maintenu en continu sur GitHub `ANSSI-FR/rust-guide` (dernier push 2026-05-19)
  - règles DENV / LANG / LIBS / MEM / FFI / UNSAFE
- **Applicabilité** : **Recommandé, à traiter comme obligatoire** pour COG. Un CESTI ou un PASSI s'en servira comme grille
- **Déjà couvert** :
  - `LANG-UNSAFE` / `UNSAFE-NOUB` → C-02
  - `DENV-CARGO-LOCK` / `LIBS-AUDIT` → C-04, EX-15
  - `DENV-LINTER` → C-01, C-03
  - `LANG-LIMIT-PANIC` → C-10
  - `LANG-ERRWRAP` → C-11
  - fuzzing (chapitre test/fuzz) → C-26, EX-10
- **Manques** :
  - **NC-ANSSI-21** `DENV-STABLE` + C-05 : ajouter `rust-toolchain.toml` (stable, version figée) ; `DENV-TIERS` : cibles tier 1 uniquement pour les Cores et la crypto (attention aux cibles mobiles et ARM, **à vérifier**)
  - **NC-ANSSI-22** `LANG-ARITH` / `DENV-CARGO-OPTS` : `overflow-checks = true` en profil release pour les crates sécurité, ou lints clippy `arithmetic_side_effects`. `LANG-ARRINDEXING` : lint `indexing_slicing` dans les décodeurs (Portail, Sceau)
  - **NC-ANSSI-23** `LIBS-VETTING-DIRECT/TRANSITIVE` : revue documentée des dépendances (`cargo vet`). `LIBS-UNSAFE` : mesure de l'`unsafe` des dépendances (`cargo geiger`). `LIBS-OUTDATED` en CI
  - **NC-ANSSI-24** Règles `FFI-*` pour whisper.cpp et llama.cpp :
    - encapsulation sûre (`FFI-SAFEWRAPPING`, `FFI-NOPANIC`, `FFI-MEM-OWNER`)
    - **exécution dans un processus ou bac à sable séparé**, sans clés ni accès à la DB (complète EX-21)
  - **NC-ANSSI-25** `LANG-DROP-SEC` : `zeroize` (C-25) ne suffit pas seul. Ajouter verrouillage mémoire (`mlock`) et désactivation du core dump pour les processus qui manipulent la racine ou les clés d'émission (A2)
- **Sources** :
  - Guide en ligne : https://anssi-fr.github.io/rust-guide
  - PDF v1.0 : https://cyber.gouv.fr/sites/default/files/2020/06/anssi-guide-regles_de_programmation_pour_le_developpement_dapplications_securisees_en_rust-v1.0.pdf

### 2.5 Journalisation — « Recommandations de sécurité pour l'architecture d'un système de journalisation » v2.0 (2022-01-28)

- **Ce que c'est** : architecture de collecte, centralisation, horodatage, protection et rétention des journaux
- **Applicabilité** : **Recommandé**
- **Couvert** : clé de journal (A2), registre Merkle (A4), TAMR « traçabilité absolue », EX-12 (incidents IDS), C-25 (aucun secret journalisé)
- **Manques** :
  - **NC-ANSSI-26** Politique de journalisation, avec :
    - événements obligatoires : authentification, administration, décisions StrongFather, refus Douane, mises à jour
    - **chaînage et signature** des journaux par la clé de journal
    - horodatage local monotone + MTP (EX-22)
    - stockage en zone COG en ajout seul
    - durées de rétention (6–12 mois par défaut, **à vérifier** au regard de la CNIL)
    - export signé pour analyse
  - **NC-ANSSI-27** Journal des **actions d'administration** séparé et non effaçable par l'administrateur (TAMR OVERRIDE inclus)
- **Source** : https://cyber.gouv.fr/sites/default/files/2022/01/anssi-guide-recommandations_securite_architecture_systeme_journalisation.pdf

### 2.6 Administration sécurisée des SI — ANSSI-PA-022 (2021-05-11)

- **Ce que c'est** :
  - SI d'administration dédié : poste d'administration, réseau ou zone d'administration, bastion
  - administration jamais depuis un poste bureautique ou exposé à Internet
  - comptes nominatifs, MFA
- **Applicabilité** : **Recommandé**
- **Couvert** : D7 / C-35 (MiyukiniAdmin en zone COG uniquement, jamais exposé), EX-24 (racine hors ligne)
- **Manques** :
  - **NC-ANSSI-28** Administration distante du COG (pour un intégrateur ou une petite organisation) :
    - uniquement par un canal d'administration dédié et authentifié par MFA matériel
    - session journalisée (NC-ANSSI-27)
    - APPROVAL TAMR sur place pour les actions critiques
    - jamais via les façades de la Franche
  - **NC-ANSSI-29** Séparation des rôles : utilisateur, administrateur COG, éditeur. Aucun accès éditeur aux COG des clients
- **Source** : https://cyber.gouv.fr/publications/securiser-ladministration-des-systemes-dinformation (PA-022, **URL à vérifier**)

### 2.7 Guide d'hygiène informatique (42 mesures) — v2.0 (2017)

- **Ce que c'est** : socle de 42 mesures (inventaire, comptes, mises à jour, sauvegardes, cloisonnement, journalisation, etc.). Une révision après 2017 est **à vérifier**
- **Applicabilité** : **Recommandé**, surtout pour l'**exploitant** (TPE, association, foyer). Le produit doit permettre de l'appliquer
- **Couvert** par conception : cloisonnement (EX-19/20/21), mises à jour signées (EX-01/26), sauvegardes chiffrées hors site (EX-27), journalisation (§2.5)
- **Manque** :
  - **NC-ANSSI-30** Guide d'exploitation « COG et hygiène ANSSI » : table des 42 mesures → fonction COG ou action de l'exploitant. Contrôle automatique dans MiyukiniWatch (score d'hygiène)
- **Source** : https://cyber.gouv.fr/publications/guide-dhygiene-informatique (**URL à vérifier**)

### 2.8 Mises à jour sécurisées

- **Ce que c'est** : il n'existe pas de guide ANSSI dédié à la distribution de mises à jour logicielles (aucun trouvé, **à vérifier**). Les exigences viennent de l'hygiène (mesures « mises à jour »), du guide GNU/Linux (§2.11) et des critères CSPN/EUCC (livraison sécurisée)
- **Applicabilité** : **Recommandé**
- **Couvert (fort)** : EX-01 (2 sur 2), EX-02 (quarantaine), EX-03 (fraîcheur, principe TUF), EX-04 (reproductible), EX-26, EX-28
- **Manques** :
  - **NC-ANSSI-31** Retour arrière sûr : partitions A/B ou image précédente vérifiée, sans violer l'anti-rollback (sinon un seul « dernier bon » état est autorisé)
  - **NC-ANSSI-32** **SBOM signée** par version (CycloneDX ou SPDX), publiée au registre Merkle, avec avis de vulnérabilité signés (lien NC-ANSSI-03, CRA)

### 2.9 Objets connectés / domotique — ANSSI-PA-087 v1.0 (2021-08-27)

- **Ce que c'est** : « Recommandations relatives à la sécurité des (systèmes d')objets connectés »
- **Applicabilité** : **Recommandé** pour Alicia (Franche locale, MQTT)
- **Couvert** : Alicia hors zone COG (Franche locale), intentions via StrongFather
- **Manques** :
  - **NC-ANSSI-33** Objets en réseau ou VLAN dédié **sans route Internet** par défaut. Inventaire des objets. Identifiants par défaut refusés à l'enrôlement
  - **NC-ANSSI-34** MQTT : TLS avec authentification mutuelle ou par appareil, ACL par sujet, taille de message bornée (C-13). Passerelle Alicia sans secret de la zone COG
- **Source** : https://cyber.gouv.fr/sites/default/files/2021/09/anssi-guide-securite_des_systemes_objets_connectes_iot-v1.0.pdf

### 2.10 IA générative — « Recommandations de sécurité pour un système d'IA générative » v1.0 (2024-04-29)

- **Ce que c'est** :
  - environ 35 recommandations sur le cycle de vie : entraînement, déploiement, production
  - cloisonnement, pas d'action critique automatique, filtrage des entrées et sorties, intégrité des modèles, journalisation, hébergement maîtrisé
  - complété par ANSSI/BSI « AI Coding Assistants » (2024-10)
- **Applicabilité** : **Recommandé** (MAIA, Whisper, IA fédérée)
- **Couvert (fort)** :
  - IA locale, sans fuite vers un fournisseur
  - EX-13 / C-50 : actions par intentions, StrongFather + TAMR
  - C-51 / EX-60 : marquage « étranger »
  - C-52 / EX-63 : pas d'apprentissage sur données étrangères
  - EX-26 / C-53 : modèles signés via le canal officiel
  - interdiction de publication par MAIA sans validation humaine
- **Manques** :
  - **NC-ANSSI-35** Journalisation des interactions IA : entrées, outils appelés, intentions émises, décisions. Rétention bornée et données sensibles masquées
  - **NC-ANSSI-36** Chaîne d'approvisionnement des modèles :
    - formats sans exécution de code uniquement (GGUF / safetensors, pickle interdit)
    - empreinte et provenance (dépôt d'origine, licence C-72) inscrites au registre
    - téléchargement initial depuis un miroir maîtrisé plutôt que Hugging Face en direct
  - **NC-ANSSI-37** Processus d'inférence MAIA isolé : sans réseau, sans accès direct à KindMother, avec quotas (EX-11). Campagne de **red-team d'injection de prompt** avant chaque version de MAIA
- **Sources** :
  - Guide FR : https://cyber.gouv.fr/sites/default/files/document/Recommandations_de_s%C3%A9curit%C3%A9_pour_un_syst%C3%A8me_d_IA_g%C3%A9n%C3%A9rative.pdf
  - Version EN : https://cyber.gouv.fr/sites/default/files/document/security_recommandations_for_a_generative_ai_system.pdf
  - ANSSI/BSI : https://cyber.gouv.fr/actualites/lanssi-et-le-bsi-publient-leurs-recommandations-de-securite-concernant-les-assistants-de-programmation-bases-sur-lia/

### 2.11 GNU/Linux — ANSSI-BP-028 v2.0 (2022-10-03)

- **Ce que c'est** :
  - « Recommandations de configuration d'un système GNU/Linux » à 4 niveaux : minimal, intermédiaire, renforcé, élevé
  - durcissement du noyau, démarrage, partitions, comptes, contrôle d'accès obligatoire (MAC), services, journalisation
  - profils OpenSCAP disponibles (RHEL, Debian/Ubuntu via ComplianceAsCode)
- **Applicabilité** : **Recommandé, cible du futur Linux-COG**
- **Couvert** : Secure Boot, TPM, dm-verity (§8 du modèle de menaces), image signée (EX-28)
- **Manques** :
  - **NC-ANSSI-38** Linux-COG conforme à BP-028 **niveau renforcé**, mesuré en CI par OpenSCAP/ComplianceAsCode :
    - sysctl et options noyau
    - `/tmp`, `/var`, `/home` en `nodev,nosuid,noexec`
    - MAC (AppArmor ou SELinux) par service
    - durcissement systemd (`ProtectSystem`, `PrivateNetwork` pour la zone COG)
    - aucun service inutile
  - **NC-ANSSI-39** Mesure d'intégrité au démarrage (IMA ou measured boot) avec attestation locale, reliée à l'état « Compromis » de l'EIP
- **Sources** :
  - PDF v2.0 : https://cyber.gouv.fr/sites/default/files/document/fr_np_linux_configuration-v2.0.pdf
  - Note Red Hat : https://redhat.com/fr/blog/anssi-bp-028-security-recommendations-updated-version-2.0

### 2.12 Guides connexes utiles (lecture rapide)

- **Interconnexion d'un SI à Internet** (passerelle, DMZ) : valide le schéma Franche + Portail (rupture protocolaire, flux à sens unique). **Recommandé**. Version **à vérifier**
- **Cloisonnement système** et **conteneurs/Docker** : appui pour EX-05/EX-21/EX-29. **Recommandé**. Versions **à vérifier**

---

## 3. Doctrines de souveraineté et droit extraterritorial

### 3.1 « Cloud de confiance » et doctrine « cloud au centre »

- **Ce que c'est** :
  - stratégie nationale cloud (mai 2021) et label « cloud de confiance » = offre qualifiée SecNumCloud + immunité extra-européenne
  - circulaire « cloud au centre » (n° 6282/SG du 05/07/2021, mise à jour 2023, **à vérifier**) : le cloud par défaut pour les projets numériques de l'État ; les données sensibles sur une offre qualifiée SecNumCloud ou un cloud interne de l'État
- **Applicabilité** :
  - **Non applicable** au COG d'un foyer ou d'une PME privée
  - **Visé optionnel** comme argument pour les collectivités et associations subventionnées. Un COG sur site n'est **pas un service cloud commercial**, donc hors doctrine. Mais une offre « COG hébergé » ou « Origin » vendue à l'État y tomberait
- **Exigence** :
  - **NC-ANSSI-40** Fiche de positionnement « COG ≠ cloud » : traitement sur site, éditeur sans accès aux données, Origin sans donnée utilisateur. À fournir aux acheteurs publics

### 3.2 Loi SREN (n° 2024-449 du 21/05/2024)

- **Ce que c'est** : loi « visant à sécuriser et réguler l'espace numérique » :
  - **art. 31** : l'État, ses opérateurs et certains GIP qui recourent à un cloud commercial pour des données d'une sensibilité particulière doivent garantir la protection contre tout accès non autorisé par des autorités d'États tiers
  - décret d'application **n° 2026-272 du 14/04/2026** (fiche DAJ du 06/05/2026) : en substance, recours à des offres **SecNumCloud**, avec un régime de dérogation temporaire (détails **à vérifier** au JO)
  - **art. 32** : santé (HDS)
  - **titre « cloud »** (anticipation du Data Act) : interopérabilité, portabilité, fin des frais de transfert sortant, encadrement des crédits cloud
- **Applicabilité** :
  - **Non applicable** au COG sur site
  - **Recommandé** de s'aligner sur les principes portabilité/interopérabilité (argument commercial et Data Act)
  - **Obligatoire** seulement si l'éditeur vend un service cloud (COG hébergé) à l'État
- **Exigence** :
  - **NC-ANSSI-41** Export complet et documenté des données de chaque service dans des formats ouverts (CalDAV/CardDAV/WebDAV/ODF déjà présents), sans frais ni verrou. Commande « quitter le COG » testée
- **Sources** :
  - Fiche DAJ art. 31 (2026-05-06) : https://www.economie.gouv.fr/daj/nouvelle-fiche-technique-sur-la-mise-en-oeuvre-de-larticle-31-de-la-loi-du-21-mai-2024-visant-securiser-et-reguler-lespace-numerique-associee-des
  - Décret attendu (presse) : https://www.solutions-numeriques.com/loi-sren-le-decret-attendu-qui-redefinit-les-regles-du-cloud-pour-les-administrations/
  - Dispositions pour les collectivités : https://www.banquedesterritoires.fr/loi-regulant-le-numerique-sren-adoptee-les-dispositions-qui-interessent-les-collectivites

### 3.3 Exposition au droit extraterritorial (CLOUD Act, FISA 702)

**Rappel juridique**

- **CLOUD Act (2018)** : un fournisseur soumis à la juridiction américaine doit communiquer les données sous sa « possession, garde ou contrôle », où qu'elles soient stockées
- **FISA §702** :
  - surveillance de non-Américains via des « fournisseurs de services de communications électroniques »
  - réautorisée en 2024 (RISAA) jusqu'au 20/04/2026, puis prolongations courtes (30/04, puis 12/06/2026)
  - sort définitif **à vérifier** ; certification FISC jusqu'à ~mars 2027 selon une source militante
- **EU-US Data Privacy Framework** : validé par le Tribunal de l'UE (Latombe, 03/09/2025), **pourvoi devant la CJUE en cours** (introduit le 31/10/2025)

**Positionnement de COG**

- Données au domicile de l'utilisateur, chiffrées, clés en sa possession : aucun fournisseur tiers n'a la « garde » des données ⇒ **hors portée directe du CLOUD Act**. C'est l'argument central
- Origin ne détient ni donnée ni clé utilisateur ; la confiance tient à des signatures 2 sur 2 dont une hors ligne (EX-01) ⇒ une contrainte légale sur l'hébergeur d'Origin ne permet ni de lire les données ni de pousser une mise à jour
- **Vecteurs résiduels** (contrôle américain ou extra-UE) :
  - GitHub et GitHub Actions (code, CI) : intégrité couverte par EX-04 (reproductibilité)
  - crates.io : intégrité couverte par `Cargo.lock`
  - Hugging Face : modèles
  - Google/Apple : magasins d'applications et notifications push du mobile (métadonnées)
  - anti-DDoS ou CDN américain devant Origin
  - fournisseurs d'IA en nuage : aucun aujourd'hui

**Exigences**

- **NC-ANSSI-42** Registre des dépendances d'infrastructure par juridiction (hébergeur, CDN, CI, forge, magasins, push), avec mitigation pour chacune :
  - miroir européen ou auto-hébergé de la forge et de la CI (Forgejo, par exemple)
  - miroir des crates et modèles
  - notifications sans contenu ni identifiant stable (réveil uniquement, puis relève E2E)
- **NC-ANSSI-43** Aucune télémétrie vers l'éditeur par défaut. Toute métrique fédérale est agrégée, signée et soumise à l'accord de l'utilisateur (cohérent avec la requête générique, §3 du modèle de menaces)
- **NC-ANSSI-44** Société éditrice et gouvernance d'Origin (EX-30) sous droit UE, sans contrôle extra-européen ; clé éditeur hors ligne détenue dans l'UE. Prérequis de toute démarche « cloud de confiance » ou SecNumCloud ultérieure

**Sources**

- FISA 702 (Brookings) : https://www.brookings.edu/articles/a-key-intelligence-law-expires-in-april-and-the-path-for-reauthorization-is-unclear
- FISA 702 (Wiley) : https://www.wileyconnect.com/congress-again-approaches-deadline-for-extending-fisa-702-authorities-creating-uncertainty-for-communications-providers
- DPF, pourvoi CJUE (2025-12-01) : https://www.wilmerhale.com/insights/blogs/wilmerhale-privacy-and-cybersecurity-law/20251201-european-court-of-justice-to-review-challenge-to-eu-us-data-privacy-framework
- Avis CNIL sur EUCS et immunité : https://www.solutions-numeriques.com/?p=229108

### 3.4 Contexte réglementaire voisin (pour mémoire)

- **NIS2 (loi « Résilience »)** :
  - texte adopté par le Sénat le 12/03/2025 ; examen à l'Assemblée programmé à partir du 07/10/2026
  - France renvoyée devant la CJUE pour non-transposition le 08/07/2026
  - promulgation **à vérifier**
  - COG n'est pas une entité NIS2 ; ses clients PME peuvent l'être
- **Cyber Resilience Act** (UE) : obligations produit, SBOM, gestion et signalement des vulnérabilités. Couvert par NC-ANSSI-03 et NC-ANSSI-32 ; à traiter dans le rapport CRA dédié
- **Sources** :
  - NIS2 à l'ordre du jour : https://next.ink/brief-article/alleluia-la-transposition-de-nis2-est-enfin-a-lordre-du-jour-de-lassemblee-nationale/
  - Le brief du 23/09/2026 : https://next.ink/briefs/le-brief-du-23-septembre-2026/

---

## 4. Tableau récapitulatif

| Référentiel | Applicabilité COG | Exigences existantes | Nouvelles |
|---|---|---|---|
| CSPN | Visé optionnel — **1re cible** | EX-01/03/09/10, C-23/26/61 | NC-01, 02 |
| EUCC / CC | Non applicable court terme | EX-04/28 | NC-03 |
| Qualification / Visa | Visé optionnel moyen terme | — | NC-04 |
| SecNumCloud | Recommandé (Origin), N/A (COG) | EX-30 | NC-05, 06, 44 |
| PASSI / PRIS / PDIS | Recommandé (recours) | EX-12 | NC-07, 08 |
| Crypto PG-083 v3.00 | Obligatoire de fait (certif) | C-24 (à réécrire), EX-24/25 | NC-09 → 17 |
| Authentification PG-078 | Recommandé | C-20/22, EX-14 | NC-18, 19 |
| TLS | Recommandé | C-21 | NC-20 |
| Guide Rust | Recommandé → traité comme obligatoire | C-01..05, C-10/11, C-26 | NC-21 → 25 |
| Journalisation | Recommandé | EX-12/22, C-25 | NC-26, 27 |
| Administration PA-022 | Recommandé | C-35, D7 | NC-28, 29 |
| Hygiène (42) | Recommandé (exploitant) | EX-19/21/27 | NC-30 |
| Mises à jour | Recommandé | EX-01..04/26/28 | NC-31, 32 |
| IoT PA-087 | Recommandé (Alicia) | — | NC-33, 34 |
| IA générative | Recommandé | EX-13/26/60/63, C-50..53 | NC-35 → 37 |
| GNU/Linux BP-028 | Recommandé (Linux-COG) | §8 menaces, EX-28 | NC-38, 39 |
| Cloud au centre / cloud de confiance | N/A (COG), optionnel (offre hébergée) | — | NC-40 |
| SREN | N/A (COG), Recommandé (portabilité) | — | NC-41 |
| CLOUD Act / FISA | Positionnement favorable | EX-01/30 | NC-42 → 44 |

---

## 5. Feuille de route priorisée (crédibilité d'abord)

1. **T+0 → 1 mois — Aligner la crypto sur PG-083 v3.00** (coût : interne)
   - CBOM et réécriture de C-24 (NC-ANSSI-09)
   - note de justification Ed25519/X25519/XChaCha20/Argon2id + demande d'avis ANSSI (NC-ANSSI-10)
   - `verify_strict` et séparation de domaine (NC-ANSSI-11)
   - AES-256 uniquement, BLAKE3 hors sécurité (NC-ANSSI-15/16)
2. **T+0 → 2 mois — Appliquer le guide Rust ANSSI** (interne)
   - `rust-toolchain.toml` (NC-ANSSI-21), `overflow-checks` et lints (NC-ANSSI-22)
   - `cargo vet` + `cargo geiger` en CI (NC-ANSSI-23)
   - isolement des FFI whisper.cpp et llama.cpp (NC-ANSSI-24)
   - tableau de conformité règle par règle publié dans `docs/securite/`
3. **T+1 → 3 mois — Agilité crypto et plan post-quantique** (interne)
   - identifiant d'algorithme dans Sceau (NC-ANSSI-12)
   - hybride X25519+ML-KEM-768 dans le pont mobile (NC-ANSSI-14)
   - profil TLS avec X25519MLKEM768 (NC-ANSSI-20)
   - spécification Sceau v2 (Ed25519‖ML-DSA-65, SLH-DSA pour la clé éditeur, NC-ANSSI-13)
   - Objectif : être **prêt pour l'exigence PQ 2027** des certifications
4. **T+2 → 4 mois — Souveraineté de l'infrastructure éditeur**
   - Origin sur IaaS européen (SecNumCloud si possible), pas de CDN américain qui termine TLS (NC-ANSSI-05)
   - registre des juridictions (NC-ANSSI-42), miroir forge/CI
   - fiche « COG ≠ cloud » (NC-ANSSI-40)
5. **T+3 → 5 mois — Audit PASSI** (≈ 10–30 k€, **à vérifier**) : code `miyusceau` + Portail + BorderGuard, test d'intrusion d'Origin et de la Franche (NC-ANSSI-07). Corriger avant toute certification
6. **T+4 → 6 mois — Journalisation, administration, mises à jour** (NC-ANSSI-26..32), SBOM signée et processus de vulnérabilités (NC-ANSSI-03), en commun avec le CRA
7. **T+6 → 9 mois — 1re CSPN : canal de mise à jour + vérificateur Sceau (`miyusceau`)**
   - environ 35 h.j CESTI, ≈ 30–60 k€ (**à vérifier**)
   - périmètre petit, crypto centrale, preuves déjà là (vecteurs, fuzzing, reproductibilité)
   - en parallèle : cible de sécurité et guides (NC-ANSSI-01/02)
8. **T+9 → 18 mois — 2e CSPN** : BorderGuard/Douane ou pont mobile E2E (méthodologie mobile) ; Linux-COG au niveau BP-028 renforcé (NC-ANSSI-38/39)
9. **Au-delà de 18 mois** : qualification élémentaire, puis EUCC « substantiel » seulement si un marché public le justifie. Offre « COG hébergé » qualifiée SecNumCloud (par composition) seulement si ce modèle commercial est retenu

---

## 6. Points « à vérifier » prioritaires

- Position de l'ANSSI sur **EdDSA/Ed25519** et **X25519** au regard de PG-083 v3.00 (absents des listes nominatives)
- Statut formel de l'exigence « post-quantique dès 2027 » pour la CSPN (texte officiel vs annonces)
- Version courante du guide TLS (v1.2 ?) et du guide d'hygiène (révision après 2017 ?)
- Texte du décret n° 2026-272 (SREN art. 31) au JO ; date exacte de SecNumCloud v3.2 et éventuel arrêté de 2026
- Devis CESTI réels (CSPN) et PASSI
- Prise en charge de X25519MLKEM768 par rustls avec le fournisseur `ring` (vraisemblablement non : passer à `aws-lc-rs` ou attendre)
