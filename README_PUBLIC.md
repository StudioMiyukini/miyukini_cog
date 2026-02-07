# Depot README public

## Depot public separe

Le README et la documentation conceptuelle de ce projet sont egalement disponibles dans un **depot public separe** pour permettre la decouverte du projet sans exposer le code source.

**URL du depot public** : `https://github.com/StudioMiyukini/miyukini-core-system-public`

**Emplacement local** : `c:\Users\miyuk\Documents\Cursor\miyukini-core-system-readme`

---

## Contenu publie dans le depot public

Le depot public contient :

1. **README.md** — Presentation complete du projet (philosophie, strates, mecanismes, services, etat des lieux)
2. **docs/public/** — Documentation conceptuelle de reference :
   - Glossaire officiel
   - Definition COG
   - Pyramide Architecture
   - Lois d'Autonomie
   - Vision Strategique
   - Objectif Final
   - Souverainete Environnement
   - Operateurs et Terminologie
   - Tools et Toolkits
   - Mandats et Equipes
   - Connexion Inter-COG
   - Kernel Maintenance Observability

---

## IMPORTANT : Synchronisation des deux depots

**En cas de modification du README ou de la doc publique, les fichiers doivent etre pousses sur les 2 depots :**

1. **Depot principal (prive)** : `Miyukini_COG` — commit et push normal
2. **Depot public** : `miyukini-core-system-public` — synchronisation manuelle ou via script

### Procedure complete apres modification

#### Etape 1 : Commit et push dans le depot principal

```powershell
cd "c:\Users\miyuk\Documents\Cursor\Miyukini_COG"
git add README.md docs/public/
git commit -m "docs: mise a jour README et documentation publique"
git push
```

#### Etape 2 : Synchroniser le depot public

**Methode rapide (script)** :

```powershell
cd "c:\Users\miyuk\Documents\Cursor\miyukini-core-system-readme"
.\sync-readme.ps1
```

**Methode manuelle** :

```powershell
cd "c:\Users\miyuk\Documents\Cursor\miyukini-core-system-readme"
Copy-Item "..\Miyukini_COG\README.md" . -Force
Copy-Item "..\Miyukini_COG\docs\public\*" "docs\public\" -Force -Recurse
git add .
git commit -m "Sync README et docs publiques depuis depot principal"
git push
```

---

## Documentation complete

Pour plus de details sur l'acces et la synchronisation, voir :
- **Dans le depot README public** : `ACCES_ET_SYNCHRONISATION.md`
- **Script de synchronisation** : `sync-readme.ps1`

---

**Note** : Ce depot principal reste **prive**. Le README et `docs/public/` sont rendus publics via le depot separe. **N'oubliez pas de synchroniser les deux depots apres chaque modification.**
