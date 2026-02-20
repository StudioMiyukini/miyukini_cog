#!/usr/bin/env python3
"""Génère la structure de dossiers et fichiers pour chaque point MGE."""

import re
from pathlib import Path

BASE = Path(__file__).parent.parent / "points"

# Données extraites de la référence MGE (point, description)
CATEGORIES = {
    "01-affichage-rendu": [
        ("Affichage et résolution", "Fenêtre, résolution logique/physique, scale factor, fullscreen, vsync."),
        ("Coordonnées", "Système de coordonnées 2D ; origine ; unités (px, tiles)."),
        ("Gestion des sprites", "Chargement textures, sprite sheets, atlas ; taille ; anchor/pivot."),
        ("Animations de sprites", "Frames, boucles, transitions ; flip ; directions multiples."),
        ("Caméra", "Suivi joueur ; zoom ; limites ; couches parallax ; shake."),
        ("Z-order / couches", "Ordre d'affichage ; calques."),
        ("Particules et effets", "Système de particules ; effets visuels."),
        ("Monde tile-based", "Grille 2D isométrique ; tuiles terrain, objets, murs."),
    ],
    "02-physique-collisions": [
        ("Hitbox", "Forme et taille ; alignement sur le sprite."),
        ("Collision", "Détection (AABB, cercle, polygone) ; réponse (rebond, blocage)."),
        ("Collision layers", "Masques ; qui collisionne avec qui."),
    ],
    "03-deplacement-locomotion": [
        ("Déplacement 8 directions", "Haut, bas, gauche, droite + 4 diagonales ; input normalisé."),
        ("Accélération / décélération", "Interpolation de vitesse ; friction ; inertie."),
        ("Vitesse max", "Limite de vitesse ; clamp."),
        ("Pathfinding", "Recherche de chemin (A*, Dijkstra) ; obstacles."),
        ("Navmesh", "Graphe de navigation ; zones navigables ; ports."),
        ("Run / walk", "Mode course vs marche ; impact aggro."),
        ("Stamina", "Jauge d'endurance ; régénération."),
        ("Shift-clic (stand and attack)", "Rester sur place et attaquer."),
        ("Dash / esquive", "Déplacement rapide ; invincibilité ; cancel."),
        ("Bateaux", "Navigation ; multi-passagers ; ancrage."),
        ("Combat naval", "Cannons ; abordage ; dégâts."),
        ("Continents", "Traversée entre continents ; attente ; horaires."),
        ("PNJ de téléportation", "Téléport vers zones connues ; coût."),
        ("Runes et atlas", "Marquer des lieux ; Recall et Portail."),
    ],
    "04-entites-monde": [
        ("Unicité des entités", "ID unique ; registre ; lifecycle."),
        ("Spawn", "Création d'entités ; position ; préfab ; pool."),
        ("Despawn", "Destruction ; nettoyage références."),
        ("Gestion des chunks", "Chargement/déchargement ; culling."),
        ("Monde persistant vs instancié", "Zones partagées vs privées."),
        ("Instances / donjons", "Zones isolées ; entrée/sortie ; difficultés."),
        ("Raids", "Instances grand groupe ; bosses ; phases."),
        ("Respawn dynamique", "Points de spawn ; timers ; tables."),
        ("World bosses / événements", "Spawns mondiaux ; participation multi-joueurs."),
        ("Facets / shards", "Mondes parallèles ; miroirs."),
        ("Culling agressif", "Ne pas traiter entités hors écran."),
        ("Grands effectifs à l'écran", "Centaines ou milliers d'unités ; optimisation."),
        ("Comportement en foule", "Mouvements de masse ; évitement ; boids."),
    ],
    "05-joueur-personnage": [
        ("Données du joueur", "Caractéristiques ; état ; persistance (KindMother)."),
        ("Multi-personnages", "Plusieurs personnages par compte ; sélection login."),
        ("Slots d'équipement", "Armure, armes ; stats ; transmog optionnel."),
        ("Customisation", "Visage, corps ; coiffures ; teinture ; costumes."),
        ("Stats", "Attaque, défense, vitesse, précision ; impact dégâts, esquive."),
        ("Moveset par personnage", "Arsenal d'attaques propre à chaque personnage."),
        ("Arme unique / signature", "Arme ou attaque spécifique."),
        ("Relation / affinité", "Liens entre personnages ; dialogues ; bonus."),
    ],
    "06-progression": [
        ("Système de niveau", "XP ; paliers ; récompenses par niveau."),
        ("Gain de compétences / aptitudes", "Progression ; arbres de compétences."),
        ("Arbres de talents", "Points de talent ; branches ; reset."),
        ("Skills par usage", "Compétences montent à l'utilisation ; peuvent baisser."),
        ("Cap total de skills", "Plafond global réparti."),
        ("Répartition libre", "Joueur choisit compétences à faire évoluer."),
        ("Skill gains dégressifs", "Plus haute la skill, plus lents les gains."),
        ("Achievements / succès", "Objectifs ; récompenses ; progression affichée."),
        ("Titres", "Succès débloqués ; affichage sous le nom."),
        ("Reset quotidien / hebdo", "Limites par jour/semaine."),
        ("Saisons / battle pass", "Contenu limité dans le temps."),
        ("Jobs et changement de classe", "Quêtes de changement ; évolution carrière."),
    ],
    "07-combat": [
        ("Action", "Compétences ; cooldowns ; cast time ; after-cast ; mana, endurance."),
        ("Auto-attaque de base", "Attaque automatique ; portée ; cadence ; dégâts."),
        ("Projectiles", "Création ; trajectoire ; collision ; dégâts ; durée de vie."),
        ("Gestion du parcours de combat", "Ciblage ; verrouillage ; priorité ; ordres."),
        ("Click-to-attack", "Ciblage au clic ; attaque cible ou lieu."),
        ("Chance de toucher", "Formule attaque vs défense ; précision vs esquive."),
        ("Esquive / flee", "Formule vs précision ; réduction dégâts."),
        ("Parade / block", "Parade bouclier ; % blocage ; contre-attaque."),
        ("Modificateurs de taille", "Petit, Moyen, Grand ; bonus/malus dégâts."),
        ("Modificateurs de race", "Bonus/malus selon race cible."),
        ("Vitesse d'attaque (ASPD)", "Vitesse attaque ; cap ; bonus."),
        ("Changement d'arme", "Swap en combat ; armes secondaires."),
        ("Zone d'effet (AOE)", "Cercle, cône ; dégâts multiples ; indicateurs."),
        ("Barre de cast", "Canalisation ; interruption ; feedback visuel."),
        ("Aggro / menace", "Gestion menace ; priorité ciblage."),
        ("Rôles", "Tank, DPS, soigneur ; synergies."),
        ("Officiers vs mooks", "Ennemis uniques vs soldats standards."),
        ("Officiers alliés", "PNJ contrôlables ; ordres ; perte possible."),
    ],
    "08-degats-resistances-effets": [
        ("Résistances", "Feu, froid, éclair, poison, physique, magique ; cap ; immunités."),
        ("Éléments", "Feu, Eau, Vent, Terre, Saint, Obscur, Neutre."),
        ("Immunités", "Immunes à un élément ou plusieurs."),
        ("Critical strike", "Coup critique ; dégâts amplifiés."),
        ("Dégâts en pourcentage", "Pourcentage vie enlevé en un coup."),
        ("Vol de vie / mana", "Sur les coups."),
        ("Knockback", "Repousser l'ennemi."),
        ("Effets de statut", "Buffs, debuffs ; stack ; durée ; dispel."),
        ("Crowd control (CC)", "Stun, root, silence, freeze, slow ; résistances."),
        ("Poison over time", "Dégâts poison dans le temps."),
        ("Autres effets", "Curse, stone, aveuglement, sleep."),
    ],
    "09-mort-resurrection": [
        ("Mort joueur / NPC", "États mort, en récupération."),
        ("Respawn", "Lieu réapparition ; run back."),
        ("Corps (corpse)", "Corps au sol ; lootable ; décomposition ; récupération."),
        ("Récupération du corps", "Retour corpse ; risque plusieurs corps."),
        ("Résurrection", "Skill, objet, NPC ; coût ; Rez in-combat ; cooldown."),
        ("Perte d'XP", "À la mort ; possibilité perdre niveau."),
        ("Drop à la mort", "Objets et monnaie au sol ; ou équipement conservé."),
        ("Cimetière / point de réapparition", "Lieu respawn."),
    ],
    "10-loot-tables": [
        ("Tables de loot", "Par type monstre ; rareté."),
        ("Ramassage", "Automatique zone pick-up joueur."),
        ("Bosses", "Spawn fixe ou long ; loot spécial ; annonces spawn/kill."),
        ("Droits de loot", "Premier coup, dernier coup ; partage."),
        ("Champion spawns", "Vagues mobs ; boss final ; artefacts."),
        ("Super uniques / mini-boss", "Boss nommés ; groupe modifié ; bonus."),
        ("Niveau de zone", "Niveau monstres selon zone ; scaling."),
        ("Cartes au trésor", "Trouver carte ; point ; creuser ; spawn + loot."),
    ],
    "11-inventaire-objets": [
        ("Inventaire", "Slots + poids max ; pas drag and drop ; empilable ; tri."),
        ("Stockage persistant", "Stash, coffre, stockage guilde ; partagé."),
        ("Poids / encumbrance", "Limite charge ; ralentissement ; surcharge."),
        ("Limite de stack", "Plafond par pile."),
        ("Menus contextuels", "Clic droit ; actions selon type objet."),
        ("Gumps / fenêtres", "Dialogs réutilisables ; sérialisables réseau."),
        ("Slots rapides", "Ceinture potions ; raccourcis ; empilable."),
        ("Prérequis", "Niveau ; stats requises équiper."),
        ("Durabilité", "Usure ; réparation ; casse définitive."),
        ("Renforcement", "Enchantement / raffinement ; risque échec ; over-upgrade."),
        ("Affixes", "Préfixe + suffixe ; génération procédurale."),
        ("Objets Set", "Pièces ensemble ; bonus 2, 3, 5+ pièces."),
        ("Objets uniques", "Stats fixes ; noms et histoires."),
        ("Slots et insertions", "Emplacements équipement ; gemmes ; cartes ; combinaisons."),
        ("Objets maudits", "Ne peuvent pas être droppés ; effets négatifs."),
        ("Objets bénis", "Ne tombent pas à la mort ; protection."),
        ("Charrette", "Inventaire supplémentaire ; suit personnage."),
        ("Vente (vending)", "S'asseoir ; boutique ; vendre hors-ligne."),
        ("Overcharge / Bargain", "Compétences vendre plus cher / acheter moins cher."),
    ],
    "12-economie-commerce": [
        ("Commerce", "Vente / achat ; marchands ; prix."),
        ("Échange entre joueurs", "Trade ; offre ; validation ; fenêtre échange."),
        ("Devises", "Or, argent ; monnaie premium ; monnaie avec poids."),
        ("Vendeurs joueur", "PNJ gérés par joueur ; vente hors-ligne."),
        ("Crafting / artisanat", "Recettes ; matériaux ; niveaux artisanat."),
        ("Récolte", "Ressources monde ; outils ; respawn."),
        ("Mining", "Minage minerais ; types ; veines."),
        ("Lumberjacking", "Abattage arbres ; types bois."),
        ("Fishing", "Pêche ; types poissons ; trésors."),
        ("Recettes et matériaux", "Combinatoire ; ressources ; qualité variable."),
    ],
    "13-social-groupes": [
        ("Liste d'amis", "Ajout, suppression ; statut en ligne ; invite."),
        ("Clans", "Création ; membres ; rangs ; banque ; log ; emblème."),
        ("Stockage de clan", "Coffre partagé ; dépôt/retrait par rangs."),
        ("Compétences de clan", "Skills financées ; effets groupe."),
        ("Groupe / party / raid", "Invitation ; composition ; partage loot."),
        ("Chat", "Canaux ; whisper ; modération."),
        ("Boîte aux lettres", "Envoi objets/monnaie ; expiration ; limite."),
        ("Liste noire", "Ignorer joueur ; blocage messages."),
        ("Réputation / factions", "Rang faction ; récompenses ; hostilité."),
        ("Guerre de factions", "Ordre vs Chaos ; contrôle villes ; bonus."),
        ("Clan wars", "Guerre clan vs clan ; déclaration ; objectifs."),
    ],
    "14-siege-territoire": [
        ("Organisation hiérarchique", "Seigneur, vassaux ; serment ; loyauté."),
        ("Roi et royaume", "Couronne ; déclaration guerre ; tributs ; protection."),
        ("Possession de château", "Un clan contrôle un château."),
        ("Siège de château", "Événement périodique ; attaque vs défense."),
        ("Portes et murailles", "Destructibles ; points contrôle ; prise et perte."),
        ("Zones de contrôle", "Bases, postes, fortifications."),
        ("Points de capture", "Objectifs ; temps capture ; résistance."),
        ("Armes de siège", "Catapultes ; engins ; dégâts structures."),
        ("Gardes NPC", "Défenseurs automatiques."),
        ("Taxes", "Perception territoire contrôlé."),
        ("Bannière", "Drapeau clan ; affichage."),
        ("Morale des troupes", "Dépend combat ; fuite ; renforcement."),
        ("Territoire", "Contrôle zones ; influence flux."),
    ],
    "15-reputation-criminalite": [
        ("Karma et Fame", "Réputation bien/mal ; influence PNJ et PvP."),
        ("Alignement", "Loyal, Neutre, Chaotique ; couleur nom."),
        ("Système de meurtrier", "Noms rouges ; perte stats respawn ; statut persistant."),
        ("PK (Player Kill)", "Tuer loyal = devenir chaotique."),
        ("Rédemption", "Retour loyal ; temps ; quêtes ; expiation."),
        ("Full loot PvP", "Zones hostiles ; tout droppé à mort."),
        ("Zones PK", "Zones PvP autorisé ; zones sûres."),
        ("Actions criminelles", "Vol, meurtre, intrusion ; flag criminel ; gardes."),
        ("Lockpicking", "Crochetage ; conteneurs verrouillés."),
        ("Stealing / pickpocket", "Vol sacs ; flag criminel ; échec possible."),
        ("Snooping", "Consulter contenu sac d'un autre."),
    ],
    "16-magie-sorts": [
        ("Mana", "Ressource lancer sorts."),
        ("Coût en mana", "Coût par sort ; régénération."),
        ("Cercles de magie", "Organisation sorts ; puissance progressive."),
        ("Composants / réagents", "Consommation ressources pour sorts."),
        ("Livres de sorts", "Sorts connus ; grimoires ; apprentissage via objet."),
        ("Compétences passives", "Auras ; maîtrises ; bonus permanents."),
        ("Sorts de zone", "AOE ; dégâts zone."),
        ("Hotkeys", "Raccourcis ; clic gauche, droit."),
        ("Barding / provocation", "Faire combattre créatures entre elles."),
    ],
    "17-montures-familiers": [
        ("Montures", "Acquisition ; vitesses ; animations ; combat monté."),
        ("Combat à cheval", "Attaques montées ; désarçonnement."),
        ("Montures spéciales", "Capacités uniques."),
        ("Familiers / pets", "Suivi ; buffs passifs ; cosmétiques."),
        ("Capture", "Oeufs ; éclosion ; évolution."),
        ("Faim / intimité", "Nourrir ; lien affectif ; évolution."),
        ("Compétences de pet", "Capacités actives ou passives."),
        ("Taming", "Dompter créatures ; difficulté par type."),
        ("Bonding", "Lien durée ; devient permanent."),
        ("Mercenaires", "Recrutement ; suit joueur ; équipement ; évolution ; résurrection."),
        ("Homunculus", "Création ingrédients ; IA ; compétences ; évolution ; nourriture."),
    ],
    "18-logement-monde": [
        ("Logement / housing", "Maison joueur ; placement objets ; visite."),
        ("Maisons dans le monde", "Monde persistant, pas instance."),
        ("Decay des maisons", "Abandon = dégradation puis suppression."),
        ("Placement d'objets", "Décor ; coffres ; objets posés."),
        ("Cycle jour / nuit", "Heure jeu ; luminosité ; événements liés."),
        ("Météo", "Pluie, neige, brouillard ; effets visuels/audio."),
        ("Zones PvP", "Conflit ouvert ; récompenses PvP."),
        ("Zones de faction", "Territoire ; contrôle ; bonus."),
        ("Zones sûres", "Pas PvP ; villes ; hubs."),
        ("Kafra / services centralisés", "Sauvegarde ; téléport ; stockage ; courrier ; location."),
    ],
    "19-quetes-missions": [
        ("Quêtes", "Journal ; suivi ; objectifs ; récompenses ; chaînes."),
        ("Objectifs dynamiques", "Changements objectif selon état monde."),
        ("Branchements", "Succès / échec selon actions ; chemins alternatifs."),
        ("Pression temporelle", "Timer ; objectifs à temps."),
        ("Sauvetage d'alliés", "Alliés en danger ; mission secours."),
        ("Défaite d'alliés", "Mort alliés ; impact mission."),
        ("Quêtes de changement de rang", "Épreuves ; objectifs ; récompense = nouveau rang."),
    ],
    "20-interface": [
        ("GUI", "Menus ; HUD ; boutons ; inventaire visuel ; tooltips."),
        ("Carte du monde", "Vue globale ; waypoints ; découverte."),
        ("Suivi de quêtes", "Objectifs actuels ; distance ; indicateurs."),
        ("Unit frames", "Barres vie/mana party/raid ; cibles ; buffs."),
        ("Barre de cast", "Progression sort ; annulation."),
        ("Indicateurs de cooldown", "Temps restant compétences."),
        ("Notifications", "Alertes ; loot ; invitations ; événements."),
    ],
    "21-ia-bots": [
        ("IA bot", "Comportements ; états Idle, Chase, Attack, Flee ; machine à états."),
        ("Pathfinding", "Recherche chemin ; obstacles."),
        ("Navmesh", "Graphe navigation."),
    ],
    "22-reseau-serveurs": [
        ("Réseau / MWS", "Connexion multijoueur ; synchronisation ; Lobbys."),
        ("Sélection de serveur", "Liste mondes ; population ; latence."),
        ("Sharding", "Fragmentation monde ; répartition joueurs."),
        ("Cross-server / cross-realm", "Groupes entre serveurs ; files attente."),
        ("Compensation de latence", "Prédiction ; réconciliation ; lag compensation."),
        ("Anti-cheat", "Détection ; validation côté serveur."),
        ("Tick serveur", "Fréquence simulation ; synchronisation."),
        ("Trading", "Échange direct ; fenêtre trade."),
        ("PvP", "Duel ; zones hostiles."),
        ("Co-op partagé", "Plusieurs joueurs champ bataille."),
        ("Écran partagé", "Split screen ou vue partagée."),
    ],
    "23-systeme": [
        ("Boucle de jeu", "Game loop ; delta time ; frame rate ; pause."),
        ("Entrées utilisateur", "Clavier ; souris ; manette ; mapping."),
        ("Gestion du temps", "Delta time ; temps jeu ; timers ; cooldowns."),
        ("Chargement des assets", "Textures ; sons ; fonts ; hot reload (dev)."),
        ("Sauvegarde / chargement", "Persistance ; KindMother ; sérialisation ; slots."),
        ("Audio", "Musique ; SFX ; volume ; canaux ; spatial."),
        ("Musique adaptative", "Intensité selon contexte ; transitions."),
        ("États du jeu", "Menu ; jeu ; pause ; game over ; transitions."),
        ("Système d'événements", "Événements internes ; signaux ; délégués."),
        ("Règles du jeu", "Règles ; conditions ; validations ; mods."),
        ("Debug et outils dev", "Overlays ; logs ; inspector ; raccourcis."),
        ("Configuration", "Options ; paramètres ; préférences ; persistance."),
        ("Localisation (i18n)", "Textes traduisibles ; clés ; langues."),
        ("Optimisation", "Culling ; LOD ; pooling ; cache."),
    ],
    "24-meta-moderation": [
        ("Classements / leaderboards", "PvP ; PvE ; économie ; saison."),
        ("Outils de modération", "Ban ; mute ; logs ; outils GM."),
        ("Système de signalement", "Report joueur ; catégories ; suivi."),
        ("Actualités in-game", "Patch notes ; annonces ; événements."),
    ],
}

