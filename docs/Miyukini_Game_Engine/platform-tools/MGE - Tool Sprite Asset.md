# MGE — Tool Sprite & Asset

## Contexte

Outil de gestion des assets 2D : pivot, hitbox, animations, atlas, layering. Produit des metadata exportables pour le runtime.

## Portée / Scope

- **Applicable à :** Assets Allumina, sprites, animations.
- **Statut :** Spécification.

---

## Rôle

Gestion assets 2D (sprites, animations, atlas).

## Permet

- Définir pivot
- Définir hitbox (collision)
- Définir animations (frames, timing)
- Générer atlas (sprite sheets)
- Tester layering (RenderLayer)

## Produit

- Metadata exportée (JSON/TOML)
- Atlas générés
- Fichiers de config pour le runtime

## Règles

- Ne modifie pas le kernel
- Consomme images brutes, produit metadata
- Intégration Export Pipeline

---

**Référence** : [MGE - Platform Tooling Layer v1](../MGE%20-%20Platform%20Tooling%20Layer%20v1.md)
