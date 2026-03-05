# Miyukini Core System â€” Border Guard Documentation Fondatrice

## 1. Introduction

### RÃ´le de Border Guard

Border Guard (BG) est le **core de dÃ©finition des frontiÃ¨res et des rÃ¨gles d'entrÃ©e/sortie** du Miyukini Core System. Il incarne la capacitÃ© conceptuelle du systÃ¨me Ã  distinguer ce qui est interne de ce qui est externe, Ã  classifier les niveaux de confiance, et Ã  Ã©tablir les rÃ¨gles qui gouvernent toute interaction traversant une frontiÃ¨re.

Border Guard ne filtre pas lui-mÃªme, ne bloque pas lui-mÃªme, n'exÃ©cute pas lui-mÃªme. Il **dÃ©finit** les frontiÃ¨res, **Ã©tablit** les rÃ¨gles, et **classifie** les niveaux de confiance. L'application de ces rÃ¨gles est dÃ©lÃ©guÃ©e Ã  Bonding Brother et aux autres cores opÃ©rationnels.

### Question fondamentale

Border Guard rÃ©pond Ã  une question fondamentale : **"OÃ¹ sont les frontiÃ¨res du systÃ¨me, et quelles rÃ¨gles gouvernent leur franchissement ?"**

Cette question se dÃ©cline en plusieurs sous-questions :
- Qu'est-ce qui est "interne" et qu'est-ce qui est "externe" ?
- Quel niveau de confiance accorder Ã  une source ou une destination ?
- Quelles conditions doivent Ãªtre respectÃ©es pour franchir une frontiÃ¨re ?
- Comment classifier les intÃ©grations selon leur nature et leur risque ?

### PortÃ©e

Ce contrat s'applique Ã  **toutes les dÃ©finitions de frontiÃ¨res** dans le systÃ¨me Miyukini et dÃ©finit de maniÃ¨re absolue :
- La dÃ©finition formelle des frontiÃ¨res et de leur nature
- La classification des niveaux de confiance
- Les rÃ¨gles de franchissement des frontiÃ¨res
- Les invariants de dÃ©finition de frontiÃ¨re
- Les garanties offertes par Border Guard
- Les distinctions entre dÃ©finition conceptuelle et application technique

### Statut contractuel

Ce document est **contractuel, normatif, non discutable, et de statut FONDATION**. Il Ã©tablit des rÃ¨gles absolues qui ne peuvent Ãªtre contournÃ©es, nÃ©gociÃ©es, ou modifiÃ©es. Le contrat prime sur toute considÃ©ration pratique.

---

## 2. Raison d'Ãªtre

### ProblÃ¨me que Border Guard rÃ©sout

Dans l'architecture actuelle de MCS, les frontiÃ¨res entre l'interne et l'externe sont implicites, dispersÃ©es, et incohÃ©rentes. Cette absence de dÃ©finition formelle prÃ©sente plusieurs limitations :

1. **FrontiÃ¨res implicites** : Chaque composant dÃ©finit ses propres frontiÃ¨res sans vision globale, conduisant Ã  des dÃ©finitions contradictoires

2. **Niveaux de confiance non standardisÃ©s** : Chaque intÃ©gration gÃ¨re ses propres niveaux de confiance sans classification cohÃ©rente Ã  l'Ã©chelle du systÃ¨me

3. **RÃ¨gles de franchissement dispersÃ©es** : Les rÃ¨gles qui gouvernent le passage d'une frontiÃ¨re sont rÃ©pliquÃ©es et incohÃ©rentes entre composants

4. **Absence de gouvernance des intÃ©grations** : Aucun point central pour dÃ©finir la politique d'intÃ©gration avec les systÃ¨mes externes

5. **Confusion entre dÃ©finition et application** : La dÃ©finition des frontiÃ¨res est mÃ©langÃ©e avec leur application technique, crÃ©ant un couplage fort

Border Guard rÃ©sout ces problÃ¨mes en fournissant un core dÃ©diÃ© qui :
- DÃ©finit formellement les frontiÃ¨res du systÃ¨me
- Ã‰tablit une classification standardisÃ©e des niveaux de confiance
- Centralise les rÃ¨gles de franchissement des frontiÃ¨res
- Gouverne conceptuellement toutes les intÃ©grations
- SÃ©pare strictement la dÃ©finition de l'application

### Positionnement architectural

Border Guard est un **core conceptuel** :
- Il ne possÃ¨de aucune capacitÃ© d'exÃ©cution
- Il ne filtre pas, ne bloque pas, n'intercepte pas
- Il dÃ©finit, classifie, et Ã©tablit des rÃ¨gles
- Ses dÃ©finitions sont consommÃ©es par Bonding Brother pour l'application

