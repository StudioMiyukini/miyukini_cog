# Genere TOUTES les categories 05-24 (01-04 deja creees)
$base = Join-Path $PSScriptRoot "..\points"

$catTitles = @{
    "05" = "Joueur et personnage"
    "06" = "Progression"
    "07" = "Combat"
    "08" = "Degats, resistances et effets"
    "09" = "Mort et resurrection"
    "10" = "Loot et tables"
    "11" = "Inventaire et objets"
    "12" = "Economie et commerce"
    "13" = "Social et groupes"
    "14" = "Siege et territoire"
    "15" = "Reputation et criminalite"
    "16" = "Magie et sorts"
    "17" = "Montures et familiers"
    "18" = "Logement et monde"
    "19" = "Quetes et missions"
    "20" = "Interface"
    "21" = "IA et bots"
    "22" = "Reseau et serveurs"
    "23" = "Systeme"
    "24" = "Meta et moderation"
}

$allPoints = @{
    "05-joueur-personnage" = @(
        @("Donnees-joueur", "Caracteristiques ; etat ; persistance (KindMother)."),
        @("Multi-personnages", "Plusieurs personnages par compte ; selection login."),
        @("Slots-equipement", "Armure, armes ; stats ; transmog optionnel."),
        @("Customisation", "Visage, corps ; coiffures ; teinture ; costumes."),
        @("Stats", "Attaque, defense, vitesse, precision ; impact degats, esquive."),
        @("Moveset-personnage", "Arsenal attaques propre a chaque personnage."),
        @("Arme-unique-signature", "Arme ou attaque specifique."),
        @("Relation-affinite", "Liens entre personnages ; dialogues ; bonus.")
    )
    "06-progression" = @(
        @("Systeme-niveau", "XP ; paliers ; recompenses par niveau."),
        @("Gain-competences-aptitudes", "Progression ; arbres competences."),
        @("Arbres-talents", "Points talent ; branches ; reset."),
        @("Skills-usage", "Competences montent a l'utilisation ; peuvent baisser."),
        @("Cap-total-skills", "Plafond global reparti."),
        @("Repartition-libre", "Joueur choisit competences a faire evoluer."),
        @("Skill-gains-degressifs", "Plus haute la skill, plus lents les gains."),
        @("Achievements-succes", "Objectifs ; recompenses ; progression affichee."),
        @("Titres", "Succes debloques ; affichage sous le nom."),
        @("Reset-quotidien-hebdo", "Limites par jour/semaine."),
        @("Saisons-battle-pass", "Contenu limite dans le temps."),
        @("Jobs-changement-classe", "Quetes changement ; evolution carriere.")
    )
    "07-combat" = @(
        @("Action", "Competences ; cooldowns ; cast time ; after-cast ; mana, endurance."),
        @("Auto-attaque-base", "Attaque automatique ; portee ; cadence ; degats."),
        @("Projectiles", "Creation ; trajectoire ; collision ; degats ; duree vie."),
        @("Gestion-parcours-combat", "Ciblage ; verrouillage ; priorite ; ordres."),
        @("Click-to-attack", "Ciblage clic ; attaque cible ou lieu."),
        @("Chance-toucher", "Formule attaque vs defense ; precision vs esquive."),
        @("Esquive-flee", "Formule vs precision ; reduction degats."),
        @("Parade-block", "Parade bouclier ; % blocage ; contre-attaque."),
        @("Modificateurs-taille", "Petit, Moyen, Grand ; bonus/malus degats."),
        @("Modificateurs-race", "Bonus/malus selon race cible."),
        @("Vitesse-attaque-ASPD", "Vitesse attaque ; cap ; bonus."),
        @("Changement-arme", "Swap combat ; armes secondaires."),
        @("Zone-effet-AOE", "Cercle, cone ; degats multiples ; indicateurs."),
        @("Barre-cast", "Canalisation ; interruption ; feedback visuel."),
        @("Aggro-menace", "Gestion menace ; priorite ciblage."),
        @("Roles", "Tank, DPS, soigneur ; synergies."),
        @("Officiers-vs-mooks", "Ennemis uniques vs soldats standards."),
        @("Officiers-allies", "PNJ controlables ; ordres ; perte possible.")
    )
    "08-degats-resistances-effets" = @(
        @("Resistances", "Feu, froid, eclair, poison, physique, magique ; cap ; immunites."),
        @("Elements", "Feu, Eau, Vent, Terre, Saint, Obscur, Neutre."),
        @("Immunites", "Immunes a un element ou plusieurs."),
        @("Critical-strike", "Coup critique ; degats amplifies."),
        @("Degats-pourcentage", "Pourcentage vie enleve en un coup."),
        @("Vol-vie-mana", "Sur les coups."),
        @("Knockback", "Repousser l'ennemi."),
        @("Effets-statut", "Buffs, debuffs ; stack ; duree ; dispel."),
        @("Crowd-control-CC", "Stun, root, silence, freeze, slow ; resistances."),
        @("Poison-over-time", "Degats poison dans le temps."),
        @("Autres-effets", "Curse, stone, aveuglement, sleep.")
    )
    "09-mort-resurrection" = @(
        @("Mort-joueur-NPC", "Etats mort, en recuperation."),
        @("Respawn", "Lieu reapparition ; run back."),
        @("Corps-corpse", "Corps au sol ; lootable ; decomposition ; recuperation."),
        @("Recuperation-corps", "Retour corpse ; risque plusieurs corps."),
        @("Resurrection", "Skill, objet, NPC ; cout ; Rez in-combat ; cooldown."),
        @("Perte-XP", "A la mort ; possibilite perdre niveau."),
        @("Drop-mort", "Objets et monnaie au sol ; ou equipement conserve."),
        @("Cimetiere-point-reapparition", "Lieu respawn.")
    )
    "10-loot-tables" = @(
        @("Tables-loot", "Par type monstre ; rarete."),
        @("Ramassage", "Automatique zone pick-up joueur."),
        @("Bosses", "Spawn fixe ou long ; loot special ; annonces spawn/kill."),
        @("Droits-loot", "Premier coup, dernier coup ; partage."),
        @("Champion-spawns", "Vagues mobs ; boss final ; artefacts."),
        @("Super-uniques-mini-boss", "Boss nommes ; groupe modifie ; bonus."),
        @("Niveau-zone", "Niveau monstres selon zone ; scaling."),
        @("Cartes-tresor", "Trouver carte ; point ; creuser ; spawn + loot.")
    )
    "11-inventaire-objets" = @(
        @("Inventaire", "Slots + poids max ; pas drag and drop ; empilable ; tri."),
        @("Stockage-persistant", "Stash, coffre, stockage guilde ; partage."),
        @("Poids-encumbrance", "Limite charge ; ralentissement ; surcharge."),
        @("Limite-stack", "Plafond par pile."),
        @("Menus-contextuels", "Clic droit ; actions selon type objet."),
        @("Gumps-fenetres", "Dialogs reutilisables ; serialisables reseau."),
        @("Slots-rapides", "Ceinture potions ; raccourcis ; empilable."),
        @("Prerequis", "Niveau ; stats requises equiper."),
        @("Durabilite", "Usure ; reparation ; casse definitive."),
        @("Renforcement", "Enchantement / raffinement ; risque echec ; over-upgrade."),
        @("Affixes", "Prefixe + suffixe ; generation procedurale."),
        @("Objets-Set", "Pieces ensemble ; bonus 2, 3, 5+ pieces."),
        @("Objets-uniques", "Stats fixes ; noms et histoires."),
        @("Slots-insertions", "Emplacements equipement ; gemmes ; cartes ; combinaisons."),
        @("Objets-maudits", "Ne peuvent pas etre droppes ; effets negatifs."),
        @("Objets-benis", "Ne tombent pas a la mort ; protection."),
        @("Charrette", "Inventaire supplementaire ; suit personnage."),
        @("Vente-vending", "S'asseoir ; boutique ; vendre hors-ligne."),
        @("Overcharge-Bargain", "Competences vendre plus cher / acheter moins cher.")
    )
    "12-economie-commerce" = @(
        @("Commerce", "Vente / achat ; marchands ; prix."),
        @("Echange-joueurs", "Trade ; offre ; validation ; fenetre echange."),
        @("Devises", "Or, argent ; monnaie premium ; monnaie avec poids."),
        @("Vendeurs-joueur", "PNJ geres par joueur ; vente hors-ligne."),
        @("Crafting-artisanat", "Recettes ; materiaux ; niveaux artisanat."),
        @("Recolte", "Ressources monde ; outils ; respawn."),
        @("Mining", "Minage minerais ; types ; veines."),
        @("Lumberjacking", "Abattage arbres ; types bois."),
        @("Fishing", "Peche ; types poissons ; tresors."),
        @("Recettes-materiaux", "Combinatoire ; ressources ; qualite variable.")
    )
    "13-social-groupes" = @(
        @("Liste-amis", "Ajout, suppression ; statut en ligne ; invite."),
        @("Clans", "Creation ; membres ; rangs ; banque ; log ; embleme."),
        @("Stockage-clan", "Coffre partage ; depot/retrait par rangs."),
        @("Competences-clan", "Skills financees ; effets groupe."),
        @("Groupe-party-raid", "Invitation ; composition ; partage loot."),
        @("Chat", "Canaux ; whisper ; moderation."),
        @("Boite-lettres", "Envoi objets/monnaie ; expiration ; limite."),
        @("Liste-noire", "Ignorer joueur ; blocage messages."),
        @("Reputation-factions", "Rang faction ; recompenses ; hostilite."),
        @("Guerre-factions", "Ordre vs Chaos ; controle villes ; bonus."),
        @("Clan-wars", "Guerre clan vs clan ; declaration ; objectifs.")
    )
    "14-siege-territoire" = @(
        @("Organisation-hierarchique", "Seigneur, vassaux ; serment ; loyaute."),
        @("Roi-royaume", "Couronne ; declaration guerre ; tributs ; protection."),
        @("Possession-chateau", "Un clan controle un chateau."),
        @("Siege-chateau", "Evenement periodique ; attaque vs defense."),
        @("Portes-murailles", "Destructibles ; points controle ; prise et perte."),
        @("Zones-controle", "Bases, postes, fortifications."),
        @("Points-capture", "Objectifs ; temps capture ; resistance."),
        @("Armes-siege", "Catapultes ; engins ; degats structures."),
        @("Gardes-NPC", "Defenseurs automatiques."),
        @("Taxes", "Perception territoire controle."),
        @("Banniere", "Drapeau clan ; affichage."),
        @("Morale-troupes", "Depend combat ; fuite ; renforcement."),
        @("Territoire", "Controle zones ; influence flux.")
    )
    "15-reputation-criminalite" = @(
        @("Karma-Fame", "Reputation bien/mal ; influence PNJ et PvP."),
        @("Alignement", "Loyal, Neutre, Chaotique ; couleur nom."),
        @("Systeme-meurtrier", "Noms rouges ; perte stats respawn ; statut persistant."),
        @("PK-Player-Kill", "Tuer loyal = devenir chaotique."),
        @("Redemption", "Retour loyal ; temps ; quetes ; expiation."),
        @("Full-loot-PvP", "Zones hostiles ; tout droppe a mort."),
        @("Zones-PK", "Zones PvP autorise ; zones sures."),
        @("Actions-criminelles", "Vol, meurtre, intrusion ; flag criminel ; gardes."),
        @("Lockpicking", "Crochetage ; conteneurs verrouilles."),
        @("Stealing-pickpocket", "Vol sacs ; flag criminel ; echec possible."),
        @("Snooping", "Consulter contenu sac d'un autre.")
    )
    "16-magie-sorts" = @(
        @("Mana", "Ressource lancer sorts."),
        @("Cout-mana", "Cout par sort ; regeneration."),
        @("Cercles-magie", "Organisation sorts ; puissance progressive."),
        @("Composants-reagents", "Consommation ressources pour sorts."),
        @("Livres-sorts", "Sorts connus ; grimoires ; apprentissage via objet."),
        @("Competences-passives", "Auras ; maitrises ; bonus permanents."),
        @("Sorts-zone", "AOE ; degats zone."),
        @("Hotkeys", "Raccourcis ; clic gauche, droit."),
        @("Barding-provocation", "Faire combattre creatures entre elles.")
    )
    "17-montures-familiers" = @(
        @("Montures", "Acquisition ; vitesses ; animations ; combat monte."),
        @("Combat-cheval", "Attaques montees ; desarconnement."),
        @("Montures-speciales", "Capacites uniques."),
        @("Familiers-pets", "Suivi ; buffs passifs ; cosmetiques."),
        @("Capture", "Oeufs ; eclosion ; evolution."),
        @("Faim-intimite", "Nourrir ; lien affectif ; evolution."),
        @("Competences-pet", "Capacites actives ou passives."),
        @("Taming", "Dompter creatures ; difficulte par type."),
        @("Bonding", "Lien duree ; devient permanent."),
        @("Mercenaires", "Recrutement ; suit joueur ; equipement ; evolution ; resurrection."),
        @("Homunculus", "Creation ingredients ; IA ; competences ; evolution ; nourriture.")
    )
    "18-logement-monde" = @(
        @("Logement-housing", "Maison joueur ; placement objets ; visite."),
        @("Maisons-monde", "Monde persistant, pas instance."),
        @("Decay-maisons", "Abandon = degradation puis suppression."),
        @("Placement-objets", "Decor ; coffres ; objets poses."),
        @("Cycle-jour-nuit", "Heure jeu ; luminosite ; evenements lies."),
        @("Meteo", "Pluie, neige, brouillard ; effets visuels/audio."),
        @("Zones-PvP", "Conflit ouvert ; recompenses PvP."),
        @("Zones-faction", "Territoire ; controle ; bonus."),
        @("Zones-sures", "Pas PvP ; villes ; hubs."),
        @("Kafra-services-centralises", "Sauvegarde ; teleport ; stockage ; courrier ; location.")
    )
    "19-quetes-missions" = @(
        @("Quetes", "Journal ; suivi ; objectifs ; recompenses ; chaines."),
        @("Objectifs-dynamiques", "Changements objectif selon etat monde."),
        @("Branchements", "Succes / echec selon actions ; chemins alternatifs."),
        @("Pression-temporelle", "Timer ; objectifs a temps."),
        @("Sauvetage-allies", "Allies en danger ; mission secours."),
        @("Defaite-allies", "Mort allies ; impact mission."),
        @("Quetes-changement-rang", "Epreuves ; objectifs ; recompense = nouveau rang.")
    )
    "20-interface" = @(
        @("GUI", "Menus ; HUD ; boutons ; inventaire visuel ; tooltips."),
        @("Carte-monde", "Vue globale ; waypoints ; decouverte."),
        @("Suivi-quetes", "Objectifs actuels ; distance ; indicateurs."),
        @("Unit-frames", "Barres vie/mana party/raid ; cibles ; buffs."),
        @("Barre-cast", "Progression sort ; annulation."),
        @("Indicateurs-cooldown", "Temps restant competences."),
        @("Notifications", "Alertes ; loot ; invitations ; evenements.")
    )
    "21-ia-bots" = @(
        @("IA-bot", "Comportements ; etats Idle, Chase, Attack, Flee ; machine etats."),
        @("Pathfinding", "Recherche chemin ; obstacles."),
        @("Navmesh", "Graphe navigation.")
    )
    "22-reseau-serveurs" = @(
        @("Reseau-MWS", "Connexion multijoueur ; synchronisation ; Lobbys."),
        @("Selection-serveur", "Liste mondes ; population ; latence."),
        @("Sharding", "Fragmentation monde ; repartition joueurs."),
        @("Cross-server-cross-realm", "Groupes entre serveurs ; files attente."),
        @("Compensation-latence", "Prediction ; reconciliation ; lag compensation."),
        @("Anti-cheat", "Detection ; validation cote serveur."),
        @("Tick-serveur", "Frequence simulation ; synchronisation."),
        @("Trading", "Echange direct ; fenetre trade."),
        @("PvP", "Duel ; zones hostiles."),
        @("Co-op-partage", "Plusieurs joueurs champ bataille."),
        @("Ecran-partage", "Split screen ou vue partagee.")
    )
    "23-systeme" = @(
        @("Boucle-jeu", "Game loop ; delta time ; frame rate ; pause."),
        @("Entrees-utilisateur", "Clavier ; souris ; manette ; mapping."),
        @("Gestion-temps", "Delta time ; temps jeu ; timers ; cooldowns."),
        @("Chargement-assets", "Textures ; sons ; fonts ; hot reload (dev)."),
        @("Sauvegarde-chargement", "Persistance ; KindMother ; serialisation ; slots."),
        @("Audio", "Musique ; SFX ; volume ; canaux ; spatial."),
        @("Musique-adaptative", "Intensite selon contexte ; transitions."),
        @("Etats-jeu", "Menu ; jeu ; pause ; game over ; transitions."),
        @("Systeme-evenements", "Evenements internes ; signaux ; delegues."),
        @("Regles-jeu", "Regles ; conditions ; validations ; mods."),
        @("Debug-outils-dev", "Overlays ; logs ; inspector ; raccourcis."),
        @("Configuration", "Options ; parametres ; preferences ; persistance."),
        @("Localisation-i18n", "Textes traduisibles ; cles ; langues."),
        @("Optimisation", "Culling ; LOD ; pooling ; cache.")
    )
    "24-meta-moderation" = @(
        @("Classements-leaderboards", "PvP ; PvE ; economie ; saison."),
        @("Outils-moderation", "Ban ; mute ; logs ; outils GM."),
        @("Systeme-signalement", "Report joueur ; categories ; suivi."),
        @("Actualites-in-game", "Patch notes ; annonces ; evenements.")
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

$count = 0
foreach ($catSlug in ($allPoints.Keys | Sort-Object)) {
    $catNum = $catSlug.Substring(0,2)
    $catTitle = $catTitles[$catNum]
    $catDir = Join-Path $base $catSlug
    New-Item -ItemType Directory -Force -Path $catDir | Out-Null
    
    $rows = @()
    foreach ($point in $allPoints[$catSlug]) {
        $name = $point[0] -replace "-", " "
        $desc = $point[1]
        $fileSlug = $point[0].ToLower()
        $rows += "| $name | [$fileSlug]($fileSlug.md) | $desc |"
        
        $content = $template -f $name, "$catNum. $catTitle", $desc
        $filePath = Join-Path $catDir "$fileSlug.md"
        $content | Out-File -FilePath $filePath -Encoding UTF8
        $count++
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
}

Write-Host "Genere $count points dans categories 05-24."
