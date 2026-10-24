# Services hybrides et durcissement du Portail interne

<!-- @id arch.cog.services-hybrides -->
<!-- @role architecture-specification -->
<!-- @layer 3-8 -->
<!-- @human Services à cheval Franche / zone COG : classes d'exposition, gabarit moteur-façade-traité, durcissement du Portail interne -->
<!-- @do design_hybrid_services_and_harden_portal -->

- **Statut** : v1, 2026-10-24
- **Base** : [modèle zones et couches](MODELE-ZONES-ET-COUCHES-COG.md) (§9 services relayés, §10 contrat) · [menaces](../securite/MODELE-MENACES-COG.md)

## 1. Classes d'exposition

<!-- @id arch.hybride.classes -->
<!-- @role exposure-classification -->
<!-- @do classify_service_exposure -->

- **H0 — Interne pur** : aucune façade, aucun flux externe (MAIA, Cores, compta)
- **H1 — Hybride diffusion** : la zone COG **publie**, rien n'entre (site, partage en lecture seule)
- **H2 — Hybride collaboratif** : des données **entrent** depuis l'extérieur (bureautique coopérative, messagerie, mail entrant)
- **H3 — Hybride actif** : la façade **agit** à l'extérieur pour le compte de la zone COG (explorateur web, mail sortant, domotique)
- Règles par classe :
  - H0 : jamais de façade, même à la demande
  - H1 : voie **publication** seule
  - H2 : voies quarantaine + publication, validation à chaque entrée
  - H3 : comme H2 + destinations sortantes en liste blanche + intention StrongFather pour chaque action
- La classe est déclarée dans le **contrat** (manifeste signé) ; changer de classe = nouvelle installation

## 2. Gabarit d'un service hybride

<!-- @id arch.hybride.gabarit -->
<!-- @role service-template -->
<!-- @layer 5-7 -->
<!-- @do structure_hybrid_service -->

- **Moteur** (zone COG) : logique métier, données, prérequis
- **Façade** (Franche) : terminaison des protocoles externes, assainissement, rien d'autre
- **Traité** : schéma versionné des déclarations échangées moteur ⇄ façade, signé dans le manifeste
- Règles :
  - **INV-H1** le moteur **ne parse jamais** un protocole externe (HTTP, TLS, SMTP, formats de fichiers bruts)
  - **INV-H2** la façade n'a **aucune logique métier** ni secret durable
  - **INV-H3** seuls les types définis par le traité traversent ; tout le reste est refusé
  - **INV-H4** décodeurs générés depuis le traité, des deux côtés (aucune divergence)
  - **INV-H5** façade **jetable** : reconstruite depuis une image signée à chaque session (ou chaque jour), jamais réparée
  - **INV-H6** identifiants de façade **éphémères**, émis par la zone COG pour la session

## 3. Durcissement du Portail interne

<!-- @id arch.hybride.portail -->
<!-- @role boundary-hardening -->
<!-- @layer 3-6 -->
<!-- @do harden_internal_portal -->

**Structure**
- Instance dédiée, espace isolé puis micro-VM
- **Deux voies à sens unique séparées** (diode logicielle) :
  - voie **entrante** (quarantaine → zone COG)
  - voie **sortante** (zone COG → publication)
  - processus distincts, aucune mémoire partagée, aucun chemin retour (acquittements par compteurs seulement)
- **Une voie par service** : files, quotas et clés distincts ; une façade compromise n'atteint pas les autres voies

**Contrôles sur la voie entrante**
1. Trame `MSCE` uniquement, taille bornée (IPS BorderGuard)
2. Signature de la façade (clé de session) **et** de l'auteur (participant)
3. Type autorisé par le traité du service, débit dans le contrat
4. **Désarmement et reconstruction (CDR)** : l'objet est décodé puis **réencodé** dans un format interne sûr ; aucun octet brut ne traverse
   - documents → format interne (sans macro, script, objet embarqué)
   - images → réencodées
   - archives → refusées, ou dépaquetées et contrôlées une à une
5. Inscription au registre Merkle (empreinte)

**Côté zone COG**
- **Sas de décodage** : processus séparé, seccomp strict, qui décode la déclaration en structures typées
- Le moteur ne reçoit que des **structures typées** par IPC (« parse, don't validate »)
- **Deuxième validation**, indépendante de celle du Portail (deux implémentations)
- Décision StrongFather ; TAMR pour les types sensibles
- La zone COG **relève** à son rythme (contre-pression naturelle)

**Contrôles sur la voie sortante**
- **Étiquettes de sensibilité** posées par KindMother sur chaque donnée :
  - `nationale` : ne sort jamais
  - `session` : visible des participants d'une session
  - `publiable` : diffusable
- Seules les données `session` / `publiable` peuvent sortir, **par action explicite de l'utilisateur**
- MAIA ne peut rien étiqueter `publiable` ni publier
- Types de publication déclarés dans le traité, taille bornée

**Réaction**
- WorrySentinel (IDS) peut **couper une voie** sans toucher aux autres
- Coupure = façade détruite et reconstruite, session close, incident au registre

## 4. Cycle de vie d'un service hybride

<!-- @id arch.hybride.cycle-de-vie -->
<!-- @role lifecycle -->
<!-- @do manage_hybrid_service_lifecycle -->

- **Installation** : contrat et traité vérifiés (StrongFather), voies créées, espace Franche dimensionné (§10 du modèle)
- **Exposition désactivée par défaut** : l'utilisateur l'active explicitement, service par service
- **Session** : visa de session, façade neuve, DB secondaire vierge
- **Fin de session** : façade détruite, DB secondaire purgée
- **Mise à jour** : moteur et façade mis à jour ensemble ; versions de traité compatibles ou refus
- **Désactivation** : voies supprimées, aucun résidu en Franche

## 5. Catalogue

<!-- @id arch.hybride.catalogue -->
<!-- @role service-catalog -->
<!-- @do classify_existing_services -->

- **H0** : MAIA, Cores, JayKonta (compta), KindMother
- **H1** : site / page web publique, partage MiyuCloud en lecture seule
- **H2** : JayOffice (Docs, Sheets, Slides, Réunion), JayMessage, partage MiyuCloud en écriture, JayKoa partagé, mail entrant, accès mobile (via Fédération)
- **H3** : explorateur web, mail sortant, Alicia domotique (Franche locale)
- Hors classe : Market et mises à jour → **canal officiel** (§7 du modèle)

## 6. Exigences

<!-- @id arch.hybride.exigences -->
<!-- @role security-requirements -->
<!-- @do derive_hybrid_requirements -->

- **EX-40** Classe d'exposition déclarée dans le contrat signé ; H0 jamais exposé
- **EX-41** Moteur sans parseur de protocole externe (INV-H1)
- **EX-42** Traité versionné et décodeurs générés des deux côtés
- **EX-43** Façades jetables, reconstruites depuis une image signée
- **EX-44** Portail en deux voies à sens unique, une voie par service
- **EX-45** CDR obligatoire sur toute entrée de fichier ou document
- **EX-46** Sas de décodage seccomp en zone COG + deuxième validation indépendante
- **EX-47** Étiquettes de sensibilité KindMother ; seule une action utilisateur rend une donnée publiable
- **EX-48** Exposition désactivée par défaut, activée service par service
- **EX-49** Coupure de voie par WorrySentinel sans impact sur les autres services