Border Guard est conÃ§u comme une **autoritÃ© de dÃ©finition** :
- AutoritÃ© exclusive sur la dÃ©finition des frontiÃ¨res
- AutoritÃ© exclusive sur la classification des niveaux de confiance
- Aucune autoritÃ© sur l'application des rÃ¨gles
- Aucune autoritÃ© sur l'exÃ©cution technique

---

## 3. Positionnement familial

### Relation avec Kind Mother

Border Guard et Kind Mother sont complÃ©mentaires mais distincts :

**Kind Mother** gouverne les donnÃ©es et leur persistance. Elle dÃ©finit ce qui est une donnÃ©e, comment elle est stockÃ©e, comment elle est synchronisÃ©e.

**Border Guard** gouverne les frontiÃ¨res et les niveaux de confiance. Il dÃ©finit si une donnÃ©e venant de l'extÃ©rieur peut entrer, avec quel niveau de confiance, selon quelles rÃ¨gles.

La relation est de complÃ©mentaritÃ© : Kind Mother traite les donnÃ©es une fois qu'elles sont "Ã  l'intÃ©rieur" ; Border Guard dÃ©finit les conditions pour qu'elles y entrent.

Border Guard ne connaÃ®t pas les dÃ©tails de persistance de Kind Mother. Kind Mother ne connaÃ®t pas les dÃ©tails de classification de Border Guard. Chacun reste souverain dans son domaine.

### Relation avec Strong Father

Border Guard et Strong Father sont complÃ©mentaires et collaboratifs :

**Strong Father** prend les dÃ©cisions stratÃ©giques et politiques. Il Ã©value les intentions et produit des dÃ©cisions (acceptÃ©e, refusÃ©e, ambiguÃ«).

**Border Guard** dÃ©finit le contexte de confiance dans lequel Strong Father opÃ¨re. Il fournit Ã  Strong Father l'information sur le niveau de confiance de l'origine d'une intention, la nature de la frontiÃ¨re franchie, les rÃ¨gles applicables.

La relation est de conseil : Border Guard informe Strong Father sur le contexte de frontiÃ¨re ; Strong Father dÃ©cide en tenant compte de cette information.

Border Guard ne dÃ©cide jamais. Strong Father dÃ©cide en s'appuyant sur les dÃ©finitions de Border Guard.

### Relation avec Bonding Brother

Border Guard et Bonding Brother ont une relation fondamentale et asymÃ©trique :

**Border Guard dÃ©finit les rÃ¨gles** de franchissement des frontiÃ¨res, les niveaux de confiance, les conditions d'entrÃ©e et de sortie.

**Bonding Brother applique ces rÃ¨gles** lors de la mÃ©diation entre les produits et l'Ã©cosystÃ¨me. Il consulte les dÃ©finitions de Border Guard et les applique concrÃ¨tement.

La relation est de dÃ©finition/application : Border Guard est l'autoritÃ© conceptuelle, Bonding Brother est l'exÃ©cutant opÃ©rationnel.

Cette relation est non nÃ©gociable : Bonding Brother ne dÃ©finit jamais de frontiÃ¨re, Border Guard n'applique jamais de rÃ¨gle. La sÃ©paration est absolue.

### Relation avec Caring Nanny

Border Guard et Caring Nanny sont complÃ©mentaires dans l'observation :

**Caring Nanny** observe l'Ã©tat global du systÃ¨me (healthy, degraded, offline, syncing, error).

**Border Guard** dÃ©finit comment l'Ã©tat des frontiÃ¨res influence l'Ã©tat global. Une frontiÃ¨re compromise peut signaler un Ã©tat dÃ©gradÃ©. Une intÃ©gration dÃ©faillante peut signaler un problÃ¨me.

La relation est d'information : Border Guard informe Caring Nanny sur l'Ã©tat des frontiÃ¨res ; Caring Nanny intÃ¨gre cette information dans l'Ã©tat global.

### La famille Miyukini

Dans la famille Miyukini, Border Guard est le **gardien des limites** : il connaÃ®t les frontiÃ¨res de la maison, il sait qui peut entrer par quelle porte, il dÃ©finit les rÃ¨gles d'accueil des visiteurs.

Border Guard ne dÃ©cide pas qui entre (c'est Strong Father), ne stocke pas les informations des visiteurs (c'est Kind Mother), n'accueille pas lui-mÃªme les visiteurs (c'est Bonding Brother). Il dÃ©finit oÃ¹ sont les portes, quelles sont les rÃ¨gles, quel niveau de confiance accorder.

---

## 4. Concepts fondamentaux

### FrontiÃ¨re

Une **frontiÃ¨re** est une dÃ©marcation conceptuelle qui sÃ©pare deux zones de confiance diffÃ©rentes. Une frontiÃ¨re peut Ãªtre :

**FrontiÃ¨re externe** : SÃ©pare l'Ã©cosystÃ¨me Miyukini du monde extÃ©rieur (internet, systÃ¨mes tiers, utilisateurs non authentifiÃ©s). C'est la limite entre le "dehors" et le "dedans".

