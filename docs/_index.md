# Miyukini COG - Index Documentation

## Contexte

Index global de la documentation du projet Miyukini COG.

## Carte des strates actives

Numérotation unifiée K / C / T / S / A / H (décision D6, [vue globale](architecture/COG-VUE-GLOBALE.md) §2).

| Strate | Dossier | Description |
|---|---|---|
| K | [kernel/](kernel/) | Substrat technique neutre (Id, Logger, Clock, Config, Lifecycle) |
| C | [cores/](cores/_index.md) | 10 Cores : gouvernent, n'exécutent pas ([index](cores/_index.md)) |
| T | [tools/](tools/) | Toolkits et outils executables |
| S | [services/](services/) | Services produits et operateurs |
| A | [admin/](admin/) | MiyukiniAdmin (zone COG uniquement, D7) |
| H | [cores/TAMR/](cores/TAMR/_index.md) | Intervention humaine (TAMR), transversale |
| — | [contrats/](contrats/) | Invariants, protocoles et templates transverses |

## Dossiers transverses actifs

| Dossier | Description |
|---|---|
| [implementation/](implementation/) | [Contraintes](implementation/CONTRAINTES-IMPLEMENTATION.md), [revue de code](implementation/REVUE-CODE-PREPARATION.md), [plan d'implementation](implementation/PLAN-IMPLEMENTATION.md) |
| [conformite/](conformite/CONFORMITE-SYNTHESE.md) | Conformité : ANSSI, souveraineté, réglementaire UE/FR (CRA, RGPD, AI Act), normes ISO/OWASP/SLSA |
| [doc_for_website/](doc_for_website/) | Documentation preparee pour le site |
| [miyukini-webway-system/](miyukini-webway-system/) | Documents lies au Webway System |
| [ux_ui/](ux_ui/) | Ressources UX/UI et composants |
| [legal/](legal/) | Documentation juridique |
| [securite/](securite/) | Audits de securite transverses (auth, WAN, federation, secrets) |
| [architecture/](architecture/) | Modele de reference (zones, couches, services hybrides, IA) — point d'entree : [vue globale](architecture/COG-VUE-GLOBALE.md) |

## Migration documentaire recente

- Le dossier preliminaire `docs/spm-cms/` est retire.
- Les notions SPM CMS conservees sont maintenant centralisees dans [docs/tools/MiyuCMS/MiyuCMS - Notions SPM CMS Consolidees.md](tools/MiyuCMS/MiyuCMS%20-%20Notions%20SPM%20CMS%20Consolidees.md).
- Le dossier `docs/references/` n'est pas present dans l'arborescence active.

---

Date de mise a jour : 2026-10-24
