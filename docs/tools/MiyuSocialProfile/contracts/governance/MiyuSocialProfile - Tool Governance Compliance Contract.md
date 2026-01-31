# MiyuSocialProfile — Tool Governance Compliance Contract

## Contexte

Conformité aux obligations communes : [Master Butler - Tool Governance Compliance Template](../../../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Compliance%20Template.md).

**ToolkitId :** `toolkit.social.profile`

---

## Obligations spécifiques MiyuSocialProfile

- **Décision** (modification profil autorisée, follow autorisé) = **StrongFather**.
- Toute écriture (profil, follow add/remove) = **WriteIntent** vers KindMother.
- Distinction avec **MiyuProfile** (toolkit.identity.profile) : MiyuSocialProfile = profil social (bio, liens, abonnés) ; MiyuProfile = profil étendu forum (signature, avatar, rangs).

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Contrat de conformité
