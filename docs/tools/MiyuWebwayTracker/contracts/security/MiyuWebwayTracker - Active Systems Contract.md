# MiyuWebwayTracker -- Contrat des systemes actifs

## Contexte

Les **systemes actifs** du COG Tracker MWS (MiyuWebwayTracker) sont des mecanismes qui **agissent sur les flux** du Webway (annonces, requetes, connexions) pour **filtrer, degrader ou bloquer** en fonction des listes de statuts et des politiques. Ils protegent le maillage en appliquant des decisions de rejet ou de degradation conformes au devoir de protection des Trackers. Ce contrat formalise les **declencheurs, actions et limites** des systemes actifs.

> **Role du Tracker :** Le Tracker est le **douanier du reseau**. Il controle l'identite et le **Permis de circulation** des COGs (contrôle tracker), gere les **whitelists, blacklists et quarantaines**, dirige les **pools par version des Cores**, et peut **fermer des connexions** pour circonscrire les attaques sur annonce des relays. La verification lourde de conformite (Passeport, cle Cores, blocs de code Services) reste du ressort des **relays** qui delivrent les Permis de circulation (accord relay) (voir [Miyukini Webway Relay](../../../reference/Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20Relay.md) section 2).

**Reference conceptuelle :** [Miyukini Conceptual References - Miyukini Webway System](../../../reference/Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System.md) (section 5.2).

## Portee / Scope

- **Blocage** : refus d'enregistrer ou de relayer les annonces, refus de traiter les requetes ou connexions pour des COGs/adresses cibles.
- **Signalement** : emission d'alertes ou de mises a jour de statuts vers d'autres Trackers ou COGs (reputation, liste de statuts).
- **Degradation** : limitation du debit, throttling, ou reponses degradees pour des COGs en statut Under review ou selon politique.
- **Alerte** : notification vers les Cores (ex. WorrySentinel, Border Guard) ou vers le maillage pour declencher des mesures.
- **Blacklisting** : gestion de listes noires locales et partagees (IP, cog_id) avec expiration et audit.

Ce contrat **ne definit pas** les systemes passifs (validation, journalisation sans action sur le flux) ; voir [MiyuWebwayTracker - Passive Systems Contract](MiyuWebwayTracker%20-%20Passive%20Systems%20Contract.md).

---

## 1. Declencheurs (systemes actifs)

Les actions actives sont declenchees lorsque une ou plusieurs conditions suivantes sont satisfaites. Les **Cores** (Border Guard, WorrySentinel, StrongFather) fournissent la **politique** ; le Tracker **applique** selon les contrats.

