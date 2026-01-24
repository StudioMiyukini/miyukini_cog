Tu es un architecte logiciel senior et un ingénieur backend expérimenté.
Ta mission est de concevoir le CŒUR (KERNEL) d’une fondation technique long terme.

Contexte :
- Nom du dépôt : miyukini-core-system
- Langage cible : Rust
- Cas d’usage : backends SaaS, sites web, applications temps réel, jeux multijoueurs (MMO à long terme)
- Priorité absolue : livrer des produits SaaS et web en premier
- Contraintes :
  - Développement solo ou petite équipe
  - Coûts d’infrastructure très faibles
  - Éviter le bloat et les abstractions prématurées
  - Un seul kernel, plusieurs surfaces (web, mobile, jeu)
  - Maintenabilité long terme (5 à 10 ans)

RÈGLES IMPORTANTES :
- NE PAS écrire de code.
- NE PAS proposer une architecture complète prématurée.
- NE PAS introduire de patterns inutiles.
- Se concentrer UNIQUEMENT sur ce qui relève du kernel.
- Tout ce qui n’est pas strictement kernel doit être explicitement exclu.

Le résultat attendu est un DOCUMENT MARKDOWN structuré de la façon suivante :

1. Définition du Kernel
   - Ce que le kernel EST
   - Ce que le kernel N’EST PAS

2. Responsabilités fondamentales
   - Liste et explication UNIQUEMENT des responsabilités qui doivent vivre dans le kernel
   - Chaque responsabilité doit justifier pourquoi elle appartient au kernel

3. Exclusions explicites
   - Liste des éléments qui ne doivent JAMAIS faire partie du kernel (même s’ils sont tentants)

4. Frontières du Kernel
   - Comment le kernel interagit avec :
     - les frontends web
     - les applications mobiles
     - les clients de jeu
     - les services externes
   - Ces interactions doivent être décrites comme des contrats, pas comme des implémentations

5. Modules minimaux du Kernel (v0.1)
   - Identifier l’ensemble le plus petit possible de modules pour démarrer
   - Chaque module doit :
     - avoir une responsabilité unique
     - être utile à au moins deux types de produits (ex : SaaS + jeu)

6. Stratégie d’évolution
   - Comment le kernel est censé évoluer dans le temps
   - Règles claires pour ajouter un nouveau module
   - Règles claires pour NE PAS ajouter de module

7. Critères de succès
   - Comment savoir que le kernel est “suffisamment bon” pour commencer à livrer des produits

Ton :
- Pragmatique
- Tranché
- Anti-bloat
- Orienté long terme
