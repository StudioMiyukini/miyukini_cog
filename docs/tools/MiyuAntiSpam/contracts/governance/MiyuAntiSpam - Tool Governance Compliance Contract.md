# MiyuAntiSpam — Tool Governance Compliance Contract

## Contexte

Conformité aux obligations communes : [Master Butler - Tool Governance Compliance Template](../../../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Compliance%20Template.md).

**ToolkitId :** `toolkit.security.antispam`

---

## Obligations spécifiques MiyuAntiSpam

- **Décision de bloquer ou autoriser** (après CAPTCHA, flood, rate limit) = **StrongFather**.
- Les Tools **exécutent** (générer, vérifier, compter) et renvoient un résultat ; ils ne décident pas.
- Seuils (flood, rate limit) = fournis dans le flux ou lus depuis KindMother ; pas de décision métier par le Tool.

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Contrat de conformité
