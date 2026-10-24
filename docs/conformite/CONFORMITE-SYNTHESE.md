# Conformité COG — synthèse

<!-- @id conformite.synthese -->
<!-- @role compliance-summary -->
<!-- @layer 0-9+zones -->
<!-- @human Point d'entrée conformité : échéances légales, cibles de certification, décisions à prendre, intégration au plan -->
<!-- @do summarize_compliance -->

- **Date** : 2026-10-24 · **Statut** : recherche documentaire, **pas un avis juridique** ; les points « à vérifier » doivent être confirmés (juriste, ANSSI, organismes)
- **Rapports** : [ANSSI et souveraineté](ANSSI-ET-SOUVERAINETE.md) (NC-ANSSI-01..44) · [Réglementaire UE/FR](REGLEMENTAIRE-UE-FR.md) (NC-REG-01..20) · [Normes et standards](NORMES-ET-STANDARDS.md) (NC-STD-01..29)
- **Base** : [doc gelée](../architecture/GEL-DOC-CONCEPTUELLE-v1.md) · [contraintes](../implementation/CONTRAINTES-IMPLEMENTATION.md) · [plan](../implementation/PLAN-IMPLEMENTATION.md) · [matrice](../implementation/MATRICE-EXIGENCES.md)

## 1. Échéances légales

<!-- @id conformite.synthese.echeances -->
<!-- @role legal-deadlines -->
<!-- @do list_legal_deadlines -->

| Échéance | Obligation | Concerne | Statut COG |
|---|---|---|---|
| **en vigueur** (11/09/2026) | CRA : signalement ENISA des vulnérabilités activement exploitées et incidents graves (24 h / 72 h / rapport final), y compris produits déjà diffusés | éditeur | **absent** : pas de SECURITY.md, security.txt, processus |
| **en vigueur** (02/08/2026) | AI Act art. 50 : MAIA et Alicia se déclarent IA, marquage des contenus générés | MAIA, Alicia | absent |
| **en vigueur** | RGPD : AIPD (Origin, Market, notation des pairs, voix), registre, privacy by design | Origin, services | partiel |
| **en vigueur** | LCEN / DSA : obligations d'hébergeur pour Origin et le Market (signalement, retrait) | Origin, Market | absent |
| **avant diffusion** | Déclaration des moyens de cryptologie à l'ANSSI ; classification double usage hors UE | COG entier | absent — exemption open source **à vérifier** |
| 01/09/2026 · 01/09/2027 | Facturation électronique : réception via plateforme agréée, émission PME/TPE | JayKonta | absent (FEC, clôture inaltérable aussi) |
| 09/12/2026 | Directive responsabilité du fait des produits révisée (logiciels inclus) | éditeur | à anticiper |
| 11/12/2027 | CRA complet : exigences essentielles, SBOM, support 5 ans, marquage CE, évaluation de conformité | COG entier | partiel |
| 2027 (annoncé) | ANSSI : post-quantique attendu pour les candidats à certification | identité, canal officiel | absent |
| 2030 | Sécurité post-quantique visée pour tout usage au-delà de 2030 | crypto | absent |

