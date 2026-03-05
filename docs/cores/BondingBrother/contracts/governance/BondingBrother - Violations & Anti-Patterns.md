# BondingBrother - Violations & Anti-Patterns

## 1. Contexte

Ce document liste exhaustivement les violations et anti-patterns que Bonding Brother ne doit **JAMAIS** commettre. Il complÃ¨te la Section 10 de la [Documentation Fondatrice](../../foundation/BondingBrother%20-%20Documentation%20Fondatrice.md) et les [Invariants & Guarantees](./BondingBrother%20-%20Invariants%20&%20Guarantees.md) en dÃ©taillant ce qui est explicitement interdit.

Ce document sert de rÃ©fÃ©rence pour :
- Les dÃ©veloppeurs implÃ©mentant Bonding Brother
- Les audits de code et d'architecture
- Les revues de design
- Les tests de non-rÃ©gression

Les violations incluent Ã©galement celles des [Lois d'Autonomie SystÃ¨me](..//..//..//..//miyukini-webway-system//reference//_index.md) : dÃ©pendances externes critiques (**LOI-1**), blocage en attente de ressources externes (**LOI-2**), remise en question de l'Ã©tat local (**LOI-3**), dÃ©pendance au temps global (**LOI-4**), consommation excessive de ressources (**LOI-5**).

## 2. PortÃ©e / Scope

Ce document couvre :
- Les violations d'invariants (ce que BB ne doit jamais faire)
- Les anti-patterns architecturaux (structures interdites)
- Les anti-patterns comportementaux (comportements interdits)
- Les anti-patterns d'intÃ©gration (interactions interdites)
- Les mÃ©canismes de dÃ©tection

Ce document **ne couvre pas** :
- Les erreurs de traduction (voir [Error & Rejection Model](../error/BondingBrother%20-%20Error%20&%20Rejection%20Model.md))
- Les cas d'erreur lÃ©gitimes (voir [Error & Rejection Model](../error/BondingBrother%20-%20Error%20&%20Rejection%20Model.md))
- Les dÃ©tails d'implÃ©mentation des vÃ©rifications

---

## 3. Principe fondamental

**Toute violation listÃ©e ici est une faute critique qui remet en question la nature mÃªme de Bonding Brother.**

Ces violations ne sont pas des erreurs Ã  gÃ©rer, mais des impossibilitÃ©s structurelles. Si une violation est dÃ©tectÃ©e, c'est un dÃ©faut de conception ou d'implÃ©mentation qui doit Ãªtre corrigÃ© immÃ©diatement.

---

## 4. Violations de nature (ce que Bonding Brother ne peut pas Ãªtre)

### 4.1 VIOL-NAT-01 : Devenir une autoritÃ©

**Violation :** Bonding Brother dÃ©tient une vÃ©ritÃ©, prend une dÃ©cision finale, ou dÃ©finit une rÃ¨gle.

**Exemples de violation :**
- Un composant stocke l'Ã©tat des permissions et dÃ©cide de l'autorisation
- Un composant crÃ©e une rÃ¨gle mÃ©tier dynamiquement
- Un composant dÃ©tient une copie de donnÃ©es mÃ©tier comme source de vÃ©ritÃ©

**DÃ©tection :**
- Recherche de mÃ©thodes `decide()`, `rule()`, `store_truth()` dans le code
- Audit des structures de donnÃ©es : aucune ne doit reprÃ©senter un "Ã©tat courant" mÃ©tier
- VÃ©rification qu'aucun composant ne prend de dÃ©cision basÃ©e sur des critÃ¨res mÃ©tier

**Correction :**
- DÃ©lÃ©guer toute dÃ©cision Ã  Strong Father ou Kind Mother
- Supprimer tout stockage de vÃ©ritÃ© mÃ©tier
- Transmettre les rÃ¨gles depuis les autoritÃ©s, ne jamais les crÃ©er

---

### 4.2 VIOL-NAT-02 : ExÃ©cuter des actions mÃ©tier

**Violation :** Bonding Brother modifie, crÃ©e, ou supprime des entitÃ©s mÃ©tier directement.

**Exemples de violation :**
- Un composant Ã©crit directement dans une base de donnÃ©es mÃ©tier
- Un composant crÃ©e un contenu sans passer par Kind Mother
- Un composant supprime une ressource sans dÃ©lÃ©gation

**DÃ©tection :**
- Recherche d'appels directs Ã  des bases de donnÃ©es mÃ©tier
- VÃ©rification que tous les accÃ¨s aux donnÃ©es passent par les adaptateurs d'autoritÃ©
- Audit des opÃ©rations CRUD : toutes doivent Ãªtre dÃ©lÃ©guÃ©es

**Correction :**
- Toute action mÃ©tier doit Ãªtre traduite en intention et dÃ©lÃ©guÃ©e Ã  une autoritÃ©
- Supprimer tout accÃ¨s direct aux donnÃ©es mÃ©tier

---

### 4.3 VIOL-NAT-03 : ÃŠtre source d'information

**Violation :** Bonding Brother gÃ©nÃ¨re ou fabrique des donnÃ©es sans source autoritaire.

**Exemples de violation :**
- Un composant gÃ©nÃ¨re des identifiants de ressources mÃ©tier
- Un composant fabrique une rÃ©ponse sans avoir reÃ§u de rÃ©ponse d'autoritÃ©
- Un composant crÃ©e des donnÃ©es de synthÃ¨se non demandÃ©es par une autoritÃ©

**DÃ©tection :**
- TraÃ§abilitÃ© complÃ¨te : toute donnÃ©e sortante doit avoir une source (autoritÃ©) identifiÃ©e
- VÃ©rification qu'aucun composant ne gÃ©nÃ¨re de donnÃ©es mÃ©tier
- Audit des rÃ©ponses : toutes doivent provenir d'une autoritÃ©

**Correction :**
- Toute donnÃ©e doit provenir d'une autoritÃ© ou Ãªtre une mÃ©tadonnÃ©e technique
- Supprimer toute gÃ©nÃ©ration de donnÃ©es mÃ©tier

---

## 5. Violations de non-action (ce que Bonding Brother ne doit jamais faire)

### 5.1 VIOL-NEG-01 : Prendre une dÃ©cision mÃ©tier

**Violation :** Bonding Brother prend une dÃ©cision stratÃ©gique, politique, ou opÃ©rationnelle.

**Exemples de violation :**
- Un composant autorise ou refuse un accÃ¨s sans consulter Strong Father
- Un composant valide une donnÃ©e mÃ©tier sans consulter Kind Mother
- Un composant choisit entre plusieurs options mÃ©tier
- Un composant dÃ©finit une prioritÃ© mÃ©tier

**DÃ©tection :**
- Analyse de code : aucune logique conditionnelle basÃ©e sur des critÃ¨res mÃ©tier
- VÃ©rification que toutes les dÃ©cisions sont dÃ©lÃ©guÃ©es
- Audit des branches conditionnelles : seules les dÃ©cisions techniques sont autorisÃ©es

**Correction :**
- DÃ©lÃ©guer toute dÃ©cision mÃ©tier Ã  l'autoritÃ© appropriÃ©e
- Transformer les conditions mÃ©tier en dÃ©lÃ©gations

---

### 5.2 VIOL-NEG-02 : Stocker la vÃ©ritÃ©

**Violation :** Bonding Brother stocke l'Ã©tat des donnÃ©es, des identitÃ©s, ou des permissions.

**Exemples de violation :**
- Un composant maintient un cache de donnÃ©es mÃ©tier
- Un composant stocke les permissions des utilisateurs
- Un composant rÃ©plique l'Ã©tat d'une autoritÃ©
- Un composant garde une copie de donnÃ©es "pour performance"

**DÃ©tection :**
- Audit des structures de donnÃ©es : aucune ne doit reprÃ©senter un Ã©tat mÃ©tier
- Recherche de caches de donnÃ©es mÃ©tier
- VÃ©rification que seuls le journal et le buffer offline sont utilisÃ©s pour stockage

**Correction :**
- Supprimer tout cache de donnÃ©es mÃ©tier
- Utiliser uniquement le journal (immuable) et le buffer offline (temporaire)
- Toujours interroger l'autoritÃ© pour obtenir la vÃ©ritÃ©

---

### 5.3 VIOL-NEG-03 : CrÃ©er une rÃ¨gle

**Violation :** Bonding Brother crÃ©e, modifie, ou supprime une rÃ¨gle.

**Exemples de violation :**
- Un composant dÃ©finit dynamiquement qui peut accÃ©der Ã  quoi
- Un composant crÃ©e une rÃ¨gle de validation
- Un composant modifie une rÃ¨gle de filtrage sans source externe

**DÃ©tection :**
- VÃ©rification que toutes les rÃ¨gles sont chargÃ©es depuis une source externe
- Audit des rÃ¨gles : aucune ne doit Ãªtre gÃ©nÃ©rÃ©e par le code
- Recherche de mÃ©thodes `create_rule()`, `modify_rule()`

**Correction :**
- Charger toutes les rÃ¨gles depuis les autoritÃ©s ou la configuration
- Supprimer toute gÃ©nÃ©ration de rÃ¨gles

---

### 5.4 VIOL-NEG-04 : Permettre le contournement d'autoritÃ©

**Violation :** Bonding Brother permet Ã  un produit d'accÃ©der directement aux autoritÃ©s.

**Exemples de violation :**
- Exposition d'une API directe vers Kind Mother ou Strong Father
- Mode "bypass" qui contourne Bonding Brother
- Endpoint qui permet d'appeler une autoritÃ© sans passer par BB

**DÃ©tection :**
- Analyse rÃ©seau : les autoritÃ©s ne doivent Ãªtre accessibles que via BB
- Audit des API exposÃ©es : aucune ne doit pointer vers une autoritÃ©
- VÃ©rification qu'aucun produit ne peut contourner BB

**Correction :**
- Supprimer toute API directe vers les autoritÃ©s
- Forcer tous les accÃ¨s Ã  passer par Bonding Brother

---

### 5.5 VIOL-NEG-05 : Modifier une dÃ©cision d'autoritÃ©

**Violation :** Bonding Brother modifie, interprÃ¨te, ou remplace une dÃ©cision d'autoritÃ©.

**Exemples de violation :**
- Transformer un "refusÃ©" en "acceptÃ©"
- Ajouter des permissions non accordÃ©es par Strong Father
- Supprimer des restrictions imposÃ©es par Kind Mother
- InterprÃ©ter une dÃ©cision de maniÃ¨re diffÃ©rente

**DÃ©tection :**
- Comparaison automatisÃ©e entre dÃ©cision reÃ§ue et dÃ©cision transmise
- VÃ©rification que la traduction ne modifie pas le sens de la dÃ©cision
- Audit des filtrages : ils ne doivent pas modifier la dÃ©cision

**Correction :**
- Transmettre fidÃ¨lement toute dÃ©cision sans modification
- SÃ©parer clairement traduction (format) et dÃ©cision (sens)

---

### 5.6 VIOL-NEG-06 : Cacher l'origine d'une intention

**Violation :** Bonding Brother masque, modifie, ou omet l'origine d'une intention aux autoritÃ©s.

**Exemples de violation :**
- Ne pas transmettre l'identitÃ© du produit
- Modifier l'identitÃ© de l'utilisateur
- Oublier des Ã©lÃ©ments du contexte
- Anonymiser les intentions

**DÃ©tection :**
- Audit des demandes transmises : toutes doivent contenir le contexte complet
- VÃ©rification que l'origine est toujours prÃ©sente
- Tests de traÃ§abilitÃ© : l'origine doit Ãªtre traÃ§able

**Correction :**
- Transmettre intÃ©gralement le contexte Ã  chaque demande
- Ne jamais filtrer ou modifier l'origine

---

## 6. Anti-patterns architecturaux

### 6.1 ANTI-ARCH-01 : Couche qui saute une Ã©tape

**Anti-pattern :** Une couche accÃ¨de directement Ã  une couche non adjacente.

**Exemple :**
- La Couche Produit accÃ¨de directement Ã  la Couche AutoritÃ©
- La Couche Traduction accÃ¨de directement au journal

**DÃ©tection :**
- Analyse des dÃ©pendances entre composants
- VÃ©rification que chaque couche n'accÃ¨de qu'aux couches adjacentes

**Correction :**
- RÃ©organiser les appels pour respecter l'ordre des couches
- Utiliser les interfaces des couches adjacentes

---

### 6.2 ANTI-ARCH-02 : Partage d'Ã©tat entre couches

**Anti-pattern :** Deux couches partagent un Ã©tat mutable.

**Exemple :**
- La Couche Produit et la Couche MÃ©diation partagent un cache
- La Couche Traduction modifie un Ã©tat global

**DÃ©tection :**
- Audit des structures de donnÃ©es partagÃ©es
- VÃ©rification que chaque couche a son propre Ã©tat isolÃ©

**Correction :**
- Isoler l'Ã©tat de chaque couche
- Utiliser des interfaces immutables pour la communication

---

### 6.3 ANTI-ARCH-03 : DÃ©pendance circulaire

**Anti-pattern :** Deux composants dÃ©pendent l'un de l'autre.

**Exemple :**
- ProductGateway dÃ©pend de FilterEngine, qui dÃ©pend de ProductGateway
- IntentTranslator dÃ©pend de ResponseTranslator, qui dÃ©pend de IntentTranslator

**DÃ©tection :**
- Analyse des dÃ©pendances : dÃ©tection de cycles
- VÃ©rification de la structure acyclique

**Correction :**
- RÃ©organiser les dÃ©pendances pour Ã©liminer les cycles
- Introduire une abstraction commune si nÃ©cessaire

---

### 6.4 ANTI-ARCH-04 : Composant avec responsabilitÃ©s multiples

**Anti-pattern :** Un composant assume plusieurs responsabilitÃ©s non liÃ©es.

**Exemple :**
- Un composant traduit ET filtre ET journalise
- Un composant reÃ§oit les intentions ET prend des dÃ©cisions

**DÃ©tection :**
- Analyse de la responsabilitÃ© unique de chaque composant
- VÃ©rification qu'aucun composant ne fait trop de choses

**Correction :**
- SÃ©parer les responsabilitÃ©s en composants distincts
- Respecter le principe de responsabilitÃ© unique

---

## 7. Anti-patterns comportementaux

### 7.1 ANTI-COMP-01 : Traduction avec effet de bord

**Anti-pattern :** La traduction modifie un Ã©tat ou appelle une autoritÃ©.

**Exemple :**
- La traduction met Ã  jour un cache
- La traduction interroge Strong Father pour enrichir les donnÃ©es

**DÃ©tection :**
- VÃ©rification que les fonctions de traduction sont pures
- Tests unitaires : mÃªme entrÃ©e = mÃªme sortie

**Correction :**
- Rendre la traduction pure (sans effet de bord)
- DÃ©placer les effets de bord vers d'autres composants

---

### 7.2 ANTI-COMP-02 : Filtrage qui dÃ©cide

**Anti-pattern :** Le filtrage prend une dÃ©cision mÃ©tier au lieu d'appliquer une rÃ¨gle.

**Exemple :**
- Le filtrage autorise ou refuse sans consulter une autoritÃ©
- Le filtrage valide des donnÃ©es mÃ©tier

**DÃ©tection :**
- VÃ©rification que le filtrage applique uniquement des rÃ¨gles dÃ©finies
- Audit des rÃ¨gles de filtrage : aucune ne doit Ãªtre une dÃ©cision

**Correction :**
- Le filtrage applique des rÃ¨gles, ne prend pas de dÃ©cision
- DÃ©lÃ©guer les dÃ©cisions aux autoritÃ©s

---

### 7.3 ANTI-COMP-03 : Journalisation sÃ©lective

**Anti-pattern :** Certaines interactions ne sont pas journalisÃ©es.

**Exemple :**
- Les erreurs ne sont pas journalisÃ©es
- Certains types d'intentions sont omis du journal

**DÃ©tection :**
- VÃ©rification que toutes les interactions sont journalisÃ©es
- Tests de couverture : 100% des interactions doivent Ãªtre tracÃ©es

**Correction :**
- Journaliser systÃ©matiquement toutes les interactions
- Aucune exception Ã  la journalisation

---

### 7.4 ANTI-COMP-04 : Retry avec modification

**Anti-pattern :** Un retry modifie l'intention au lieu de rÃ©essayer l'identique.

**Exemple :**
- Un retry change le contexte
- Un retry modifie le payload

**DÃ©tection :**
- VÃ©rification que les retries sont identiques Ã  l'intention originale
- Tests de retry : l'intention doit Ãªtre prÃ©servÃ©e

**Correction :**
- Les retries doivent Ãªtre identiques Ã  l'intention originale
- Ne jamais modifier une intention lors d'un retry

---

## 8. Anti-patterns d'intÃ©gration

### 8.1 ANTI-INT-01 : Adaptation bidirectionnelle

**Anti-pattern :** Bonding Brother s'adapte aux produits au lieu de l'inverse.

**Exemple :**
- BB modifie son interface pour un produit spÃ©cifique
- BB supporte un format propriÃ©taire d'un produit

**DÃ©tection :**
- VÃ©rification que l'interface de BB est stable
- Audit des adaptations : aucune ne doit Ãªtre spÃ©cifique Ã  un produit

**Correction :**
- Les produits s'adaptent Ã  BB, jamais l'inverse
- L'interface de BB reste stable

---

### 8.2 ANTI-INT-02 : Cache d'autoritÃ©

**Anti-pattern :** Bonding Brother maintient un cache des rÃ©ponses d'autoritÃ©.

**Exemple :**
- Cache des permissions pour "performance"
- Cache des donnÃ©es pour Ã©viter les appels rÃ©pÃ©tÃ©s

**DÃ©tection :**
- Recherche de caches d'autoritÃ©
- VÃ©rification qu'aucun cache ne stocke de vÃ©ritÃ©

**Correction :**
- Supprimer tout cache d'autoritÃ©
- Toujours interroger l'autoritÃ© pour la vÃ©ritÃ©

---

### 8.3 ANTI-INT-03 : Aggregation de rÃ©ponses

**Anti-pattern :** Bonding Brother agrÃ¨ge des rÃ©ponses de plusieurs autoritÃ©s pour crÃ©er une rÃ©ponse composite.

**Exemple :**
- BB combine une rÃ©ponse de Kind Mother et Strong Father
- BB synthÃ©tise des donnÃ©es de plusieurs sources

**DÃ©tection :**
- VÃ©rification qu'aucune rÃ©ponse n'est agrÃ©gÃ©e
- Audit des rÃ©ponses : chaque rÃ©ponse doit Ãªtre transmise individuellement

**Correction :**
- Transmettre chaque rÃ©ponse individuellement
- Ne jamais agrÃ©ger les rÃ©ponses d'autoritÃ©s

---

## 9. MÃ©canismes de dÃ©tection

### 9.1 DÃ©tection statique (au build)

**Outils :**
- Analyse statique de code (dÃ©tection de patterns interdits)
- VÃ©rification des dÃ©pendances (dÃ©tection de cycles)
- Audit des structures de donnÃ©es (dÃ©tection de stockage de vÃ©ritÃ©)

**FrÃ©quence :** Ã€ chaque build / CI

### 9.2 DÃ©tection dynamique (au runtime)

**Outils :**
- Comparaison dÃ©cision reÃ§ue / transmise
- VÃ©rification de traÃ§abilitÃ© (origine toujours prÃ©sente)
- Monitoring des violations (alertes en temps rÃ©el)

**FrÃ©quence :** Temps rÃ©el

### 9.3 DÃ©tection par audit

**Outils :**
- Revue architecturale pÃ©riodique
- Audit de sÃ©curitÃ©
- Tests de non-rÃ©gression

**FrÃ©quence :** Mensuel / Ã  chaque release

---

## 10. Processus de correction

### 10.1 DÃ©tection d'une violation

**Action immÃ©diate :**
1. ArrÃªter le traitement si la violation est critique
2. Journaliser la violation avec tous les dÃ©tails
3. Notifier les administrateurs

### 10.2 Analyse de la violation

**Ã‰tapes :**
1. Identifier la cause racine
2. Ã‰valuer l'impact (donnÃ©es affectÃ©es, produits impactÃ©s)
3. DÃ©terminer la correction nÃ©cessaire

### 10.3 Correction

**Processus :**
1. Corriger le code / l'architecture
2. Ajouter des tests pour prÃ©venir la rÃ©currence
3. VÃ©rifier que la correction n'introduit pas d'autres violations
4. DÃ©ployer la correction

### 10.4 PrÃ©vention

**Actions :**
1. Mettre Ã  jour ce document si une nouvelle violation est dÃ©couverte
2. Ajouter des tests de dÃ©tection
3. Documenter la leÃ§on apprise

---

## 11. Liste de vÃ©rification

Cette liste peut Ãªtre utilisÃ©e lors des revues de code et d'architecture :

- [ ] Aucun composant ne dÃ©tient de vÃ©ritÃ© mÃ©tier
- [ ] Aucun composant ne prend de dÃ©cision mÃ©tier
- [ ] Aucun composant ne crÃ©e de rÃ¨gle
- [ ] Aucun cache de donnÃ©es mÃ©tier n'existe
- [ ] Toutes les dÃ©cisions sont dÃ©lÃ©guÃ©es aux autoritÃ©s
- [ ] Toutes les interactions sont journalisÃ©es
- [ ] Aucune couche n'accÃ¨de Ã  une couche non adjacente
- [ ] Aucune dÃ©pendance circulaire n'existe
- [ ] La traduction est pure (sans effet de bord)
- [ ] Le filtrage applique des rÃ¨gles, ne dÃ©cide pas
- [ ] Aucune API directe vers les autoritÃ©s n'est exposÃ©e
- [ ] Les dÃ©cisions d'autoritÃ© sont transmises fidÃ¨lement
- [ ] L'origine des intentions est toujours transmise
- [ ] Aucune agrÃ©gation de rÃ©ponses d'autoritÃ©s

---

## 12. Statut contractuel

Ce document est **contractuel, normatif, et de statut INTERDICTION**. Il Ã©tablit les violations et anti-patterns que Bonding Brother ne doit jamais commettre, sous peine de remettre en question sa nature mÃªme.

Toute violation dÃ©tectÃ©e est un dÃ©faut critique qui doit Ãªtre corrigÃ© immÃ©diatement. Toute implÃ©mentation de Bonding Brother doit Ãªtre vÃ©rifiÃ©e contre cette liste.

---

**Version :** 2.0  
**Date :** 2026-01-28  
**Statut :** INTERDICTION â€” Non nÃ©gociable  
**DÃ©pendances :** 
- [Documentation Fondatrice v1.0](../../foundation/BondingBrother%20-%20Documentation%20Fondatrice.md) (Section 10)
- [Invariants & Guarantees v1.0](./BondingBrother%20-%20Invariants%20&%20Guarantees.md)
- [Error & Rejection Model v1.0](../error/BondingBrother%20-%20Error%20&%20Rejection%20Model.md)
- [Lois d'Autonomie SystÃ¨me](..//..//..//..//miyukini-webway-system//reference//_index.md)

