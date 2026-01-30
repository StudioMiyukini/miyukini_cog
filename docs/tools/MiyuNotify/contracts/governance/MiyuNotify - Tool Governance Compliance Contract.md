# MiyuNotify — Tool Governance Compliance Contract

## Contexte

Conformité aux obligations communes : [Master Butler - Tool Governance Compliance Template](../../../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Compliance%20Template.md).

**ToolkitId :** `toolkit.notify.miyunotify`

---

## Obligations spécifiques MiyuNotify

- **Décision d'envoi** (email, push) = **StrongFather** ; les Tools exécutent l'envoi après autorisation.
- `tool.notify.inbox.write` : écriture = **WriteIntent** vers KindMother.
- Aucun Tool ne décide à qui envoyer ni quel contenu : destinataire et contenu fournis dans le flux gouverné.

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Contrat de conformité
