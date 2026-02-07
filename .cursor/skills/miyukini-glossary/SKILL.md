---
name: miyukini-glossary
description: Terminologie officielle et glossaire normatif de l'ecosysteme Miyukini COG. Utiliser quand on travaille sur du code, de la documentation, des noms de variables, des commentaires, ou toute communication impliquant la terminologie Miyukini. Inclut les correspondances termes interdits/corrects, les noms des Cores, Operateurs, Outils, Kits d'Outils, Services, et les phrases fondatrices.
---

# Glossaire Miyukini COG

## Regle fondamentale

Toujours utiliser la terminologie officielle. Pour le glossaire complet, voir [references/glossaire-complet.md](references/glossaire-complet.md).

## Correspondance terminologique rapide

| Terme INTERDIT | Terme CORRECT |
|----------------|---------------|
| Produit | **Operateur** |
| App | **Operateur** ou **Operateur d'Interface** |
| Creer un produit | **Deployer un Operateur** |
| Utiliser une app | **Interagir avec un Operateur** |
| Marketplace | **Registre d'Operateurs** |
| Super-Operateur | **Equipe d'Operateurs** |
| Tool | **Outil** |
| Toolkit | **Kit d'Outils** |
| Operator | **Operateur** |
| Decision Window | **Mandat de Permission** |
| Fast Path | **Chemin Mandate** |

## Nomenclature des noms

| Prefixe | Signification |
|---------|---------------|
| `MiyuXxx` | Toolkit concu par Miyukini (Strate 6) |
| `MiyukiniOpsXxx` | Operateur concu par Miyukini (Strate 7) |
| `MiyukiniXxx` | Service concu par Miyukini |
| `JayXxx` | Service officiel de la famille "Jay" |

## Les 8 Cores (Strate 4)

| Core | Domaine | Question fondamentale |
|------|---------|-----------------------|
| StrongFather | Decision strategique | "Devrait-on faire cette action ?" |
| KindMother | Donnees et persistance | "Comment les donnees sont-elles persistees ?" |
| Caring Nanny | Observation d'etat | "Dans quel etat est le systeme ?" |
| Master Butler | Capacites et permissions | "Qu'est-ce qui est possible ?" |
| Border Guard | Frontieres et confiance | "Ou sont les frontieres ?" |
| Ever Buddy | Cycle de vie et evolution | "Comment evoluer sans rompre ?" |
| WorrySentinel | Gouvernance de securite | "Quel niveau de securite ?" |
| TAMR | Intervention humaine | "Quand l'humain intervient-il ?" |

**Regle invariante :** Les Cores decident ou gouvernent, mais n'executent jamais.

## Hierarchie des concepts

- **Service** = Capacite percue par l'utilisateur
- **Operateur** = Entite fonctionnelle gouvernee qui execute
- **Outil** = Capacite executable gouvernee, sans autorite
- **Kit d'Outils** = Composition officielle d'Outils

## Phrases fondatrices cles

> "Dans Miyukini, les utilisateurs n'installent pas d'applications. Ils interagissent avec des Operateurs gouvernes."

> "Un Outil fait, mais ne decide jamais."

> "La complexite est geree par la collaboration, pas par l'accumulation."

> "Un Mandat de Permission n'est pas une optimisation. C'est un acte de gouvernance delegue."

## References

- Glossaire complet : [references/glossaire-complet.md](references/glossaire-complet.md)
- Documentation officielle : `docs/reference/Miyukini Conceptual References - Glossaire.md`
