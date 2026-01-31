# MiyuPolls — Tool Governance Compliance Contract

## Contexte

Conformité aux obligations communes : [Master Butler - Tool Governance Compliance Template](../../../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Compliance%20Template.md).

**ToolkitId :** `toolkit.content.polls`

---

## Obligations spécifiques MiyuPolls

- **Décision** (création sondage, vote autorisé, clôture) = **StrongFather**.
- Toute écriture (sondage, vote) = **WriteIntent** vers KindMother.
- Règles d'unicité de vote (un vote par utilisateur par sondage) = politique StrongFather ; l'outil exécute la vérification fournie.

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Contrat de conformité
