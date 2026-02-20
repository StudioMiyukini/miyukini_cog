# MGE Platform — Index des outils (Tooling Layer)

## Contexte

Les 8 outils du Tooling Layer MGE gravitent autour du Kernel sans le modifier. Chaque outil produit des données exportables vers le runtime.

## Portée / Scope

- **Applicable à :** Développement outils, workflow Allumina.
- **Statut :** Index normatif. Voir [MGE - Platform Tooling Layer v1](../MGE%20-%20Platform%20Tooling%20Layer%20v1.md).

---

## Outils

| Outil | Document | Rôle |
|-------|----------|------|
| **Data Authoring** | [MGE - Tool Data Authoring](./MGE%20-%20Tool%20Data%20Authoring.md) | Édition game design via SQL |
| **Prefab Editor** | [MGE - Tool Prefab Editor](./MGE%20-%20Tool%20Prefab%20Editor.md) | Création visuelle entités |
| **Balance Lab** | [MGE - Tool Balance Lab](./MGE%20-%20Tool%20Balance%20Lab.md) | Simulation paramètres |
| **Battle Sandbox** | [MGE - Tool Battle Sandbox](./MGE%20-%20Tool%20Battle%20Sandbox.md) | Mini runtime formations/LOD |
| **Sprite Tool** | [MGE - Tool Sprite Asset](./MGE%20-%20Tool%20Sprite%20Asset.md) | Gestion assets 2D |
| **Rule Editor** | [MGE - Tool Rule Editor](./MGE%20-%20Tool%20Rule%20Editor.md) | Règles gameplay déclaratives |
| **Export Pipeline** | [MGE - Tool Export Pipeline](./MGE%20-%20Tool%20Export%20Pipeline.md) | Transformation → runtime |
| **AI Assist** | [MGE - Tool AI Assist](./MGE%20-%20Tool%20AI%20Assist.md) | Assistant IA sur outils |

---

## Flux

```
Data Authoring → Balance Lab → Prefab Editor → Export Pipeline → Runtime Data
                     ↑                              ↑
                Rule Editor                    AI Assist
                Sprite Tool
                Battle Sandbox
```
