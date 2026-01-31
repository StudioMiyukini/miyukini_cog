# docs_tools — Écarts à la documentation enrichie (protocole Toolkits)

**Version :** 1.0  
**Statut :** Audit de conformité  
**Date :** 2026-01-30  
**Référence :** [Miyukini Protocol - Ecriture Enrichie Toolkits](../protocols/Miyukini%20Protocol%20-%20Ecriture%20Enrichie%20Toolkits.md)

---

## 1. Contexte

Ce document identifie les **Kits d'Outils (Toolkits)** qui n'ont pas encore une **documentation enrichie complète** au sens du protocole [Miyukini Protocol - Ecriture Enrichie Toolkits](../protocols/Miyukini%20Protocol%20-%20Ecriture%20Enrichie%20Toolkits.md).

**Publication officielle d'un Toolkit** requiert au minimum :

1. **Documentation Fondatrice** — `docs/tools/<MiyuXXX>/<MiyuXXX> - Documentation Fondatrice.md`
2. **Reference Outils** — `docs/tools/<MiyuXXX>/<MiyuXXX> - Reference Outils.md`
3. **Tool Governance Compliance Contract** — `docs/tools/<MiyuXXX>/contracts/governance/<MiyuXXX> - Tool Governance Compliance Contract.md`

---

## 2. Portée / Scope

| Inclus | Exclus |
|--------|--------|
| Toolkits sous `docs/tools/<MiyuXXX>/` | Cores (docs/core), Kernel, Opérateurs (pas de protocole enrichi dédié dans docs/protocols) |
| Conformité aux 3 livrables obligatoires | Contrats optionnels (KindMother, Security, Boundaries, Dependencies) et Reference Implementation Guidelines |

**Note — Opérateurs et Services :** Le protocole « Écriture enrichie » actuel s’applique aux **Toolkits** (Strate 6). Les **Opérateurs** (Strate 7) et les **Services** (capacité perçue par l’utilisateur, portée par un Opérateur ou une Équipe d’Opérateurs) n’ont pas, à ce jour, de protocole d’écriture enrichie normatif dans `docs/protocols/`. Le présent audit porte donc uniquement sur les Toolkits.

---

## 3. Toolkits sans documentation enrichie complète

Les kits ci‑dessous disposent d’une **Documentation Fondatrice** mais n’ont **tous les** livrables obligatoires — tous ont été complétés (voir tableau et note ci-dessous).

| Kit | Doc Fondatrice | Reference Outils | Contrat Governance | Lien dossier |
|-----|----------------|------------------|---------------------|--------------|
| **MiyuBilling** | Oui | Oui | Oui | [MiyuBilling](./MiyuBilling/) |
| **MiyuBooking** | Oui | Oui | Oui | [MiyuBooking](./MiyuBooking/) |
| **MiyuCMS** | Oui | Oui | Oui | [MiyuCMS](./MiyuCMS/) |
| **MiyuMedia** | Oui | Oui | Oui | [MiyuMedia](./MiyuMedia/) |
| **MiyuShipping** | Oui | Oui | Oui | [MiyuShipping](./MiyuShipping/) |
| **MiyuStore** | Oui | Oui | Oui | [MiyuStore](./MiyuStore/) |
| **MiyuWidgets** | Oui | Oui | Oui | [MiyuWidgets](./MiyuWidgets/) |

**Dernière mise à jour :** 2026-01-30 — Les 7 kits ci‑dessus ont été complétés. **Total : 0 Toolkit** en attente ; tous disposent de la documentation enrichie complète (Doc Fondatrice + Reference Outils + Tool Governance Compliance Contract).

---

## 4. Actions recommandées (pour tout nouveau kit ou kit incomplet)

Pour tout Toolkit n'ayant pas encore les trois livrables obligatoires :

1. **Créer** `docs/tools/<MiyuXXX>/<MiyuXXX> - Reference Outils.md` (liste détaillée des ToolIds : action, niveau sécurité, note).
2. **Créer** `docs/tools/<MiyuXXX>/contracts/governance/<MiyuXXX> - Tool Governance Compliance Contract.md` (conformité au [Master Butler - Tool Governance Compliance Template](../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Compliance%20Template.md) ; obligations spécifiques du kit).
3. **Mettre à jour** la Documentation Fondatrice existante pour ajouter le lien explicite vers le contrat : *« Les obligations de conformité détaillées sont dans [MiyuXXX - Tool Governance Compliance Contract](…). »* et une section ou phrase d’**alignement MIP** (référence MIP v1).

S’appuyer sur le [Template - Ecriture Enrichie Toolkits](../protocols/Template%20-%20Ecriture%20Enrichie%20Toolkits.md) et sur un kit déjà conforme (ex. MiyuSQL, MiyuBilling, MiyuStore) pour la structure et le ton.

---

## 5. Références

| Document | Lien |
|----------|------|
| Protocole Écriture Enrichie Toolkits | [Miyukini Protocol - Ecriture Enrichie Toolkits](../protocols/Miyukini%20Protocol%20-%20Ecriture%20Enrichie%20Toolkits.md) |
| Template Écriture Enrichie Toolkits | [Template - Ecriture Enrichie Toolkits](../protocols/Template%20-%20Ecriture%20Enrichie%20Toolkits.md) |
| Master Butler - Tool Governance Compliance Template | [Master Butler - Tool Governance Compliance Template](../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Compliance%20Template.md) |
| Index Tools | [docs/tools - _index](./_index.md) |

---

**Date de création :** 2026-01-30  
**Statut :** Audit de conformité — à mettre à jour après chaque complétion de kit
