# KindMother â€” Adapter Examples (Conceptual, Non-Normative)

## Statut du document

**POST-FONDATION / NON NORMATIF / INFORMATIF**

Ce document est **informatif, non normatif, et non contractuel**. Il illustre comment utiliser KindMother correctement via des scÃ©narios conceptuels narratifs.

**Objectif pÃ©dagogique :** Ce document vise Ã  aider les dÃ©veloppeurs Ã  comprendre comment utiliser KindMother dans diffÃ©rents contextes d'application, en illustrant les concepts contractuels par des scÃ©narios narratifs.

**Avertissement :** Ce document contient uniquement des exemples narratifs conceptuels. Aucun code, pseudo-code, technologie, protocole, ou format de donnÃ©es n'est inclus. Ces exemples sont purement illustratifs et ne prescrivent aucune implÃ©mentation.

**Relation avec les contrats FONDATION :** Ce document fait rÃ©fÃ©rence aux contrats FONDATION existants mais ne les Ã©tend pas, ne les modifie pas, et ne crÃ©e aucune nouvelle obligation contractuelle.

---

## 1. Introduction

### 1.1. Objectif

Ce document illustre comment utiliser KindMother correctement via des scÃ©narios conceptuels narratifs. Il montre comment les concepts contractuels se traduisent en situations d'utilisation rÃ©elles, sans exposer d'implÃ©mentation.

### 1.2. Nature narrative

Ce document est **purement narratif et conceptuel**. Il dÃ©crit des scÃ©narios d'utilisation sous forme d'histoires conceptuelles, sans entrer dans les dÃ©tails techniques ou d'implÃ©mentation.

### 1.3. Sources contractuelles

Ce document se base sur les contrats FONDATION suivants :

