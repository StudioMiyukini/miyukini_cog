# MiyuSceau — index

<!-- @id tools.miyusceau.index -->
<!-- @role toolkit-reference -->
<!-- @layer T -->
<!-- @human Toolkit des titres signés « Sceau v1 » : passeports, visas, manifestes, registre, certificats, reconnaissances -->
<!-- @do reference_miyusceau -->

- **Crate** : `crates/miyusceau` · **Strate** : T (toolkit sécurité)
- **Spécification du format** : [lot 2 — identité signée](../../securite/ARCHITECTURE-lot2-identite-signee.md) §4
- **Statut** : incrément 1 du lot 2 (format, signature, vérification) ; 15 tests, vecteur de référence figé

## 1. Principes

<!-- @id tools.miyusceau.principes -->
<!-- @role security-principles -->
<!-- @do summarize_sceau_principles -->

- Une seule enveloppe pour tous les titres d'identité du COG
- Ed25519 vérifié avec `verify_strict` (clés faibles, signatures malléables refusées)
- Encodage binaire canonique : une seule représentation valide par titre
- Séparation de domaine : le type du titre fait partie des octets signés
- **Fail-closed** : format inattendu, signataire inconnu, titre hors fenêtre → refus

## 2. Format Sceau v1

<!-- @id tools.miyusceau.format -->
<!-- @role binary-format -->
<!-- @do describe_sceau_format -->

- Octets : `MSCE` · version `0x01` · type · kid (32 o) · longueur u32 BE · charge TLV · signature (64 o)
- Octets signés : `"miyukini/sceau/v1\0"` ‖ version ‖ type ‖ kid ‖ longueur ‖ charge
- Charge : TLV, tags strictement croissants, aucun champ non lu toléré ; ≤ 65 536 o (`TAILLE_MAX_CHARGE`)
- Horloge : tolérance ± 120 s (`TOLERANCE_HORLOGE_SECS`)
- `kid` = SHA-256 de la clé publique (`empreinte_cle`)

| Code | Type |
|---|---|
| `0x01` | Passeport (personne + appareil) |
| `0x02` | Visa (droit d'entrée ciblé) |
| `0x03` | Manifeste de paquet |
| `0x04` | Entrée du registre |
| `0x05` | Tête signée du registre |
| `0x06` | Certificat (clé d'émission, clé éditeur) |
| `0x07` | Reconnaissance entre COG |
| `0x08` | Fiche IA fédérée — **prévu** ([IA fédérée](../../architecture/IA-FEDEREE.md)), non implémenté |

## 3. API

<!-- @id tools.miyusceau.api -->
<!-- @role public-api -->
<!-- @do list_miyusceau_api -->

- `sceller(&titre, &CleSignature) -> Vec<u8>`
- `verifier::<T>(octets, &AncresDeConfiance, maintenant_secs) -> Verifie<T>`
  - ordre : format → type attendu → signataire épinglé → `verify_strict` → décodage canonique → fenêtre de validité
- `Titre` : trait des titres (`TYPE`, `encoder`, `decoder`, `fenetre_validite`)
- `Verifie<T>` : seul `verifier` le construit ; le détenir prouve le contrôle (titre, signataire, niveau)
- `AncresDeConfiance` : clés publiques épinglées ; vide = refuse tout
  - `NiveauConfiance::Souverain` (BorderGuard TRUSTED) · `Reconnu` (VERIFIED)
- `EcrivainTlv` / `LecteurTlv` : encodage des champs
- `ErreurSceau` : une variante par cause de refus (taille, magic, version, type, signataire, signature, TLV…)

## 4. Suite prévue

<!-- @id tools.miyusceau.suite -->
<!-- @role roadmap -->
<!-- @do plan_miyusceau_next -->

- Signature à seuil et fraîcheur (Origin 2 sur 2), fuzzing du décodeur
- Racine COG SLIP-39 (USB hors ligne) et clés dérivées SLIP-10
- Type `0x08 Fiche` pour l'IA fédérée
