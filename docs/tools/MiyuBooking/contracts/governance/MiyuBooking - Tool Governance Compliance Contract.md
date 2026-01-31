# MiyuBooking — Tool Governance Compliance Contract

## Contexte

Conformité aux obligations communes : [Master Butler - Tool Governance Compliance Template](../../../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Compliance%20Template.md).

**ToolkitId :** `toolkit.booking.reservations`

---

## Obligations spécifiques MiyuBooking

- **Décision** (création réservation, annulation) = StrongFather ; aucun Tool n'exécute de décision métier.
- **Toute écriture** (réservation) = **WriteIntent** vers KindMother ; aucun accès direct à la persistance depuis le kit.
- **Schéma et périmètre** (règles de créneaux, ressources, réservations, tarifs) = KindMother ; les règles sont fournies par KindMother ou dans le flux.
- **Niveau de sécurité :** lecture créneaux 0–1, création / annulation réservation 1–2 ; cohérent WorrySentinel.
- Les créneaux peuvent être exposés dans le fuseau horaire de l'utilisateur ; les règles de disponibilité et ressources sont des données KindMother.

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Contrat de conformité