- **Instance Model Contract** : Relations MÃ¨re/Fille, rÃ´les systÃ©miques
- **Authority Graph & Cross-Domain Contract** : Multi-domaines, Authority Domains
- **Identity & Cross-Domain Trust Contract** : SÃ©paration identitÃ©/autorisation
- **CoreDataAPI Contract** : OpÃ©rations autorisÃ©es, rÃ¨gles d'appel
- **Sync & Conflict Resolution Contract** : Synchronisation, rÃ©solution de conflits
- **Write Intent Lifecycle Contract** : Cycle de vie des intentions
- **[Miyukini Conceptual References â€” Lois Autonomie SystÃ¨me](..//..//..//miyukini-webway-system//reference//_index.md)** : Les exemples illustrent notamment **LOI-2** (isolement comme Ã©tat normal), **LOI-3** (Ã©tat local souverain), et **LOI-6** (autonomie n'empÃªche pas la fÃ©dÃ©ration)

---

## 2. ScÃ©nario 1 : Application offline-first

### 2.1. Contexte

**Situation :** Une application mobile qui doit fonctionner de maniÃ¨re autonome, mÃªme en l'absence de connexion rÃ©seau avec le serveur.

**Architecture :**
- **Instance MÃ¨re :** Serveur central qui constitue la source d'autoritÃ© de rÃ©fÃ©rence
- **Instance Fille :** Application mobile qui maintient une copie locale des donnÃ©es

**Relation :** L'application mobile (Instance Fille) reconnaÃ®t l'autoritÃ© de l'Instance MÃ¨re (serveur) et synchronise pÃ©riodiquement avec elle.

### 2.2. Intentions Ã©mises

**CrÃ©ation d'entitÃ©s en offline :**

L'utilisateur crÃ©e de nouvelles entitÃ©s (par exemple, des notes, des tÃ¢ches, des contacts) pendant que l'application est hors ligne. Ces crÃ©ations sont formulÃ©es comme des Write Intents et soumises Ã  l'Instance Fille locale.

**Modification d'entitÃ©s en offline :**

L'utilisateur modifie des entitÃ©s existantes (par exemple, met Ã  jour le contenu d'une note, change le statut d'une tÃ¢che) pendant que l'application est hors ligne. Ces modifications sont formulÃ©es comme des Write Intents et soumises Ã  l'Instance Fille locale.

**Suppression d'entitÃ©s en offline :**

L'utilisateur supprime des entitÃ©s (par exemple, supprime une note, archive une tÃ¢che) pendant que l'application est hors ligne. Ces suppressions sont formulÃ©es comme des Write Intents et soumises Ã  l'Instance Fille locale.

### 2.3. Validations attendues

**Validation locale :**

L'Instance Fille valide localement chaque Write Intent selon les rÃ¨gles de cohÃ©rence locales. Les validations locales vÃ©rifient que :
- Le contexte est complet (utilisateur, permissions, instance)
- Les permissions locales sont suffisantes
- La cohÃ©rence locale est prÃ©servÃ©e
- Aucune contrainte locale n'est violÃ©e

Si la validation locale rÃ©ussit, l'intention est appliquÃ©e localement, permettant Ã  l'utilisateur de continuer Ã  utiliser l'application mÃªme hors ligne.

**Validation MÃ¨re lors de sync :**

Lorsque la connexion rÃ©seau est rÃ©tablie, l'Instance Fille synchronise avec l'Instance MÃ¨re. Les Write Intents appliquÃ©es localement sont soumises Ã  l'Instance MÃ¨re pour validation dÃ©finitive.

L'Instance MÃ¨re valide chaque intention selon les rÃ¨gles de cohÃ©rence de rÃ©fÃ©rence :
- Le contexte est valide et complet
- Les permissions sont suffisantes selon les rÃ¨gles de rÃ©fÃ©rence
- La cohÃ©rence de rÃ©fÃ©rence est prÃ©servÃ©e
- Aucune contrainte de rÃ©fÃ©rence n'est violÃ©e
- Aucun conflit avec d'autres modifications n'est dÃ©tectÃ©

### 2.4. Rejets possibles

**Conflit autoritaire :**

L'Instance MÃ¨re peut rejeter une intention locale si elle entre en conflit avec une modification effectuÃ©e sur l'Instance MÃ¨re pendant que l'application Ã©tait hors ligne. Par exemple, si l'utilisateur a modifiÃ© une note localement, mais que la mÃªme note a Ã©tÃ© supprimÃ©e sur l'Instance MÃ¨re, l'intention de modification est rejetÃ©e.

**Violation de cohÃ©rence :**

L'Instance MÃ¨re peut rejeter une intention locale si elle viole une contrainte de cohÃ©rence de rÃ©fÃ©rence. Par exemple, si l'utilisateur a crÃ©Ã© localement une entitÃ© qui viole une contrainte mÃ©tier dÃ©finie sur l'Instance MÃ¨re, l'intention de crÃ©ation est rejetÃ©e.

**Permissions insuffisantes :**

L'Instance MÃ¨re peut rejeter une intention locale si les permissions de l'utilisateur ne sont pas suffisantes selon les rÃ¨gles de rÃ©fÃ©rence. Par exemple, si l'utilisateur a crÃ©Ã© localement une entitÃ© qui nÃ©cessite des permissions qu'il n'a pas selon l'Instance MÃ¨re, l'intention est rejetÃ©e.

### 2.5. Garanties fournies

**DurabilitÃ© locale :**

Les Write Intents appliquÃ©es localement sont persistÃ©es localement, garantissant que les modifications de l'utilisateur ne sont pas perdues mÃªme si l'application est fermÃ©e ou si l'appareil est redÃ©marrÃ©.

**Synchronisation ultÃ©rieure :**

Les Write Intents appliquÃ©es localement sont marquÃ©es pour synchronisation ultÃ©rieure. Lorsque la connexion rÃ©seau est rÃ©tablie, ces intentions sont automatiquement soumises Ã  l'Instance MÃ¨re pour validation dÃ©finitive.

**CohÃ©rence locale :**

L'Instance Fille maintient la cohÃ©rence locale, garantissant que les donnÃ©es locales sont cohÃ©rentes mÃªme si elles ne sont pas encore synchronisÃ©es avec l'Instance MÃ¨re.

**ConformitÃ© LOI-2 et LOI-3 :** Ce scÃ©nario illustre **LOI-2** (le systÃ¨me accepte l'isolement comme Ã©tat normal) : l'application fonctionne localement mÃªme sans connexion rÃ©seau, l'isolement n'est pas traitÃ© comme une erreur mais comme un Ã©tat valide. Il illustre Ã©galement **LOI-3** (l'Ã©tat local est souverain) : l'Instance Fille dÃ©tient l'autoritÃ© locale sur son Ã©tat, et la rÃ©conciliation avec l'Instance MÃ¨re est explicite et traÃ§able.

**Alignement contractuel :**

- Respecte l'invariant INST-F-2 (copie locale synchronisÃ©e)
- Respecte l'invariant INST-F-4 (autonomie limitÃ©e)
- Respecte l'invariant INST-F-5 (soumission des opÃ©rations Ã  la validation)
- AlignÃ© avec le Write Intent Lifecycle Contract (intentions locales vs intentions dÃ©finitives)

---

## 3. ScÃ©nario 2 : CMS local avec synchronisation

### 3.1. Contexte

**Situation :** Un systÃ¨me de gestion de contenu (CMS) avec Ã©dition locale et publication sur un serveur.

**Architecture :**
- **Instance MÃ¨re :** Serveur de publication qui constitue la source d'autoritÃ© de rÃ©fÃ©rence pour le contenu publiÃ©
- **Instance Fille :** Ã‰diteur local qui permet l'Ã©dition de contenu avant publication

**Relation :** L'Ã©diteur local (Instance Fille) reconnaÃ®t l'autoritÃ© de l'Instance MÃ¨re (serveur de publication) et synchronise pÃ©riodiquement pour publier le contenu Ã©ditÃ©.

### 3.2. Intentions Ã©mises

**CrÃ©ation de contenu :**

L'Ã©diteur crÃ©e de nouveaux contenus (par exemple, des articles, des pages, des mÃ©dias) localement. Ces crÃ©ations sont formulÃ©es comme des Write Intents et soumises Ã  l'Instance Fille locale.

**Modification de contenu :**

L'Ã©diteur modifie des contenus existants (par exemple, met Ã  jour le texte d'un article, change les mÃ©tadonnÃ©es d'une page) localement. Ces modifications sont formulÃ©es comme des Write Intents et soumises Ã  l'Instance Fille locale.

**Publication de contenu :**

L'Ã©diteur dÃ©clenche la publication de contenu, ce qui synchronise les Write Intents locales avec l'Instance MÃ¨re pour validation et publication.

### 3.3. Validations attendues

**Validation locale :**

L'Instance Fille valide localement chaque Write Intent selon les rÃ¨gles d'Ã©dition locales. Les validations locales vÃ©rifient que :
- Le contexte est complet
- Les permissions d'Ã©dition sont suffisantes
- La cohÃ©rence Ã©ditoriale locale est prÃ©servÃ©e
- Aucune contrainte d'Ã©dition locale n'est violÃ©e

**Validation MÃ¨re pour publication :**

Lors de la synchronisation pour publication, l'Instance MÃ¨re valide chaque intention selon les rÃ¨gles de publication :
- Le contexte est valide et complet
- Les permissions de publication sont suffisantes
- La cohÃ©rence de publication est prÃ©servÃ©e
- Aucune contrainte de publication n'est violÃ©e
- Le contenu respecte les rÃ¨gles de publication (format, longueur, qualitÃ©, etc.)

### 3.4. Rejets possibles

**Contrainte de publication violÃ©e :**

L'Instance MÃ¨re peut rejeter une intention de publication si elle viole une contrainte de publication. Par exemple, si un article dÃ©passe la longueur maximale autorisÃ©e, ou si le contenu ne respecte pas les rÃ¨gles de qualitÃ©, l'intention de publication est rejetÃ©e.

**Permissions de publication insuffisantes :**

L'Instance MÃ¨re peut rejeter une intention de publication si les permissions de l'Ã©diteur ne sont pas suffisantes pour publier. Par exemple, si l'Ã©diteur a les permissions d'Ã©dition mais pas les permissions de publication, l'intention est rejetÃ©e.

**Conflit avec contenu publiÃ© :**

L'Instance MÃ¨re peut rejeter une intention de publication si elle entre en conflit avec du contenu dÃ©jÃ  publiÃ©. Par exemple, si l'Ã©diteur a modifiÃ© localement un article qui a Ã©tÃ© modifiÃ© diffÃ©remment sur l'Instance MÃ¨re, l'intention de publication est rejetÃ©e.

### 3.5. Garanties fournies

**Ã‰dition locale :**

L'Instance Fille permet l'Ã©dition locale de contenu, garantissant que l'Ã©diteur peut travailler localement sans dÃ©pendre de la connexion rÃ©seau.

**Publication contrÃ´lÃ©e :**

L'Instance MÃ¨re contrÃ´le la publication, garantissant que seuls les contenus validÃ©s et conformes aux rÃ¨gles de publication sont publiÃ©s.

**CohÃ©rence Ã©ditoriale :**

L'Instance Fille maintient la cohÃ©rence Ã©ditoriale locale, garantissant que les contenus Ã©ditÃ©s localement sont cohÃ©rents mÃªme s'ils ne sont pas encore publiÃ©s.

**ConformitÃ© LOI-2 et LOI-3 :** Ce scÃ©nario illustre **LOI-2** (le systÃ¨me accepte l'isolement comme Ã©tat normal) : l'Ã©diteur peut travailler localement sans dÃ©pendre de la connexion rÃ©seau. Il illustre Ã©galement **LOI-3** (l'Ã©tat local est souverain) : les contenus Ã©ditÃ©s localement sont valables localement jusqu'Ã  la rÃ©conciliation explicite avec l'Instance MÃ¨re lors de la publication.

**Alignement contractuel :**

- Respecte l'invariant INST-F-1 (reconnaissance de l'autoritÃ© de l'Instance MÃ¨re)
- Respecte l'invariant INST-F-2 (copie locale synchronisÃ©e)
- Respecte l'invariant INST-F-5 (soumission des opÃ©rations Ã  la validation)
- AlignÃ© avec le Sync & Conflict Resolution Contract (synchronisation, rÃ©solution de conflits)

---

## 4. ScÃ©nario 3 : Jeu multi-domaines

### 4.1. Contexte

**Situation :** Un jeu avec plusieurs domaines d'autoritÃ© distincts (Identity, Game, Commerce) qui doivent communiquer de maniÃ¨re contrÃ´lÃ©e.

**Architecture :**
- **Instance MÃ¨re Identity :** GÃ¨re l'identitÃ© et l'authentification des joueurs
- **Instance MÃ¨re Game :** GÃ¨re les donnÃ©es de jeu (scores, progression, inventaire)
- **Instance MÃ¨re Commerce :** GÃ¨re les transactions commerciales (achats, paiements)
- **Instance Fille (client) :** Application cliente qui interagit avec les trois domaines

**Relation :** L'application cliente (Instance Fille) reconnaÃ®t l'autoritÃ© de chaque Instance MÃ¨re pour son domaine respectif et synchronise avec chacune d'elles.

### 4.2. Intentions Ã©mises

**Actions de jeu :**

Le joueur effectue des actions de jeu (par exemple, complÃ¨te un niveau, gagne des points, obtient un objet). Ces actions sont formulÃ©es comme des Write Intents et soumises Ã  l'Instance MÃ¨re Game via l'Instance Fille.

**Transactions commerciales :**

Le joueur effectue des transactions commerciales (par exemple, achÃ¨te un objet, effectue un paiement). Ces transactions sont formulÃ©es comme des Write Intents et soumises Ã  l'Instance MÃ¨re Commerce via l'Instance Fille.

**Mises Ã  jour d'identitÃ© :**

Le joueur met Ã  jour son profil ou ses prÃ©fÃ©rences. Ces mises Ã  jour sont formulÃ©es comme des Write Intents et soumises Ã  l'Instance MÃ¨re Identity via l'Instance Fille.

### 4.3. Validations attendues

**Validation par domaine :**

Chaque Instance MÃ¨re valide les intentions selon les rÃ¨gles de son domaine :
- **Instance MÃ¨re Identity :** Valide les intentions d'identitÃ© selon les rÃ¨gles d'identitÃ©
- **Instance MÃ¨re Game :** Valide les intentions de jeu selon les rÃ¨gles de jeu
- **Instance MÃ¨re Commerce :** Valide les intentions commerciales selon les rÃ¨gles commerciales

**Intentions CertifiÃ©es inter-domaines :**

Lorsqu'une action nÃ©cessite une communication entre domaines (par exemple, un achat dans le jeu nÃ©cessite Ã  la fois une validation Game et une validation Commerce), des Intentions CertifiÃ©es sont crÃ©Ã©es et validÃ©es par KindMother pour permettre la communication contrÃ´lÃ©e entre domaines.

### 4.4. Rejets possibles

**Violation inter-domaines :**

Une intention peut Ãªtre rejetÃ©e si elle viole les rÃ¨gles de communication inter-domaines. Par exemple, si une intention de jeu tente d'accÃ©der directement aux donnÃ©es du domaine Commerce sans passer par une Intention CertifiÃ©e, elle est rejetÃ©e.

**AutoritÃ© non reconnue :**

Une intention peut Ãªtre rejetÃ©e si l'autoritÃ© du domaine n'est pas reconnue. Par exemple, si l'Instance Fille tente de soumettre une intention Ã  un domaine qui n'est pas reconnu ou qui n'a pas autorisÃ© l'Instance Fille, l'intention est rejetÃ©e.

**Permissions insuffisantes :**

Une intention peut Ãªtre rejetÃ©e si les permissions du joueur ne sont pas suffisantes pour le domaine. Par exemple, si un joueur tente d'effectuer une action de jeu qui nÃ©cessite des permissions qu'il n'a pas, l'intention est rejetÃ©e.

### 4.5. Garanties fournies

**Isolation par domaine :**

Chaque domaine maintient son isolation, garantissant que les donnÃ©es d'un domaine ne sont pas directement accessibles depuis un autre domaine.

**Communication contrÃ´lÃ©e :**

La communication entre domaines est contrÃ´lÃ©e par KindMother via des Intentions CertifiÃ©es, garantissant que seules les communications autorisÃ©es et validÃ©es sont permises.

**CohÃ©rence par domaine :**

Chaque domaine maintient sa propre cohÃ©rence, garantissant que les donnÃ©es de chaque domaine sont cohÃ©rentes selon les rÃ¨gles de ce domaine.

**ConformitÃ© LOI-6 :** Ce scÃ©nario illustre **LOI-6** (l'autonomie n'empÃªche pas la fÃ©dÃ©ration) : chaque domaine reste autonome (LOI-1 Ã  LOI-5) tout en participant Ã  une fÃ©dÃ©ration contrÃ´lÃ©e via des Intentions CertifiÃ©es. La communication inter-domaines est explicite, contrÃ´lÃ©e, observable, et rÃ©versible.

**Alignement contractuel :**

- Respecte l'Authority Graph & Cross-Domain Contract (multi-domaines, isolation)
- Respecte l'Identity & Cross-Domain Trust Contract (sÃ©paration identitÃ©/autorisation, Intentions CertifiÃ©es)
- Respecte l'invariant INST-3 (isolation systÃ©mique)
- AlignÃ© avec le CoreDataAPI Contract (pas de communication directe inter-domaines)

---

## 5. ScÃ©nario 4 : Application hybride (local + serveur)

### 5.1. Contexte

**Situation :** Une application avec donnÃ©es locales et synchronisation pÃ©riodique avec un serveur.

**Architecture :**
- **Instance MÃ¨re :** Serveur central qui constitue la source d'autoritÃ© de rÃ©fÃ©rence
- **Instance Fille :** Application cliente qui maintient une copie locale des donnÃ©es

**Relation :** L'application cliente (Instance Fille) reconnaÃ®t l'autoritÃ© de l'Instance MÃ¨re (serveur) et synchronise pÃ©riodiquement pour maintenir la cohÃ©rence.

### 5.2. Intentions Ã©mises

**Modifications locales avec sync pÃ©riodique :**

L'utilisateur modifie des donnÃ©es localement (par exemple, met Ã  jour un profil, modifie des prÃ©fÃ©rences, crÃ©e des entitÃ©s). Ces modifications sont formulÃ©es comme des Write Intents et soumises Ã  l'Instance Fille locale.

La synchronisation avec l'Instance MÃ¨re est effectuÃ©e pÃ©riodiquement (par exemple, toutes les heures, ou lorsque l'utilisateur le demande explicitement).

### 5.3. Validations attendues

**Validation locale immÃ©diate :**

L'Instance Fille valide localement chaque Write Intent selon les rÃ¨gles de cohÃ©rence locales. Si la validation locale rÃ©ussit, l'intention est appliquÃ©e localement, permettant Ã  l'utilisateur de voir immÃ©diatement ses modifications.

**Validation MÃ¨re diffÃ©rÃ©e :**

Lors de la synchronisation pÃ©riodique, les Write Intents appliquÃ©es localement sont soumises Ã  l'Instance MÃ¨re pour validation dÃ©finitive. L'Instance MÃ¨re valide chaque intention selon les rÃ¨gles de cohÃ©rence de rÃ©fÃ©rence.

### 5.4. Rejets possibles

**Conflit lors de synchronisation :**

L'Instance MÃ¨re peut rejeter une intention locale si elle entre en conflit avec une modification effectuÃ©e sur l'Instance MÃ¨re pendant que l'application Ã©tait locale. Par exemple, si l'utilisateur a modifiÃ© localement une entitÃ© qui a Ã©tÃ© supprimÃ©e sur l'Instance MÃ¨re, l'intention de modification est rejetÃ©e.

**Violation de cohÃ©rence de rÃ©fÃ©rence :**

L'Instance MÃ¨re peut rejeter une intention locale si elle viole une contrainte de cohÃ©rence de rÃ©fÃ©rence. Par exemple, si l'utilisateur a crÃ©Ã© localement une entitÃ© qui viole une contrainte mÃ©tier dÃ©finie sur l'Instance MÃ¨re, l'intention de crÃ©ation est rejetÃ©e.

### 5.5. Garanties fournies

**Fonctionnement autonome :**

L'Instance Fille permet un fonctionnement autonome, garantissant que l'utilisateur peut utiliser l'application mÃªme en l'absence de connexion rÃ©seau, dans les limites autorisÃ©es.

**CohÃ©rence ultÃ©rieure :**

La synchronisation pÃ©riodique garantit que la cohÃ©rence avec l'Instance MÃ¨re est rÃ©tablie ultÃ©rieurement, mÃªme si des modifications locales ont Ã©tÃ© effectuÃ©es.

**DurabilitÃ© locale :**

Les Write Intents appliquÃ©es localement sont persistÃ©es localement, garantissant que les modifications de l'utilisateur ne sont pas perdues mÃªme si l'application est fermÃ©e.

**ConformitÃ© LOI-2 et LOI-3 :** Ce scÃ©nario illustre **LOI-2** (le systÃ¨me accepte l'isolement comme Ã©tat normal) : l'application fonctionne de maniÃ¨re autonome mÃªme en l'absence de connexion rÃ©seau, avec synchronisation pÃ©riodique diffÃ©rÃ©e. Il illustre Ã©galement **LOI-3** (l'Ã©tat local est souverain) : les modifications locales sont valables localement jusqu'Ã  la rÃ©conciliation explicite avec l'Instance MÃ¨re.

**Alignement contractuel :**

- Respecte l'invariant INST-F-2 (copie locale synchronisÃ©e)
- Respecte l'invariant INST-F-3 (synchronisation pÃ©riodique)
- Respecte l'invariant INST-F-4 (autonomie limitÃ©e)
- AlignÃ© avec le Sync & Conflict Resolution Contract (synchronisation, rÃ©solution de conflits)

---

## 6. Exemples d'erreurs courantes cÃ´tÃ© adaptateur

### 6.1. Erreur 1 : Supposer que l'identitÃ© = autorisation

**ScÃ©nario :**

Un adaptateur pense qu'une identitÃ© reconnue par KindMother autorise automatiquement toutes les opÃ©rations pour cette identitÃ©. L'adaptateur suppose que si l'identitÃ© est valide, les permissions sont automatiquement accordÃ©es.

**ConsÃ©quence :**

KindMother rejette les opÃ©rations car l'identitÃ© ne confÃ¨re pas automatiquement des autorisations. Les permissions doivent Ãªtre Ã©valuÃ©es sÃ©parÃ©ment selon les rÃ¨gles du domaine d'autoritÃ©.

**Exemple conceptuel :**

Un adaptateur soumet une Write Intent avec une identitÃ© valide mais sans fournir les permissions nÃ©cessaires. L'adaptateur suppose que l'identitÃ© valide suffit, mais KindMother rejette l'intention car les permissions ne sont pas suffisantes.

**Correction :**

L'adaptateur DOIT fournir un contexte complet incluant Ã  la fois l'identitÃ© ET les permissions. L'identitÃ© et l'autorisation sont sÃ©parÃ©es selon le Identity & Cross-Domain Trust Contract.

**Alignement contractuel :**

- Respecte la sÃ©paration identitÃ©/autorisation du Identity & Cross-Domain Trust Contract
- Respecte l'invariant INST-6 (validation obligatoire)
- Respecte la boundary de permissions du Runtime Boundary Contract

### 6.2. Erreur 2 : Contourner la CoreDataAPI pour performance

**ScÃ©nario :**

Un adaptateur tente d'accÃ©der directement aux donnÃ©es pour "optimiser" les performances, pensant que passer par la CoreDataAPI est trop lent.

**ConsÃ©quence :**

KindMother dÃ©tecte la tentative de contournement et rejette l'opÃ©ration. Si la tentative est rÃ©pÃ©tÃ©e, l'adaptateur peut Ãªtre mis en quarantaine.

**Exemple conceptuel :**

Un adaptateur crÃ©e un mÃ©canisme de "lecture directe" qui contourne la CoreDataAPI pour accÃ©der directement aux donnÃ©es, pensant amÃ©liorer les performances. KindMother dÃ©tecte cette tentative et bloque l'accÃ¨s.

**Correction :**

L'adaptateur DOIT utiliser exclusivement la CoreDataAPI pour toutes les opÃ©rations. Aucun contournement n'est autorisÃ©, mÃªme pour des raisons de performance.

**Alignement contractuel :**

- Respecte l'invariant INV-API-1 (unicitÃ© de la surface d'appel)
- Respecte les rÃ¨gles UNIQ-1 Ã  UNIQ-5 (unicitÃ© de la CoreDataAPI)
- Respecte l'interdiction INTERDIT-2 (pas d'accÃ¨s direct Ã  la persistance)
- AlignÃ© avec la rÃ©ponse systÃ©mique R3 (mise en quarantaine) pour violations rÃ©pÃ©tÃ©es

### 6.3. Erreur 3 : Ignorer les rejets de synchronisation

**ScÃ©nario :**

Un adaptateur ignore les rejets de synchronisation, pensant que les modifications locales sont suffisantes et que les rejets de l'Instance MÃ¨re peuvent Ãªtre ignorÃ©s.

**ConsÃ©quence :**

Les donnÃ©es locales deviennent incohÃ©rentes avec l'Instance MÃ¨re. Les modifications locales rejetÃ©es par l'Instance MÃ¨re restent dans l'Instance Fille, crÃ©ant une divergence permanente.

**Exemple conceptuel :**

Un adaptateur soumet des Write Intents locales Ã  l'Instance MÃ¨re lors de la synchronisation. Certaines intentions sont rejetÃ©es par l'Instance MÃ¨re, mais l'adaptateur ignore ces rejets et continue Ã  utiliser les donnÃ©es locales comme si elles Ã©taient valides.

**Correction :**

L'adaptateur DOIT accepter les dÃ©cisions de l'Instance MÃ¨re. Si une intention locale est rejetÃ©e par l'Instance MÃ¨re, l'adaptateur DOIT annuler les modifications locales correspondantes et informer l'utilisateur.

**Alignement contractuel :**

- Respecte l'invariant INST-F-1 (reconnaissance de l'autoritÃ© de l'Instance MÃ¨re)
- Respecte l'invariant INST-F-5 (soumission des opÃ©rations Ã  la validation)
- AlignÃ© avec le Sync & Conflict Resolution Contract (acceptation des dÃ©cisions de la MÃ¨re)

---

## 7. Ce que KindMother accepte / refuse

### 7.1. Accepte : Intentions valides avec contexte complet

**KindMother accepte :**

- **Intentions avec contexte complet :** Intentions accompagnÃ©es d'un contexte complet incluant l'identitÃ©, les permissions, l'instance, et le domaine d'autoritÃ©
- **Intentions conformes aux rÃ¨gles :** Intentions qui respectent toutes les rÃ¨gles de cohÃ©rence, de permissions, et de validation
- **Intentions lÃ©gales :** Intentions qui utilisent des opÃ©rations lÃ©gales de la CoreDataAPI
- **Intentions non conflictuelles :** Intentions qui ne crÃ©ent pas de conflits avec l'Ã©tat actuel des donnÃ©es

**Exemple conceptuel :**

Un adaptateur soumet une Write Intent pour crÃ©er une nouvelle entitÃ©. L'intention est accompagnÃ©e d'un contexte complet (utilisateur identifiÃ©, permissions suffisantes, instance valide, domaine d'autoritÃ© valide). L'intention respecte toutes les rÃ¨gles de cohÃ©rence. KindMother accepte l'intention, la valide, et l'applique.

### 7.2. Refuse : Intentions sans contexte, tentatives de contournement, violations d'invariants

**KindMother refuse :**

- **Intentions sans contexte complet :** Intentions qui ne sont pas accompagnÃ©es d'un contexte complet (identitÃ© manquante, permissions manquantes, instance manquante, domaine manquant)
- **Tentatives de contournement :** Tentatives d'accÃ©der directement aux donnÃ©es, de contourner la CoreDataAPI, ou de contourner les validations
- **Violations d'invariants :** Intentions qui violeraient un invariant contractuel (par exemple, violation de l'isolation, violation de l'autoritÃ© exclusive)
- **Intentions non conformes :** Intentions qui ne respectent pas les rÃ¨gles de cohÃ©rence, de permissions, ou de validation
- **Intentions conflictuelles :** Intentions qui crÃ©ent des conflits avec l'Ã©tat actuel des donnÃ©es

**Exemple conceptuel :**

Un adaptateur tente de soumettre une Write Intent sans fournir les permissions nÃ©cessaires. KindMother rejette l'intention car le contexte est incomplet. L'adaptateur reÃ§oit une erreur explicite indiquant que les permissions sont manquantes.

**Exemple conceptuel :**

Un adaptateur tente d'accÃ©der directement aux donnÃ©es sans passer par la CoreDataAPI. KindMother dÃ©tecte cette tentative de contournement et rejette l'opÃ©ration. Si la tentative est rÃ©pÃ©tÃ©e, l'adaptateur est mis en quarantaine.

---

## 8. Conclusion

Ce document illustre comment utiliser KindMother correctement via des scÃ©narios conceptuels narratifs.

**Points clÃ©s :**
- Les scÃ©narios illustrent les concepts contractuels dans diffÃ©rents contextes d'utilisation
- Les exemples d'erreurs montrent ce qu'il ne faut pas faire
- Les rÃ¨gles d'acceptation/refus clarifient ce que KindMother accepte et refuse

**Nature informative :**
Ce document est purement informatif et narratif. Il ne crÃ©e aucune nouvelle obligation contractuelle. Il sert uniquement Ã  illustrer les concepts contractuels par des exemples narratifs.

**Rappel :** Les contrats FONDATION priment toujours sur ces exemples. En cas de doute, se rÃ©fÃ©rer aux contrats FONDATION.

---

**Document crÃ©Ã© le :** 2026-01-25  
**Version :** 1.0  
**Statut :** POST-FONDATION â€” Informatif, non normatif, non contractuel  
**RÃ©fÃ©rence :** Miyukini Core System v2.4, KindMother Documentation, Instance Model Contract, Authority Graph & Cross-Domain Contract, Identity & Cross-Domain Trust Contract, CoreDataAPI Contract, Sync & Conflict Resolution Contract, Write Intent Lifecycle Contract  
**Type :** Document informatif narratif conceptuel

---

## 9. Mini log â€” erreurs / warnings / arbitrages rencontrÃ©s

### Arbitrage A1 : Niveau de dÃ©tail narratif

**Arbitrage rencontrÃ© :** Quel niveau de dÃ©tail inclure dans les scÃ©narios narratifs ? Les scÃ©narios doivent rester conceptuels sans devenir trop abstraits ou trop techniques.

**DÃ©cision prise :** Les scÃ©narios sont narratifs et descriptifs, dÃ©crivant des situations d'utilisation rÃ©elles de maniÃ¨re conceptuelle, sans dÃ©tails techniques. Ils illustrent les concepts contractuels par des histoires comprÃ©hensibles.

**Justification :** Cette approche rend les concepts contractuels accessibles tout en restant purement conceptuelle. Les scÃ©narios aident Ã  comprendre sans prescrire d'implÃ©mentation.

**Documentation :** Tous les scÃ©narios (sections 2 Ã  5) sont narratifs et descriptifs, sans dÃ©tails techniques.

### Arbitrage A2 : Exemples d'erreurs vs prescriptions

**Arbitrage rencontrÃ© :** Comment illustrer les erreurs courantes sans crÃ©er l'impression que certaines erreurs sont "acceptables" ou "tolÃ©rÃ©es" ?

**DÃ©cision prise :** Les exemples d'erreurs sont clairement prÃ©sentÃ©s comme des erreurs Ã  Ã©viter, avec leurs consÃ©quences et leurs corrections. Aucune ambiguÃ¯tÃ© n'est laissÃ©e sur le fait que ces erreurs sont interdites.

**Justification :** Cette approche Ã©ducative aide les dÃ©veloppeurs Ã  comprendre les erreurs courantes et Ã  les Ã©viter, tout en restant claire sur le fait que ces erreurs sont interdites.

**Documentation :** Section 6 (Exemples d'erreurs courantes) avec consÃ©quences et corrections explicites.

### Arbitrage A3 : ScÃ©narios multi-domaines

**Arbitrage rencontrÃ© :** Comment illustrer le scÃ©nario multi-domaines sans crÃ©er de confusion sur la complexitÃ© de la gestion multi-domaines ?

**DÃ©cision prise :** Le scÃ©nario multi-domaines est simplifiÃ© pour illustrer les concepts clÃ©s (isolation, communication contrÃ´lÃ©e, Intentions CertifiÃ©es) sans entrer dans tous les dÃ©tails de la gestion multi-domaines.

**Justification :** Cette simplification permet d'illustrer les concepts essentiels sans surcharger le document avec tous les dÃ©tails de la gestion multi-domaines, qui sont couverts dans les contrats FONDATION.

**Documentation :** Section 4 (ScÃ©nario 3 : Jeu multi-domaines) avec focus sur les concepts clÃ©s.

### Arbitrage A4 : Balance entre exemples positifs et nÃ©gatifs

**Arbitrage rencontrÃ© :** Comment Ã©quilibrer les exemples positifs (scÃ©narios d'utilisation correcte) et les exemples nÃ©gatifs (erreurs courantes) ?

**DÃ©cision prise :** Les scÃ©narios positifs (sections 2 Ã  5) illustrent l'utilisation correcte, tandis que la section 6 illustre les erreurs courantes. La section 7 clarifie ce que KindMother accepte et refuse.

**Justification :** Cette organisation permet de montrer d'abord l'utilisation correcte, puis les erreurs Ã  Ã©viter, puis les rÃ¨gles d'acceptation/refus. Cette progression pÃ©dagogique facilite la comprÃ©hension.

**Documentation :** Organisation en sections : scÃ©narios positifs (2-5), erreurs (6), rÃ¨gles d'acceptation/refus (7).

### Arbitrage A5 : RÃ©fÃ©rences aux contrats dans les scÃ©narios

**Arbitrage rencontrÃ© :** Comment rÃ©fÃ©rencer les contrats FONDATION dans les scÃ©narios narratifs sans interrompre le flux narratif ?

**DÃ©cision prise :** Les rÃ©fÃ©rences aux contrats sont incluses dans une sous-section "Alignement contractuel" Ã  la fin de chaque scÃ©nario, permettant de maintenir le flux narratif tout en fournissant les rÃ©fÃ©rences contractuelles.

**Justification :** Cette approche permet de maintenir la lisibilitÃ© narrative tout en fournissant les rÃ©fÃ©rences contractuelles nÃ©cessaires pour comprendre l'alignement avec les contrats FONDATION.

**Documentation :** Chaque scÃ©nario (sections 2 Ã  5) inclut une sous-section "Alignement contractuel" avec rÃ©fÃ©rences explicites.

---

*Aucune autre erreur, warning, ou arbitrage rencontrÃ© lors de la rÃ©daction de ce document.*

