# BondingBrother - FAQ & Common Questions

## 1. Contexte

Ce document rÃ©pond aux questions frÃ©quentes sur Bonding Brother, basÃ©es sur l'ensemble de la documentation contractuelle et conceptuelle. Il sert de point d'entrÃ©e pour les dÃ©veloppeurs, architectes, et utilisateurs cherchant des rÃ©ponses rapides aux questions courantes.

Ce document s'appuie sur l'ensemble de la documentation Bonding Brother pour fournir des rÃ©ponses prÃ©cises et cohÃ©rentes.

Les rÃ©ponses tiennent compte des [Lois d'Autonomie SystÃ¨me](..//..//..//miyukini-webway-system//reference//_index.md) qui garantissent le fonctionnement autonome du systÃ¨me.

**Navigation :** [Index BondingBrother](../_index.md)

## 2. PortÃ©e / Scope

Ce document couvre :
- Les questions frÃ©quentes sur les concepts fondamentaux
- Les questions sur l'utilisation pratique
- Les questions sur les erreurs et leur rÃ©solution
- Les questions sur l'intÃ©gration
- Les questions sur les performances
- Les questions sur le mode offline

Ce document **ne remplace pas** :
- Les contrats normatifs (voir les documents contractuels)
- La documentation technique dÃ©taillÃ©e
- Les guides d'implÃ©mentation

---

## 3. Questions fondamentales

### Q1 : Qu'est-ce que Bonding Brother exactement ?

**R :** Bonding Brother est l'interface fraternelle standard qui relie les produits autonomes Ã  l'Ã©cosystÃ¨me autoritaire. Il traduit les intentions des produits en demandes pour les autoritÃ©s (Kind Mother et Strong Father), et traduit les rÃ©ponses des autoritÃ©s en rÃ©sultats pour les produits. Il est le seul chemin autorisÃ© entre les produits et les autoritÃ©s.

**RÃ©fÃ©rence :** [Documentation Fondatrice - Section 1 et 12](../foundation/BondingBrother%20-%20Documentation%20Fondatrice.md)

---

### Q2 : Pourquoi Bonding Brother existe-t-il ?

**R :** Bonding Brother existe pour isoler les produits de la complexitÃ© des autoritÃ©s tout en garantissant que chaque interaction respecte les rÃ¨gles de l'Ã©cosystÃ¨me. Sans Bonding Brother, chaque produit devrait connaÃ®tre les dÃ©tails internes de Kind Mother et Strong Father, crÃ©ant des dÃ©pendances fragiles et des violations architecturales.

**RÃ©fÃ©rence :** [Documentation Fondatrice - Section 1](../foundation/BondingBrother%20-%20Documentation%20Fondatrice.md)

---

### Q3 : Bonding Brother est-il une autoritÃ© ?

**R :** Non, absolument pas. Bonding Brother est un mÃ©diateur, pas une autoritÃ©. Il ne dÃ©cide jamais, ne crÃ©e jamais de rÃ¨gle, ne dÃ©tient jamais de vÃ©ritÃ©. Toute dÃ©cision appartient aux autoritÃ©s (KindMother pour les donnÃ©es, StrongFather pour les dÃ©cisions stratÃ©giques et politiques).

**RÃ©fÃ©rence :** [Documentation Fondatrice - Section 6](../foundation/BondingBrother%20-%20Documentation%20Fondatrice.md), [Invariants & Guarantees - INV-NAT-01](../contracts/governance/BondingBrother%20-%20Invariants%20%26%20Guarantees.md)

---

### Q4 : Quelle est la diffÃ©rence entre une intention et une commande ?

**R :** Une intention est une dÃ©claration de volontÃ©, pas une instruction d'exÃ©cution. Les produits expriment ce qu'ils souhaitent faire, pas ce qu'ils ordonnent. L'Ã©valuation et la dÃ©cision appartiennent exclusivement aux autoritÃ©s. Une commande serait une instruction directe, ce que Bonding Brother refuse structurellement.

**RÃ©fÃ©rence :** [Documentation Fondatrice - Section 4](../foundation/BondingBrother%20-%20Documentation%20Fondatrice.md), [Intent Model Contract - Section 3](../contracts/intent/BondingBrother%20-%20Intent%20Model%20Contract.md)

---

## 4. Questions sur l'utilisation

### Q5 : Comment soumettre une intention ?

**R :** Utilisez l'interface `IIntentSubmission` via `POST /api/v1/intentions`. L'intention doit respecter le schÃ©ma dÃ©fini dans l'Intent Model Contract, avec les champs obligatoires : `produit_id`, `type`, `payload`, `contexte`, `timestamp`, `version`.

**Exemple :**
```json
POST /api/v1/intentions
{
  "produit_id": "miyukini-cms",
  "type": "CREATE_CONTENT",
  "payload": { ... },
  "contexte": { ... },
  "timestamp": "2026-01-26T10:00:00Z",
  "version": "1.0.0"
}
```

**RÃ©fÃ©rence :** [Product Interface Contract - Section 4](../contracts/product/BondingBrother%20-%20Product%20Interface%20Contract.md), [Examples & Use Cases - Section 4.1](./BondingBrother%20-%20Examples%20%26%20Use%20Cases.md)

---

### Q6 : Comment recevoir les rÃ©sultats ?

**R :** Trois mÃ©canismes sont disponibles :
1. **Callback (recommandÃ©)** : Fournissez une URL de callback lors de la soumission. Bonding Brother appellera cette URL avec le rÃ©sultat.
2. **Polling** : Interrogez rÃ©guliÃ¨rement `GET /api/v1/intentions/{intention_id}/result`.
3. **Webhook** : Abonnez-vous Ã  un webhook pour recevoir les rÃ©sultats.

**RÃ©fÃ©rence :** [Product Interface Contract - Section 5](../contracts/product/BondingBrother%20-%20Product%20Interface%20Contract.md)

---

### Q7 : Comment s'abonner aux notifications ?

**R :** Utilisez l'interface `INotificationSubscription` via `POST /api/v1/notifications/subscribe`. SpÃ©cifiez les types de notifications souhaitÃ©s et votre URL de callback.

**Exemple :**
```json
POST /api/v1/notifications/subscribe
{
  "produit_id": "miyukini-cms",
  "types": ["CONTENT_CREATED", "CONTENT_UPDATED"],
  "callback_url": "https://produit.example.com/notifications"
}
```

**RÃ©fÃ©rence :** [Product Interface Contract - Section 6](../contracts/product/BondingBrother%20-%20Product%20Interface%20Contract.md)

---

### Q8 : Quels types d'intentions sont supportÃ©s ?

**R :** Les types d'intentions sont organisÃ©s par domaine d'autoritÃ© :
- **DonnÃ©es** (Kind Mother) : `CREATE_CONTENT`, `UPDATE_CONTENT`, `DELETE_CONTENT`, `READ_CONTENT`, `QUERY_CONTENT`
- **HiÃ©rarchie** (Kind Mother) : `CREATE_NODE`, `MOVE_NODE`, `DELETE_NODE`
- **IdentitÃ©** (Strong Father) : `AUTHENTICATE`, `AUTHORIZE`, `CREATE_SESSION`, `REVOKE_SESSION`

**RÃ©fÃ©rence :** [Intent Model Contract - Section 6](../contracts/intent/BondingBrother%20-%20Intent%20Model%20Contract.md)

---

### Q9 : Comment savoir quelle autoritÃ© est concernÃ©e par mon intention ?

**R :** Le routage est automatique selon le type d'intention :
- Intentions de donnÃ©es/hiÃ©rarchie â†’ Kind Mother
- Intentions d'identitÃ©/session â†’ Strong Father

Vous n'avez pas besoin de spÃ©cifier l'autoritÃ©, Bonding Brother route automatiquement.

**RÃ©fÃ©rence :** [Product-to-Ecosystem Flow - Section 5.6](../contracts/flows/BondingBrother%20-%20Product-to-Ecosystem%20Flow.md)

---

## 5. Questions sur les erreurs

### Q10 : Mon intention a Ã©tÃ© rejetÃ©e avec le code VAL-002. Que signifie cela ?

**R :** `VAL-002` signifie qu'un champ obligatoire est manquant dans votre intention. VÃ©rifiez que tous les champs marquÃ©s comme obligatoires sont prÃ©sents : `produit_id`, `type`, `payload`, `contexte`, `timestamp`, `version`.

**RÃ©fÃ©rence :** [Error & Rejection Model - Section 4.2](../contracts/error/BondingBrother%20-%20Error%20%26%20Rejection%20Model.md)

---

### Q11 : Mon intention a Ã©tÃ© rejetÃ©e avec le code TRAD-001. Que faire ?

**R :** `TRAD-001` signifie qu'aucun mapping n'existe pour votre type d'intention vers l'autoritÃ© cible. VÃ©rifiez que vous utilisez un type d'intention canonique supportÃ©. Si vous avez besoin d'un nouveau type, contactez l'Ã©quipe d'architecture.

**RÃ©fÃ©rence :** [Error & Rejection Model - Section 4.2](../contracts/error/BondingBrother%20-%20Error%20%26%20Rejection%20Model.md), [Translation Contract - Section 10](../contracts/intent/BondingBrother%20-%20Translation%20Contract.md)

---

### Q12 : Mon intention a Ã©tÃ© refusÃ©e par l'autoritÃ© (AUTH-001). Pourquoi ?

**R :** `AUTH-001` signifie que l'autoritÃ© a explicitement refusÃ© votre demande. Les raisons possibles incluent :
- Permissions insuffisantes
- DonnÃ©es invalides
- RÃ¨gles mÃ©tier non respectÃ©es
- Ressource verrouillÃ©e

Consultez le message d'erreur pour plus de dÃ©tails. La dÃ©cision vient de l'autoritÃ©, pas de Bonding Brother.

**RÃ©fÃ©rence :** [Error & Rejection Model - Section 4.2](../contracts/error/BondingBrother%20-%20Error%20%26%20Rejection%20Model.md)

---

### Q13 : J'ai reÃ§u un timeout (TIMEOUT-002). Que faire ?

**R :** `TIMEOUT-002` signifie que l'autoritÃ© n'a pas rÃ©pondu dans le dÃ©lai imparti. Vous pouvez :
1. RÃ©essayer l'intention (si elle est toujours valide)
2. VÃ©rifier l'Ã©tat de l'autoritÃ©
3. Contacter le support si le problÃ¨me persiste

**RÃ©fÃ©rence :** [Error & Rejection Model - Section 11](../contracts/error/BondingBrother%20-%20Error%20%26%20Rejection%20Model.md)

---

### Q14 : Quelle est la diffÃ©rence entre un rejet et une erreur ?

**R :** 
- **Rejet** : L'intention est rejetÃ©e par Bonding Brother avant transmission Ã  l'autoritÃ© (validation, traduction, filtrage Ã©chouÃ©s). Pas de retry automatique.
- **Erreur** : L'intention a Ã©tÃ© transmise Ã  l'autoritÃ©, mais l'autoritÃ© a refusÃ© ou une erreur technique s'est produite. Retry possible selon le type d'erreur.

**RÃ©fÃ©rence :** [Error & Rejection Model - Section 9](../contracts/error/BondingBrother%20-%20Error%20%26%20Rejection%20Model.md)

---

## 6. Questions sur le mode offline

### Q15 : Que se passe-t-il si je soumets une intention en mode offline ?

**R :** Bonding Brother continue de fonctionner normalement :
1. L'intention est reÃ§ue et validÃ©e
2. L'intention est traduite et journalisÃ©e
3. L'intention est mise en buffer pour transmission diffÃ©rÃ©e
4. Lors de la reconnexion, toutes les intentions en buffer sont transmises
5. Les rÃ©sultats sont transmis au produit de maniÃ¨re diffÃ©rÃ©e

**RÃ©fÃ©rence :** [Offline & Deferred Authority Contract](../contracts/offline/BondingBrother%20-%20Offline%20%26%20Deferred%20Authority%20Contract.md)

---

### Q16 : Comment savoir si Bonding Brother est en mode offline ?

**R :** Bonding Brother notifie les produits du passage en mode offline via une notification systÃ¨me. Vous pouvez Ã©galement interroger le statut via l'API.

**RÃ©fÃ©rence :** [Offline & Deferred Authority Contract - Section 5.1](../contracts/offline/BondingBrother%20-%20Offline%20%26%20Deferred%20Authority%20Contract.md)

---

### Q17 : Les intentions en mode offline sont-elles perdues ?

**R :** Non, jamais. Toutes les intentions sont journalisÃ©es de maniÃ¨re persistante avant d'Ãªtre mises en buffer. MÃªme en cas de redÃ©marrage, les intentions sont prÃ©servÃ©es et transmises lors de la reconnexion.

**RÃ©fÃ©rence :** [Offline & Deferred Authority Contract - Section 4.3](../contracts/offline/BondingBrother%20-%20Offline%20%26%20Deferred%20Authority%20Contract.md), [Invariants & Guarantees - INV-FLUX-04](../contracts/governance/BondingBrother%20-%20Invariants%20%26%20Guarantees.md)

---

### Q18 : L'ordre des intentions est-il prÃ©servÃ© en mode offline ?

**R :** Oui, l'ordre est prÃ©servÃ©. Les intentions sont traitÃ©es dans l'ordre d'arrivÃ©e (FIFO), mÃªme aprÃ¨s reconnexion.

**RÃ©fÃ©rence :** [Invariants & Guarantees - INV-FLUX-03](../contracts/governance/BondingBrother%20-%20Invariants%20%26%20Guarantees.md), [Sync & Reconnection Contract](../contracts/offline/BondingBrother%20-%20Sync%20%26%20Reconnection%20Contract.md)

---

## 7. Questions sur la traduction

### Q19 : Comment fonctionne la traduction ?

**R :** La traduction transforme les intentions (vocabulaire produit) en demandes (vocabulaire autoritÃ©) et les rÃ©ponses (vocabulaire autoritÃ©) en rÃ©sultats (vocabulaire produit). La sÃ©mantique est prÃ©servÃ©e, seul le format est adaptÃ©.

**RÃ©fÃ©rence :** [Translation Contract](../contracts/intent/BondingBrother%20-%20Translation%20Contract.md)

---

### Q20 : Puis-je utiliser mon propre vocabulaire ?

**R :** Oui, mais vous devez fournir un mapping vers le vocabulaire canonique. Bonding Brother traduit automatiquement votre vocabulaire vers celui des autoritÃ©s. Consultez le Translation Contract pour les rÃ¨gles de mapping.

**RÃ©fÃ©rence :** [Translation Contract - Section 8](../contracts/intent/BondingBrother%20-%20Translation%20Contract.md)

---

### Q21 : La traduction peut-elle modifier le sens de mon intention ?

**R :** Non, jamais. La traduction prÃ©serve intÃ©gralement la sÃ©mantique. Seul le format est adaptÃ©. Si vous pensez qu'une traduction a modifiÃ© le sens, c'est un bug Ã  signaler.

**RÃ©fÃ©rence :** [Translation Contract - Section 5.1](../contracts/intent/BondingBrother%20-%20Translation%20Contract.md), [Invariants & Guarantees - GAR-PROD-02](../contracts/governance/BondingBrother%20-%20Invariants%20%26%20Guarantees.md)

---

## 8. Questions sur les performances

### Q22 : Quelle est la latence typique d'une intention ?

**R :** Le temps de traitement par Bonding Brother est <50ms (hors attente autoritÃ©). Le temps total dÃ©pend de l'autoritÃ© :
- Temps de validation : <10ms
- Temps de traduction : <5ms
- Temps de filtrage : <5ms
- Temps d'attente autoritÃ© : variable (100ms-5s typiquement)

**RÃ©fÃ©rence :** [Performance & Scalability Contract](../contracts/performance/BondingBrother%20-%20Performance%20%26%20Scalability%20Contract.md)

---

### Q23 : Combien d'intentions par seconde peut traiter Bonding Brother ?

**R :** Le throughput dÃ©pend de la configuration :
- Minimum garanti : 100 intentions/seconde
- Cible : 500 intentions/seconde
- Maximum : 1000 intentions/seconde (selon configuration)

**RÃ©fÃ©rence :** [Performance & Scalability Contract](../contracts/performance/BondingBrother%20-%20Performance%20%26%20Scalability%20Contract.md)

---

### Q24 : Y a-t-il une limite de taille pour les intentions ?

**R :** Oui, des limites configurables :
- Taille maximale du payload : 1 MB (par dÃ©faut)
- Taille maximale du contexte : 100 KB (par dÃ©faut)

**RÃ©fÃ©rence :** [Intent Model Contract - Section 11](../contracts/intent/BondingBrother%20-%20Intent%20Model%20Contract.md)

---

## 9. Questions sur la sÃ©curitÃ©

### Q25 : Comment Bonding Brother garantit-il l'isolation des produits ?

**R :** Bonding Brother garantit l'isolation par :
- Filtrage strict des rÃ©sultats (chaque produit ne reÃ§oit que ses informations)
- Isolation des contextes (pas de mÃ©lange entre produits)
- Validation d'authentification (via Strong Father)
- TraÃ§abilitÃ© complÃ¨te

**RÃ©fÃ©rence :** [Security & Threat Model Contract](../contracts/security/BondingBrother%20-%20Security%20%26%20Threat%20Model%20Contract.md), [Invariants & Guarantees - GAR-PROD-03](../contracts/governance/BondingBrother%20-%20Invariants%20%26%20Guarantees.md)

---

### Q26 : Les donnÃ©es sont-elles chiffrÃ©es en transit ?

**R :** Oui, toutes les communications utilisent HTTPS en production. HTTP est autorisÃ© uniquement en dÃ©veloppement.

**RÃ©fÃ©rence :** [Product Interface Contract - Section 8.1](../contracts/product/BondingBrother%20-%20Product%20Interface%20Contract.md)

---

### Q27 : Comment l'authentification fonctionne-t-elle ?

**R :** L'authentification est gÃ©rÃ©e par Strong Father. Toutes les requÃªtes doivent inclure un token d'authentification valide dans l'en-tÃªte `Authorization: Bearer <token>`. Bonding Brother transmet ce token aux autoritÃ©s sans le valider lui-mÃªme.

**RÃ©fÃ©rence :** [Product Interface Contract - Section 8.2](../contracts/product/BondingBrother%20-%20Product%20Interface%20Contract.md), [StrongFather Integration Contract](../contracts/integration/BondingBrother%20-%20StrongFather%20Integration%20Contract.md)

---

## 10. Questions sur la traÃ§abilitÃ©

### Q28 : Comment puis-je tracer une intention ?

**R :** Chaque intention a un `intention_id` unique. Vous pouvez :
1. Conserver l'`intention_id` lors de la soumission
2. Utiliser l'API de traÃ§abilitÃ© : `GET /api/v1/intentions/{intention_id}/trace`
3. Consulter le journal d'audit (si vous avez les permissions)

**RÃ©fÃ©rence :** [Audit & Traceability Contract](../contracts/governance/BondingBrother%20-%20Audit%20%26%20Traceability%20Contract.md)

---

### Q29 : Combien de temps les journaux sont-ils conservÃ©s ?

**R :** La rÃ©tention des journaux est configurable selon les exigences de conformitÃ©. Par dÃ©faut, les journaux sont conservÃ©s pendant 90 jours, mais cela peut Ãªtre ajustÃ©.

**RÃ©fÃ©rence :** [Journaling Contract](../contracts/offline/BondingBrother%20-%20Journaling%20Contract.md)

---

### Q30 : Puis-je consulter les intentions d'autres produits ?

**R :** Non, jamais. Chaque produit ne peut consulter que ses propres intentions. L'isolation est garantie par Bonding Brother.

**RÃ©fÃ©rence :** [Invariants & Guarantees - GAR-PROD-05](../contracts/governance/BondingBrother%20-%20Invariants%20%26%20Guarantees.md), [Security & Threat Model Contract](../contracts/security/BondingBrother%20-%20Security%20%26%20Threat%20Model%20Contract.md)

---

## 11. Questions sur l'intÃ©gration

### Q31 : Comment intÃ©grer mon produit Ã  Bonding Brother ?

**R :** Suivez ces Ã©tapes :
1. ImplÃ©mentez l'interface `IIntentSubmission` pour soumettre des intentions
2. ImplÃ©mentez un endpoint de callback pour recevoir les rÃ©sultats
3. Respectez le schÃ©ma d'intention dÃ©fini dans l'Intent Model Contract
4. GÃ©rez les erreurs selon l'Error & Rejection Model
5. Consultez le Product Adaptation Rules pour les rÃ¨gles spÃ©cifiques

**RÃ©fÃ©rence :** [Product Interface Contract](../contracts/product/BondingBrother%20-%20Product%20Interface%20Contract.md), [Product Adaptation Rules](../contracts/product/BondingBrother%20-%20Product%20Adaptation%20Rules.md)

---

### Q32 : Puis-je accÃ©der directement Ã  Kind Mother ou Strong Father ?

**R :** Non, jamais. Bonding Brother est le seul chemin autorisÃ©. Toute tentative d'accÃ¨s direct est une violation architecturale et sera bloquÃ©e.

**RÃ©fÃ©rence :** [Documentation Fondatrice - Section 2](../foundation/BondingBrother%20-%20Documentation%20Fondatrice.md), [Invariants & Guarantees - INV-NEG-04](../contracts/governance/BondingBrother%20-%20Invariants%20%26%20Guarantees.md)

---

### Q33 : Comment Ã©tendre Bonding Brother avec de nouvelles fonctionnalitÃ©s ?

**R :** Bonding Brother s'Ã©tend par spÃ©cialisation, jamais par modification du cÅ“ur. Consultez l'Extension & Specialization Contract pour les rÃ¨gles d'extension.

**RÃ©fÃ©rence :** [Extension & Specialization Contract](../contracts/product/BondingBrother%20-%20Extension%20%26%20Specialization%20Contract.md), [Documentation Fondatrice - Section 7](../foundation/BondingBrother%20-%20Documentation%20Fondatrice.md)

---

## 12. Questions sur la version

### Q34 : Comment fonctionne le versionnement de l'interface ?

**R :** L'interface est versionnÃ©e selon semver :
- **Version majeure** : Changements incompatibles (migration requise)
- **Version mineure** : Nouvelles fonctionnalitÃ©s compatibles
- **Version patch** : Corrections de bugs

Les versions sont spÃ©cifiÃ©es dans l'URL : `/api/v1/...`

**RÃ©fÃ©rence :** [Product Interface Contract - Section 10](../contracts/product/BondingBrother%20-%20Product%20Interface%20Contract.md), [Versioning & Evolution Contract](../contracts/evolution/BondingBrother%20-%20Versioning%20%26%20Evolution%20Contract.md)

---

### Q35 : Que se passe-t-il si j'utilise une version obsolÃ¨te ?

**R :** Les versions obsolÃ¨tes sont progressivement dÃ©prÃ©ciÃ©es avec un prÃ©avis. AprÃ¨s la pÃ©riode de dÃ©prÃ©ciation, elles sont rejetÃ©es. Consultez le Versioning & Evolution Contract pour les dÃ©tails.

**RÃ©fÃ©rence :** [Versioning & Evolution Contract](../contracts/evolution/BondingBrother%20-%20Versioning%20%26%20Evolution%20Contract.md)

---

### Q36 : Comment migrer vers une nouvelle version majeure ?

**R :** Consultez le Migration & Compatibility Contract pour les rÃ¨gles de migration. En gÃ©nÃ©ral :
1. Consultez le guide de migration
2. Testez votre produit avec la nouvelle version
3. Migrez progressivement
4. Utilisez la pÃ©riode de support multi-versions

**RÃ©fÃ©rence :** [Migration & Compatibility Contract](../contracts/evolution/BondingBrother%20-%20Migration%20%26%20Compatibility%20Contract.md)

---

## 13. Questions sur les garanties

### Q37 : Quelles garanties Bonding Brother offre-t-il ?

**R :** Bonding Brother garantit :
- **Interface stable** : Pas de breaking changes entre versions mineures
- **Traduction fidÃ¨le** : SÃ©mantique prÃ©servÃ©e
- **RÃ©sultat filtrÃ© et sÃ»r** : Seulement les informations autorisÃ©es
- **Transparence des erreurs** : Messages clairs et actionnables
- **TraÃ§abilitÃ© accessible** : Historique de vos interactions

**RÃ©fÃ©rence :** [Invariants & Guarantees - Section 7](../contracts/governance/BondingBrother%20-%20Invariants%20%26%20Guarantees.md)

---

### Q38 : Que se passe-t-il si Bonding Brother ne respecte pas une garantie ?

**R :** Toute violation de garantie est considÃ©rÃ©e comme un dÃ©faut critique. Signalez-la immÃ©diatement. Les garanties sont contractuelles et doivent Ãªtre respectÃ©es en toutes circonstances.

**RÃ©fÃ©rence :** [Invariants & Guarantees - Section 11](../contracts/governance/BondingBrother%20-%20Invariants%20%26%20Guarantees.md)

---

## 14. Questions sur les tests

### Q39 : Comment tester mon intÃ©gration avec Bonding Brother ?

**R :** Utilisez l'environnement de test fourni. Vous pouvez :
1. Soumettre des intentions de test
2. VÃ©rifier les rÃ©sultats
3. Tester les scÃ©narios d'erreur
4. Tester le mode offline

Consultez le Testing & Validation Contract pour les stratÃ©gies de test.

**RÃ©fÃ©rence :** [Testing & Validation Contract](../contracts/testing/BondingBrother%20-%20Testing%20%26%20Validation%20Contract.md)

---

### Q40 : Y a-t-il des exemples de code pour tester ?

**R :** Oui, consultez le document Examples & Use Cases pour des exemples complets de flux et de cas d'usage.

**RÃ©fÃ©rence :** [Examples & Use Cases](./BondingBrother%20-%20Examples%20%26%20Use%20Cases.md)

---

## 15. Questions gÃ©nÃ©rales

### Q41 : OÃ¹ puis-je trouver plus d'informations ?

**R :** Consultez la documentation complÃ¨te :
- **Documentation Fondatrice** : Concepts fondamentaux
- **Contrats spÃ©cifiques** : RÃ¨gles dÃ©taillÃ©es par domaine
- **Examples & Use Cases** : Exemples pratiques
- **Reference Implementation Guidelines** : Guidelines d'implÃ©mentation

**RÃ©fÃ©rence :** [Index BondingBrother](../_index.md)

---

### Q42 : Comment signaler un bug ou demander une fonctionnalitÃ© ?

**R :** Utilisez le systÃ¨me de tickets du projet. Pour les bugs critiques, contactez directement l'Ã©quipe d'architecture.

---

### Q43 : Bonding Brother est-il open source ?

**R :** Consultez la licence du projet pour les dÃ©tails. La documentation est accessible Ã  tous les dÃ©veloppeurs de l'Ã©cosystÃ¨me.

---

### Q44 : Puis-je contribuer Ã  Bonding Brother ?

**R :** Les contributions sont les bienvenues. Consultez les guidelines de contribution et respectez les contrats et invariants.

---

## 16. Questions sur les cas limites

### Q45 : Que se passe-t-il si je soumets la mÃªme intention deux fois ?

**R :** Chaque intention doit avoir un `intention_id` unique. Si vous soumettez deux intentions avec le mÃªme ID, la seconde sera rejetÃ©e. Si vous soumettez deux intentions identiques avec des IDs diffÃ©rents, elles seront traitÃ©es comme deux intentions distinctes.

**RÃ©fÃ©rence :** [Intent Model Contract - Section 4.2](../contracts/intent/BondingBrother%20-%20Intent%20Model%20Contract.md)

---

### Q46 : Puis-je annuler une intention dÃ©jÃ  soumise ?

**R :** Non, une fois soumise et acceptÃ©e, une intention ne peut pas Ãªtre annulÃ©e. Elle suivra son cycle de vie complet. Si vous avez besoin d'annuler une action, soumettez une nouvelle intention pour l'annulation (si supportÃ© par l'autoritÃ©).

**RÃ©fÃ©rence :** [Intent Model Contract - Section 7](../contracts/intent/BondingBrother%20-%20Intent%20Model%20Contract.md)

---

### Q47 : Que se passe-t-il si mon callback URL est indisponible ?

**R :** Bonding Brother retente la transmission selon une stratÃ©gie configurable. Si tous les retry Ã©chouent, le rÃ©sultat est mis en queue. Vous pouvez Ã©galement utiliser le polling pour rÃ©cupÃ©rer les rÃ©sultats manquÃ©s.

**RÃ©fÃ©rence :** [Product Interface Contract - Section 5.4](../contracts/product/BondingBrother%20-%20Product%20Interface%20Contract.md), [Ecosystem-to-Product Flow - Section 6.9](../contracts/flows/BondingBrother%20-%20Ecosystem-to-Product%20Flow.md)

---

### Q48 : Puis-je soumettre des intentions en parallÃ¨le ?

**R :** Oui, vous pouvez soumettre plusieurs intentions en parallÃ¨le. Chaque intention est traitÃ©e indÃ©pendamment. L'ordre de traitement peut varier, mais l'ordre d'arrivÃ©e est prÃ©servÃ© pour les rÃ©sultats.

**RÃ©fÃ©rence :** [Bilateral Flow Contract - Section 7](../contracts/flows/BondingBrother%20-%20Bilateral%20Flow%20Contract.md)

---

## 17. Statut contractuel

Ce document est **informatif, non normatif, et de statut FAQ**. Il fournit des rÃ©ponses aux questions frÃ©quentes mais ne remplace pas les contrats.

En cas de contradiction avec un contrat, le contrat prime toujours.

---

**Version :** 2.0  
**Date :** 2026-01-28  
**Statut :** FAQ â€” Informatif  
**DÃ©pendances :** Tous les documents de la documentation Bonding Brother

