# Cores — index

<!-- @id cores.index -->
<!-- @role navigation-index -->
<!-- @layer C -->
<!-- @human Point d'entrée des 10 Cores : rôle en une ligne, opérateurs, lien vers le modèle cible -->
<!-- @do navigate_cores -->

- **Règle** : les Cores **gouvernent**, ils n'exécutent pas (décision D1) ; des opérateurs exécutent sous leur autorité
- **Modèle cible** : [vue globale](../architecture/COG-VUE-GLOBALE.md) §3 · [zones et couches](../architecture/MODELE-ZONES-ET-COUCHES-COG.md) · schéma [V2 gouvernance](../architecture/schema-global/V2-gouvernance.mmd)

## Cores

<!-- @id cores.index.liste -->
<!-- @role core-inventory -->
<!-- @do list_cores -->

| Core | Rôle | Opérateurs |
|---|---|---|
| [StrongFather](StrongFather/_index.md) | décide (ALLOW / DENY), tranche les TAMR APPROVAL | — |
| [KindMother](KindMother/_index.md) | persiste ; Mère = DB principale, Fille = DB de session (D5) | — |
| [BondingBrother](BondingBrother/_index.md) | passerelle : traduit, filtre, transmet | — |
| [BorderGuard](BorderGuard/_index.md) | définit frontières et niveaux de confiance | **Douane** (pare-feu + IPS) |
| [WorrySentinel](WorrySentinel/_index.md) | gouverne la sécurité (niveaux 0–4) | **Vigies** (IDS) |
| [CaringNanny](CaringNanny/_index.md) | observe, ne modifie rien | — |
| [MasterButler](MasterButler/_index.md) | capacités, permissions, catalogue des toolkits | — |
| [LogisticsSteward](LogisticsSteward/_index.md) | arbitre ressources et quotas ; le Kernel exécute (D9) | Kernel |
| [EverBuddy](EverBuddy/_index.md) | cycle de vie, évolution, historique immuable | — |
| [TAMR](TAMR/_index.md) | cadre de l'intervention humaine | — |
