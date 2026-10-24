# MiyuSceau — index

<!-- @id tools.miyusceau.index -->
<!-- @role toolkit-reference -->
<!-- @layer T -->
<!-- @human Toolkit des titres signés « Sceau v1.1 » : passeports, visas, manifestes, registre, certificats, reconnaissances -->
<!-- @do reference_miyusceau -->

- **Crate** : `crates/miyusceau` · **Strate** : T (toolkit sécurité)
- **Spécification du format** : [lot 2 — identité signée](../../securite/ARCHITECTURE-lot2-identite-signee.md) §4
- **Statut** : lot 2, incrément 1 (format, signature, vérification) + format v1.1 de l'incrément 2 (algorithme, multi-signature à seuil, amendement A-002) ; 30 tests, vecteurs de référence simple et 2 sur 2 figés

## 1. Principes

<!-- @id tools.miyusceau.principes -->
<!-- @role security-principles -->
<!-- @do summarize_sceau_principles -->

- Une seule enveloppe pour tous les titres d'identité du COG
- Ed25519 vérifié avec `verify_strict` (clés faibles, signatures malléables refusées)
- Encodage binaire canonique : une seule représentation valide par titre
- Séparation de domaine : version, algorithme et type du titre font partie des octets signés
- Multi-signature : 1 à 8 signataires distincts, vérification à seuil
- **Fail-closed** : format inattendu, signataire inconnu, titre hors fenêtre → refus

## 2. Format Sceau v1.1

<!-- @id tools.miyusceau.format -->
<!-- @role binary-format -->
<!-- @do describe_sceau_format -->

- Octets : `MSCE` · version `0x02` · alg · type · longueur u32 BE · charge TLV · nombre (1–8) · signatures
  - signature : kid (32 o) · alg · longueur u16 BE · octets
- Octets signés (communs à tous les signataires) : `"miyukini/sceau/v1.1\0"` ‖ version ‖ alg ‖ type ‖ longueur ‖ charge
- Algorithmes : `0x01` Ed25519 (implémenté) ; `0x02` ML-DSA-65 et `0x03` hybride **réservés, refusés** ; autre code refusé
- Signatures : `kid` strictement croissants, doublon refusé, alg = alg de l'entête, longueur imposée
- Sceau v1.0 (version `0x01`) : abandonné, refusé
- Charge : TLV, tags strictement croissants, aucun champ non lu toléré ; ≤ 65 536 o (`TAILLE_MAX_CHARGE`)
- Horloge : tolérance ± 120 s (`TOLERANCE_HORLOGE_SECS`)
- `kid` = SHA-256 de la clé publique (`empreinte_cle`)
- Spécification normative : [lot 2 §4.1](../../securite/ARCHITECTURE-lot2-identite-signee.md)

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

- `sceller(&titre, &CleSignature) -> Vec<u8>` · `sceller_multi(&titre, &[&CleSignature])`
- `cosigner::<T>(octets, &CleSignature)` : ajoute une signature (type et charge contrôlés, doublon refusé)
- `verifier::<T>(octets, &AncresDeConfiance, maintenant_secs) -> Verifie<T>` (seuil 1)
- `verifier_seuil::<T>(octets, &AncresDeConfiance, seuil, maintenant_secs)`
  - ordre : format → type attendu → algorithme implémenté → `verify_strict` de chaque signataire épinglé (une invalide = refus) → seuil → décodage canonique → fenêtre de validité
  - signataires non épinglés : non comptés ; niveau = le plus faible des signataires comptés
- `Titre` : trait des titres (`TYPE`, `encoder`, `decoder`, `fenetre_validite`)
- `Verifie<T>` : seuls `verifier` / `verifier_seuil` le construisent ; le détenir prouve le contrôle (titre, `signataires()`, niveau)
- `AncresDeConfiance` : clés publiques épinglées ; vide = refuse tout
  - `NiveauConfiance::Souverain` (BorderGuard TRUSTED) · `Reconnu` (VERIFIED)
- `EcrivainTlv` / `LecteurTlv` : encodage des champs
- `ErreurSceau` : une variante par cause de refus (taille, magic, version, algorithme, type, signataire, doublon, seuil, signature, TLV…)

## 4. Suite prévue

<!-- @id tools.miyusceau.suite -->
<!-- @role roadmap -->
<!-- @do plan_miyusceau_next -->

- Politique 2 sur 2 (Origin + éditeur) branchée sur les mises à jour, fraîcheur de tête, fuzzing du décodeur
- ML-DSA-65 et hybride (activation par nouvel amendement)
- Racine COG SLIP-39 (USB hors ligne) et clés dérivées SLIP-10
- Type `0x08 Fiche` pour l'IA fédérée
