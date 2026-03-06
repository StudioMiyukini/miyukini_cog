# P0 Temps 6 - Specification technique

## Statut

- Etat : Termine
- Phase : P0 Temps 6
- Responsable principal : Francois
- Date : 2026-03-05

## TL;DR

T6 produit une specification technique detaillee de Miyukini Connect, alignee sur T1..T5. Le design finalise couvre: architecture modulaire, contrats API v1, schema de donnees, modeles de session AAL, comportements online/offline/suspicious, controles securite MUST, criteres PASS et plan de migration depuis l'auth Central legacy.

## Perimetre T6

1. Consolidation architecture option D (local-first durci).
2. Definition des contrats API inter-services.
3. Definition modele de donnees et claims session.
4. Integration complete des exigences securite T5.
5. Definition prete pour execution P3.

## Decisions techniques valides

1. **Service dedie** `miyukini-connect` lance conjointement avec Central.
2. **Auth locale souveraine** comme chemin critique, Origin en extension facultative.
3. **Moteur AAL** central (AAL1..AAL4) + `permission_tier` harmonise.
4. **Step-up obligatoire** pour actions sensibles/critique.
5. **Mode isolation durci**: gel enrolment/recovery faible + session integrity checks.
6. **Journal auth chaine par hash** pour preuve locale anti-tampering.

## Livrable principal

Specification detaillee:

- `.mip/sequences/2026-03-05-miyukini-connect-auth-general/specs/2026-03-05-miyukini-connect-auth-general-spec.md`

## Resultat T6

- Spec technique complete et exploitable pour lancer P3.
- Preconditions P3 clarifiees (API, data, securite, test).
- Passage recommande vers T7.