| ID | Declencheur | Description |
|----|-------------|-------------|
| T1 | **Statut Rejected** : le `cog_id` (ou l'adresse associee) figure dans la liste locale avec statut **Rejected**. | Blocage annonces/connexions de ce COG |
| T2 | **Statut Distrusted** : le `cog_id` figure avec statut **Distrusted** et la politique locale impose un refus ou une degradation pour ce statut. | Blocage ou degradation selon politique |
| T3 | **Statut Under review** : le `cog_id` figure avec statut **Under review** et la politique impose un throttling ou des reponses degradees. | Degradation (throttling, delai, reponse limitee) |
| T4 | **Echec de verification** : la declaration (annonce) a echoue a la verification de signature ou a la validation syntaxique et la politique impose un rejet ou un signalement. | Blocage de l'annonce, signalement |
| T5 | **Alerte ou decision Core** : un Core (WorrySentinel, Border Guard) a emis une decision explicite de bloquer, degrader ou signaler un `cog_id` ou une adresse. | Application de la decision |
| T6 | **Blacklist locale ou partagee** : l'adresse IP ou un identifiant est inscrit sur une blacklist (locale ou recue selon protocole MWS). | Blocage des connexions/requetes concernees |
| T7 | **Rate limit depasse** : une adresse source ou un `cog_id` a depasse les seuils de requetes sur une fenetre de temps (detection par les systemes passifs). | Throttling ou blocage temporaire |
| T8 | **Pattern d'attaque detecte** : les systemes passifs ont detecte un pattern anormal (scan sequentiel, tentatives de brute-force, injection) et emis un signalement. | Blocage temporaire, alerte, escalade |
| T9 | **Incompatibilite de version** : le COG se presente avec une `core_version.MAJOR` incompatible avec la politique locale (version obsolete, non supportee, ou inferieure au seuil minimal). Les Cores etant immuables, une incompatibilite de version Cores empeche toute interaction fiable. | Degradation, refus d'annonce, signalement |
| T10 | **Version de protocole non supportee** : le COG utilise un `protocol_version` que le Tracker ne supporte pas. | Refus d'annonce, signalement |
| T11 | **Service non repertorie detecte** : le `service_manifest` du COG contient un ou plusieurs `service_id` absents du Registre de Services du Relay Origin (statut NOT_FOUND ou SUSPENDED). Le service a ete installe hors ligne sans verification prealable ou a ete retire du Registre. | Isolation reseau, notification, surveillance |
| T12 | **Mise a jour critique non appliquee** : le COG utilise un service dont la version est inferieure a la version minimale du Registre (`min_version`) ou pour lequel une mise a jour `critical` (securite) est disponible depuis un delai depasse (configurable, ex. 72 h). | Degradation, signalement, isolation progressive |
| T13 | **Permis expire ou invalide** : le COG se presente au Tracker avec un Permis de circulation expire, revoque, ou emis par un relay non reconnu (ou se connecte a un tracker non officiel). | Refus de connexion, redirection vers relay pour re-verification |
| T14 | **Connexion inter-pool** : un COG tente de se connecter a un COG d'un pool de version differente (`core_version.MAJOR` differente). | Refus de connexion |
| T15 | **Congestion sur un COG** : un COG accumule un nombre anormalement eleve de connexions (seuil configurable). Particulierement surveille pour les COGs avec Passeport special. | Surveillance renforcee, throttling, alerte |
| T16 | **Alerte reseau des relays** : les relays annoncent une alerte (multiples rejets, attaque detectee). | Confinement, fermeture de connexions, controle renforce |

---

## 2. Actions (systemes actifs)

Pour chaque declencheur, les actions possibles sont limitees aux suivantes. Toute action **doit** etre conforme a la politique fournie par les Cores (le Tracker n'invente pas la politique).

| ID | Action | Description | Declencheurs typiques |
|----|--------|-------------|------------------------|
| A1 | **Refus d'enregistrement d'annonce** | Ne pas enregistrer ni relayer l'annonce de presence du COG ; retourner une erreur ou ignorer silencieusement selon protocole. | T1, T2, T4, T5, T6, T8 |
| A2 | **Refus de requete de decouverte** | Ne pas repondre ou repondre par un refus aux requetes de decouverte provenant du COG ou de l'adresse ciblee. | T1, T2, T5, T6, T8 |
| A3 | **Blocage de connexion** | Fermer ou refuser la connexion Webway (port 21000 ou canal MWS) pour le COG ou l'adresse. | T1, T5, T6, T8 |
| A4 | **Throttling / degradation** | Limiter le debit ou le nombre de reponses, ou renvoyer des reponses degradees (ex. liste partielle, delai ajoute). | T2, T3, T7 |
| A5 | **Signalement** | Emettre un signal ou une mise a jour de statut vers d'autres Trackers ou COGs (liste de statuts, alerte). | T4, T5, T8 |
| A6 | **Alerte interne** | Notifier les Cores (WorrySentinel, Border Guard) pour decision ou tracabilite. | T4, T5, T6, T7, T8 |
| A7 | **Blacklist temporaire** | Inscrire l'adresse IP ou le `cog_id` sur la blacklist locale avec une duree d'expiration (ex. 15 min, 1 h, 24 h selon severite). | T6, T7, T8 |
| A8 | **Escalade** | En cas de persistence ou de gravite croissante (ex. attaque soutenue), escalader la severite : prolonger le ban, elargir la portee du blocage, signaler a d'autres Trackers. | T7, T8 |
| A9 | **Refus pour incompatibilite de version** | Rejeter l'annonce de presence du COG avec un motif explicite (core_version incompatible ou protocol_version non supportee). Le COG recoit un message d'erreur informatif indiquant les versions minimales acceptees. | T9, T10 |
| A10 | **Exclusion des reponses de decouverte** | Ne pas inclure un COG avec core_version incompatible dans les reponses de decouverte destinees a un demandeur d'une autre core_version.MAJOR. Cela protege les demandeurs de decouvrir des pairs non interoperables. | T9 |
| A11 | **Isolation reseau (service non repertorie)** | Exclure le COG du maillage MWS actif : pas d'annonces de presence relayees, pas d'inclusion dans les reponses de decouverte, pas de routing de donnees vers d'autres COGs. Le tunnel relay est maintenu en mode surveillance (heartbeats, notifications, consultation du Registre). Le COG est notifie de la raison et des actions correctives. | T11 |
| A12 | **Notification utilisateur** | Emettre une notification explicite vers le COG (via les Cores, WorrySentinel) informant l'utilisateur du service non repertorie ou de la mise a jour critique non appliquee, avec le `service_id`, la raison, et les actions recommandees (soumettre au Registre, mettre a jour, desinstaller). | T11, T12 |
| A13 | **Journalisation reseau du service inconnu** | Journaliser l'evenement au niveau du maillage (Relay Origin + Trackers connectes) : cog_id, service_id non repertorie, horodatage, adresse source. Cela enrichit la surveillance globale et permet aux autres Trackers de connaitre l'incident. | T11 |
| A14 | **Refus de connexion (permis invalide)** | Refuser la connexion au maillage et rediriger le COG vers un relay pour re-verification et obtention d'un nouveau Permis de circulation. | T13 |
| A15 | **Blocage inter-pool** | Refuser systematiquement toute tentative de connexion entre COGs de pools de version differentes. Les pools sont strictement isoles. | T14 |
| A16 | **Surveillance renforcee (congestion)** | Renforcer le monitoring d'un COG a forte concentration de connexions. Peut inclure : throttling des nouvelles connexions, journalisation detaillee, alerte WorrySentinel. | T15 |
| A17 | **Confinement reseau** | Sur alerte des relays, fermer **tout ou partie** des connexions inter-COG pour circonscrire l'attaque. Les COGs doivent se re-verifier aupres des relays (lecture seule) avant de pouvoir reconnecter. | T16 |

---

## 3. Limites (systemes actifs)

Les contraintes suivantes **doivent** etre respectees par les systemes actifs.

| ID | Limite | Description |
|----|--------|-------------|
| L1 | **Politique par les Cores** : les decisions de **qui** bloquer, degrader ou signaler (statuts, blacklist) sont definies par les **Cores** (Border Guard, WorrySentinel, StrongFather). Le Tracker **applique** ces decisions ; il ne definit pas la politique metier. | Separation des responsabilites |
| L2 | **Pas de gouvernance Permis/Passeport** : les systemes actifs du Tracker **ne delivrent ni ne refusent** de Permis de circulation ni de Passeport. Ils protegent le **maillage MWS** (presence, decouverte) ; la gouvernance des visites reste du ressort du COG Hebergeur. | Portee MWS |
| L3 | **Pas de donnees metier** : les actions (blocage, signalement, alerte) ne doivent **pas exposer** de donnees metier ni de secrets de gouvernance. Les signaux echanges portent sur identifiants COG, statuts, adresses de connexion Webway. | Confidentialite |
| L4 | **Proportionnalite** : les actions actives doivent etre **proportionnees** aux declencheurs. Regles de proportionnalite : Under review -> degradation/throttling (pas de blocage total sauf politique explicite). Distrusted -> degradation ou blocage selon politique. Rejected -> blocage complet autorise. Rate limit -> throttling d'abord, blocage temporaire si persistant. | Proportionnalite |
| L5 | **Tracabilite** : toute action active (refus, blocage, signalement) doit etre **journalisee** (qui, quand, declencheur, action, duree) pour audit et conformite, sans inclure de donnees metier. | Audit |
| L6 | **Reversibilite** : toute action de blocage ou de blacklist **doit** etre reversible. Les blacklists temporaires expirent automatiquement ; les blocages permanents necessitent une decision explicite des Cores et doivent pouvoir etre leves manuellement. | Recuperation |
| L7 | **Pas de blocage silencieux indefini** : un blocage ou une degradation appliquee par les systemes actifs **doit** avoir une duree ou un critere de fin (expiration, decision de levee). Le Tracker ne doit pas maintenir un blocage sans reexamen periodique. | Equite |

---

## 4. Exigences de securite renforcees (systemes actifs)

### 4.1 Protection contre le deni de service (DDoS MWS)

- **Seuils de rate limiting** : les seuils (requetes/minute par source, connexions/seconde) sont configurables et documentes. Valeurs recommandees : 100 requetes de decouverte / minute / source, 10 connexions / seconde / source, 50 annonces / minute / source.
- **Reaction progressive** : 1) Throttling (reduction du debit de reponse), 2) Blocage temporaire (blacklist 15 min), 3) Escalade (blacklist 1 h, signalement aux Trackers voisins), 4) Alerte Core (WorrySentinel) pour decision manuelle ou automatisee.
- **Protection du port 21000** : le Tracker **doit** limiter le nombre de connexions TCP simultanees sur le port MWS (ex. max 1000 connexions simultanees, configurable) et fermer les connexions inactives apres un timeout (ex. 30 s sans message).