**FrontiÃ¨re interne** : SÃ©pare diffÃ©rentes zones de confiance au sein de l'Ã©cosystÃ¨me (zone admin vs zone utilisateur, module sensible vs module standard, donnÃ©es critiques vs donnÃ©es publiques).

**FrontiÃ¨re d'intÃ©gration** : SÃ©pare l'Ã©cosystÃ¨me d'un systÃ¨me externe avec lequel il interagit de maniÃ¨re contrÃ´lÃ©e (API partenaire, service tiers, base de donnÃ©es externe).

Une frontiÃ¨re possÃ¨de :
- Une identitÃ© unique et stable
- Une direction (entrÃ©e, sortie, bidirectionnelle)
- Un niveau de permÃ©abilitÃ© (ouvert, contrÃ´lÃ©, fermÃ©)
- Des rÃ¨gles de franchissement associÃ©es

### Niveau de confiance

Un **niveau de confiance** est une classification qui indique le degrÃ© de fiabilitÃ© accordÃ© Ã  une source, une destination, ou une interaction. Border Guard dÃ©finit quatre niveaux canoniques :

**Trusted (Confiance totale)** : La source ou destination fait partie du cercle de confiance absolu. Aucune vÃ©rification supplÃ©mentaire n'est requise. RÃ©servÃ© aux composants internes validÃ©s, aux autoritÃ©s du systÃ¨me.

**Verified (Confiance vÃ©rifiÃ©e)** : La source ou destination a Ã©tÃ© authentifiÃ©e et validÃ©e selon des critÃ¨res stricts. Des vÃ©rifications ont Ã©tÃ© effectuÃ©es. Niveau accordÃ© aux utilisateurs authentifiÃ©s, aux intÃ©grations certifiÃ©es.

**Unknown (Confiance inconnue)** : La source ou destination n'a pas encore Ã©tÃ© classifiÃ©e. Niveau par dÃ©faut pour tout ce qui arrive de l'extÃ©rieur. Toute interaction avec ce niveau est soumise Ã  des rÃ¨gles restrictives.

**Hostile (Confiance nulle)** : La source ou destination a Ã©tÃ© identifiÃ©e comme malveillante, compromise, ou violant les rÃ¨gles. Aucune interaction n'est autorisÃ©e. Niveau appliquÃ© aux sources blacklistÃ©es, aux patterns d'attaque dÃ©tectÃ©s.

### RÃ¨gle de franchissement

Une **rÃ¨gle de franchissement** est une condition qui doit Ãªtre satisfaite pour qu'une interaction puisse traverser une frontiÃ¨re. Une rÃ¨gle est :

**DÃ©clarative** : Elle exprime ce qui est requis, pas comment le vÃ©rifier techniquement.

**Non ambiguÃ«** : Elle spÃ©cifie clairement les conditions sans interprÃ©tation possible.

**AssociÃ©e Ã  une frontiÃ¨re** : Elle est dÃ©finie pour une frontiÃ¨re spÃ©cifique ou un ensemble de frontiÃ¨res.

Une rÃ¨gle de franchissement peut porter sur :
- Le niveau de confiance requis
- L'authentification requise
- Les donnÃ©es autorisÃ©es Ã  traverser
- Les actions autorisÃ©es
- Les conditions temporelles

### Zone de confiance

Une **zone de confiance** est un espace conceptuel dÃ©limitÃ© par des frontiÃ¨res, oÃ¹ tous les Ã©lÃ©ments partagent un mÃªme niveau de confiance. Une zone de confiance :

- Est dÃ©limitÃ©e par une ou plusieurs frontiÃ¨res
- PossÃ¨de un niveau de confiance homogÃ¨ne
- Contient des composants, des donnÃ©es, des services
- Interagit avec d'autres zones via des frontiÃ¨res

### IntÃ©gration

Une **intÃ©gration** est une relation Ã©tablie entre l'Ã©cosystÃ¨me Miyukini et un systÃ¨me externe. Une intÃ©gration est classifiÃ©e par Border Guard selon :

- Son niveau de confiance initial
- Les frontiÃ¨res qu'elle traverse
- Les rÃ¨gles de franchissement applicables
- Son Ã©tat (active, suspendue, rÃ©voquÃ©e)

---

## 5. ResponsabilitÃ©s exclusives

### DÃ©finition des frontiÃ¨res

Border Guard est **exclusivement responsable** de la dÃ©finition formelle des frontiÃ¨res du systÃ¨me. Cette responsabilitÃ© inclut :

- Identifier et nommer chaque frontiÃ¨re
- Classifier la nature de chaque frontiÃ¨re (externe, interne, intÃ©gration)
- DÃ©finir la direction de chaque frontiÃ¨re (entrÃ©e, sortie, bidirectionnelle)
- Ã‰tablir le niveau de permÃ©abilitÃ© de chaque frontiÃ¨re
- Maintenir le registre exhaustif des frontiÃ¨res du systÃ¨me

