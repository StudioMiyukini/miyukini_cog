# 18 - MVP cible : camp de depart + Acte 1 complet

## Definition exacte du MVP

Le MVP de `Sodomight` n'est pas un micro-slice incomplet. Il doit livrer:

- toutes les fonctionnalites systemiques majeures de D2
- un contenu jouable borne au camp de depart et a l'Acte 1 complet
- une integration `Central` suffisamment robuste pour installer, lancer et tester le jeu

## Systeme complet vs contenu borne

### Systeme complet

Le MVP doit couvrir des le premier livrable:

- creation personnage
- classes / skills / stats / level up
- combat temps reel
- loot et itemisation
- vendors / stash / cube equivalent / gambling
- quetes / waypoints / portal
- mercenaire
- hardcore
- party / coop / PvP
- ladder au niveau runtime, donnees, classement et contrats
- packaging et saves

Les systemes D2 dont l'expression naturelle est hors Acte 1 doivent quand meme exister:

- via contrats de donnees stables
- via harness, commandes debug ou scenes de validation
- via tests ou preuves de fonctionnement documentees

### Contenu borne

Le MVP n'a pas a livrer:

- contenu authored des actes 2 a 5
- campagnes secondaires
- endgame complet au-dela de la campagne cible
- exploitation live saisonniere complete

Il doit en revanche livrer:

- camp de depart complet
- progression de quetes et zones de l'Acte 1 complete
- boss final d'Acte 1
- boucles de farm et retour ville deja satisfaisantes

## Raison de cette borne

- garder la profondeur systemique de D2
- eviter un "prototype faux" qui ne prouve rien
- limiter la masse de contenu pour converger
- rendre les tests et l'equilibrage possibles

## Definition de fini MVP

- le joueur cree un personnage
- traverse le camp de depart
- termine toutes les quetes critiques de l'Acte 1
- tue le boss final d'Acte 1
- a acces aux systemes D2 cibles, directement en jeu ou via harness de validation
- peut fermer, relancer et reprendre sa progression
