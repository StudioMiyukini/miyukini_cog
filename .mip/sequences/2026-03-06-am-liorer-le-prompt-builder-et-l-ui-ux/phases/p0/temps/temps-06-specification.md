# P0 Temps 6 - Specification technique

## Statut

- Etat : TERMINE
- Phase : P0 Temps 6
- Responsable principal : Francois

## TL;DR

Spec complete dans specs/...spec.md. Decisions cles : PromptBuilderInput etendu (5 champs nouveaux + tags utilise), template prompt canonique documente, layout bi-panneaux CSS flex, preview live JS locale (debounce 300ms), section avancee details HTML natif, localStorage pour persistance, 0 nouvelle dependance Cargo.

## Decisions architecture

1. **Preview locale** : template JS miroir du template Rust. Pas d'appel API pour la preview — appel serveur uniquement au submit. Template documente en spec comme source de verite unique.
2. **Section avancee** : `<details>` HTML natif — zero JS pour l'accordion, semantique accessible.
3. **Persistance** : localStorage uniquement — suffit pour usage local mono-utilisateur.
4. **Validations** : cote Rust uniquement (whitelist agents, longueur inputs). JS valide uniquement HTML5 required.
5. **Stacks predefinis** : liste statique HTML, option "Autre" active champ texte libre pb-stack.

## API publiques

- `POST /api/prompt` : body etendu (autonomy_mode, agents[], tags[], urgency, sensitive_data, msw_toggle), template enrichi, nouvelles validations 400 (longueur, whitelist)
- Tous les autres endpoints : inchanges

## Schema de donnees

Voir specs/...spec.md. `PromptBuilderInput` etendu avec `autonomy_mode: Option<String>`, `urgency: bool`, `sensitive_data: bool`, `msw_toggle: bool`. `agents: Vec<String>` et `tags: Vec<String>` existaient mais sont maintenant utilises et valides.