Aucun autre core ne dÃ©finit de frontiÃ¨re. Toute dÃ©finition de frontiÃ¨re provient exclusivement de Border Guard.

### Classification des niveaux de confiance

Border Guard est **exclusivement responsable** de la classification des niveaux de confiance. Cette responsabilitÃ© inclut :

- DÃ©finir les critÃ¨res de chaque niveau de confiance (trusted, verified, unknown, hostile)
- Classifier les sources et destinations selon ces niveaux
- Ã‰tablir les rÃ¨gles de transition entre niveaux
- Maintenir la cohÃ©rence de la classification Ã  travers le systÃ¨me

Aucun autre core ne classifie les niveaux de confiance. Toute classification provient exclusivement de Border Guard.

### Ã‰tablissement des rÃ¨gles de franchissement

Border Guard est **exclusivement responsable** de l'Ã©tablissement des rÃ¨gles de franchissement. Cette responsabilitÃ© inclut :

- DÃ©finir les rÃ¨gles associÃ©es Ã  chaque frontiÃ¨re
- SpÃ©cifier les conditions de franchissement
- Ã‰tablir les exceptions et cas particuliers
- Maintenir la cohÃ©rence des rÃ¨gles entre frontiÃ¨res

Aucun autre core n'Ã©tablit de rÃ¨gle de franchissement. Toute rÃ¨gle provient exclusivement de Border Guard.

### Gouvernance conceptuelle des intÃ©grations

Border Guard est **exclusivement responsable** de la gouvernance conceptuelle des intÃ©grations. Cette responsabilitÃ© inclut :

- Classifier chaque intÃ©gration selon sa nature et son risque
- DÃ©finir le cadre d'interaction avec chaque systÃ¨me externe
- Ã‰tablir les conditions de suspension ou rÃ©vocation d'une intÃ©gration
- Maintenir le registre des intÃ©grations et leur Ã©tat

Aucun autre core ne gouverne conceptuellement les intÃ©grations. Cette gouvernance provient exclusivement de Border Guard.

### Conseil aux autres cores

Border Guard est **responsable** de fournir les informations de frontiÃ¨re aux autres cores. Cette responsabilitÃ© inclut :

- Informer Strong Father du contexte de confiance d'une intention
- Informer Bonding Brother des rÃ¨gles Ã  appliquer
- Informer Caring Nanny de l'Ã©tat des frontiÃ¨res

Cette responsabilitÃ© de conseil n'est pas une autoritÃ© : Border Guard informe, les autres cores dÃ©cident ou agissent.

---

## 6. Ce que Border Guard ne fait PAS

### Ne filtre pas

Border Guard ne filtre **jamais** les interactions. Le filtrage est une action d'application, pas de dÃ©finition. Border Guard dÃ©finit les rÃ¨gles de filtrage ; Bonding Brother les applique.

### Ne bloque pas

Border Guard ne bloque **jamais** les accÃ¨s. Le blocage est une action d'exÃ©cution. Border Guard dÃ©finit les conditions qui peuvent conduire Ã  un blocage ; Bonding Brother ou Strong Father exÃ©cute le blocage.

### N'authentifie pas

Border Guard ne gÃ¨re **jamais** l'authentification technique. L'authentification (tokens, sessions, OAuth, JWT) est du ressort du produit ou d'un module auth dÃ©diÃ©. Border Guard dÃ©finit les niveaux de confiance ; l'authentification technique dÃ©termine comment atteindre ces niveaux.

### Ne persiste pas

Border Guard ne persiste **jamais** de donnÃ©es. La persistance est du ressort exclusif de Kind Mother. Border Guard dÃ©finit des frontiÃ¨res et des rÃ¨gles ; leur stockage est dÃ©lÃ©guÃ© Ã  Kind Mother.

### Ne dÃ©cide pas

Border Guard ne prend **jamais** de dÃ©cision stratÃ©gique ou politique. La dÃ©cision est du ressort exclusif de Strong Father. Border Guard informe sur le contexte de confiance ; Strong Father dÃ©cide.

### N'exÃ©cute pas

Border Guard n'exÃ©cute **jamais** d'action technique. L'exÃ©cution est du ressort des cores opÃ©rationnels (Bonding Brother, adaptateurs, produits). Border Guard est purement conceptuel.

### Ne modifie pas l'Ã©tat

Border Guard ne modifie **jamais** l'Ã©tat du systÃ¨me. L'observation de l'Ã©tat est du ressort de Caring Nanny, la modification de l'Ã©tat est du ressort des cores exÃ©cutants. Border Guard dÃ©finit, il ne modifie pas.

### Ne contient pas de logique mÃ©tier

