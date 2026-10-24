# Modèle de menaces — Miyukini COG

<!-- @id securite.menaces.cog -->
<!-- @role threat-model -->
<!-- @layer 3-8+kernel -->
<!-- @human Menaces, contre-mesures et risques résiduels : canal officiel, BorderGuard, agents relais, DB secondaire, Portail interne, zone COG -->
<!-- @do model_cog_threats -->

- **Statut** : v1, 2026-10-24
- **Base** : [modèle zones et couches](../architecture/MODELE-ZONES-ET-COUCHES-COG.md) · [audit](AUDIT-2026-10-23-auth-wan.md) · [lot 2](ARCHITECTURE-lot2-identite-signee.md)
- **Méthode** : STRIDE par élément — **S**poofing, **T**ampering, **R**epudiation, **I**nformation disclosure, **D**enial of service, **E**levation of privilege

## 1. Actifs

<!-- @id securite.menaces.actifs -->
<!-- @role asset-inventory -->
<!-- @do list_protected_assets -->

- **A1** DB principale et stockage local
- **A2** Clés : racine COG, émission, journal, appareils
- **A3** Moteurs des services et Cores (intégrité du code)
- **A4** Registre Merkle (historique, révocations)
- **A5** Constitution réseau (règles des pare-feu)
- **A6** Disponibilité du COG pour l'utilisateur hors ligne

## 2. Attaquants

<!-- @id securite.menaces.attaquants -->
<!-- @role threat-actors -->
<!-- @do list_threat_actors -->

- **M1** Internet anonyme (scans, exploits, DDoS)
- **M2** COG fédéré malveillant ou compromis
- **M3** Participant de session malveillant (document, message piégé)
- **M4** Origin compromis (canal officiel, Market)
- **M5** Appareil de l'utilisateur volé ou infecté
- **M6** Contenu qui détourne l'IA (injection de prompt)
- **M7** Accès physique à la machine
- **M8** Chaîne d'approvisionnement (dépendance, compilation)

## 3. Canal officiel

<!-- @id securite.menaces.canal-officiel -->
<!-- @role threat-analysis -->
<!-- @layer 3-7 -->
<!-- @do analyze_official_channel_threats -->

**Menaces**
- **T** Origin compromis (M4) signe une mise à jour malveillante des Cores
- **T** Retour à une version vulnérable (rollback)
- **D** Gel : l'attaquant bloque les mises à jour pour garder une faille ouverte
- **E** Faille dans le code de vérification lui-même (parseur)
- **I** Les demandes de mise à jour révèlent l'état du COG
- **S** Un tiers se fait passer pour Origin auprès de BorderGuard
- **T** Chaîne de compilation compromise (M8)

**Contre-mesures**
- Signature **à seuil** : Origin **+** clé éditeur hors ligne (2 sur 2) pour toute mise à jour des Cores
- Registre Merkle public + preuve de cohérence : substitution détectable
- **Délai de quarantaine** avant installation (ex. 48 h), sauf correctif critique co-signé
- Version strictement croissante (anti-rollback)
- **Fraîcheur** : tête de registre signée récente obligatoire (anti-gel, principe TUF)
- Vérification de signature **avant** tout décodage de contenu (déjà le cas dans `miyusceau`)
- Requêtes génériques et identiques pour tous les COG (aucune donnée locale)
- Compilations **reproductibles** vérifiées par plusieurs COG
- Canal ouvert à la seule identité Origin (certification globale)

**Risque résiduel**
- Compromission simultanée d'Origin et de la clé éditeur hors ligne

## 4. BorderGuard (pare-feu externe)

<!-- @id securite.menaces.borderguard -->
<!-- @role threat-analysis -->
<!-- @layer 3-5 -->
<!-- @do analyze_borderguard_threats -->

**Menaces**
- **S** Clé d'un COG fédéré volée (M2) → entrée en Fédération
- **S** Appareil volé (M5) avec passeport valide
- **D** Saturation du lien (M1)
- **E** Exploit de la pile TLS / Noise / WireGuard
- **T** Modification des règles du pare-feu
- **E** Détournement du canal officiel par une autre origine

**Contre-mesures**
- Révocation au registre Merkle, rotation des clés WireGuard
- Passeport + **preuve de possession** (clé d'appareil en Keystore matériel), révocation à distance
- Limitation de débit, bannissement auto, **zéro port ouvert côté International**
- BorderGuard dans son propre espace réseau puis micro-VM ; piles en Rust
- Règles issues de la **constitution réseau signée** uniquement
- Canal officiel réservé à l'identité Origin, vérifié à chaque objet

**Risque résiduel**
- DDoS volumétrique sur la connexion domestique (non mitigeable localement)
- Fenêtre entre vol d'appareil et révocation

## 5. Agents relais

<!-- @id securite.menaces.agents-relais -->
<!-- @role threat-analysis -->
<!-- @layer 5-7 -->
<!-- @do analyze_relay_agent_threats -->

**Menaces**
- **T** Participant malveillant (M3) injecte du contenu piégé (macro, script, fichier)
- **E** Exploit du relais (protocole de collaboration) → relais compromis
- **I** Relais compromis lit les **autres sessions** du même service
- **S** Usurpation d'un participant
- **R** Contestation d'une modification (« ce n'est pas moi »)
- **D** Épuisement des ressources par une session
- **E** Explorateur web relais : SSRF vers le réseau interne

**Contre-mesures**
- **Un relais par session** (processus ou micro-VM dédié) : compromission limitée à une session
- Contenu traité comme **donnée** : schéma strict, assainissement, aucun script exécuté côté relais
- Visa limité à la session + preuve de possession pour chaque participant
- Chaque opération **signée par son auteur** et horodatée (non-répudiation)
- Quotas CPU / RAM / débit par session (LogisticsSteward)
- Explorateur : sorties vers International uniquement, aucune route vers Portail ni zone COG
- Relais sans secret, sans identifiant du moteur

