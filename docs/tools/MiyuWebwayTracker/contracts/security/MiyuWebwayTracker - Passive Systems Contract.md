# MiyuWebwayTracker -- Contrat des systemes passifs

## Contexte

Les **systemes passifs** du COG Tracker MWS (MiyuWebwayTracker) sont des mecanismes qui **observent, enregistrent et signalent** sans modifier de maniere proactive le comportement des connexions ou des annonces. Ils informent et alimentent la decision ; ils ne coupent ni ne modifient le flux par eux-memes. Ce contrat formalise les **preconditions, postconditions et invariants** applicables aux systemes passifs du Tracker.

> **Role du Tracker :** Le Tracker est le **douanier du reseau**. Il controle l'**identite** et le **Permis de circulation** des COGs avant de les laisser se connecter au maillage (contrÃ´le tracker). Il dirige des **pools par version des Cores** pour ne jamais connecter des COGs avec des versions differentes. Il **ne fait pas** de verification lourde de conformite de l'environnement (Passeport, cle Cores, blocs de code Services) -- cette responsabilite incombe aux **relays** qui delivrent les Permis de circulation (accord relay) (voir [Miyukini Webway Relay](..//..//..//..//miyukini-webway-system//reference//_index.md) section 2). Le Tracker gere les **whitelists, blacklists et quarantaines** et peut **fermer des connexions** pour circonscrire les attaques sur annonce des relays.

**Reference conceptuelle :** [Miyukini Conceptual References - Miyukini Webway System](..//..//..//..//miyukini-webway-system//reference//_index.md) (section 5.1).

## Portee / Scope

- **Validation syntaxique** des annonces et requetes MWS (conformite au schema, champs obligatoires).
- **Verification de signature** (integrite et authenticite des declarations selon la norme de declaration securisee).
- **Filtrage par statut** (consultation des listes de COGs avec statuts pour alimenter la decision ; pas d'action de blocage dans le passif).
- **Journalisation** (tracabilite des annonces, requetes, mises a jour de statuts, detection d'anomalies a posteriori).
- **Mise a jour et partage des listes de COGs avec statuts** (Trusted, Neutral, Under review, Distrusted, Rejected).
- **Signalement** vers d'autres Trackers ou COGs (reputation, alertes) sans couper le flux.

Ce contrat **ne definit pas** les systemes actifs (blocage, refus, degradation) ; voir [MiyuWebwayTracker - Active Systems Contract](MiyuWebwayTracker%20-%20Active%20Systems%20Contract.md).

---

## 1. Preconditions (systemes passifs)

Avant d'appliquer un mecanisme passif (validation, journalisation, mise a jour de liste, signalement), les preconditions suivantes sont supposees ou doivent etre assurees :

| ID | Precondition | Applicabilite |
|----|--------------|---------------|
| P1 | Le message ou l'evenement entrant est un **flux MWS** (annonce de presence, requete de decouverte, mise a jour de liste de statuts, ou message de protocole MWS reconnu). | Tous les traitements passifs |
| P2 | Le Tracker dispose d'un **contexte de verification** (acces aux cles ou secrets necessaires pour verifier les signatures, selon la norme de declaration securisee). | Verification de signature |
| P3 | Les **schemas et versions** de messages supportes sont definis (reference aux [Normes et Standards](..//..//..//..//miyukini-webway-system//reference//_index.md)). | Validation syntaxique |
| P4 | La **liste locale de COGs avec statuts** (Webway COG List) est initialisee ou chargee ; les politiques de mise a jour (qui peut fournir un statut, agregation) sont connues. | Filtrage par statut, journalisation, signalement |
| P5 | Les **canaux de signalement** (autres Trackers, COGs, ou composants internes tels que WorrySentinel) sont configures si le passif doit emettre des alertes ou partager des signaux. | Signalement |
| P6 | L'**empreinte de version** (core_version, service_manifest, protocol_version) est presente dans l'annonce ou la requete MWS et le Tracker connait les versions supportees / minimales (politique locale). | Verification de version |
| P7 | Le Tracker a acces au **Registre de Services** du Relay Origin (directement ou via un cache local synchronise periodiquement). La liste des services officiels et tiers repertories est disponible pour verification. | Verification du Registre de Services |
| P8 | Les **canaux de notification utilisateur** sont configures pour informer le proprietaire du COG en cas de service non repertorie ou de mise a jour critique disponible. | Notification utilisateur |
| P9 | Le Tracker possede les **listes de whitelists, blacklists et quarantaines** synchronisees avec Origin et les relays. | Controle d'identite et contrÃ´le tracker |
| P10 | Les **pools par version des Cores** sont configures : le Tracker connait les versions actives et peut diriger chaque COG vers le pool correspondant a sa `core_version.MAJOR`. | Pools de version |
| P11 | Le COG qui se presente au Tracker possede un **Permis de circulation valide** delivre par un relay ou Origin (accord relay). | ContrÃ´le tracker |
| P12 | Le Tracker expose un **service web de catalogue** (port 80) presentant les **services WEB publics** des COGs connectes (URLs, type moteur de recherche) ; les **Lobbys des autres services ne sont pas visibles** depuis ce portail. Le Tracker gere le **catalogue de Lobbys** (surfaces, attentes, desirs) ; ce catalogue est **visible depuis les services COG** concernes, pas depuis le portail web. Lobbys mis a jour et diffuses automatiquement, global. | Catalogue web ; catalogue de Lobbys |

---

## 2. Postconditions (systemes passifs)

Apres l'execution d'un mecanisme passif, les postconditions suivantes doivent etre satisfaites :

| ID | Postcondition | Mecanisme concerne |
|----|----------------|--------------------|
| Q1 | **Validation syntaxique** : si le message est accepte pour traitement, il a ete verifie conforme au schema (version, type, champs obligatoires, types et contraintes). Les messages non conformes sont rejetes ou ignores sans modifier le flux des autres connexions. | Validation |
| Q2 | **Verification de signature** : si la norme de declaration securisee s'applique et que la verification est effectuee, le resultat (valide / invalide) est enregistre ; en cas d'echec, l'evenement est journalise et peut alimenter un signalement. Aucune modification du flux (blocage) n'est imposee par le passif seul. | Verification signature |
| Q3 | **Journalisation** : chaque annonce, requete ou mise a jour de statut traitee fait l'objet d'une entree de journal (tracabilite, horodatage, identifiants pertinents, resultat de validation/verification). Les donnees enregistrees ne contiennent pas de donnees metier ni de secrets de gouvernance. | Journalisation |
| Q4 | **Liste de statuts** : toute mise a jour de statut (locale ou recue d'un autre COG/Tracker) est refletee dans la liste locale avec source et horodatage ; la liste reste coherente pour les usages ulterieurs (filtrage, signalement, systemes actifs). | Filtrage par statut, listes |
| Q5 | **Signalement** : si une anomalie ou un critere de signalement est atteint (ex. signature invalide, comportement suspect), un signal ou une alerte est emis vers les canaux configures (sans bloquer le flux dans le cadre du passif). | Signalement |
| Q6 | **Verification de version** : l'empreinte de version du COG est validee (format core_version correct, protocol_version supportee). Le resultat (compatible / obsolete / incompatible) est enregistre. Si la core_version est obsolete, un signalement peut etre emis (Q5) sans blocage dans le cadre du passif. La liste locale de COGs est enrichie avec l'empreinte de version pour le filtrage ulterieur. | Verification de version |
| Q7 | **Verification du Registre de Services** : chaque `service_id` present dans le `service_manifest` du COG annonceur est verifie contre le Registre de Services du Relay Origin. Le resultat pour chaque service (repertorie / non repertorie / suspendu) est enregistre dans la liste locale de COGs. Si un service non repertorie est detecte, l'evenement est journalise (Q3) et un signalement est emis (Q5). | Verification Registre |
| Q8 | **Suivi des mises a jour** : le Tracker compare les versions des services du COG avec le Registre. Si une mise a jour est disponible (en particulier `critical`), l'information est enregistree et un signalement peut etre emis. Le Tracker ne force pas la mise a jour ; il alimente les systemes actifs et les notifications. | Suivi mises a jour |
| Q9 | **ContrÃ´le tracker** : le Tracker verifie la validite du Permis de circulation (non expire, emis par un relay ou Origin reconnu, scope coherent avec la requete). Le resultat est journalise. | ContrÃ´le tracker |
| Q10 | **Assignation au pool de version** : le COG est dirige vers le pool correspondant a sa `core_version.MAJOR`. Le Tracker journalise l'assignation et n'autorise aucune connexion inter-COG entre pools de versions differentes. | Pools de version |
| Q11 | **Monitoring de congestion** : le Tracker enregistre le nombre de connexions par COG et detecte les points de congestion. Si un COG accumule un nombre anormalement eleve de connexions, un signalement est emis. | Monitoring reseau |
| Q12 | **Catalogue web (port 80)** : catalogue des services WEB publics (URLs, recherche) ; les Lobbys ne sont pas affiches sur ce portail. **Catalogue de Lobbys** : les COGs declarent leurs surfaces (services, ports, acceptation de connexions) ; les Lobbys sont repertories et **visibles/joignables depuis les services COG** concernes (ex. client jeu, client SaaS). Le tracker indique les chemins aux clients pour joindre les hÃ´tes. Lobbys prives : mot de passe, 5 echecs puis ban, notification au hÃ´te, de-ban manuel (voir [Miyukini Webway Relay](..//..//..//..//miyukini-webway-system//reference//_index.md) section 9.1â€“9.2). | Catalogue web ; catalogue de Lobbys |

---

## 3. Invariants (systemes passifs)

Les invariants suivants doivent etre maintenus en permanence par le Tracker pour les systemes passifs :

| ID | Invariant | Description |
|----|-----------|-------------|
| I1 | **Pas d'action de blocage** : les systemes passifs **ne refusent pas** eux-memes les connexions ni les annonces ; ils informent et alimentent la decision. Toute decision de rejet ou blocage releve des **systemes actifs** ou des Cores (Border Guard, WorrySentinel). | Role passif |
| I2 | **Pas de donnees metier** : les donnees collectees, journalisees ou signalees ne contiennent **aucune donnee metier** ni secret de gouvernance ; uniquement identifiants COG, statuts, metadonnees de tracabilite et signaux de conformite (ex. signature valide/invalide). | Confidentialite / portee |
| I3 | **Coherence liste de statuts** : la liste locale de COGs avec statuts est **coherente** (pas de contradiction non resolue pour un meme `cog_id` dans le cadre des regles d'agregation definies) et **tracable** (source, updated_at). | Listes |
| I4 | **Souverainete** : le Tracker peut **ignorer** un statut fourni par un autre COG et appliquer sa propre politique d'agregation ; l'invariant porte sur la coherence interne, pas sur l'obligation d'accepter tout statut externe. | Souverainete |
| I5 | **Pas d'enumeration** : les systemes passifs ne doivent **jamais** exposer la liste complete des `cog_id` enregistres a un demandeur non autorise. Les reponses de decouverte sont filtrees selon la politique des Cores ; un attaquant ne doit pas pouvoir deduire l'ensemble des COGs par des requetes successives. | Anti-enumeration |
| I6 | **Integrite des logs** : les entrees de journal ne peuvent pas etre modifiees retroactivement par le Tracker ni par un tiers. Si le systeme de logging le permet, les logs doivent etre proteges en ecriture (append-only) et signes ou hashes pour detecter toute alteration. | Integrite audit |
| I7 | **Coherence de version dans la liste** : chaque entree de la liste locale de COGs avec statuts contient l'empreinte de version (au minimum `core_version` et `protocol_version`). Les reponses de decouverte peuvent etre filtrees par compatibilite de `core_version.MAJOR` pour ne retourner que les COGs avec lesquels le demandeur peut interagir. | Versioning |
| I8 | **Conformite au Registre de Services** : la liste locale de COGs avec statuts inclut pour chaque COG le resultat de la verification du Registre (tous services repertories / service(s) non repertorie(s)). Les COGs avec un service non repertorie sont marques dans la liste et peuvent etre exclus des reponses de decouverte (selon politique). | Registre de Services |
| I9 | **Tracabilite des mises a jour** : chaque verification de mise a jour effectuee (service par service) est journalisee (date, service_id, version COG vs version Registre, resultat). L'historique permet d'auditer la conformite des COGs et la reactivite aux mises a jour critiques. | Suivi mises a jour |

---

## 4. Exigences de securite renforcees (systemes passifs)

### 4.1 Validation et verification

- **Validation stricte** : toute annonce ou requete MWS recue **doit** etre validee syntaxiquement **avant** tout traitement. Un message malform est immediatement rejete (postcondition Q1) et l'evenement est journalise avec l'adresse source et l'horodatage.
- **Verification de signature obligatoire** : si la norme de declaration securisee (MWS) s'applique, la verification de signature est **obligatoire** pour toute annonce de presence. Une annonce avec signature invalide **ne doit pas** etre enregistree dans la liste locale de COGs ; elle est journalisee et signalee (Q2, Q5).
- **Validation de timestamp** : les annonces doivent contenir un horodatage ; le Tracker **doit** rejeter les annonces avec un timestamp hors de la fenetre d'acceptation (ex. +/- 5 minutes) pour empecher le rejeu d'annonces perimees.
- **Rejet des cog_id malformes** : les `cog_id` qui ne respectent pas les contraintes de format (longueur, encodage UTF-8 valide, pas de caracteres de controle) sont rejetes a la validation syntaxique.

### 4.2 Journalisation securisee

- **Donnees sensibles interdites dans les logs** : ne **jamais** logger les tokens d'authentification relay, les secrets partages, les cles cryptographiques, ni le contenu des donnees metier. Uniquement les `cog_id`, adresses IP, horodatages, types d'evenements, resultats de validation (valide/invalide) et codes d'erreur.
- **Correlation** : chaque requete ou annonce traitee recoit un identifiant de session unique (ex. UUID) pour permettre la correlation des evenements lies a un meme echange.
- **Retention** : les logs sont conserves selon la politique de retention locale du COG, avec une duree minimale recommandee de 30 jours pour les evenements de securite (echecs de verification, anomalies).
- **Protection des logs** : les fichiers de logs doivent etre proteges en ecriture (droits restreints, append-only si possible) et accessibles uniquement par le processus du Tracker et les administrateurs autorises.

### 4.3 Protection contre les abus passifs

- **Rate limiting sur les requetes** : le Tracker **doit** limiter le nombre de requetes de decouverte par adresse source ou par `cog_id` demandeur sur une fenetre de temps (ex. 100 requetes / minute) pour prevenir le scraping et le deni de service.
- **Profondeur de reponse limitee** : les reponses de decouverte ne retournent qu'un **sous-ensemble filtre** de la liste de COGs, selon les criteres des Cores (pas de dump complet de la liste).
- **Detection d'anomalies** : le Tracker **devrait** detecter les patterns anormaux (rafales de requetes, scans sequentiels de `cog_id`) et emettre un signalement (Q5) vers les Cores (WorrySentinel) pour decision.
- **Protection contre l'empoisonnement de liste** : les mises a jour de statuts recues d'autres Trackers ou COGs sont soumises a verification (source autorisee, signature, coherence) avant integration dans la liste locale. Une mise a jour non verifiee est journalisee mais **pas** integree.

### 4.4 Verification de version (systeme passif)

- **Validation de l'empreinte de version** : toute annonce de presence MWS **doit** contenir une empreinte de version (au minimum `core_version` et `protocol_version`). Si l'empreinte est absente ou malformee, l'annonce est rejetee a la validation syntaxique (Q1).
- **Compatibilite protocol_version** : le Tracker verifie que le `protocol_version` du COG annonceur est supporte. Si la version n'est pas supportee, l'annonce est journalisee avec le motif et un signalement peut etre emis.
- **Verification core_version** : le Tracker verifie la `core_version` du COG annonceur. Les Cores etant **immuables**, la `core_version` definit le socle de compatibilite. La verification passive enregistre le resultat (compatible / obsolete / inconnue) dans la liste locale de COGs.
- **Filtrage par compatibilite dans les reponses de decouverte** : lorsqu'un COG demande la liste de COGs (decouverte), le Tracker peut filtrer les resultats pour ne retourner que les COGs avec une `core_version.MAJOR` compatible avec celle du demandeur (invariant I7). Cela evite qu'un COG decouvre des pairs avec lesquels il ne pourra pas interagir.
- **Signalement de version obsolete** : si un COG se presente avec une `core_version` en dessous d'un seuil defini par la politique locale (ex. `core_version.MAJOR < N`), le Tracker emet un signalement (Q5) vers WorrySentinel. Cela ne bloque pas l'annonce (systeme passif), mais informe pour decision eventuelle par les systemes actifs.
- **Mise a jour de version** : lorsqu'un COG deja present dans la liste reenvoie une annonce avec une empreinte de version differente (ex. apres un patch de Service), le Tracker met a jour l'entree correspondante dans la liste locale avec la nouvelle version, source et horodatage. La `core_version` ne **devrait pas** changer pour un meme `cog_id` sans re-enregistrement complet.

### 4.5 Verification du Registre de Services et suivi des mises a jour (systeme passif)

Le Tracker passif effectue une **verification du Registre de Services** pour chaque COG qui s'annonce, en complement de la verification de version (section 4.4) :

- **Consultation du Registre** : pour chaque `service_id` dans le `service_manifest` du COG annonceur, le Tracker interroge le Registre de Services du Relay Origin (directement ou via un cache local). Si le Registre est temporairement inaccessible, le Tracker peut accepter l'annonce avec un statut `registry_pending` et reevaluer lors de la prochaine synchronisation du cache.

- **Detection de service non repertorie** : si un `service_id` n'est pas present dans le Registre (statut NOT_FOUND) ou est suspendu (statut SUSPENDED), le Tracker :
  1. Journalise l'evenement (cog_id, service_id, statut registre, horodatage) -- postcondition Q3.
  2. Marque le COG dans la liste locale avec le flag `unregistered_service` (invariant I8).
  3. Emet un signalement (Q5) vers les canaux configures (WorrySentinel, autres Trackers).
  4. N'exclut pas le COG des annonces (systeme passif) mais alimente les systemes actifs qui peuvent decider de l'isolation.

- **Verification des mises a jour disponibles** : le Tracker compare les versions des services du COG avec les versions courantes du Registre. Pour chaque service dont la version installee est inferieure a la version courante :
  1. Le resultat est enregistre dans la liste locale (service_id, version COG, version Registre, severite de la mise a jour).
  2. Si la mise a jour est `critical` (securite), un signalement est emis (Q5).
  3. L'information est disponible pour les systemes actifs et les notifications utilisateur.

- **Notification utilisateur** : le Tracker alimente les Cores du COG concerne (via WorrySentinel ou canal configure) pour que l'utilisateur soit notifie des services non repertories et des mises a jour disponibles. Le message inclut : le `service_id` concerne, la raison, les actions recommandees (soumettre au Registre, mettre a jour, desinstaller).

- **Redirection vers les sources de mise a jour** : le Tracker peut fournir au COG les URLs de telechargement ou les sources officielles de l'editeur (pour les services tiers) telles que communiquees par le Registre du Relay Origin. Le Tracker ne distribue pas les binaires ; il redirige.

---

## 5. Synthese des responsabilites passives

| Mecanisme | Preconditions | Postconditions | Invariant |
|-----------|---------------|----------------|----------|
| Validation syntaxique | P1, P3 | Q1 | I1, I2, I5 |
| Verification signature | P1, P2, P3 | Q2, Q3 | I1, I2 |
| Filtrage par statut (lecture) | P1, P4 | Q4 | I1, I3, I4, I5 |
| Journalisation | P1 | Q3 | I2, I6 |
| Mise a jour / partage listes | P4 | Q4 | I3, I4 |
| Signalement | P1, P4, P5 | Q5 | I1, I2 |
| Verification de version | P1, P3, P6 | Q6, Q3 | I1, I7 |
| Verification Registre de Services | P1, P7 | Q7, Q3, Q5 | I1, I8, I9 |
| Suivi des mises a jour | P1, P7, P8 | Q8, Q3 | I1, I9 |

---

## References

- [Miyukini Conceptual References - Miyukini Webway System](..//..//..//..//miyukini-webway-system//reference//_index.md) -- section 5.1 (systemes passifs)
- [Miyukini Webway System - Normes et Standards](..//..//..//..//miyukini-webway-system//reference//_index.md) -- schemas, norme de declaration securisee
- [Miyukini Conceptual References - Miyukini Webway Relay](..//..//..//..//miyukini-webway-system//reference//_index.md) -- section 5 : Relay Origin et Registre de Services
- [MiyuWebwayTracker - Active Systems Contract](MiyuWebwayTracker%20-%20Active%20Systems%20Contract.md) -- systemes actifs (blocage, degradation, isolation)
- [MiyuWebwayTracker - Tool Governance Compliance Contract](../governance/MiyuWebwayTracker%20-%20Tool%20Governance%20Compliance%20Contract.md)

---

*Document cree pour le contrat des systemes passifs du MiyuWebwayTracker. Classification : Contrat de securite.*