Border Guard ne contient **jamais** de logique mÃ©tier spÃ©cifique aux produits. Il dÃ©finit des concepts gÃ©nÃ©raux (frontiÃ¨res, confiance, rÃ¨gles) applicables Ã  tous les produits. La logique mÃ©tier spÃ©cifique reste dans les produits.

---

## 7. Invariants non nÃ©gociables

### INV-BG-1 : Aucune capacitÃ© d'exÃ©cution

Border Guard ne possÃ¨de **jamais** de capacitÃ© d'exÃ©cution. Il ne filtre pas, ne bloque pas, n'intercepte pas, n'applique pas. Toute capacitÃ© d'exÃ©cution viole cet invariant fondamental.

### INV-BG-2 : Aucune persistance directe

Border Guard n'accÃ¨de **jamais** directement Ã  la persistance. Toute dÃ©finition de frontiÃ¨re ou de rÃ¨gle qui doit Ãªtre persistÃ©e est transmise Ã  Kind Mother via les canaux appropriÃ©s.

### INV-BG-3 : Aucune dÃ©cision autonome

Border Guard ne prend **jamais** de dÃ©cision de maniÃ¨re autonome. Il informe, il classifie, il dÃ©finit, mais la dÃ©cision finale appartient toujours Ã  Strong Father ou aux autoritÃ©s appropriÃ©es.

### INV-BG-4 : Classification exhaustive

Toute source, destination, ou interaction **doit** Ãªtre classifiÃ©e selon un niveau de confiance. Aucune interaction ne peut exister sans classification. Par dÃ©faut, tout ce qui n'est pas explicitement classifiÃ© est considÃ©rÃ© comme "unknown".

### INV-BG-5 : FrontiÃ¨res explicites

Toute frontiÃ¨re **doit** Ãªtre explicitement dÃ©finie et documentÃ©e. Aucune frontiÃ¨re implicite n'est autorisÃ©e. Si une dÃ©marcation existe dans le systÃ¨me, elle doit Ãªtre formalisÃ©e par Border Guard.

### INV-BG-6 : RÃ¨gles dÃ©claratives

Toutes les rÃ¨gles de franchissement **doivent** Ãªtre dÃ©claratives. Aucune rÃ¨gle procÃ©durale ou impÃ©rative n'est autorisÃ©e. Une rÃ¨gle exprime ce qui est requis, pas comment le vÃ©rifier.

### INV-BG-7 : SÃ©paration dÃ©finition/application

La dÃ©finition des frontiÃ¨res et des rÃ¨gles est **strictement sÃ©parÃ©e** de leur application. Border Guard dÃ©finit, Bonding Brother applique. Cette sÃ©paration est non nÃ©gociable et ne peut Ãªtre contournÃ©e.

### INV-BG-8 : TraÃ§abilitÃ© complÃ¨te

Toute dÃ©finition de frontiÃ¨re, toute classification de confiance, toute rÃ¨gle Ã©tablie **doit** Ãªtre traÃ§able avec son origine, sa date, et sa justification.

### INV-BG-9 : CohÃ©rence globale

Les dÃ©finitions de Border Guard **doivent** Ãªtre globalement cohÃ©rentes. Aucune contradiction entre frontiÃ¨res, niveaux de confiance, ou rÃ¨gles n'est autorisÃ©e.

### INV-BG-10 : NeutralitÃ© conceptuelle

Border Guard **ne fait jamais** de supposition sur la technologie d'implÃ©mentation. Les dÃ©finitions sont purement conceptuelles et peuvent Ãªtre implÃ©mentÃ©es par n'importe quelle technologie.

---

## 8. Interactions avec l'Ã©cosystÃ¨me

### Flux d'information vers Strong Father

Quand Strong Father Ã©value une intention, il peut consulter Border Guard pour obtenir le contexte de confiance :

1. **Strong Father** reÃ§oit une intention Ã  Ã©valuer
2. **Strong Father** demande Ã  Border Guard le contexte de frontiÃ¨re (quelle frontiÃ¨re est traversÃ©e, quel niveau de confiance de la source)
3. **Border Guard** retourne les informations de classification et les rÃ¨gles applicables
4. **Strong Father** utilise ces informations pour prendre sa dÃ©cision

Ce flux est purement informatif : Border Guard ne participe pas Ã  la dÃ©cision, il fournit le contexte.

### Flux de rÃ¨gles vers Bonding Brother

Quand Bonding Brother doit mÃ©dier une interaction traversant une frontiÃ¨re, il consulte Border Guard :

1. **Bonding Brother** reÃ§oit une intention de mÃ©diation
2. **Bonding Brother** identifie qu'une frontiÃ¨re est traversÃ©e
3. **Bonding Brother** demande Ã  Border Guard les rÃ¨gles de franchissement applicables
4. **Border Guard** retourne les rÃ¨gles dÃ©claratives
5. **Bonding Brother** applique ces rÃ¨gles concrÃ¨tement

