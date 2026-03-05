<!-- @id cert.lise.wcag -->
<!-- @do provide_wcag_reference_knowledge -->
<!-- @role accessibility -->
<!-- @layer reference -->
<!-- @human Referentiel WCAG 2.2 pour Lise -->

# WCAG 2.2 â€” Lise

> **TL;DR** AccessibilitÃ©. 4 principes POUR. Niveaux A/AA/AAA. Cible AA pour MiyuCloud web.

**IdentitÃ©** : W3C | Obligatoire public (RGAA) | 2.2 oct 2023

## POUR | CritÃ¨res clÃ©s

| Principe | CritÃ¨res (AA) |
|----------|---------------|
| Percevable | Alt texte, contraste 4.5:1 |
| Operable | Clavier, focus visible, 24px cibles |
| Understandable | Labels, erreurs identifiÃ©es |
| Robust | Nom/role/valeur (ARIA) |

## WCAG 2.2 nouveaux

2.4.11 Focus non obscurci | 2.5.8 Taille cible 24px | 3.3.8 Auth sans test cognitif

## Checklist

- [ ] Alt sur tout non-textuel
- [ ] Clavier complet, pas de piÃ¨ge
- [ ] Contraste 4.5:1 (tokens miyuki-ui)
- [ ] Focus visible (ring/border)
- [ ] Labels sur tous champs
- [ ] Erreurs + suggestion correction

## Anti-patterns

| Erreur | Correction |
|--------|------------|
| outline:none | Indicateur focus visible |
| Contraste insuffisant | VÃ©rifier tokens 4.5:1 |
| Pas de labels | label ou aria-label |
| Cible <24px | Minimum 24x24px |

## Miyukini

miyuki-ui-dioxus: prop alt, Focus ring atoms. COG_THEME contraste. MiyuCloud web: cible AA.
## Parcours obtention
Voir KNOWLEDGE.md pour les connaissances requises et les preuves de maitrise.