- **NIS2** : non transposée en France (projet retiré de l'ordre du jour le 06/10/2026) — touche COG via la chaîne d'approvisionnement des clients

## 2. Décisions à prendre (propriétaire)

<!-- @id conformite.synthese.decisions -->
<!-- @role owner-decisions -->
<!-- @do list_compliance_decisions -->

1. **Positionnement CRA** : présenter COG comme pare-feu / IDS / IPS le classe probablement en **classe II** (évaluation par tiers) ; sinon **classe I** (OS) — décider avant fin 2026
2. **Hébergement d'Origin** : quitter un VPS non qualifié pour un hébergeur UE (SecNumCloud si marchés publics) ; **refuser un anti-DDoS américain** qui déchiffrerait le trafic d'Origin
3. **Licence MPL-2.0** : C-71 l'exclut mais `deny.toml` l'autorise (dépendances Rust courantes) → amender C-71 (copyleft par fichier, acceptable) ou retirer ces dépendances
4. **Cryptographie** : C-24 ne correspond pas au code (x25519, chacha20poly1305, hkdf, aes-gcm, argon2, blake3, ring) ; l'ANSSI (PG-083 v3.00) ne cite pas Ed25519/X25519 → inventaire crypto, justification ou avis ANSSI ; **identifiant d'algorithme dans Sceau** avant gel du format (amendement de la doc gelée)
5. **Feuille de route post-quantique** : hybride X25519 + ML-KEM-768 (pont mobile, TLS), Sceau v2 Ed25519 + ML-DSA ; TLS hybride probablement via `aws-lc-rs` plutôt que `ring` (à vérifier)
6. **Budget** : audit PASSI (≈ 10–30 k€, à vérifier) puis CSPN (≈ 30–60 k€, ≈ 8 semaines, à vérifier)
7. **Gemma** : **pas de conflit avec C-72** pour Gemma 4 (Apache-2.0, vérifié sur les fiches le 2026-10-24) ; Gemma ≤ 3 reste exclu (« Gemma Terms of Use »)

## 3. Cibles

<!-- @id conformite.synthese.cibles -->
<!-- @role compliance-targets -->
<!-- @do set_compliance_targets -->

| Domaine | Cible | Composants |
|---|---|---|
| Vulnérabilités | ISO/IEC 29147 + 30111, RFC 9116, CVSS 4.0 | éditeur |
| Applicatif | OWASP ASVS 5.0 **L2** | Central, Origin, portail web, MiyuCloud, Market |
| Applicatif | OWASP ASVS 5.0 **L3** | miyauth, pont E2E, miyusceau, KindMother, Portail |
| IA | OWASP Top 10 LLM 2025 + Agentic, ETSI EN 304 223 | MAIA, Alicia |
| Mobile | OWASP MASVS 2.1 **L2** + R sélectif | central-mobile |
| Chaîne d'approvisionnement | SLSA Build **L2** (lot 4) → **L3** + builds reproductibles (lot 8), SBOM CycloneDX signé, OpenSSF Scorecard ≥ 7 | CI, Market |
| IoT / domotique | ETSI EN 303 645, IEC 62443-4-1 / 4-2 SL-C 2 | Alicia, boîtier, Douane |
| Développement | Guide ANSSI Rust, NIST SSDF | tout le code |
| Organisation | ISO 27001 (Origin), 27701 et 42001 en alignement | éditeur, Origin |
| **1re certification** | **CSPN** sur canal de mise à jour + vérificateur Sceau (`miyusceau`) à ≈ T+6–9 mois | miyusceau |

## 4. Intégration au plan

<!-- @id conformite.synthese.plan -->
<!-- @role plan-integration -->
<!-- @do integrate_compliance_plan -->

- **Lot 0 bis (en cours)** : SECURITY.md, security.txt, processus de signalement CRA (24 h / 72 h) ; mention IA (AI Act art. 50) dans MAIA et Alicia ; actions CI épinglées par empreinte
- **Lot C** : journalisation conforme ANSSI des décisions des Cores (traçabilité RGPD et CRA)
- **Lot 2** : inventaire crypto et C-24 révisée ; identifiant d'algorithme dans Sceau ; préparation post-quantique ; `rust-toolchain.toml`, `overflow-checks`, `cargo vet`
- **Lot 3** : EN 303 645 / IEC 62443 pour Douane et Alicia ; isolement en processus séparé des bindings C/C++ (llama.cpp, whisper.cpp)
- **Lot 4** : SBOM signé, SLSA L2, canal de mise à jour prêt pour CSPN
- **Lot 5** : AIPD des services ; facturation électronique, FEC et clôture inaltérable pour JayKonta
- **Lot 6** : AIPD de la fédération (notation des pairs, sanctions) ; Origin hébergé en UE
- **Lot 8** : SLSA L3, builds reproductibles, audit PASSI puis CSPN
- **Hors code** : déclaration de cryptologie ANSSI, avis juridique CRA/RGPD/facturation, devis PASSI/CSPN