Ce flux est de dÃ©finition/application : Border Guard fournit les rÃ¨gles, Bonding Brother les exÃ©cute.

### Flux d'Ã©tat vers Caring Nanny

Quand l'Ã©tat d'une frontiÃ¨re change (intÃ©gration dÃ©faillante, frontiÃ¨re compromise), Border Guard informe Caring Nanny :

1. **Border Guard** dÃ©tecte un changement d'Ã©tat d'une frontiÃ¨re ou d'une intÃ©gration
2. **Border Guard** notifie Caring Nanny de ce changement
3. **Caring Nanny** intÃ¨gre cette information dans l'Ã©tat global du systÃ¨me

Ce flux est d'observation : Border Guard signale, Caring Nanny observe et agrÃ¨ge.

### Flux de classification

Quand une nouvelle source ou intÃ©gration doit Ãªtre classifiÃ©e :

1. **Le produit** ou **Bonding Brother** soumet une demande de classification
2. **Border Guard** Ã©value selon ses critÃ¨res et dÃ©finitions
3. **Border Guard** attribue un niveau de confiance
4. **Border Guard** Ã©tablit les rÃ¨gles de franchissement applicables
5. **Border Guard** notifie les cores concernÃ©s de cette nouvelle classification

Ce flux est de classification : Border Guard est l'autoritÃ© qui attribue les niveaux de confiance.

### Diagramme des interactions

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚                     Ã‰COSYSTÃˆME MIYUKINI                      â”‚
â”‚                                                              â”‚
â”‚    â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”                    â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”       â”‚
â”‚    â”‚   Strong    â”‚â—„â”€â”€â”€â”€â”€ contexte â”€â”€â”€â”€â”‚   Border    â”‚       â”‚
â”‚    â”‚   Father    â”‚      de confiance  â”‚   Guard     â”‚       â”‚
â”‚    â”‚  (DÃ©cision) â”‚                    â”‚ (DÃ©finition)â”‚       â”‚
â”‚    â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜                    â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜       â”‚
â”‚                                            â”‚ â”‚              â”‚
â”‚                                   rÃ¨gles â”€â”€â”˜ â””â”€â”€ Ã©tat       â”‚
â”‚                                            â”‚ â”‚              â”‚
â”‚    â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”                         â–¼ â”‚              â”‚
â”‚    â”‚  Bonding    â”‚â—„â”€â”€â”€â”€â”€â”€ rÃ¨gles â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜ â”‚              â”‚
â”‚    â”‚  Brother    â”‚        de franchissement   â”‚              â”‚
â”‚    â”‚(Application)â”‚                            â”‚              â”‚
â”‚    â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜                            â–¼              â”‚
â”‚                                        â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”       â”‚
â”‚                                        â”‚   Caring    â”‚       â”‚
â”‚                                        â”‚   Nanny     â”‚       â”‚
â”‚                                        â”‚   (Ã‰tat)    â”‚       â”‚
â”‚                                        â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜       â”‚
â”‚                                                              â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                          â”‚
                          â”‚ FrontiÃ¨re externe
                          â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚                     MONDE EXTÃ‰RIEUR                          â”‚
â”‚   (SystÃ¨mes tiers, utilisateurs, intÃ©grations)               â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

---

## 9. Vocabulaire canonique

Le vocabulaire de Border Guard est prÃ©cis, stable, non ambigu. Chaque terme a une dÃ©finition canonique, non nÃ©gociable.

### FrontiÃ¨re

Une **frontiÃ¨re** est une dÃ©marcation conceptuelle entre deux zones de confiance diffÃ©rentes. Elle possÃ¨de une identitÃ©, une direction, un niveau de permÃ©abilitÃ©, et des rÃ¨gles de franchissement associÃ©es. Une frontiÃ¨re est toujours explicitement dÃ©finie par Border Guard.

### Zone de confiance

Une **zone de confiance** est un espace conceptuel dÃ©limitÃ© par des frontiÃ¨res oÃ¹ tous les Ã©lÃ©ments partagent un niveau de confiance homogÃ¨ne. Les zones de confiance sont organisÃ©es hiÃ©rarchiquement, de la plus sÃ©curisÃ©e (zone interne) Ã  la moins sÃ©curisÃ©e (zone externe).

### Niveau de confiance

Un **niveau de confiance** est une classification attribuÃ©e Ã  une source, une destination, ou une interaction. Les niveaux canoniques sont : trusted (confiance totale), verified (confiance vÃ©rifiÃ©e), unknown (confiance inconnue), hostile (confiance nulle).

### Franchissement

Un **franchissement** est l'acte de traverser une frontiÃ¨re. Chaque franchissement est soumis aux rÃ¨gles dÃ©finies pour la frontiÃ¨re concernÃ©e. Un franchissement peut Ãªtre autorisÃ©, conditionnel, ou interdit selon les rÃ¨gles.

