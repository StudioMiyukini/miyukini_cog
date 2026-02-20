# Genere la structure points MGE (dossiers + fichiers)
$base = Join-Path $PSScriptRoot "..\points"

$categories = @{
    "03-deplacement-locomotion" = @(
        @("Deplacement-8-directions", "Haut, bas, gauche, droite + 4 diagonales ; input normalise."),
        @("Acceleration-deceleration", "Interpolation vitesse ; friction ; inertie."),
        @("Vitesse-max", "Limite vitesse ; clamp."),
        @("Pathfinding", "Recherche chemin (A*, Dijkstra) ; obstacles."),
        @("Navmesh", "Graphe navigation ; zones navigables ; ports."),
        @("Run-walk", "Mode course vs marche ; impact aggro."),
        @("Stamina", "Jauge endurance ; regeneration."),
        @("Shift-clic-stand-attack", "Rester sur place et attaquer."),
        @("Dash-esquive", "Deplacement rapide ; invincibilite ; cancel."),
        @("Bateaux", "Navigation ; multi-passagers ; ancrage."),
        @("Combat-naval", "Cannons ; abordage ; degats."),
        @("Continents", "Traversee continents ; attente ; horaires."),
        @("PNJ-teleportation", "Teleport zones connues ; cout."),
        @("Runes-atlas", "Marquer lieux ; Recall et Portail.")
    )
    "04-entites-monde" = @(
        @("Unicite-entites", "ID unique ; registre ; lifecycle."),
        @("Spawn", "Creation entites ; position ; prefab ; pool."),
        @("Despawn", "Destruction ; nettoyage references."),
        @("Gestion-chunks", "Chargement/dechargement ; culling."),
        @("Monde-persistant-instancie", "Zones partagees vs privees."),
        @("Instances-donjons", "Zones isolees ; entree/sortie ; difficultes."),
        @("Raids", "Instances grand groupe ; bosses ; phases."),
        @("Respawn-dynamique", "Points spawn ; timers ; tables."),
        @("World-bosses-evenements", "Spawns mondiaux ; participation multi-joueurs."),
        @("Facets-shards", "Mondes paralleles ; miroirs."),
        @("Culling-agressif", "Ne pas traiter entites hors ecran."),
        @("Grands-effectifs-ecran", "Centaines ou milliers unites ; optimisation."),
        @("Comportement-foule", "Mouvements masse ; evitement ; boids.")
    )
}

$template = @"
# {0}

**Categorie :** {1}  
**Description :** {2}

## Contexte

Point de la reference technique MGE.

## Portee

- A definir

## A developper

- [ ] Specifications
- [ ] Implementation
- [ ] Tests

## References

- [Index categorie](_index.md)
- [Index MGE](../_index.md)
"@

$catNum = 3
foreach ($catSlug in $categories.Keys) {
    $catDir = Join-Path $base $catSlug
    New-Item -ItemType Directory -Force -Path $catDir | Out-Null
    
    $catTitle = $catSlug -replace "^\d+-", "" -replace "-", " "
    $rows = @()
    foreach ($point in $categories[$catSlug]) {
        $name = $point[0] -replace "-", " "
        $desc = $point[1]
        $fileSlug = $point[0].ToLower()
        $rows += "| $name | [$fileSlug]($fileSlug.md) | $desc |"
        
        $content = $template -f $name, "$catNum. $catTitle", $desc
        $filePath = Join-Path $catDir "$fileSlug.md"
        $content | Out-File -FilePath $filePath -Encoding UTF8
    }
    
    $indexContent = @"
# $catNum. $catTitle

Categorie MGE.

## Points

| Point | Fichier | Description |
|-------|---------|-------------|
$($rows -join "`n")

[<- Retour a l'index MGE](../_index.md)
"@
    $indexContent | Out-File -FilePath (Join-Path $catDir "_index.md") -Encoding UTF8
    $catNum++
}

Write-Host "Genere categories 03 et 04."
