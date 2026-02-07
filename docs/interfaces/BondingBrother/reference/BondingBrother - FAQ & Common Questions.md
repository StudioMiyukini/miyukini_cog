# BondingBrother - FAQ & Common Questions

## 1. Contexte

Ce document répond aux questions fréquentes sur Bonding Brother, basées sur l'ensemble de la documentation contractuelle et conceptuelle. Il sert de point d'entrée pour les développeurs, architectes, et utilisateurs cherchant des réponses rapides aux questions courantes.

Ce document s'appuie sur l'ensemble de la documentation Bonding Brother pour fournir des réponses précises et cohérentes.

Les réponses tiennent compte des [Lois d'Autonomie Système](../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md) qui garantissent le fonctionnement autonome du système.

**Navigation :** [Index BondingBrother](../_index.md)

## 2. Portée / Scope

Ce document couvre :
- Les questions fréquentes sur les concepts fondamentaux
- Les questions sur l'utilisation pratique
- Les questions sur les erreurs et leur résolution
- Les questions sur l'intégration
- Les questions sur les performances
- Les questions sur le mode offline

Ce document **ne remplace pas** :
- Les contrats normatifs (voir les documents contractuels)
- La documentation technique détaillée
- Les guides d'implémentation

---

## 3. Questions fondamentales

### Q1 : Qu'est-ce que Bonding Brother exactement ?

**R :** Bonding Brother est l'interface fraternelle standard qui relie les produits autonomes à l'écosystème autoritaire. Il traduit les intentions des produits en demandes pour les autorités (Kind Mother et Strong Father), et traduit les réponses des autorités en résultats pour les produits. Il est le seul chemin autorisé entre les produits et les autorités.

**Référence :** [Documentation Fondatrice - Section 1 et 12](../foundation/BondingBrother%20-%20Documentation%20Fondatrice.md)

---

### Q2 : Pourquoi Bonding Brother existe-t-il ?

**R :** Bonding Brother existe pour isoler les produits de la complexité des autorités tout en garantissant que chaque interaction respecte les règles de l'écosystème. Sans Bonding Brother, chaque produit devrait connaître les détails internes de Kind Mother et Strong Father, créant des dépendances fragiles et des violations architecturales.

**Référence :** [Documentation Fondatrice - Section 1](../foundation/BondingBrother%20-%20Documentation%20Fondatrice.md)

---

### Q3 : Bonding Brother est-il une autorité ?

**R :** Non, absolument pas. Bonding Brother est un médiateur, pas une autorité. Il ne décide jamais, ne crée jamais de règle, ne détient jamais de vérité. Toute décision appartient aux autorités (KindMother pour les données, StrongFather pour les décisions stratégiques et politiques).

**Référence :** [Documentation Fondatrice - Section 6](../foundation/BondingBrother%20-%20Documentation%20Fondatrice.md), [Invariants & Guarantees - INV-NAT-01](../contracts/governance/BondingBrother%20-%20Invariants%20%26%20Guarantees.md)

---

### Q4 : Quelle est la différence entre une intention et une commande ?

**R :** Une intention est une déclaration de volonté, pas une instruction d'exécution. Les produits expriment ce qu'ils souhaitent faire, pas ce qu'ils ordonnent. L'évaluation et la décision appartiennent exclusivement aux autorités. Une commande serait une instruction directe, ce que Bonding Brother refuse structurellement.

**Référence :** [Documentation Fondatrice - Section 4](../foundation/BondingBrother%20-%20Documentation%20Fondatrice.md), [Intent Model Contract - Section 3](../contracts/intent/BondingBrother%20-%20Intent%20Model%20Contract.md)

---

## 4. Questions sur l'utilisation

### Q5 : Comment soumettre une intention ?

**R :** Utilisez l'interface `IIntentSubmission` via `POST /api/v1/intentions`. L'intention doit respecter le schéma défini dans l'Intent Model Contract, avec les champs obligatoires : `produit_id`, `type`, `payload`, `contexte`, `timestamp`, `version`.

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

**Référence :** [Product Interface Contract - Section 4](../contracts/product/BondingBrother%20-%20Product%20Interface%20Contract.md), [Examples & Use Cases - Section 4.1](./BondingBrother%20-%20Examples%20%26%20Use%20Cases.md)

---

### Q6 : Comment recevoir les résultats ?

**R :** Trois mécanismes sont disponibles :
1. **Callback (recommandé)** : Fournissez une URL de callback lors de la soumission. Bonding Brother appellera cette URL avec le résultat.
2. **Polling** : Interrogez régulièrement `GET /api/v1/intentions/{intention_id}/result`.
3. **Webhook** : Abonnez-vous à un webhook pour recevoir les résultats.

**Référence :** [Product Interface Contract - Section 5](../contracts/product/BondingBrother%20-%20Product%20Interface%20Contract.md)

---

### Q7 : Comment s'abonner aux notifications ?

**R :** Utilisez l'interface `INotificationSubscription` via `POST /api/v1/notifications/subscribe`. Spécifiez les types de notifications souhaités et votre URL de callback.

**Exemple :**
```json
POST /api/v1/notifications/subscribe
{
  "produit_id": "miyukini-cms",
  "types": ["CONTENT_CREATED", "CONTENT_UPDATED"],
  "callback_url": "https://produit.example.com/notifications"
}
```

**Référence :** [Product Interface Contract - Section 6](../contracts/product/BondingBrother%20-%20Product%20Interface%20Contract.md)

---

### Q8 : Quels types d'intentions sont supportés ?

**R :** Les types d'intentions sont organisés par domaine d'autorité :
- **Données** (Kind Mother) : `CREATE_CONTENT`, `UPDATE_CONTENT`, `DELETE_CONTENT`, `READ_CONTENT`, `QUERY_CONTENT`
- **Hiérarchie** (Kind Mother) : `CREATE_NODE`, `MOVE_NODE`, `DELETE_NODE`
- **Identité** (Strong Father) : `AUTHENTICATE`, `AUTHORIZE`, `CREATE_SESSION`, `REVOKE_SESSION`

**Référence :** [Intent Model Contract - Section 6](../contracts/intent/BondingBrother%20-%20Intent%20Model%20Contract.md)

---

### Q9 : Comment savoir quelle autorité est concernée par mon intention ?

**R :** Le routage est automatique selon le type d'intention :
- Intentions de données/hiérarchie → Kind Mother
- Intentions d'identité/session → Strong Father

Vous n'avez pas besoin de spécifier l'autorité, Bonding Brother route automatiquement.

**Référence :** [Product-to-Ecosystem Flow - Section 5.6](../contracts/flows/BondingBrother%20-%20Product-to-Ecosystem%20Flow.md)

---

## 5. Questions sur les erreurs

### Q10 : Mon intention a été rejetée avec le code VAL-002. Que signifie cela ?

**R :** `VAL-002` signifie qu'un champ obligatoire est manquant dans votre intention. Vérifiez que tous les champs marqués comme obligatoires sont présents : `produit_id`, `type`, `payload`, `contexte`, `timestamp`, `version`.

**Référence :** [Error & Rejection Model - Section 4.2](../contracts/error/BondingBrother%20-%20Error%20%26%20Rejection%20Model.md)

---

### Q11 : Mon intention a été rejetée avec le code TRAD-001. Que faire ?

**R :** `TRAD-001` signifie qu'aucun mapping n'existe pour votre type d'intention vers l'autorité cible. Vérifiez que vous utilisez un type d'intention canonique supporté. Si vous avez besoin d'un nouveau type, contactez l'équipe d'architecture.

**Référence :** [Error & Rejection Model - Section 4.2](../contracts/error/BondingBrother%20-%20Error%20%26%20Rejection%20Model.md), [Translation Contract - Section 10](../contracts/intent/BondingBrother%20-%20Translation%20Contract.md)

---

### Q12 : Mon intention a été refusée par l'autorité (AUTH-001). Pourquoi ?

**R :** `AUTH-001` signifie que l'autorité a explicitement refusé votre demande. Les raisons possibles incluent :
- Permissions insuffisantes
- Données invalides
- Règles métier non respectées
- Ressource verrouillée

Consultez le message d'erreur pour plus de détails. La décision vient de l'autorité, pas de Bonding Brother.

**Référence :** [Error & Rejection Model - Section 4.2](../contracts/error/BondingBrother%20-%20Error%20%26%20Rejection%20Model.md)

---

### Q13 : J'ai reçu un timeout (TIMEOUT-002). Que faire ?

**R :** `TIMEOUT-002` signifie que l'autorité n'a pas répondu dans le délai imparti. Vous pouvez :
1. Réessayer l'intention (si elle est toujours valide)
2. Vérifier l'état de l'autorité
3. Contacter le support si le problème persiste

**Référence :** [Error & Rejection Model - Section 11](../contracts/error/BondingBrother%20-%20Error%20%26%20Rejection%20Model.md)

---

### Q14 : Quelle est la différence entre un rejet et une erreur ?

**R :** 
- **Rejet** : L'intention est rejetée par Bonding Brother avant transmission à l'autorité (validation, traduction, filtrage échoués). Pas de retry automatique.
- **Erreur** : L'intention a été transmise à l'autorité, mais l'autorité a refusé ou une erreur technique s'est produite. Retry possible selon le type d'erreur.

**Référence :** [Error & Rejection Model - Section 9](../contracts/error/BondingBrother%20-%20Error%20%26%20Rejection%20Model.md)

---

## 6. Questions sur le mode offline

### Q15 : Que se passe-t-il si je soumets une intention en mode offline ?

**R :** Bonding Brother continue de fonctionner normalement :
1. L'intention est reçue et validée
2. L'intention est traduite et journalisée
3. L'intention est mise en buffer pour transmission différée
4. Lors de la reconnexion, toutes les intentions en buffer sont transmises
5. Les résultats sont transmis au produit de manière différée

**Référence :** [Offline & Deferred Authority Contract](../contracts/offline/BondingBrother%20-%20Offline%20%26%20Deferred%20Authority%20Contract.md)

---

### Q16 : Comment savoir si Bonding Brother est en mode offline ?

**R :** Bonding Brother notifie les produits du passage en mode offline via une notification système. Vous pouvez également interroger le statut via l'API.

**Référence :** [Offline & Deferred Authority Contract - Section 5.1](../contracts/offline/BondingBrother%20-%20Offline%20%26%20Deferred%20Authority%20Contract.md)

---

### Q17 : Les intentions en mode offline sont-elles perdues ?

**R :** Non, jamais. Toutes les intentions sont journalisées de manière persistante avant d'être mises en buffer. Même en cas de redémarrage, les intentions sont préservées et transmises lors de la reconnexion.

**Référence :** [Offline & Deferred Authority Contract - Section 4.3](../contracts/offline/BondingBrother%20-%20Offline%20%26%20Deferred%20Authority%20Contract.md), [Invariants & Guarantees - INV-FLUX-04](../contracts/governance/BondingBrother%20-%20Invariants%20%26%20Guarantees.md)

---

### Q18 : L'ordre des intentions est-il préservé en mode offline ?

**R :** Oui, l'ordre est préservé. Les intentions sont traitées dans l'ordre d'arrivée (FIFO), même après reconnexion.

**Référence :** [Invariants & Guarantees - INV-FLUX-03](../contracts/governance/BondingBrother%20-%20Invariants%20%26%20Guarantees.md), [Sync & Reconnection Contract](../contracts/offline/BondingBrother%20-%20Sync%20%26%20Reconnection%20Contract.md)

---

## 7. Questions sur la traduction

### Q19 : Comment fonctionne la traduction ?

**R :** La traduction transforme les intentions (vocabulaire produit) en demandes (vocabulaire autorité) et les réponses (vocabulaire autorité) en résultats (vocabulaire produit). La sémantique est préservée, seul le format est adapté.

**Référence :** [Translation Contract](../contracts/intent/BondingBrother%20-%20Translation%20Contract.md)

---

### Q20 : Puis-je utiliser mon propre vocabulaire ?

**R :** Oui, mais vous devez fournir un mapping vers le vocabulaire canonique. Bonding Brother traduit automatiquement votre vocabulaire vers celui des autorités. Consultez le Translation Contract pour les règles de mapping.

**Référence :** [Translation Contract - Section 8](../contracts/intent/BondingBrother%20-%20Translation%20Contract.md)

---

### Q21 : La traduction peut-elle modifier le sens de mon intention ?

**R :** Non, jamais. La traduction préserve intégralement la sémantique. Seul le format est adapté. Si vous pensez qu'une traduction a modifié le sens, c'est un bug à signaler.

**Référence :** [Translation Contract - Section 5.1](../contracts/intent/BondingBrother%20-%20Translation%20Contract.md), [Invariants & Guarantees - GAR-PROD-02](../contracts/governance/BondingBrother%20-%20Invariants%20%26%20Guarantees.md)

---

## 8. Questions sur les performances

### Q22 : Quelle est la latence typique d'une intention ?

**R :** Le temps de traitement par Bonding Brother est <50ms (hors attente autorité). Le temps total dépend de l'autorité :
- Temps de validation : <10ms
- Temps de traduction : <5ms
- Temps de filtrage : <5ms
- Temps d'attente autorité : variable (100ms-5s typiquement)

**Référence :** [Performance & Scalability Contract](../contracts/performance/BondingBrother%20-%20Performance%20%26%20Scalability%20Contract.md)

---

### Q23 : Combien d'intentions par seconde peut traiter Bonding Brother ?

**R :** Le throughput dépend de la configuration :
- Minimum garanti : 100 intentions/seconde
- Cible : 500 intentions/seconde
- Maximum : 1000 intentions/seconde (selon configuration)

**Référence :** [Performance & Scalability Contract](../contracts/performance/BondingBrother%20-%20Performance%20%26%20Scalability%20Contract.md)

---

### Q24 : Y a-t-il une limite de taille pour les intentions ?

**R :** Oui, des limites configurables :
- Taille maximale du payload : 1 MB (par défaut)
- Taille maximale du contexte : 100 KB (par défaut)

**Référence :** [Intent Model Contract - Section 11](../contracts/intent/BondingBrother%20-%20Intent%20Model%20Contract.md)

---

## 9. Questions sur la sécurité

### Q25 : Comment Bonding Brother garantit-il l'isolation des produits ?

**R :** Bonding Brother garantit l'isolation par :
- Filtrage strict des résultats (chaque produit ne reçoit que ses informations)
- Isolation des contextes (pas de mélange entre produits)
- Validation d'authentification (via Strong Father)
- Traçabilité complète

**Référence :** [Security & Threat Model Contract](../contracts/security/BondingBrother%20-%20Security%20%26%20Threat%20Model%20Contract.md), [Invariants & Guarantees - GAR-PROD-03](../contracts/governance/BondingBrother%20-%20Invariants%20%26%20Guarantees.md)

---

### Q26 : Les données sont-elles chiffrées en transit ?

**R :** Oui, toutes les communications utilisent HTTPS en production. HTTP est autorisé uniquement en développement.

**Référence :** [Product Interface Contract - Section 8.1](../contracts/product/BondingBrother%20-%20Product%20Interface%20Contract.md)

---

### Q27 : Comment l'authentification fonctionne-t-elle ?

**R :** L'authentification est gérée par Strong Father. Toutes les requêtes doivent inclure un token d'authentification valide dans l'en-tête `Authorization: Bearer <token>`. Bonding Brother transmet ce token aux autorités sans le valider lui-même.

**Référence :** [Product Interface Contract - Section 8.2](../contracts/product/BondingBrother%20-%20Product%20Interface%20Contract.md), [StrongFather Integration Contract](../contracts/integration/BondingBrother%20-%20StrongFather%20Integration%20Contract.md)

---

## 10. Questions sur la traçabilité

### Q28 : Comment puis-je tracer une intention ?

**R :** Chaque intention a un `intention_id` unique. Vous pouvez :
1. Conserver l'`intention_id` lors de la soumission
2. Utiliser l'API de traçabilité : `GET /api/v1/intentions/{intention_id}/trace`
3. Consulter le journal d'audit (si vous avez les permissions)

**Référence :** [Audit & Traceability Contract](../contracts/governance/BondingBrother%20-%20Audit%20%26%20Traceability%20Contract.md)

---

### Q29 : Combien de temps les journaux sont-ils conservés ?

**R :** La rétention des journaux est configurable selon les exigences de conformité. Par défaut, les journaux sont conservés pendant 90 jours, mais cela peut être ajusté.

**Référence :** [Journaling Contract](../contracts/offline/BondingBrother%20-%20Journaling%20Contract.md)

---

### Q30 : Puis-je consulter les intentions d'autres produits ?

**R :** Non, jamais. Chaque produit ne peut consulter que ses propres intentions. L'isolation est garantie par Bonding Brother.

**Référence :** [Invariants & Guarantees - GAR-PROD-05](../contracts/governance/BondingBrother%20-%20Invariants%20%26%20Guarantees.md), [Security & Threat Model Contract](../contracts/security/BondingBrother%20-%20Security%20%26%20Threat%20Model%20Contract.md)

---

## 11. Questions sur l'intégration

### Q31 : Comment intégrer mon produit à Bonding Brother ?

**R :** Suivez ces étapes :
1. Implémentez l'interface `IIntentSubmission` pour soumettre des intentions
2. Implémentez un endpoint de callback pour recevoir les résultats
3. Respectez le schéma d'intention défini dans l'Intent Model Contract
4. Gérez les erreurs selon l'Error & Rejection Model
5. Consultez le Product Adaptation Rules pour les règles spécifiques

**Référence :** [Product Interface Contract](../contracts/product/BondingBrother%20-%20Product%20Interface%20Contract.md), [Product Adaptation Rules](../contracts/product/BondingBrother%20-%20Product%20Adaptation%20Rules.md)

---

### Q32 : Puis-je accéder directement à Kind Mother ou Strong Father ?

**R :** Non, jamais. Bonding Brother est le seul chemin autorisé. Toute tentative d'accès direct est une violation architecturale et sera bloquée.

**Référence :** [Documentation Fondatrice - Section 2](../foundation/BondingBrother%20-%20Documentation%20Fondatrice.md), [Invariants & Guarantees - INV-NEG-04](../contracts/governance/BondingBrother%20-%20Invariants%20%26%20Guarantees.md)

---

### Q33 : Comment étendre Bonding Brother avec de nouvelles fonctionnalités ?

**R :** Bonding Brother s'étend par spécialisation, jamais par modification du cœur. Consultez l'Extension & Specialization Contract pour les règles d'extension.

**Référence :** [Extension & Specialization Contract](../contracts/product/BondingBrother%20-%20Extension%20%26%20Specialization%20Contract.md), [Documentation Fondatrice - Section 7](../foundation/BondingBrother%20-%20Documentation%20Fondatrice.md)

---

## 12. Questions sur la version

### Q34 : Comment fonctionne le versionnement de l'interface ?

**R :** L'interface est versionnée selon semver :
- **Version majeure** : Changements incompatibles (migration requise)
- **Version mineure** : Nouvelles fonctionnalités compatibles
- **Version patch** : Corrections de bugs

Les versions sont spécifiées dans l'URL : `/api/v1/...`

**Référence :** [Product Interface Contract - Section 10](../contracts/product/BondingBrother%20-%20Product%20Interface%20Contract.md), [Versioning & Evolution Contract](../contracts/evolution/BondingBrother%20-%20Versioning%20%26%20Evolution%20Contract.md)

---

### Q35 : Que se passe-t-il si j'utilise une version obsolète ?

**R :** Les versions obsolètes sont progressivement dépréciées avec un préavis. Après la période de dépréciation, elles sont rejetées. Consultez le Versioning & Evolution Contract pour les détails.

**Référence :** [Versioning & Evolution Contract](../contracts/evolution/BondingBrother%20-%20Versioning%20%26%20Evolution%20Contract.md)

---

### Q36 : Comment migrer vers une nouvelle version majeure ?

**R :** Consultez le Migration & Compatibility Contract pour les règles de migration. En général :
1. Consultez le guide de migration
2. Testez votre produit avec la nouvelle version
3. Migrez progressivement
4. Utilisez la période de support multi-versions

**Référence :** [Migration & Compatibility Contract](../contracts/evolution/BondingBrother%20-%20Migration%20%26%20Compatibility%20Contract.md)

---

## 13. Questions sur les garanties

### Q37 : Quelles garanties Bonding Brother offre-t-il ?

**R :** Bonding Brother garantit :
- **Interface stable** : Pas de breaking changes entre versions mineures
- **Traduction fidèle** : Sémantique préservée
- **Résultat filtré et sûr** : Seulement les informations autorisées
- **Transparence des erreurs** : Messages clairs et actionnables
- **Traçabilité accessible** : Historique de vos interactions

**Référence :** [Invariants & Guarantees - Section 7](../contracts/governance/BondingBrother%20-%20Invariants%20%26%20Guarantees.md)

---

### Q38 : Que se passe-t-il si Bonding Brother ne respecte pas une garantie ?

**R :** Toute violation de garantie est considérée comme un défaut critique. Signalez-la immédiatement. Les garanties sont contractuelles et doivent être respectées en toutes circonstances.

**Référence :** [Invariants & Guarantees - Section 11](../contracts/governance/BondingBrother%20-%20Invariants%20%26%20Guarantees.md)

---

## 14. Questions sur les tests

### Q39 : Comment tester mon intégration avec Bonding Brother ?

**R :** Utilisez l'environnement de test fourni. Vous pouvez :
1. Soumettre des intentions de test
2. Vérifier les résultats
3. Tester les scénarios d'erreur
4. Tester le mode offline

Consultez le Testing & Validation Contract pour les stratégies de test.

**Référence :** [Testing & Validation Contract](../contracts/testing/BondingBrother%20-%20Testing%20%26%20Validation%20Contract.md)

---

### Q40 : Y a-t-il des exemples de code pour tester ?

**R :** Oui, consultez le document Examples & Use Cases pour des exemples complets de flux et de cas d'usage.

**Référence :** [Examples & Use Cases](./BondingBrother%20-%20Examples%20%26%20Use%20Cases.md)

---

## 15. Questions générales

### Q41 : Où puis-je trouver plus d'informations ?

**R :** Consultez la documentation complète :
- **Documentation Fondatrice** : Concepts fondamentaux
- **Contrats spécifiques** : Règles détaillées par domaine
- **Examples & Use Cases** : Exemples pratiques
- **Reference Implementation Guidelines** : Guidelines d'implémentation

**Référence :** [Index BondingBrother](../_index.md)

---

### Q42 : Comment signaler un bug ou demander une fonctionnalité ?

**R :** Utilisez le système de tickets du projet. Pour les bugs critiques, contactez directement l'équipe d'architecture.

---

### Q43 : Bonding Brother est-il open source ?

**R :** Consultez la licence du projet pour les détails. La documentation est accessible à tous les développeurs de l'écosystème.

---

### Q44 : Puis-je contribuer à Bonding Brother ?

**R :** Les contributions sont les bienvenues. Consultez les guidelines de contribution et respectez les contrats et invariants.

---

## 16. Questions sur les cas limites

### Q45 : Que se passe-t-il si je soumets la même intention deux fois ?

**R :** Chaque intention doit avoir un `intention_id` unique. Si vous soumettez deux intentions avec le même ID, la seconde sera rejetée. Si vous soumettez deux intentions identiques avec des IDs différents, elles seront traitées comme deux intentions distinctes.

**Référence :** [Intent Model Contract - Section 4.2](../contracts/intent/BondingBrother%20-%20Intent%20Model%20Contract.md)

---

### Q46 : Puis-je annuler une intention déjà soumise ?

**R :** Non, une fois soumise et acceptée, une intention ne peut pas être annulée. Elle suivra son cycle de vie complet. Si vous avez besoin d'annuler une action, soumettez une nouvelle intention pour l'annulation (si supporté par l'autorité).

**Référence :** [Intent Model Contract - Section 7](../contracts/intent/BondingBrother%20-%20Intent%20Model%20Contract.md)

---

### Q47 : Que se passe-t-il si mon callback URL est indisponible ?

**R :** Bonding Brother retente la transmission selon une stratégie configurable. Si tous les retry échouent, le résultat est mis en queue. Vous pouvez également utiliser le polling pour récupérer les résultats manqués.

**Référence :** [Product Interface Contract - Section 5.4](../contracts/product/BondingBrother%20-%20Product%20Interface%20Contract.md), [Ecosystem-to-Product Flow - Section 6.9](../contracts/flows/BondingBrother%20-%20Ecosystem-to-Product%20Flow.md)

---

### Q48 : Puis-je soumettre des intentions en parallèle ?

**R :** Oui, vous pouvez soumettre plusieurs intentions en parallèle. Chaque intention est traitée indépendamment. L'ordre de traitement peut varier, mais l'ordre d'arrivée est préservé pour les résultats.

**Référence :** [Bilateral Flow Contract - Section 7](../contracts/flows/BondingBrother%20-%20Bilateral%20Flow%20Contract.md)

---

## 17. Statut contractuel

Ce document est **informatif, non normatif, et de statut FAQ**. Il fournit des réponses aux questions fréquentes mais ne remplace pas les contrats.

En cas de contradiction avec un contrat, le contrat prime toujours.

---

**Version :** 2.0  
**Date :** 2026-01-28  
**Statut :** FAQ — Informatif  
**Dépendances :** Tous les documents de la documentation Bonding Brother