### RÃ¨gle de franchissement

Une **rÃ¨gle de franchissement** est une condition dÃ©clarative qui spÃ©cifie ce qui est requis pour qu'un franchissement soit autorisÃ©. Une rÃ¨gle est associÃ©e Ã  une frontiÃ¨re et s'applique Ã  toutes les interactions traversant cette frontiÃ¨re.

### IntÃ©gration

Une **intÃ©gration** est une relation Ã©tablie entre l'Ã©cosystÃ¨me Miyukini et un systÃ¨me externe. Une intÃ©gration est classifiÃ©e par Border Guard et possÃ¨de un niveau de confiance, des frontiÃ¨res associÃ©es, et des rÃ¨gles spÃ©cifiques.

### PermÃ©abilitÃ©

La **permÃ©abilitÃ©** est la caractÃ©ristique d'une frontiÃ¨re qui indique sa propension Ã  autoriser le franchissement. Une frontiÃ¨re peut Ãªtre ouverte (franchissement libre sous conditions minimales), contrÃ´lÃ©e (franchissement soumis Ã  vÃ©rification), ou fermÃ©e (franchissement interdit).

### Classification

La **classification** est l'acte d'attribuer un niveau de confiance Ã  une source, une destination, ou une interaction. Seul Border Guard a l'autoritÃ© de classifier. Toute interaction non explicitement classifiÃ©e est considÃ©rÃ©e comme "unknown".

### Gouvernance d'intÃ©gration

La **gouvernance d'intÃ©gration** est l'ensemble des rÃ¨gles et processus qui encadrent la relation avec les systÃ¨mes externes. Cette gouvernance dÃ©finit les conditions d'Ã©tablissement, de maintien, et de rÃ©vocation des intÃ©grations.

### Contexte de frontiÃ¨re

Le **contexte de frontiÃ¨re** est l'ensemble des informations relatives aux frontiÃ¨res traversÃ©es par une interaction : quelles frontiÃ¨res, quel niveau de confiance de la source, quelles rÃ¨gles applicables. Ce contexte est fourni par Border Guard aux autres cores.

---

## 10. ConformitÃ© aux Lois d'Autonomie SystÃ¨me

Ce core respecte les **Lois d'Autonomie SystÃ¨me** dÃ©finies dans [Miyukini Framework - Lois Autonomie Systeme.md](..//..//..//miyukini-webway-system//reference//_index.md). Border Guard devient **critique pour l'autonomie** en contrÃ´lant toutes les frontiÃ¨res du systÃ¨me.

### LOI-1 : Aucune dÃ©pendance externe critique Ã  l'exÃ©cution

**ConformitÃ© :** âœ… **Conforme â€” RÃ´le critique**

Border Guard respecte intÃ©gralement LOI-1 et joue un rÃ´le critique :
- **ContrÃ´le tout ce qui entre et sort du systÃ¨me** via la dÃ©finition des frontiÃ¨res
- Les rÃ¨gles de franchissement sont **locales** et chargÃ©es au dÃ©marrage
- Aucune dÃ©finition de frontiÃ¨re ne nÃ©cessite un appel externe
- L'absence de connexion ne bloque jamais la dÃ©finition des frontiÃ¨res

**Architecture :** Border Guard dÃ©finit les frontiÃ¨res de maniÃ¨re locale et autonome. C'est le gardien qui garantit qu'aucune dÃ©pendance externe critique ne peut entrer dans le systÃ¨me.

### LOI-6 : L'autonomie n'empÃªche pas la fÃ©dÃ©ration

**ConformitÃ© :** âœ… **Conforme â€” RÃ´le critique**

Border Guard joue un rÃ´le critique pour LOI-6 :
- **Validation explicite des Ã©changes fÃ©dÃ©rÃ©s** : Toute communication inter-nÅ“uds doit passer par Border Guard pour classification
- **Rien d'implicite** : Les frontiÃ¨res sont explicitement dÃ©finies, pas supposÃ©es
- **ContrÃ´le des rÃ¨gles de partage** : Border Guard dÃ©finit ce qui peut Ãªtre partagÃ© dans une fÃ©dÃ©ration
- **FÃ©dÃ©ration rÃ©versible** : Les frontiÃ¨res peuvent Ãªtre modifiÃ©es pour quitter une fÃ©dÃ©ration

**Architecture :** Border Guard dÃ©finit les rÃ¨gles de fÃ©dÃ©ration, garantissant que la fÃ©dÃ©ration reste explicite, contrÃ´lÃ©e, observable, et rÃ©versible.

### RÃ´le renforcÃ© dans l'autonomie

Border Guard devient **critique pour l'autonomie** car :
- **ContrÃ´le des entrÃ©es/sorties** : Aucune communication externe ne peut contourner Border Guard
- **Validation explicite** : Tous les Ã©changes fÃ©dÃ©rÃ©s sont validÃ©s selon les rÃ¨gles dÃ©finies par Border Guard
- **Protection de l'autonomie** : Les frontiÃ¨res dÃ©finies par Border Guard protÃ¨gent l'autonomie du systÃ¨me