def slug(s: str) -> str:
    """Convertit un nom de point en slug pour le fichier."""
    accents = {'é':'e','è':'e','ê':'e','à':'a','â':'a','ù':'u','û':'u','ô':'o','î':'i','ï':'i','ü':'u','ç':'c','/':'-','(':'',')':''}
    for a, b in accents.items():
        s = s.replace(a, b)
    s = s.lower()
    s = re.sub(r'[^\w\s-]', '', s)
    s = re.sub(r'[\s_]+', '-', s)
    s = re.sub(r'-+', '-', s).strip('-')
    return s or "point"

def make_point_file(cat_slug: str, point_name: str, desc: str, file_slug: str, cat_num: int) -> str:
    return f"""# {point_name}

**Catégorie :** {cat_num}. {cat_slug.replace("-", " ").title()}  
**Description :** {desc}

## Contexte

Point de la référence technique MGE.

## Portée

- À définir

## À développer

- [ ] Spécifications
- [ ] Implémentation
- [ ] Tests

## Références

- [Index catégorie](_index.md)
- [Index MGE](../_index.md)
"""

def main():
    BASE.mkdir(parents=True, exist_ok=True)
    
    # Skip categories already created (01-04)
    SKIP = ["01-affichage-rendu", "02-physique-collisions", "03-deplacement-locomotion", "04-entites-monde"]
    
    cat_names = {
        "01-affichage-rendu": "Affichage et rendu",
        "02-physique-collisions": "Physique et collisions",
        "03-deplacement-locomotion": "Déplacement et locomotion",
        "04-entites-monde": "Entités et monde",
        "05-joueur-personnage": "Joueur et personnage",
        "06-progression": "Progression",
        "07-combat": "Combat",
        "08-degats-resistances-effets": "Dégâts, résistances et effets",
        "09-mort-resurrection": "Mort et résurrection",
        "10-loot-tables": "Loot et tables",
        "11-inventaire-objets": "Inventaire et objets",
        "12-economie-commerce": "Économie et commerce",
        "13-social-groupes": "Social et groupes",
        "14-siege-territoire": "Siège et territoire",
        "15-reputation-criminalite": "Réputation et criminalité",
        "16-magie-sorts": "Magie et sorts",
        "17-montures-familiers": "Montures et familiers",
        "18-logement-monde": "Logement et monde",
        "19-quetes-missions": "Quêtes et missions",
        "20-interface": "Interface",
        "21-ia-bots": "IA et bots",
        "22-reseau-serveurs": "Réseau et serveurs",
        "23-systeme": "Système",
        "24-meta-moderation": "Meta et modération",
    }
    
    for cat_slug, points in CATEGORIES.items():
        if cat_slug in SKIP:
            continue
        cat_dir = BASE / cat_slug
        cat_dir.mkdir(parents=True, exist_ok=True)
        cat_num = int(cat_slug.split("-")[0])
        cat_title = cat_names.get(cat_slug, cat_slug)
        
        # Index catégorie
        rows = []
        for pname, pdesc in points:
            fslug = slug(pname)
            rows.append(f"| {pname} | [{fslug}]({fslug}.md) | {pdesc} |")
        
        index_content = f"""# {cat_num}. {cat_title}

Catégorie MGE.

## Points

| Point | Fichier | Description |
|-------|---------|-------------|
"""
        index_content += "\n".join(rows)
        index_content += "\n\n[← Retour à l'index MGE](../_index.md)\n"
        
        (cat_dir / "_index.md").write_text(index_content, encoding="utf-8")
        
        # Fichiers points
        for pname, pdesc in points:
            fslug = slug(pname)
            content = make_point_file(cat_slug, pname, pdesc, fslug, cat_num)
            (cat_dir / f"{fslug}.md").write_text(content, encoding="utf-8")
    
    print(f"Généré {sum(len(p) for p in CATEGORIES.values())} points dans {len(CATEGORIES)} catégories.")

if __name__ == "__main__":
    main()
