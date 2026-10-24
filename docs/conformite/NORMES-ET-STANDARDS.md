# Normes et référentiels de sécurité — alignement Miyukini COG

<!-- @id conformite.normes -->
<!-- @role compliance-research -->
<!-- @layer 0-9+zones -->
<!-- @human Normes ISO/IEC, IEC 62443, ETSI, OWASP, NIST, SLSA, SBOM et cryptographie : niveaux visés par composant -->
<!-- @do research_compliance -->

- **Recherche du 2026-10-24** — sources et dates dans le document ; points incertains marqués « à vérifier » ; synthèse : [CONFORMITE-SYNTHESE](CONFORMITE-SYNTHESE.md)

- **Date** : 2026-10-24 · **Périmètre** : normes internationales et référentiels industriels (pas le droit : RGPD, CRA, AI Act traités ailleurs, cités ici seulement comme moteurs)
- **Base projet** : [COG-VUE-GLOBALE](/srv/app/miyukini-cog/docs/architecture/COG-VUE-GLOBALE.md) · [menaces EX-01..63](/srv/app/miyukini-cog/docs/securite/MODELE-MENACES-COG.md) · [contraintes C-xx](/srv/app/miyukini-cog/docs/implementation/CONTRAINTES-IMPLEMENTATION.md) · [plan](/srv/app/miyukini-cog/docs/implementation/PLAN-IMPLEMENTATION.md) · [matrice](/srv/app/miyukini-cog/docs/implementation/MATRICE-EXIGENCES.md)
- **Légende pertinence** : **CERT** = cible certifiable · **ALIGN** = référence d'alignement (auto-déclaration, preuve interne) · **NR** = non pertinent à ce stade
- **Effort** : S (< 1 semaine) · M (1–4 semaines) · L (> 1 mois) · « continu »
- **Nouvelles lignes** : `NC-STD-nn` = contrainte/exigence proposée, à intégrer dans CONTRAINTES ou MATRICE après validation
- **Constats repo utiles** : pas de `SECURITY.md` ni `security.txt` ; actions CI épinglées par étiquette (`dtolnay/rust-toolchain@1.99.0`), pas par SHA ; aucune SBOM ni provenance ; XChaCha20-Poly1305 + X25519 + HKDF-SHA256 dans `crates/miyukini-cog-bridge/src/e2e.rs` ; `verify_strict` déjà utilisé (jay-collab, jaymessage, miyusceau)

## 0. Synthèse — niveaux visés

| Référentiel | Composant | Niveau visé | Échéance (lot) | Statut |
|---|---|---|---|---|
| OWASP ASVS 5.0 | Central, Origin (web/admin), cog-web-portal, MiyuCloud, Market | **L2** | 0 bis → 5 | ALIGN |
| OWASP ASVS 5.0 | Identité/appairage (`miyauth`, `central-remote` pairing, `miyusceau`, bridge E2E), KindMother, Portail | **L3** (chapitres V6 auth, V11 crypto, V9/V10 jetons, V12 com. sûres) | 2 → 3 | ALIGN |
| OWASP MASVS 2.1 + MASTG | `central-mobile` (Dioxus/WebView Android) | **MAS-L2** + **MAS-R** sélectif (anti-falsification, détection root) | 2 (incr. 4) | ALIGN |
| SLSA v1.2 Build track | Cores, services officiels, Origin, images d'amorçage | **L2** (provenance signée) → **L3** (builder isolé durci) | L2 : lot 4 · L3 : lot 8 | ALIGN |
| SLSA v1.2 Source track | dépôt `miyukini-cog` | **Source L2** (historique + provenance) → L3 | 4 → 8 | ALIGN |
| NIST SSDF SP 800-218 (v1.1, r1 en cours) | processus éditeur | toutes pratiques PO/PS/PW/RV « implémentées » | 0 bis → 8 | ALIGN |
| OWASP Top 10 LLM 2025 + Agentic 2026 | MAIA, Alicia, miou-llm-bridge | 10/10 risques traités et testés | 0 bis (LLM01/06) → 7 | ALIGN |
| ETSI EN 303 645 | Alicia, MIPOWER, Central Mobile, boîtier COG | 13 provisions **obligatoires** + recommandées pertinentes | 3 → 5 | ALIGN (base CRA/RED) |
| IEC 62443-4-1 | processus éditeur (Alicia, Douane) | **ML2** (processus défini) | 8 | ALIGN |
| IEC 62443-4-2 | Douane, Alicia (Franche locale) | **SL-C 2** | 3 → 8 | ALIGN |
| ISO/IEC 29147 + 30111 | éditeur + Origin | politique publiée + processus interne + CNA à terme | 0 bis (S) | ALIGN → base CRA art. 13–14 |
| ISO/IEC 27001:2022 | Origin (exploitation) + éditeur | certification **différée** ; SoA interne dès lot 4 | après ouverture publique | CERT (cible) |
| ISO/IEC 42001:2023 | MAIA (éditeur IA) | alignement ; certification NR | 7 | ALIGN |
| ISO/IEC 27701:2025 | Origin, Market, éditeur | alignement (contrôleur/sous-traitant) | 6–7 | ALIGN |
| OpenChain ISO/IEC 5230 / 18974 | éditeur | **auto-certification** de conformité | 4 (5230) · 8 (18974) | CERT (auto) |
| OpenSSF Scorecard | dépôt public | score ≥ 7 ; ≥ 8 visé | 0 bis → 8 | ALIGN |
| Crypto (RFC 8032/7748/5869, SP 800-56C/57) | `miyusceau`, bridge, KindMother | conformité stricte + agilité ; hybride PQ avant 2030 | 2 → 8 | ALIGN |

## 1. Gestion de la sécurité (ISO/IEC 27000)