### 4.2 Gestion des blacklists

- **Blacklist locale** : maintenue en memoire ou en stockage persistant par le Tracker. Chaque entree contient : cle (IP ou `cog_id`), raison (declencheur), date d'ajout, date d'expiration, severite.
- **Blacklist partagee** : les Trackers peuvent echanger des entrees de blacklist via le protocole MWS (signalement A5). L'integration d'une blacklist externe est soumise a la souverainete du Tracker (il peut accepter ou ignorer, invariant I4 du contrat passif).
- **Expiration automatique** : les entrees de blacklist temporaire expirent automatiquement. La duree par defaut est configurable (recommande : 15 min pour rate limit, 1 h pour echec de verification repete, 24 h pour pattern d'attaque).
- **Audit de blacklist** : chaque ajout ou suppression dans la blacklist est journalise (L5).

### 4.3 Verification d'identite avant action

- **Pas de blocage sur identite non verifiee** : un blocage permanent (statut Rejected) ne peut etre applique que si le `cog_id` a ete **verifie** (signature, source connue). Les blocages temporaires (rate limit, pattern anormal) peuvent s'appliquer sur adresse IP sans verification d'identite.
- **Risque d'usurpation** : le Tracker **doit** considerer qu'un `cog_id` dans un message peut etre usurpe si la signature n'est pas verifiee. Les actions sur un `cog_id` non verifie sont limitees au signalement (A5, A6) et au throttling (A4), pas au blocage permanent.

### 4.4 Gestion des versions et compatibilite (systemes actifs)

- **Rejet pour core_version incompatible** : si la politique locale definit une `min_core_version.MAJOR` (seuil minimal), toute annonce de presence avec une `core_version.MAJOR` inferieure est rejetee (A9) et journalisee. Le COG recoit un message indiquant la version minimale requise.
- **Degradation pour version obsolete mais compatible** : si la `core_version.MAJOR` est compatible mais la `core_version.MINOR` est inferieure au seuil recommande, le Tracker peut appliquer un throttling (A4) ou un signalement (A5) sans blocage total, conformement au principe de proportionnalite (L4).
- **Exclusion des reponses de decouverte** : les COGs avec une `core_version.MAJOR` differente de celle du demandeur sont automatiquement exclus des reponses de decouverte (A10). Cela est une action proactive qui protege l'ecosysteme en empechant les connexions entre COGs non interoperables.
- **Cores immuables, Services patchables** : les systemes actifs ne bloquent **jamais** un COG pour une difference de version de Service (MINOR, PATCH). Seule l'incompatibilite de `core_version.MAJOR` ou de `protocol_version` peut declencher un blocage ou un refus. Les patchs de Service sont transparents et n'affectent pas la compatibilite tant que la `core_version` est identique.
- **Notification de mise a jour** : lorsqu'un COG est rejete ou degrade pour cause de version obsolete, la reponse du Tracker **doit** inclure les versions minimales acceptees (`min_core_version`, `min_protocol_version`) pour que le COG puisse se mettre a jour. Cette information est un conseil, pas une obligation (souverainete du COG).

### 4.5 Gestion active des services non repertories et des mises a jour

#### 4.5.1 Isolation pour service non repertorie (T11 -> A11, A12, A13)

Lorsqu'un service non repertorie est detecte dans le manifest d'un COG (declencheur T11), le Tracker applique le protocole d'isolation suivant :

1. **Isolation immediate** (A11) : le COG est exclu du maillage MWS actif. Ses annonces de presence ne sont plus relayees, il n'apparait plus dans les reponses de decouverte. Le tunnel relay reste actif en mode surveillance (heartbeats maintenus, le COG peut consulter le Registre et recevoir des notifications).

2. **Notification utilisateur** (A12) : le COG recoit une notification explicite (via WorrySentinel ou canal configure) :
   - **service_id** du service non repertorie.
   - **Raison** : absent du Registre de Services du Relay Origin.
   - **Actions recommandees** :
     - Soumettre le service au Registre (processus d'enregistrement tiers).
     - Desinstaller le service non repertorie.
     - Se re-enregistrer avec un manifest conforme.

3. **Journalisation reseau** (A13) : l'evenement est communique au Relay Origin et aux Trackers du maillage pour enrichir la surveillance globale. Donnees journalisees : `cog_id`, `service_id` non repertorie, horodatage, adresse source (sans donnees metier ni secrets).

4. **Levee d'isolation** : l'isolation est levee **automatiquement** lorsque le COG se re-enregistre (nouveau REGISTER ou nouvelle annonce de presence) avec un `service_manifest` entierement conforme au Registre. Le Tracker verifie chaque service avant de remettre le COG en statut ACTIVE.

5. **Regle fondamentale** :

> **Un service ne peut pas etre installe dans un COG connecte au Webway sans etre present dans le Registre de Services du Relay Origin. Si un service non repertorie est installe hors ligne, le Webway isole le COG du reseau en attendant la mise en conformite.**

#### 4.5.2 Degradation progressive pour mise a jour critique non appliquee (T12)

Lorsqu'un COG utilise un service avec une mise a jour critique (securite) disponible depuis plus du delai configure :

1. **Signalement** (immediat) : WorrySentinel est notifie (A6), l'evenement est journalise.
2. **Degradation** (apres delai 1, ex. 24 h) : throttling du COG (A4), reponses de decouverte limitees.
3. **Notification repetee** (A12) : rappels periodiques a l'utilisateur du COG.
4. **Isolation** (apres delai 2, ex. 72 h, configurable) : meme protocole que T11 -> A11. Le COG est isole jusqu'a application de la mise a jour critique.
5. **Levee** : le COG se re-enregistre avec la version corrigee ; le Tracker verifie et leve l'isolation.

#### 4.5.3 Redirection vers les sources de mise a jour

- Le Tracker peut inclure dans ses notifications (A12) et dans les reponses de decouverte les **URLs de source de mise a jour** telles que fournies par le Registre du Relay Origin :
  - Services officiels Miyukini : URL de telechargement Miyukini.
  - Services tiers repertories : URL de la source officielle de l'editeur.
- Le Tracker ne distribue pas de binaires ; il redirige vers les sources officielles.

### 4.6 Isolation des canaux

- **Separation MWS / relay** : les actions actives du Tracker s'appliquent **uniquement** au canal MWS (port 21000, protocole de decouverte). Elles ne s'etendent pas au canal du relay (port 7000) ni aux connexions inter-COG gouvernees par Permis de circulation / Passeport / Visa de Connexion. Le Tracker peut signaler un COG suspect au relay (via les Cores), mais il ne bloque pas directement le tunnel relay.
- **Pas d'interference avec la gouvernance** : les systemes actifs ne modifient jamais les Passeports, Permis de circulation, Visa de Connexion, ni les decisions de Bridge. Ils protegent uniquement la couche de presence et decouverte.

---

## 5. Matrice Declencheur -> Action (orientation)

| Declencheur | Actions autorisees (selon politique) |
|-------------|--------------------------------------|
| T1 (Rejected) | A1, A2, A3, A5, A6, A7 |
| T2 (Distrusted) | A1, A2, A4, A5, A6 |
| T3 (Under review) | A4, A5, A6 |
| T4 (Echec verification) | A1, A5, A6, A7 |
| T5 (Decision Core) | A1-A8 selon decision |
| T6 (Blacklist) | A1, A2, A3, A5, A6 |
| T7 (Rate limit depasse) | A4, A6, A7, A8 |
| T8 (Pattern d'attaque) | A1, A2, A3, A5, A6, A7, A8 |
| T9 (Incompatibilite core_version) | A1, A4, A5, A6, A9, A10 |
| T10 (protocol_version non supportee) | A1, A6, A9 |
| T11 (Service non repertorie) | A5, A6, A11, A12, A13 |
| T12 (Mise a jour critique non appliquee) | A4, A5, A6, A11, A12 |
| T13 (Permis expire ou invalide) | A6, A14 |
| T14 (Connexion inter-pool) | A15 |
| T15 (Congestion sur un COG) | A4, A6, A16 |
| T16 (Alerte reseau des relays) | A3, A5, A6, A7, A17 |

La politique locale et les Cores peuvent restreindre ou etendre les actions autorisees pour un statut donne.

---

## References

- [Miyukini Conceptual References - Miyukini Webway System](../../../reference/Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System.md) -- section 5.2 (systemes actifs)
- [Miyukini Webway System - Normes et Standards](../../../reference/Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System%20Normes%20et%20Standards.md) -- matrice des statuts, regles d'echange
- [Miyukini Conceptual References - Miyukini Webway Relay](../../../reference/Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20Relay.md) -- section 5 : Relay Origin et Registre de Services
- [MiyuWebwayTracker - Passive Systems Contract](MiyuWebwayTracker%20-%20Passive%20Systems%20Contract.md) -- systemes passifs (validation, journalisation, verification Registre)
- [MiyuWebwayTracker - Tool Governance Compliance Contract](../governance/MiyuWebwayTracker%20-%20Tool%20Governance%20Compliance%20Contract.md)

---

*Document cree pour le contrat des systemes actifs du MiyuWebwayTracker. Classification : Contrat de securite.*