**Risque résiduel**
- Contenu malveillant légitimement accepté par l'utilisateur dans la session

## 6. DB secondaire

<!-- @id securite.menaces.db-secondaire -->
<!-- @role threat-analysis -->
<!-- @layer 6-7 -->
<!-- @do analyze_secondary_db_threats -->

**Menaces**
- **T** Empoisonnement : données malveillantes validées puis sauvées en DB principale
- **E** Injection dans le moteur de base (requêtes)
- **I** Fuite des données de session (relais compromis, disque)
- **D** Saturation (volume, nombre d'écritures)
- **T** Rejeu ou falsification de demandes de sauvegarde

**Contre-mesures**
- **Provenance par enregistrement** (auteur, passeport, signature)
- Dédouanement à la sauvegarde : type, schéma, taille, analyse, décision StrongFather
- DB principale **versionnée** : toute sauvegarde est réversible
- Validation humaine (TAMR) pour les types sensibles
- API typée, **aucun langage de requête** exposé au relais
- Chiffrement par clé de session, purge en fin de session, aucune sauvegarde de la DB secondaire
- Quotas par session
- Demandes de sauvegarde scellées (Sceau v1), nonce, idempotence

**Risque résiduel**
- Donnée fausse mais bien formée, acceptée par l'utilisateur

## 7. Portail interne

<!-- @id securite.menaces.portail-interne -->
<!-- @role threat-analysis -->
<!-- @layer 3-6 -->
<!-- @do analyze_internal_portal_threats -->

**Menaces**
- **E** Contournement : un autre chemin Franche → zone COG
- **E** Exploit du décodeur de déclarations
- **E** Adjoint confus : la zone COG exécute une intention forgée en Franche
- **D** Inondation de déclarations
- **I** Exfiltration depuis la zone COG via la file de publication
- **T** Altération d'une déclaration en transit

**Contre-mesures**
- Zone COG sans route (INV-Z4), espaces réseau séparés, IDS sur toute tentative de sortie
- Décodeur minimal et canonique, **signature vérifiée avant décodage**, Rust sans `unsafe`, **fuzzing** obligatoire
- Intentions **typées**, chaque intention soumise à StrongFather
- **La zone COG tire** : elle relève à son rythme (contre-pression naturelle)
- Publication par types déclarés, taille bornée, inscrite au registre (DLP)
- Sceau v1 sur chaque déclaration + liste blanche `MSCE` à l'IPS

**Risque résiduel**
- Canal caché à très faible débit dans des publications légitimes

## 8. Zone COG et IA

<!-- @id securite.menaces.zone-cog-ia -->
<!-- @role threat-analysis -->
<!-- @layer 8+kernel -->
<!-- @do analyze_core_zone_and_ai_threats -->

**Menaces**
- **E** Injection de prompt (M6) via un document ou un message relayé → MAIA déclenche une action
- **I** MAIA résume des données sensibles vers une publication
- **T** Accès physique (M7) : disque, démarrage modifié
- **T** Dépendance compromise (M8) dans un Core

**Contre-mesures**
- Contenu relayé = **donnée**, jamais instruction ; MAIA n'agit que par **intention** soumise à StrongFather + TAMR
- Publication par MAIA interdite sans validation humaine
- Chiffrement TPM, Secure Boot, dm-verity, racine hors ligne
- `Cargo.lock` versionné, `cargo audit`, compilations reproductibles, dépendances minimales dans les Cores

**Risque résiduel**
- Utilisateur qui valide une action trompeuse proposée par l'IA

## 9. Exigences dérivées

<!-- @id securite.menaces.exigences -->
<!-- @role security-requirements -->
<!-- @do derive_security_requirements -->

- **EX-01** Mise à jour des Cores : signature 2 sur 2 (Origin + clé éditeur hors ligne)
- **EX-02** Quarantaine temporelle des mises à jour, sauf correctif critique co-signé
- **EX-03** Fraîcheur obligatoire de la tête de registre (anti-gel)
- **EX-04** Compilations reproductibles des Cores et services officiels
- **EX-05** Un agent relais par session, isolé
- **EX-06** Opérations collaboratives signées par leur auteur
- **EX-07** Aucun langage de requête exposé par la DB secondaire
- **EX-08** DB principale versionnée, sauvegardes réversibles
- **EX-09** Signature vérifiée avant tout décodage (canal officiel, Portail)
- **EX-10** Fuzzing continu de `miyusceau` et des décodeurs du Portail
- **EX-11** Quotas par session et par zone (LogisticsSteward)
- **EX-12** IDS : toute tentative de sortie de la zone COG = incident critique
- **EX-13** Contenu relayé jamais interprété comme instruction par l'IA
- **EX-14** Révocation d'appareil en un geste, propagée par le registre
- **EX-15** `Cargo.lock` versionné et `cargo audit` en CI

## 10. Priorités

<!-- @id securite.menaces.priorites -->
<!-- @role prioritization -->
<!-- @do prioritize_mitigations -->

1. **EX-09 / EX-10** — vérifier avant de décoder, fuzzer : protège canal officiel et Portail
2. **EX-01 / EX-03** — signature à seuil et fraîcheur : neutralise un Origin compromis
3. **EX-05** — un relais par session : contient toute compromission de relais
4. **EX-08** — DB principale versionnée : rend l'empoisonnement réversible
5. **EX-13** — IA gouvernée : ferme l'injection de prompt vers les actions