**Relation avec Bonding Brother :** Border Guard dÃ©finit les rÃ¨gles, Bonding Brother les applique. Cette sÃ©paration garantit que les frontiÃ¨res sont dÃ©finies localement (LOI-1) et que la fÃ©dÃ©ration est contrÃ´lÃ©e (LOI-6).

### Autres lois

- **LOI-2 (Isolement comme Ã©tat normal)** : Les frontiÃ¨res dÃ©finies par Border Guard permettent de reconnaÃ®tre l'isolement comme un Ã©tat normal (pas d'erreur si une frontiÃ¨re est fermÃ©e).
- **LOI-3 (Ã‰tat local souverain)** : Les dÃ©finitions de frontiÃ¨res sont locales et souveraines.
- **LOI-5 (CoÃ»t hardware)** : Border Guard est un core conceptuel lÃ©ger, sans exÃ©cution, optimisÃ© pour les ressources limitÃ©es.

---

## 11. Conclusion et statut contractuel

### Phrase fondatrice

**Border Guard est l'autoritÃ© de dÃ©finition des frontiÃ¨res et des niveaux de confiance qui Ã©tablit les rÃ¨gles de franchissement sans jamais les appliquer lui-mÃªme, sÃ©parant strictement la dÃ©finition conceptuelle de l'exÃ©cution technique.**

Cette phrase rÃ©sume l'essence de Border Guard : autoritÃ© (mais non dÃ©cisionnel), dÃ©finition (mais non exÃ©cution), rÃ¨gles (mais non filtrage), conceptuel (mais non technique).

### Garanties offertes

Border Guard garantit :

1. **ExhaustivitÃ©** : Toute frontiÃ¨re du systÃ¨me est explicitement dÃ©finie
2. **Classification complÃ¨te** : Toute source et interaction est classifiÃ©e
3. **CohÃ©rence** : Les dÃ©finitions sont globalement cohÃ©rentes et non contradictoires
4. **TraÃ§abilitÃ©** : Toute dÃ©finition est traÃ§able avec son origine et sa justification
5. **NeutralitÃ© technique** : Les dÃ©finitions sont indÃ©pendantes de l'implÃ©mentation
6. **SÃ©paration stricte** : La dÃ©finition est strictement sÃ©parÃ©e de l'application

### Relation avec les autres contrats

Ce contrat complÃ¨te et respecte les documents contractuels existants :

- **Kind Mother â€” Documentation Fondatrice** : Border Guard ne persiste pas et ne stocke pas
- **Strong Father â€” Documentation Fondatrice** : Border Guard informe mais ne dÃ©cide pas
- **Bonding Brother â€” Documentation Fondatrice** : Border Guard dÃ©finit, Bonding Brother applique

Il n'introduit aucune contradiction et constitue la dÃ©finition formelle de ce que signifie une frontiÃ¨re, un niveau de confiance, et une rÃ¨gle de franchissement dans le systÃ¨me Miyukini.

### Statut final

Ce document est de statut **FONDATION**. Il Ã©tablit des rÃ¨gles absolues qui ne peuvent Ãªtre contournÃ©es, nÃ©gociÃ©es, ou modifiÃ©es. Toute implÃ©mentation touchant aux frontiÃ¨res, aux niveaux de confiance, ou aux rÃ¨gles de franchissement doit respecter intÃ©gralement ce document.

Les invariants dÃ©finis ici sont non nÃ©gociables. Toute violation de ces invariants constitue une faute architecturale qui doit Ãªtre corrigÃ©e.

---

**Version :** 1.5  
**Date :** 2026-01-26  
**Statut :** FONDATION â€” Non nÃ©gociable  
**RÃ©fÃ©rence :** Miyukini Core System v2.4, Kind Mother Documentation Fondatrice, Strong Father Documentation Fondatrice, Bonding Brother Documentation Fondatrice, Miyukini Conceptual References - Lois Autonomie Systeme, [Miyukini Conceptual References - Integrity Degradation System](..//..//..//miyukini-webway-system//reference//_index.md), [Miyukini Conceptual References - External Signal Trust Reinforcement Contract](..//..//..//miyukini-webway-system//reference//_index.md), [Miyukini Conceptual References - Mobile WebApp Strategy](..//..//..//miyukini-webway-system//reference//_index.md) (protection injection mobile/web), [Miyukini Conceptual References - Security Protocols](..//..//..//miyukini-webway-system//reference//_index.md) (classification sources, protection injection), [Miyukini Conceptual References - Security Levels](..//..//..//miyukini-webway-system//reference//_index.md) (adaptation frontiÃ¨res selon niveau sÃ©curitÃ© 0-4)