### ISO/IEC 27001:2022 + 27002:2022
- **Quoi** : exigences d'un SMSI certifiable (clauses 4–10) + 93 contrôles de l'annexe A (4 thèmes : organisationnels, humains, physiques, technologiques) ; amendement 1:2024 (climat). 27002 = guide des contrôles.
- **Pertinence** : **CERT différée** pour Origin (service exploité : relais, tracker, Market officiel, registre Merkle) et l'éditeur ; certification d'un COG domestique **NR** (le COG est un produit, pas une organisation). Coût d'audit disproportionné avant ouverture publique → tenir une SoA interne.
- **Contrôles prioritaires à adopter**
  - A.5.7 renseignement sur les menaces · A.5.19–5.23 fournisseurs et chaîne logicielle → EX-15, C-04, C-70
  - A.5.24–5.28 gestion des incidents, collecte de preuves → EX-12, EX-49, **NC-STD-01** (registre d'incidents Origin + runbook CRA 24 h / 72 h)
  - A.5.30 continuité TIC (Origin) → EX-30 (fédération sans Origin)
  - A.8.4 accès au code source · A.8.25–8.29 cycle de développement sûr, codage sûr, tests de sécurité → C-01..C-82
  - A.8.8 vulnérabilités techniques → EX-15, NC-STD-03
  - A.8.9 gestion de configuration · A.8.32 gestion des changements → C-82, EX-18
  - A.8.24 usage de la cryptographie → C-24, NC-STD-20
  - A.8.15/8.16 journalisation et surveillance (Origin) → EX-22 (horodatage), **NC-STD-02** (journaux Origin signés, append-only, rétention définie)
  - A.5.15–5.18 contrôle d'accès, identités → EX-30 (admin Origin mono-compte = écart, constat « contredit »)
- **Effort** : SoA + politique : M ; certification : L (12–18 mois, audit tiers)
- **Sources** : ISO/IEC 27001:2022 <https://www.iso.org/standard/27001> · 27002:2022 <https://www.iso.org/standard/75652.html>

### ISO/IEC 27005:2022
- **Quoi** : méthode d'appréciation des risques SSI (approche par événements et par actifs), alignée 27001.
- **Pertinence** : **ALIGN** — cadrer le modèle de menaces existant (actifs §1, attaquants §2) en registre de risques coté.
- **Contrôles** : **NC-STD-04** registre de risques (vraisemblance × impact, propriétaire, traitement → EX-xx) revu à chaque lot ; lien EBIOS RM (ANSSI) possible.
- **Effort** : S–M · **Source** : <https://www.iso.org/standard/80585.html>

### ISO/IEC 27034 (série, sécurité applicative)
- **Quoi** : cadre organisationnel de sécurité applicative (ONF, ANF, contrôles de sécurité applicative ASC).
- **Pertinence** : **ALIGN léger** — redondant avec SSDF + ASVS ; utile seulement pour le vocabulaire (C-xx ≈ ASC). Pas d'investissement dédié.
- **Source** : <https://www.iso.org/standard/44378.html> (27034-1:2011, à vérifier : révisions des parties 1–7)

### ISO/IEC 27017 / 27018
- **Quoi** : 27017 = contrôles cloud (fournisseur/client) ; 27018 = protection des données personnelles en cloud public (sous-traitant). 27018:2025 révisée (à vérifier).
- **Pertinence** : **NR** pour le COG (auto-hébergé, souverain). **ALIGN partiel** pour Origin seulement s'il stocke des données personnelles de tiers (relais, sauvegardes fédérées EX-27 chiffrées de bout en bout → hors champ si Origin ne voit que du chiffré).
- **Contrôle** : **NC-STD-05** documenter que Origin et les pairs de sauvegarde ne détiennent que du chiffré (clé hors ligne, EX-27) → argument de sortie de champ.
- **Sources** : <https://www.iso.org/standard/43757.html> · <https://www.iso.org/standard/76559.html>

### ISO/IEC 27701:2025 (PIMS)
- **Quoi** : système de management de la vie privée ; **autonome depuis la version 2025** (publiée le 14/10/2025, plus besoin de 27001) ; annexe A en 3 tables (responsable, sous-traitant, communs). Transition des certificats 2019 avant le 31/10/2028.
- **Pertinence** : **ALIGN** — Origin (traitements fédéraux, comptes Market, vitrine), l'éditeur (télémétrie : à proscrire), MAIA (mémoire personnelle EX-50..56).
- **Contrôles** : registre des traitements Origin ; minimisation (EX-36 identifiant par COG visité = pseudonymisation) ; droits des personnes (EX-56 mémoire consultable/exportable/oubliable) ; **NC-STD-06** « aucune télémétrie par défaut, opt-in signé » ; **NC-STD-07** AIPD pour MAIA mémoire et Alicia (capteurs domestiques).
- **Effort** : M · **Sources** : <https://www.iso.org/standard/27701> · DNV <https://www.dnv.com/news/2025/iso-iec-27701-standard-update-release/>

### ISO/IEC 29100 (cadre vie privée) · ISO/IEC 27559 (désidentification)
- **Quoi** : 29100:2024 (2ᵉ éd., à vérifier) = 11 principes de vie privée (consentement, minimisation, limitation d'usage…) ; 27559:2022 = cadre de désidentification fondé sur le risque.
- **Pertinence** : 29100 **ALIGN** (vocabulaire de référence pour la doc vie privée) ; 27559 **ALIGN ciblé** pour IA fédérée (Fiches 0x08 publiées, EX-57, EX-61) et toute statistique fédérale.
- **Contrôles** : **NC-STD-08** toute donnée quittant la zone COG vers la Fédération (Fiche, signalement, note de pair) passe une revue de désidentification (27559) et une étiquette KindMother `publiable` (EX-47).
- **Sources** : <https://www.iso.org/standard/85938.html> (29100) · <https://www.iso.org/standard/71677.html> (27559)

### ISO/IEC 42001:2023 (système de management de l'IA)
- **Quoi** : premier SM certifiable pour l'IA (risques, AIIA, cycle de vie, données, annexe A : 38 contrôles) ; complété par 23894 (risques IA) et 42005 (analyse d'impact, 2025).
- **Pertinence** : **ALIGN** pour MAIA (éditeur de système IA local + fédéré) ; certification **NR** (taille). Utile face à l'AI Act (MAIA probablement hors « haut risque », à vérifier selon usages Alicia).
- **Contrôles** : A.6 cycle de vie → C-50..C-53 ; A.7 données (provenance, qualité) → EX-53, EX-63 ; A.8 information des utilisateurs → **NC-STD-09** fiche modèle (model card) par modèle distribué : licence (C-72), provenance, empreinte, limites ; A.5 analyse d'impact → NC-STD-07.
- **Effort** : M · **Source** : <https://www.iso.org/standard/81230.html>

## 2. Divulgation et traitement des vulnérabilités (pilier CRA)

### ISO/IEC 29147:2018 (divulgation) + ISO/IEC 30111:2019 (traitement)
- **Quoi** : 29147 = réception et publication des signalements externes, avis de sécurité ; 30111 = processus interne (tri, vérification, correctif, diffusion).
- **Pertinence** : **ALIGN prioritaire** — base attendue de la future norme harmonisée CRA **prEN 40000-1-3** (vulnérabilités) ; obligations CRA de **signalement actives depuis le 11/09/2026** (ENISA Single Reporting Platform : alerte 24 h, notification 72 h, rapport final 14 j / 1 mois) ; exigences essentielles applicables le 11/12/2027.
- **Contrôles**
  - **NC-STD-10** `SECURITY.md` + `/.well-known/security.txt` (RFC 9116 : `Contact`, `Expires` obligatoires, signé OpenPGP ou Sceau) sur dépôt, Origin, vitrine — **absent aujourd'hui**
  - **NC-STD-11** processus 30111 écrit : tri sous 72 h, CVSS v4.0, délai de correctif par sévérité, avis signés publiés au registre Merkle (EX-34) et distribués par le canal officiel (EX-01/EX-02 : correctif critique co-signé hors quarantaine)
  - **NC-STD-12** runbook CRA art. 14 (vulnérabilité activement exploitée → SRP ENISA 24/72 h/14 j) ; compte SRP ouvert
  - **NC-STD-13** avis lisibles par machine : CSAF 2.0 + VEX (CycloneDX/OpenVEX) joints à chaque correctif
  - CVE : demander via GitHub Security Advisories (GitHub = CNA) à court terme ; devenir CNA **NR** avant ouverture publique
- **Effort** : S (politique) · M (processus + CSAF)
- **Sources** : <https://www.iso.org/standard/72311.html> · <https://www.iso.org/standard/69725.html> · RFC 9116 <https://www.rfc-editor.org/rfc/rfc9116> · ENISA SRP <https://www.enisa.europa.eu/news/the-cra-single-reporting-platform-is-launched> · calendrier normes CRA <https://www.cyberresilienceact.eu/news/cra-standardisation-deadlines-pushed-back-two-months.html> (secondaire, à vérifier sur la demande de normalisation M/606)

### CVSS v4.0 (FIRST, novembre 2023)
- **Quoi** : notation de sévérité (Base, Threat, Environmental, Supplemental) ; nomenclature CVSS-BTE.
- **Pertinence** : **ALIGN** — remplacer toute échelle maison de REVUE-CONSTATS (critique/haut/moyen) par un vecteur CVSS 4.0 pour tout ce qui est publié ; garder l'échelle interne pour la revue.
- **Contrôle** : intégré à NC-STD-11 · **Effort** : S · **Source** : <https://www.first.org/cvss/v4-0/>

## 3. Sécurité produit (IoT, domotique, composants)

### ETSI EN 303 645 (IoT grand public)
- **Quoi** : 13 provisions de base (pas de mot de passe par défaut universel, politique de divulgation, mises à jour, stockage sûr des paramètres, communication sûre, surface d'attaque minimale, intégrité logicielle, données personnelles, résilience, télémétrie, effacement, installation simple, validation des entrées). Version V3.1.3 (2024) — **à vérifier** sur le portail ETSI.
- **Pertinence** : **ALIGN fort** — Alicia (domotique), MIPOWER, boîtier COG pré-installé, Central Mobile ; référence de base de la famille EN 18031 (RED, applicable au 01/08/2025 pour équipements radio) et attendue comme socle des normes CRA. Si un boîtier COG avec Wi-Fi/BT est vendu → **EN 18031-1/-2/-3 devient obligatoire** (à vérifier : citation au JOUE, restrictions).
- **Contrôles mappés**
  - 5.1 pas de mot de passe par défaut → C-21, C-22, EX-28 (amorçage) ; **NC-STD-14** secret d'appairage unique par installation, jamais de compte usine
  - 5.2 politique de divulgation → NC-STD-10
  - 5.3 mises à jour sûres → EX-01, EX-02, EX-03, EX-26 ; **NC-STD-15** durée de support annoncée (CRA : ≥ 5 ans par défaut)
  - 5.4 secrets stockés sûrement → C-25, EX-24, EX-25
  - 5.5 communications sûres → bridge E2E, EX-09
  - 5.6 surface minimale → EX-48 (contredit), C-21
  - 5.7 intégrité logicielle (démarrage vérifié) → EX-28, **NC-STD-16** démarrage mesuré/vérifié du boîtier (Secure Boot + TPM, lien EX-31)
  - 5.11 effacement des données utilisateur → EX-56, EX-33
- **Effort** : M · **Sources** : <https://www.etsi.org/deliver/etsi_en/303600_303699/303645/> · RED DA 2022/30 <https://www.tuvsud.com/en-us/services/product-certification/red-cybersecurity-requirements>

### IEC 62443-4-1:2018 (cycle de développement sûr) et 4-2:2019 (exigences composants)
- **Quoi** : 4-1 = 8 pratiques SDL (gestion, spécification, conception sûre, implémentation, V&V, gestion des défauts, mises à jour, guides) avec niveaux de maturité ML1–ML4 ; 4-2 = exigences techniques composants (CR 1–7) par niveau de sécurité SL 1–4.
- **Pertinence** : **ALIGN** — 4-1 recouvre SSDF et est référencé par les normes verticales CRA (prEN 50770 : pare-feu, VPN, routeurs) → pertinent pour **la Douane (pare-feu/IPS)** qui tombe probablement dans une catégorie « importante » de l'annexe III CRA (**à vérifier** : pare-feu/IDS = classe I ou II). 4-2 SL-C 2 pour Douane et Alicia (Franche locale, ponts MQTT). Certification ISASecure **NR**.
- **Contrôles** : 4-1 SM-7 (normes de codage) → C-01..C-14 ; SI-1/2 (revue) → lot R ; SVV-3 (tests de vulnérabilité, fuzzing) → EX-10, C-26 ; SVV-4 (tests d'intrusion) → sortie lot 3 ; DM-1..6 → NC-STD-11 ; SUM-1..5 → EX-01..03 ; SG-1..7 → **NC-STD-17** guide de durcissement et de déploiement par produit (zones, ports, défauts) ; 4-2 CR 2.8 (événements auditables), CR 3.4 (intégrité logicielle), CR 7.6 (paramétrage réseau) → EX-12, EX-28, EX-19.
- **Effort** : M (4-1 ML2) · L (4-2 complet) · **Source** : <https://webstore.iec.ch/en/publication/33615> · <https://webstore.iec.ch/en/publication/34421>

### Normes harmonisées CRA (CEN/CENELEC JTC 13)
- **Quoi** : série horizontale **EN 40000** (WG9, issue d'EN 18031) : principes (-1-1), contrôles génériques, **vulnérabilités (-1-3)** ; verticales **prEN 50770** (pare-feu, VPN, routeurs…) adossées à IEC 62443. Livraison à la Commission repoussée à **31/10/2026** (types A/B) et **31/12/2026** (type C) selon un projet d'amendement — **à vérifier** ; citation au JOUE ensuite.
- **Pertinence** : **ALIGN anticipé** — veille obligatoire ; rien de certifiable aujourd'hui.
- **Contrôle** : **NC-STD-18** veille trimestrielle JTC 13 + table de correspondance CRA annexe I ↔ EX/C dès publication de prEN 40000-1-x.
- **Source** : <https://www.cyberresilienceact.eu/news/cra-standardisation-deadlines-pushed-back-two-months.html> (secondaire)

### EUCC (schéma européen Critères Communs, Règl. d'exécution (UE) 2024/482, mod. 2024/3144)
- **Quoi** : certification Critères Communs/CEM européenne, niveaux « substantiel » (AVA_VAN 1–2) et « élevé » (AVA_VAN 3–5).
- **Pertinence** : **NR à court terme** ; **à vérifier** si la Commission impose une certification européenne pour certaines catégories « critiques » CRA (annexe IV : boîtiers de sécurité domestiques/passerelles intelligentes probablement pas). Option lointaine : EUCC substantiel pour `miyusceau` + Douane si marché pro/collectivités.
- **Source** : <https://eur-lex.europa.eu/eli/reg_impl/2024/482/2025-01-08/eng>

### ETSI EN 304 223 V2.1.1 (sécurisation de l'IA, déc. 2025 / communiqué jan. 2026)
- **Quoi** : 13 principes de cybersécurité IA sur 5 phases (conception, développement, déploiement, maintenance, fin de vie) ; injection indirecte, empoisonnement, gestion des données.
- **Pertinence** : **ALIGN** pour MAIA ; complément technique de 42001 (plus concret, gratuit).
- **Contrôles** : inventaire des actifs IA (modèles, prompts système, index) → **NC-STD-09** ; mise à jour sûre des modèles → EX-26, C-53 ; supervision humaine → TAMR, C-51.
- **Source** : <https://www.telecomtv.com/content/etsi/etsi-releases-world-leading-standard-for-securing-ai-54664/> · <https://www.etsi.org/deliver/etsi_en/304200_304299/304223/> (chemin à vérifier)

## 4. Développement sûr et chaîne d'approvisionnement

### NIST SSDF — SP 800-218 v1.1 (2022) ; r1 / v1.2 en projet (IPD 17/12/2025)
- **Quoi** : 4 groupes de pratiques (PO préparer, PS protéger le logiciel, PW produire bien sécurisé, RV répondre aux vulnérabilités) ; v1.2 ajoute PO.6 (à vérifier : version finale annoncée pour 2026, non confirmée).
- **Pertinence** : **ALIGN** — grille de processus éditeur la plus légère ; base de l'attestation CISA et proche des exigences CRA annexe I partie II.
- **Mapping** : PO.1 exigences → C-xx ; PO.3 chaîne d'outils → C-05, NC-STD-21 ; PS.1 protection du code → **NC-STD-19** (protection de branche, revue obligatoire, commits signés) ; PS.2 intégrité des versions → EX-01, EX-04 ; PS.3 archivage + SBOM → NC-STD-22 ; PW.4 réutilisation de composants sûrs → C-70, C-71 ; PW.7/8 revue et tests → C-60..C-63, EX-10 ; RV.1–3 → NC-STD-11.
- **Effort** : S (mapping) · continu · **Sources** : <https://csrc.nist.gov/pubs/sp/800/218/final> · <https://csrc.nist.gov/pubs/sp/800/218/r1/ipd>

### OWASP ASVS 5.0.0 (30/05/2025) — remplace 4.0.3
- **Quoi** : ~350 exigences, 17 chapitres (nouveaux : jetons autonomes, OAuth/OIDC, WebRTC) ; 3 niveaux conservés ; exigences de documentation des décisions en tête de chapitre.
- **Pertinence** : **ALIGN** — viser **5.0** (pas 4.0.3) ; L2 pour applications web (Central, Origin web/admin, cog-web-portal, MiyuCloud/WebDAV, Market) ; **L3** pour identité/appairage, KindMother, Portail.
- **Contrôles** : **NC-STD-23** checklist ASVS 5.0 par composant, statut par exigence dans la matrice (comme EX) ; priorité aux chapitres auth/session/jetons (EX-36, O-01), contrôle d'accès (C-20, K-04), validation/SSRF (K-01, K-03, O-03), sortie/XSS (O-04/O-05, C-28), crypto (C-24).
- **Effort** : M par composant · **Sources** : <https://owasp.org/www-project-application-security-verification-standard/> · <https://softwaremill.com/whats-new-in-asvs-5-0/>

### OWASP MASVS 2.1.0 (jan. 2024) + MASTG + MASWE
- **Quoi** : 8 catégories (STORAGE, CRYPTO, AUTH, NETWORK, PLATFORM, CODE, RESILIENCE, PRIVACY) ; niveaux déplacés en **profils de test MAS-L1 / MAS-L2 / MAS-R** dans le MASTG.
- **Pertinence** : **ALIGN** — `apps/central-mobile` (Dioxus 0.7, WebView Android) : **MAS-L2 + MAS-R ciblé** (l'appareil appairé détient une clé d'accès au COG).
- **Contrôles** : STORAGE → clé d'appareil en Android Keystore adossé matériel (StrongBox si dispo), jamais en préférences ; NETWORK → épinglage de la clé du COG (déjà via appairage signé), pas de trafic clair ; PLATFORM → WebView sans accès fichiers/JS bridge minimal, `allowFileAccess=false`, liens profonds validés (EX-23) ; CODE → dépendances suivies dans la même CI ; RESILIENCE → détection root/débogueur informative (pas bloquante), signature APK v3+ ; PRIVACY → pas de SDK tiers/analytics ; **NC-STD-24** clé de signature de publication APK hors dépôt, rotation documentée (v3 lineage) ; révocation en un geste (EX-14).
- **Effort** : M · **Sources** : <https://mas.owasp.org/MASVS/> · <https://github.com/OWASP/owasp-masvs/releases>

### OWASP Top 10 for LLM Applications 2025 + Top 10 for Agentic Applications 2026 (09/12/2025)
- **Quoi** : LLM01 injection de prompt · LLM02 divulgation d'informations sensibles · LLM03 chaîne d'approvisionnement · LLM04 empoisonnement · LLM05 traitement de sortie · LLM06 agentivité excessive · LLM07 fuite de prompt système · LLM08 faiblesses vecteurs/embeddings · LLM09 désinformation · LLM10 consommation illimitée ; Agentic ASI01–ASI10 (détournement d'objectif, abus d'outils, abus d'identité/privilèges, mémoire empoisonnée, agent voyou…).
- **Pertinence** : **ALIGN prioritaire** pour MAIA/Alicia — deux constats critiques de la revue relèvent directement (K-01 SSRF outil = LLM06/ASI02 ; K-02 injection pilotant des actions = LLM01/ASI01).
- **Mapping** : LLM01/ASI01 → EX-13, EX-59 (double MAIA), EX-60, C-51 · LLM02 → EX-55, EX-47 · LLM03 → EX-26, C-53, C-72 (contredit : modèles non signés) · LLM04/ASI06 mémoire → EX-52, EX-53, EX-63 · LLM05 → C-28, EX-23 · LLM06/ASI02/ASI03 → C-50, C-12, StrongFather (lot C) · LLM07 → **NC-STD-25** aucun secret dans les prompts système · LLM08 → EX-62 · LLM09 → EX-61 (corroboration) · LLM10 → EX-11, EX-50 · **NC-STD-26** banc de tests d'injection (corpus OWASP + garak/PyRIT ou équivalent Rust) exécuté en CI sur chaque outil MAIA.
- **Effort** : M (lot 0 bis pour K-01/K-02) · L (lot 7) · **Sources** : <https://genai.owasp.org/llm-top-10/> · <https://genai.owasp.org/resource/owasp-top-10-for-agentic-applications-for-2026/> (URL à vérifier)

### SLSA v1.2 (novembre 2025 ; Build track + Source track)
- **Quoi** : niveaux de garantie de la chaîne de build : Build L1 provenance existante · L2 provenance **signée** par une plateforme hébergée · L3 builder durci, isolation entre builds, secrets de signature inaccessibles aux étapes utilisateur. Source track (nouveau) : L1–L3 (historique, provenance, contrôles continus).
- **Pertinence** : **ALIGN** — complète EX-01 (qui signe) par « comment l'artefact a été produit ».
- **Contrôles** : **NC-STD-21** provenance in-toto (format SLSA Provenance v1) générée en CI pour chaque binaire officiel → Build L2 au lot 4 ; L3 au lot 8 (builder isolé : runner éphémère dédié ou `slsa-github-generator`, clé de signature hors des jobs) ; **la vérification côté COG (zone COG, hors ligne) doit vérifier : sceau 2 sur 2 (EX-01) + provenance + empreinte reproductible (EX-04)** ; Source L2 : protection de branche + revue obligatoire (NC-STD-19).
- **Effort** : M (L2) · L (L3) · **Sources** : <https://slsa.dev/spec/v1.2/> · <https://slsa.dev/blog/2025/11/announce-slsa-v1.2>

### SBOM — CycloneDX 1.7 (ECMA-424 2ᵉ éd., déc. 2025) / SPDX 3.0 (SPDX 2.2.1 = ISO/IEC 5962:2021)
- **Quoi** : formats de nomenclature logicielle ; CycloneDX 1.7 ajoute un CBOM (inventaire cryptographique) enrichi ; SPDX 3.0 profils (sécurité, IA, données).
- **Pertinence** : **ALIGN obligatoire de fait** — CRA annexe I partie II exige une SBOM (au moins dépendances de 1er niveau, format lisible par machine) ; BSI TR-03183-2 précise (CycloneDX ≥ 1.5 ou SPDX ≥ 2.2.1, à vérifier).
- **Outils Rust** : `cargo-cyclonedx` (SBOM par crate/workspace) ; **`cargo-auditable`** (embarque la liste des dépendances dans le binaire → audit des binaires déployés par `cargo audit bin`) ; `cargo-sbom` (SPDX) ; Syft en appoint pour les images.
- **Contrôles** : **NC-STD-22** CycloneDX 1.6+/1.7 générée en CI pour chaque artefact officiel, signée (Sceau) et attachée au manifeste de paquet (EX-16) ; binaires compilés avec `cargo auditable build` ; CBOM CycloneDX pour inventorier les primitives (prépare l'agilité PQ, NC-STD-20) ; **AI-BOM** (CycloneDX ML-BOM / SPDX AI) pour chaque modèle MAIA distribué (NC-STD-09).
- **Effort** : S (génération) · M (signature + intégration manifeste) · **Sources** : <https://cyclonedx.org/news/cyclonedx-v1.7-released> · <https://ecma-international.org/publications-and-standards/standards/ecma-424/> · <https://github.com/CycloneDX/cyclonedx-rust-cargo> · <https://github.com/rust-secure-code/cargo-auditable> · <https://spdx.dev/>

### Compilations reproductibles
- **Quoi** : même source + même chaîne → binaire identique au bit près (reproducible-builds.org).
- **Pertinence** : EX-04, C-06 (déjà exigés). Rust : `rust-toolchain.toml` (C-05, absent), `--remap-path-prefix`, `SOURCE_DATE_EPOCH`, `-C codegen-units=1` si nécessaire, conteneur de build figé par empreinte ; vérification par 2 builders indépendants (un COG membre peut rebâtir → lien fédération).
- **Contrôle** : **NC-STD-27** job CI « double build + diffoscope » sur les Cores ; empreintes publiées au registre Merkle.
- **Effort** : M · **Source** : <https://reproducible-builds.org/docs/>

### Sigstore / in-toto vs format maison `miyusceau`
- **Constat** : Sigstore (cosign, Fulcio, Rekor) = signature sans clé longue durée, identité OIDC, journal de transparence **public et centralisé** → incompatible avec la souveraineté (dépendance à un tiers en ligne, LOI-1) pour la vérification côté COG.
- **Recommandation** : **garder `miyusceau` pour la chaîne de confiance COG** (2 sur 2, hors ligne, registre Merkle propre RFC 9162 — même modèle que Rekor) ; **adopter les formats** in-toto (attestations) et DSSE comme **charge utile** enveloppée dans un Sceau → interopérabilité outillage (slsa-verifier, OpenSSF) sans dépendance ; publier **en plus** une signature Sigstore des versions open source pour les vérificateurs externes (ALIGN, optionnel). **NC-STD-28** type Sceau dédié « attestation in-toto » (à spécifier, amendement doc gelée requis, C-82).
- **Sources** : <https://in-toto.io/> · <https://github.com/secure-systems-lab/dsse> · <https://docs.sigstore.dev/> · RFC 9162 <https://www.rfc-editor.org/rfc/rfc9162>

### OpenSSF Scorecard + OSPS Baseline
- **Quoi** : Scorecard = ~18 contrôles automatisés du dépôt (Branch-Protection, Code-Review, Pinned-Dependencies, Token-Permissions, Dangerous-Workflow, Fuzzing, SAST, Signed-Releases, Security-Policy, Vulnerabilities…) ; OSPS Baseline (2025) = niveaux de maturité projet open source, mappés CRA (à vérifier).
- **Pertinence** : **ALIGN** — mesure gratuite, objective, publiable.
- **Contrôles** : **NC-STD-29** action `ossf/scorecard-action` hebdomadaire ; corrections immédiates : actions épinglées par **SHA** (aujourd'hui `@1.99.0`), `permissions: contents: read` par défaut, `SECURITY.md` (NC-STD-10), fuzzing (EX-10 → coche Fuzzing), Dependabot/Renovate pour `Cargo.lock`, CodeQL ou Semgrep (SAST) en complément de clippy.
- **Effort** : S · **Sources** : <https://scorecard.dev/> · <https://baseline.openssf.org/>

### OpenChain ISO/IEC 5230:2020 (licences) et ISO/IEC 18974:2023 (sécurité open source)
- **Quoi** : 5230 = programme de conformité des licences open source ; 18974 = programme de gestion des vulnérabilités connues dans les composants open source ; **auto-certification gratuite** possible. ISO/IEC 20243 (O-TTPS) = fournisseur de technologie de confiance (contrefaçon, intégrité matérielle).
- **Pertinence** : 5230 **CERT (auto)** — C-71/C-72 et `deny.toml` en sont déjà 70 % ; 18974 **CERT (auto)** après NC-STD-11/22 ; 20243 **NR** (pas de fabrication matérielle ; à revoir si boîtier COG vendu).
- **Contrôles** : politique open source écrite, rôle « responsable conformité », archive des avis de licence par version (générés depuis la SBOM), processus 18974 = NC-STD-11 + `cargo audit`/`cargo deny advisories` + VEX.
- **Note** : `deny.toml` autorise **MPL-2.0** (copyleft faible fichier par fichier) alors que C-71 liste Apache/MIT/BSD/ISC/Zlib → **écart à arbitrer** (amender C-71 ou retirer MPL-2.0).
- **Effort** : S (5230) · M (18974) · **Sources** : <https://openchainproject.org/license-compliance> · <https://openchainproject.org/security-assurance> · <https://www.iso.org/standard/81039.html>

## 5. Cryptographie — alignement et agilité

### Courbes et primitives classiques
- **RFC 8032 (Ed25519)** — `verify_strict` (ed25519-dalek) rejette les points de petit ordre et les encodages non canoniques de R/A, mais **diffère de la règle ZIP-215** (vérification « cofactorisée », consensus) ; enjeu COG : dans la fédération (quorum de 6, registre Merkle, EX-32/EX-34), **tous les nœuds doivent appliquer exactement la même règle** sinon une signature valide pour certains et invalide pour d'autres crée un fork de consensus.
  - **NC-STD-20a** règle de vérification unique et normative pour Sceau v1 = `verify_strict` (dalek 2.x), écrite dans la spec du format, figée par vecteurs de test « cas limites » (petits ordres, S non réduit, R non canonique — vecteurs de « Taming the many EdDSAs », Chalkias et al. 2020) → C-61
  - interdire `verify` non strict dans tout le workspace (lint/grep CI)
- **RFC 7748 (X25519)** — contrôler le résultat tout-zéro (déjà fait : « X25519 contributif ») ; clés éphémères zeroizées (C-25).
- **RFC 5869 (HKDF)** — sel = domaine, `info` = contexte + rôles + transcription du handshake (lier les clés publiques des deux parties) ; une clé par sens (déjà fait).
- **XChaCha20-Poly1305** — **brouillon IRTF expiré, jamais RFC** ; construction largement analysée (HChaCha20 + RFC 8439) et sûre, mais **non normalisée** et **non approuvée NIST/ANSSI** (référentiel RGS/guide de sélection ANSSI : ChaCha20-Poly1305 accepté, XChaCha à vérifier). Acceptable pour un produit non certifié ; si une certification (EUCC, qualification ANSSI) est visée → prévoir AES-256-GCM-SIV (RFC 8452) ou ChaCha20-Poly1305 RFC 8439 avec nonces dérivés. **NC-STD-20b** identifiant d'algorithme versionné dans chaque format chiffré (agilité).
- **NIST SP 800-56C r2** (dérivation de clés après accord) — HKDF conforme (option « two-step ») ; **SP 800-56A r3** concerne ECC-CDH sur courbes NIST (X25519 hors champ FIPS, admis par SP 800-186 pour EdDSA/Curve25519 côté signature/courbes — à vérifier pour l'accord de clés).
- **NIST SP 800-57 Part 1 r5** — durées de vie des clés (cryptopériodes) : **NC-STD-20c** table des cryptopériodes par type de clé (racine SLIP-39 EX-24, clés d'usage SLIP-10 EX-25, clés de session, clés de DB EX-21) et procédure de rotation testée.
- **NIST SP 800-38D (GCM)** — limites d'usage (2³² messages par clé avec nonce aléatoire 96 bits) → justifie XChaCha ou GCM-SIV pour nonces aléatoires ; AES-GCM est utilisé dans 3 crates : vérifier la gestion des nonces (**à vérifier** dans le code).
- **Argon2id (RFC 9106)** — paramètres conformes (m ≥ 19 MiB, t ≥ 2 selon OWASP ; profil RFC 9106 « second recommended » m = 64 MiB, t = 3) à vérifier dans `kindmother-db-key`.
- **SLIP-39 / SLIP-10** — normes communautaires (SatoshiLabs), pas ISO/IETF ; acceptables, documenter le choix (vecteurs officiels en test, C-61).

### Post-quantique (FIPS 203/204/205, août 2024)
- **Normes** : FIPS 203 ML-KEM · FIPS 204 ML-DSA · FIPS 205 SLH-DSA (finales, 13/08/2024) ; FIPS 206 FN-DSA (Falcon) **encore en projet** (finale attendue fin 2026 / 2027) ; SP 800-227 (recommandations KEM, finalisée 2025 — **à vérifier**).
- **Calendriers** : NIST IR 8547 (projet, nov. 2024) : algorithmes à clé publique classiques (RSA, ECDSA, **EdDSA**, ECDH) **dépréciés après 2030, interdits après 2035** (statut final à vérifier) ; **ANSSI** : hybridation **fortement recommandée** (KEM **et** signature) pour les produits devant protéger des informations au-delà de 2030, ML-KEM / FrodoKEM, ML-DSA / SLH-DSA acceptés, PQ seul déconseillé à court terme ; **BSI TR-02102-1** (2025) : hybride, ML-KEM-768/1024, FrodoKEM, Classic McEliece (à vérifier) ; feuille de route UE coordonnée (juin 2025) : démarrage avant fin 2026, infrastructures critiques avant 2030 (à vérifier).
- **Risque COG** : « récolter maintenant, déchiffrer plus tard » sur le bridge E2E et les sauvegardes fédérées (EX-27) → **prioritaire** ; signatures (Sceau, racine) → moins urgent mais **racine longue durée** (EX-24) = à prévoir dans le format dès maintenant.
- **Contrôles**
  - **NC-STD-20d** (agilité, **[B] dès lot 2**) : Sceau v1 porte un identifiant d'algorithme de signature ; une ancre peut déclarer plusieurs clés (classique + PQ) ; vérificateur avec liste d'algorithmes autorisés pilotée par le registre (EX-39, activation par seuil)
  - **NC-STD-20e** (lot 6–8) : handshake du bridge E2E et de la fédération en **X25519 + ML-KEM-768 hybride** (combinaison type X-Wing / draft-ietf-tls-ecdhe-mlkem — à vérifier) ; crate `ml-kem` (RustCrypto) ou `aws-lc-rs` (à évaluer, C-24 à amender)
  - **NC-STD-20f** (lot 8, avant 2030) : signature composite **Ed25519 + ML-DSA-65** pour les sceaux d'Origin et de la racine (draft-ietf-lamps-pq-composite-sigs, à vérifier) ; SLH-DSA envisageable pour la clé éditeur hors ligne (signatures rares, hypothèses minimales)
  - CBOM CycloneDX (NC-STD-22) pour inventorier où chaque primitive est utilisée
- **Sources** : FIPS 203/204/205 <https://csrc.nist.gov/pubs/fips/203/final> · <https://csrc.nist.gov/pubs/fips/204/final> · <https://csrc.nist.gov/pubs/fips/205/final> · NIST IR 8547 <https://csrc.nist.gov/pubs/ir/8547/ipd> · ANSSI (résumé) <https://postquantum.com/security-pqc/anssi-france-hybrid-pqc-position/> · ANSSI primaire <https://cyber.gouv.fr/publications/avis-de-lanssi-sur-la-migration-vers-la-cryptographie-post-quantique> (URL à vérifier) · BSI TR-02102 <https://www.bsi.bund.de/EN/Themen/Unternehmen-und-Organisationen/Standards-und-Zertifizierung/Technische-Richtlinien/TR-nach-Thema-sortiert/tr02102/tr02102_node.html> · RFC 8032 <https://www.rfc-editor.org/rfc/rfc8032> · RFC 7748 <https://www.rfc-editor.org/rfc/rfc7748> · RFC 5869 <https://www.rfc-editor.org/rfc/rfc5869> · XChaCha <https://datatracker.ietf.org/doc/draft-irtf-cfrg-xchacha/> · SP 800-57 <https://csrc.nist.gov/pubs/sp/800/57/pt1/r5/final> · SP 800-56C <https://csrc.nist.gov/pubs/sp/800/56/c/r2/final>

## 6. Récapitulatif des nouvelles lignes NC-STD

| ID | Ligne proposée | Norme(s) | Lie | Niveau | Lot |
|---|---|---|---|---|---|
| NC-STD-01 | Registre d'incidents Origin + runbook | 27001 A.5.24–28, 62443-4-1 DM | EX-12 | [R] | 4 |
| NC-STD-02 | Journaux Origin signés, append-only, rétention | 27001 A.8.15 | EX-22, EX-34 | [R] | 4 |
| NC-STD-03 | Veille vulnérabilités dépendances + binaires (`cargo audit bin`) | 27001 A.8.8, 18974 | EX-15, C-04 | [B] | 0 bis |
| NC-STD-04 | Registre de risques coté, revu par lot | 27005 | modèle de menaces | [R] | 0 bis |
| NC-STD-05 | Origin/pairs ne détiennent que du chiffré (preuve) | 27017/27018 | EX-27 | [V] | 6 |
| NC-STD-06 | Aucune télémétrie par défaut ; opt-in signé | 27701, EN 303 645 5.8 | C-21 | [B] | 0 bis |
| NC-STD-07 | AIPD MAIA mémoire et Alicia | 27701, 42001 | EX-50..56 | [R] | 7 |
| NC-STD-08 | Revue de désidentification avant toute publication fédérale | 27559, 29100 | EX-47, EX-57 | [R] | 7 |
| NC-STD-09 | Fiche modèle + AI-BOM signée par modèle | 42001, EN 304 223 | EX-26, C-53, C-72 | [R] | 4 |
| NC-STD-10 | `SECURITY.md` + `security.txt` (RFC 9116) | 29147, EN 303 645 5.2 | — | [B] | 0 bis |
| NC-STD-11 | Processus 30111 + CVSS 4.0 + avis signés | 30111, CVSS 4.0 | EX-01/02, EX-34 | [B] | 0 bis |
| NC-STD-12 | Runbook CRA art. 14 (SRP ENISA 24 h/72 h/14 j) | CRA, 29147 | — | [B] | 0 bis |
| NC-STD-13 | Avis CSAF 2.0 + VEX | 29147 | NC-STD-22 | [R] | 4 |
| NC-STD-14 | Secret d'appairage unique, aucun compte usine | EN 303 645 5.1 | C-21/22, EX-28 | [B] | 2 |
| NC-STD-15 | Durée de support annoncée par produit | EN 303 645 5.3, CRA | EX-02 | [R] | 4 |
| NC-STD-16 | Démarrage vérifié/mesuré du boîtier (Secure Boot + TPM) | EN 303 645 5.7, 62443-4-2 CR 3.4 | EX-28, EX-31 | [V] | 8 |
| NC-STD-17 | Guide de durcissement/déploiement par produit | 62443-4-1 SG | EX-48 | [R] | 5 |
| NC-STD-18 | Veille JTC 13 + correspondance CRA annexe I ↔ EX/C | EN 40000, prEN 50770 | — | [R] | continu |
| NC-STD-19 | Protection de branche, revue obligatoire, commits signés | SSDF PS.1, SLSA Source L2 | C-80 | [B] | 0 bis |
| NC-STD-20a..f | Règle Ed25519 normative, ID d'algo, cryptopériodes, agilité, hybride PQ KEM puis signature | RFC 8032, SP 800-57, FIPS 203/204 | C-24, EX-24/25 | a,b,d [B] lot 2 · c [R] · e,f [V] | 2 → 8 |
| NC-STD-21 | Provenance SLSA signée (L2 puis L3) vérifiée en zone COG | SLSA v1.2 | EX-01, EX-04 | [R] | 4 / 8 |
| NC-STD-22 | SBOM CycloneDX signée + `cargo auditable` + CBOM | CRA, CycloneDX 1.7 | EX-16 | [R] | 4 |
| NC-STD-23 | Checklist ASVS 5.0 par composant (L2/L3) | ASVS 5.0 | C-20..28 | [R] | 0 bis → 5 |
| NC-STD-24 | Clé de signature APK hors dépôt, rotation v3 | MASVS | EX-14 | [B] | 2 |
| NC-STD-25 | Aucun secret dans les prompts système | LLM07 | C-25 | [B] | 0 bis |
| NC-STD-26 | Banc de tests d'injection MAIA en CI | LLM01, ASI01 | EX-13, EX-60 | [R] | 0 bis → 7 |
| NC-STD-27 | Double build + diffoscope sur les Cores | reproducible-builds, SLSA | EX-04, C-05/06 | [R] | 8 |
| NC-STD-28 | Type Sceau « attestation in-toto/DSSE » | in-toto, SLSA | EX-01 | [V] | 4 |
| NC-STD-29 | OpenSSF Scorecard hebdo ; actions épinglées par SHA ; permissions minimales | Scorecard | C-04 | [B] | 0 bis |

## 7. Feuille de route intégrée au plan

- **Lot 0 bis (maintenant, S–M)** — tout ce qui est bon marché et exigé par le CRA déjà en vigueur
  - NC-STD-10/11/12 : `SECURITY.md`, `security.txt`, processus 30111, runbook SRP ENISA (obligation active depuis le 11/09/2026 si un produit est mis sur le marché UE)
  - NC-STD-29/19/03 : Scorecard, épinglage SHA des actions, protection de branche, `cargo audit` des binaires
  - OWASP LLM01/LLM06/ASI01–02 sur K-01, K-02 (NC-STD-25, amorce NC-STD-26)
  - ASVS 5.0 L2 sur les constats O-0x, CA-0x (XSS, SSRF, auth Tracker) — NC-STD-23
  - NC-STD-04 registre de risques ; NC-STD-06 pas de télémétrie ; arbitrer MPL-2.0 dans `deny.toml` vs C-71
- **Lot C (Cores réels)** — ASVS V8 (autorisation) : refus par défaut prouvé ; 62443-4-2 CR 2.1 (application des autorisations) ; LLM06 (StrongFather devant chaque outil MAIA)
- **Lot 2 (identité)** — ASVS L3 V6/V9/V11 sur `miyauth`/appairage ; NC-STD-20a/b/d (règle Ed25519 normative, ID d'algorithme, agilité dans Sceau v1 **avant** que le format ne se fige en production) ; SP 800-57 cryptopériodes (20c) ; MASVS-L2 du mobile (NC-STD-24, NC-STD-14)
- **Lot 3 (frontières)** — 62443-4-2 SL-C 2 pour la Douane ; EN 303 645 5.5/5.6 ; ASVS L3 Portail ; test d'intrusion (62443-4-1 SVV-4) = sortie de lot ; classer la Douane au regard de l'annexe III CRA (à vérifier)
- **Lot 4 (canal officiel)** — SLSA Build L2 (NC-STD-21), SBOM/CBOM signées (NC-STD-22), CSAF/VEX (NC-STD-13), fiches modèles (NC-STD-09), durée de support (NC-STD-15) ; OpenChain 5230 auto-certification ; SoA ISO 27001 interne pour Origin (NC-STD-01/02)
- **Lot 5 (services hybrides)** — ASVS L2 par service migré (JayRDV puis MiyuCloud) ; guide de durcissement (NC-STD-17) ; EN 303 645 pour Alicia (Franche locale)
- **Lot 6 (fédération)** — 27701 (Origin responsable de traitement) ; NC-STD-05 ; hybride PQ du handshake fédéral (NC-STD-20e) ; vérification Ed25519 identique sur tous les nœuds (tests croisés)
- **Lot 7 (MAIA)** — OWASP LLM 2025 + Agentic 2026 complet ; ETSI EN 304 223 ; ISO 42001 (alignement) ; AIPD (NC-STD-07) ; désidentification 27559 (NC-STD-08)
- **Lot 8 (durcissement continu)** — SLSA Build L3 + builds reproductibles (NC-STD-27) ; OpenChain 18974 ; IEC 62443-4-1 ML2 ; signatures hybrides ML-DSA (NC-STD-20f) avant 2030 ; audit externe (ASVS L2/L3 + MASVS) ; décider la certification ISO 27001 d'Origin avant ouverture publique ; veille JTC 13 et citation JOUE des EN 40000 (NC-STD-18)

## 8. Points « à vérifier »

- Statut final de NIST SP 800-218 r1 (SSDF 1.2) et de NIST IR 8547 (finalisé ou non en 2026)
- Dates exactes de livraison des normes harmonisées CRA (amendement de la demande M/606) et citation de prEN 40000-1-3
- Version courante d'ETSI EN 303 645 (V3.1.3 ?) et citation d'EN 18031 au JOUE (avec ou sans restrictions)
- Classement CRA de la Douane (pare-feu/IDS, annexe III classe I ou II) et du boîtier COG (passerelle domotique)
- Position ANSSI la plus récente (suivi 2023, éventuelle mise à jour 2025) et BSI TR-02102-1 2025 ; statut SP 800-227 et FIPS 206
- Acceptation de XChaCha20-Poly1305 par le guide de sélection ANSSI
- Gestion des nonces AES-GCM dans les 3 crates qui l'utilisent ; paramètres Argon2id de `kindmother-db-key`
- URL exactes : ressource OWASP Agentic 2026, chemin ETSI EN 304 223, avis ANSSI sur cyber.gouv.fr
- Exigences précises de format SBOM (BSI TR-03183-2 vs futur acte CRA)
