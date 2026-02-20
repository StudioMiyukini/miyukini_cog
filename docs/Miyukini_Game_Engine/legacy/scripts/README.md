# Scripts MGE

## Génération de la structure des points

### PowerShell (recommandé, sans Python)

```powershell
.\generate_all_points.ps1
```

Génère les catégories 05 à 24 (205 points). Les catégories 01 à 04 sont créées manuellement.

### Python (alternative)

```bash
python generate_points_structure.py
```

**Prérequis :** Python 3.6+. Génère toutes les catégories (01-24) ; adapté pour régénérer ou compléter.

**Effet :** Crée les dossiers `points/XX-nom-categorie/` et les fichiers `.md` pour chaque point, avec un template de développement (Contexte, Portée, À développer).
