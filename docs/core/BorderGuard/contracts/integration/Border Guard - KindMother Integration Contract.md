# Border Guard - KindMother Integration Contract

## 1. Contexte

Ce document définit le **contrat d'intégration entre Border Guard et KindMother**. Il spécifie l'interface, le protocole, les règles de communication, et les garanties associées à l'intégration avec KindMother en tant qu'autorité des données.

Ce document complète la Section 3.1 de la [Documentation Fondatrice](../../foundation/Border%20Guard%20-%20Documentation%20Fondatrice.md) et s'appuie sur :
- [KindMother - Documentation Fondatrice](../../../KindMother/foundation/KindMother%20-%20Documentation%20Fondatrice.md) pour la nature de KindMother
- [Miyukini Conceptual References - Lois Autonomie Systeme](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md) pour la conformité LOI-1 à LOI-6

L'intégration respecte les Lois d'Autonomie Système : toutes les définitions de frontières sont locales et ne requièrent aucune dépendance externe (**LOI-1**).

## 2. Portée / Scope

Ce document couvre :
- L'interface contractuelle entre Border Guard et KindMother
- Le protocole de communication (délégation de persistance)
- La distinction entre définition et persistance
- Les règles d'intégration spécifiques
- Les garanties de l'intégration

Ce document **ne couvre pas** :
- Les détails internes de KindMother (voir documentation KindMother)
- Les détails internes du moteur de définition de frontières (voir Architecture)
- L'intégration avec StrongFather (voir StrongFather Integration Contract)
- L'intégration avec BondingBrother (voir BondingBrother Integration Contract)

---

## 3. Principe fondamental

**Border Guard définit les frontières et les règles. Si ces définitions doivent être persistées, elles sont transmises à KindMother via les canaux appropriés. Border Guard ne persiste jamais directement, KindMother ne définit jamais de frontière.**

La relation est de **complémentarité** : Border Guard gouverne les frontières et les niveaux de confiance, KindMother gouverne les données et leur persistance. Ces domaines sont distincts et non chevauchants.

---

## 4. Nature de la relation Border Guard — KindMother

### 4.1 Relation de complémentarité

**Border Guard est responsable de :**
- La définition des frontières du système
- La classification des niveaux de confiance
- L'établissement des règles de franchissement
- La gouvernance conceptuelle des intégrations

**KindMother est responsable de :**
- La persistance des données
- La cohérence des données
- La synchronisation entre instances
- L'autorité sur les écritures

**Règle BG-KM-01 : Domaines distincts**

Border Guard et KindMother gouvernent des domaines distincts. Border Guard définit les frontières, KindMother gère les données. Ces domaines ne se chevauchent jamais.

**Règle BG-KM-02 : Complémentarité sans dépendance**

Border Guard et KindMother sont complémentaires mais indépendants. Border Guard ne dépend pas de KindMother pour ses définitions. KindMother ne dépend pas de Border Guard pour sa persistance.

**Règle BG-KM-03 : Pas de connaissance croisée**

Border Guard ne connaît pas les détails de persistance de KindMother. KindMother ne connaît pas les détails de classification de Border Guard. Chacun reste souverain dans son domaine.

### 4.2 Séparation des responsabilités

| Responsabilité | Border Guard | KindMother |
|----------------|--------------|------------|
| **Définir les frontières** | ✅ Exclusif | ❌ Jamais |
| **Classifier les niveaux de confiance** | ✅ Exclusif | ❌ Jamais |
| **Établir les règles de franchissement** | ✅ Exclusif | ❌ Jamais |
| **Persister des données** | ❌ Jamais | ✅ Exclusif |
| **Gérer la cohérence des données** | ❌ Jamais | ✅ Exclusif |
| **Synchroniser les instances** | ❌ Jamais | ✅ Exclusif |
| **Valider les WriteIntent** | ❌ Jamais | ✅ Exclusif |
| **Définir les conditions d'entrée des données** | ✅ Exclusif | ❌ Jamais |

**Règle BG-KM-04 : Aucun chevauchement**

Aucun chevauchement de responsabilités n'est autorisé. Border Guard ne persiste jamais, KindMother ne définit jamais de frontière.

### 4.3 Point de contact : données venant de l'extérieur

Le point de contact entre Border Guard et KindMother concerne les **données venant de l'extérieur** :

**Border Guard définit :**
- Si une donnée venant de l'extérieur peut entrer
- Avec quel niveau de confiance
- Selon quelles règles de franchissement

**KindMother gère :**
- La persistance de la donnée une fois qu'elle est "à l'intérieur"
- La cohérence avec les données existantes
- La synchronisation si nécessaire

**Règle BG-KM-05 : Frontière puis persistance**

Une donnée externe doit d'abord satisfaire les règles de frontière (définies par Border Guard) avant d'être persistée (par KindMother). Cette séquence est non négociable.

---

## 5. Ce que Border Guard ne fait JAMAIS vis-à-vis de KindMother

### 5.1 Interdictions absolues

**INV-BG-KM-NEVER-1 : Ne persiste jamais directement**

Border Guard ne persiste **jamais** de données directement. Toute définition de frontière ou de règle qui doit être persistée est transmise à KindMother via les canaux appropriés (WriteIntent via BondingBrother).

**INV-BG-KM-NEVER-2 : N'accède jamais à la persistance**

Border Guard n'accède **jamais** directement à la couche de persistance de KindMother. Aucun accès SQLite, aucune lecture directe, aucune modification directe.

**INV-BG-KM-NEVER-3 : Ne valide jamais les WriteIntent**

Border Guard ne valide **jamais** les WriteIntent pour leur persistance. La validation de la cohérence et des permissions de persistance appartient exclusivement à KindMother.

**INV-BG-KM-NEVER-4 : Ne synchronise jamais**

Border Guard ne participe **jamais** à la synchronisation des données entre instances. La synchronisation est du ressort exclusif de KindMother.

**INV-BG-KM-NEVER-5 : Ne connaît pas les schémas**

Border Guard ne connaît **jamais** les schémas de données de KindMother. Les définitions de frontières sont conceptuelles et indépendantes des schémas de persistance.

---

## 6. Ce que KindMother ne fait JAMAIS vis-à-vis de Border Guard

### 6.1 Interdictions absolues (perspective KindMother)

**INV-KM-BG-NEVER-1 : Ne définit jamais de frontière**

KindMother ne définit **jamais** de frontière. Toute définition de frontière provient exclusivement de Border Guard.

**INV-KM-BG-NEVER-2 : Ne classifie jamais les niveaux de confiance**

KindMother ne classifie **jamais** les niveaux de confiance des sources ou destinations. La classification est du ressort exclusif de Border Guard.

**INV-KM-BG-NEVER-3 : N'établit jamais de règle de franchissement**

KindMother n'établit **jamais** de règle de franchissement. Les règles de franchissement proviennent exclusivement de Border Guard.

**INV-KM-BG-NEVER-4 : Ne connaît pas les frontières**

KindMother ne connaît pas les frontières définies par Border Guard. KindMother traite les données une fois qu'elles sont "à l'intérieur", sans savoir comment elles y sont entrées.

---

## 7. Types d'interactions

### 7.1 Interaction indirecte via BondingBrother

Border Guard et KindMother n'interagissent pas directement. Toute interaction passe par BondingBrother.

**Flux de donnée externe :**

1. Une donnée externe arrive
2. BondingBrother consulte Border Guard pour les règles de franchissement
3. Border Guard fournit les règles et le niveau de confiance requis
4. BondingBrother vérifie si les conditions sont satisfaites
5. Si oui, BondingBrother traduit en WriteIntent pour KindMother
6. KindMother valide et persiste la donnée

**Règle BG-KM-INT-01 : Pas d'interaction directe**

Border Guard et KindMother ne communiquent jamais directement. Toute interaction passe par BondingBrother.

### 7.2 Persistance des définitions de frontières

Si les définitions de frontières doivent être persistées pour garantir leur survie au redémarrage :

**Flux de persistance des définitions :**

1. Border Guard crée ou modifie une définition de frontière
2. Border Guard exprime une intention de persistance via BondingBrother
3. BondingBrother traduit en WriteIntent approprié
4. KindMother persiste la définition comme une donnée du système
5. Au redémarrage, Border Guard charge les définitions depuis KindMother (via BondingBrother)

**Règle BG-KM-INT-02 : Définitions comme données**

Les définitions de frontières peuvent être persistées comme des données système. Leur persistance suit le flux standard via BondingBrother et KindMother.

**Règle BG-KM-INT-03 : Chargement au démarrage**

Au démarrage, Border Guard peut charger ses définitions persistées depuis KindMother (via BondingBrother). Ce chargement est optionnel si les définitions sont statiques ou définies par configuration.

---

## 8. Protocole de non-interaction

### 8.1 Absence de communication directe

**Règle BG-KM-PROT-01 : Aucune API directe**

Border Guard n'expose aucune API vers KindMother. KindMother n'expose aucune API vers Border Guard.

**Règle BG-KM-PROT-02 : Aucune consultation directe**

Border Guard ne consulte jamais KindMother directement. KindMother ne consulte jamais Border Guard directement.

**Règle BG-KM-PROT-03 : BondingBrother obligatoire**

Toute interaction entre les domaines de Border Guard et KindMother passe obligatoirement par BondingBrother.

### 8.2 Isolation des domaines

**Règle BG-KM-PROT-04 : Isolation des schémas**

Les schémas de données de KindMother sont isolés des définitions de frontières de Border Guard. Aucune dépendance structurelle.

**Règle BG-KM-PROT-05 : Isolation des états**

L'état des frontières (géré par Border Guard) est isolé de l'état des données (géré par KindMother).

---

## 9. Flux d'intégration typique

### 9.1 Flux de donnée externe

**Acteurs :** Source externe, BondingBrother, Border Guard, StrongFather, KindMother

**Séquence :**

```
┌──────────────┐  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐
│   Source     │  │  BondingBrother │  │   Border Guard  │  │   StrongFather  │  │   KindMother    │
│  Externe     │  │                 │  │                 │  │                 │  │                 │
└──────┬───────┘  └────────┬────────┘  └────────┬────────┘  └────────┬────────┘  └────────┬────────┘
       │                   │                    │                    │                    │
       ├── Donnée ────────►│                    │                    │                    │
       │                   │                    │                    │                    │
       │                   ├── Règles ? ───────►│                    │                    │
       │                   │                    │                    │                    │
       │                   │◄── Règles ─────────┤                    │                    │
       │                   │                    │                    │                    │
       │                   ├── Vérifie ─────────┼────────────────────┤                    │
       │                   │   conditions       │                    │                    │
       │                   │                    │                    │                    │
       │                   ├── Décision ? ──────┼───────────────────►│                    │
       │                   │                    │                    │                    │
       │                   │◄── Acceptée ───────┼────────────────────┤                    │
       │                   │                    │                    │                    │
       │                   ├── WriteIntent ─────┼────────────────────┼───────────────────►│
       │                   │                    │                    │                    │
       │                   │◄── Succès ─────────┼────────────────────┼────────────────────┤
       │                   │                    │                    │                    │
```

### 9.2 Points clés du flux

1. **Border Guard** fournit les règles de franchissement (niveau de confiance requis, conditions)
2. **BondingBrother** vérifie les conditions et prépare le contexte
3. **StrongFather** décide si l'intention est acceptée selon les politiques
4. **KindMother** persiste la donnée une fois toutes les validations passées

**Règle BG-KM-FLOW-01 : Séquence obligatoire**

La séquence frontière → décision → persistance est obligatoire pour les données externes. Aucune étape ne peut être sautée.

---

## 10. Règles d'intégration

### 10.1 Règles de domaine

**Règle BG-KM-INT-04 : Souveraineté des domaines**

Border Guard est souverain sur les frontières. KindMother est souverain sur les données. Aucun ne peut intervenir dans le domaine de l'autre.

**Règle BG-KM-INT-05 : Indépendance opérationnelle**

Border Guard peut fonctionner sans KindMother (définitions en mémoire ou configuration). KindMother peut fonctionner sans Border Guard (données internes uniquement).

### 10.2 Règles de cohérence

**Règle BG-KM-INT-06 : Pas de référence croisée**

Les définitions de Border Guard ne référencent pas les schémas de KindMother. Les schémas de KindMother ne référencent pas les frontières de Border Guard.

**Règle BG-KM-INT-07 : Cohérence indépendante**

La cohérence des définitions de frontières est gérée par Border Guard. La cohérence des données est gérée par KindMother. Chacun gère sa propre cohérence.

---

## 11. Gestion des erreurs

### 11.1 Types d'erreurs

**Erreurs de domaine Border Guard :**
- Frontière non définie
- Règle de franchissement non satisfaite
- Niveau de confiance insuffisant

**Erreurs de domaine KindMother :**
- WriteIntent rejeté
- Incohérence de données
- Erreur de persistance

### 11.2 Traitement des erreurs

**Règle BG-KM-ERR-01 : Erreurs isolées**

Une erreur de Border Guard n'affecte pas KindMother. Une erreur de KindMother n'affecte pas Border Guard.

**Règle BG-KM-ERR-02 : Propagation via BondingBrother**

Les erreurs sont propagées via BondingBrother. Si Border Guard refuse un franchissement, BondingBrother ne soumet pas de WriteIntent à KindMother.

---

## 12. Cas particuliers

### 12.1 Données internes

Les données créées et consommées entièrement à l'intérieur du système :

**Règle BG-KM-CASE-01 : Pas de frontière pour l'interne**

Les données purement internes ne traversent pas de frontière définie par Border Guard. Elles sont gérées directement par KindMother via BondingBrother.

### 12.2 Mode offline

Lorsque le système est en mode offline :

**Règle BG-KM-CASE-02 : Indépendance en offline**

Border Guard et KindMother fonctionnent indépendamment en mode offline. Border Guard maintient ses définitions localement, KindMother maintient ses données localement.

### 12.3 Synchronisation et frontières

Lors de la synchronisation entre instances :

**Règle BG-KM-CASE-03 : Données synchronisées = données internes**

Les données synchronisées entre instances KindMother sont traitées comme des données internes. Les règles de fédération (Border Guard) s'appliquent à la communication inter-nœuds, pas à la synchronisation KindMother interne.

---

## 13. Garanties de l'intégration

### 13.1 Garantie de séparation

**Engagement :** La séparation entre les domaines de Border Guard et KindMother est absolue. Aucune exception n'est possible.

### 13.2 Garantie d'indépendance

**Engagement :** Border Guard et KindMother peuvent fonctionner indépendamment l'un de l'autre.

### 13.3 Garantie de non-persistance directe

**Engagement :** Border Guard ne persiste jamais directement. Toute persistance passe par KindMother via BondingBrother.

### 13.4 Garantie de non-définition par KindMother

**Engagement :** KindMother ne définit jamais de frontière ou de règle de franchissement.

### 13.5 Garantie de cohérence isolée

**Engagement :** La cohérence de chaque domaine est gérée par son propriétaire. Aucune dépendance de cohérence croisée.

---

## 14. Invariants de l'intégration

### 14.1 Invariants de domaine

**INV-BG-KM-1 : Domaines distincts**

Border Guard et KindMother gouvernent des domaines distincts et non chevauchants.

**INV-BG-KM-2 : Souveraineté**

Chaque core est souverain dans son domaine. Aucune intrusion n'est autorisée.

**INV-BG-KM-3 : Pas d'interaction directe**

Border Guard et KindMother ne communiquent jamais directement.

### 14.2 Invariants de persistance

**INV-BG-KM-4 : Pas de persistance par Border Guard**

Border Guard ne persiste jamais directement. Conformément à INV-BG-2 de la Documentation Fondatrice.

**INV-BG-KM-5 : Définitions comme données optionnelles**

Les définitions de frontières peuvent être persistées comme données système, mais ce n'est pas obligatoire.

### 14.3 Invariants de définition

**INV-BG-KM-6 : Pas de définition par KindMother**

KindMother ne définit jamais de frontière, de niveau de confiance, ou de règle de franchissement.

---

## 15. Conformité aux Lois d'Autonomie Système

### LOI-1 : Aucune dépendance externe critique

**Conformité :** ✅ **Conforme**

L'intégration respecte LOI-1 :
- Border Guard définit les frontières localement
- KindMother persiste les données localement
- Aucune dépendance externe pour les définitions ou la persistance

### LOI-2 : Le système accepte l'isolement comme état normal

**Conformité :** ✅ **Conforme**

L'intégration respecte LOI-2 :
- Border Guard et KindMother fonctionnent indépendamment en mode offline
- L'isolement ne dégrade pas l'intégration

### LOI-3 : L'état local est souverain

**Conformité :** ✅ **Conforme**

L'intégration respecte LOI-3 :
- Les définitions de frontières locales sont souveraines
- Les données locales sont souveraines
- Chaque domaine est souverain localement

---

## 16. Exemples

### 16.1 Flux de donnée externe (conceptuel)

**Scénario :** Une donnée arrive d'une API partenaire

1. **BondingBrother** reçoit la donnée et identifie qu'elle vient de l'extérieur
2. **BondingBrother** consulte **Border Guard** : "Quelles sont les règles pour cette frontière ?"
3. **Border Guard** retourne : "Niveau verified requis, conditions : api_key_valid, rate_limit_ok"
4. **BondingBrother** vérifie les conditions et prépare le contexte
5. **StrongFather** évalue et décide : "Acceptée"
6. **BondingBrother** traduit en WriteIntent pour **KindMother**
7. **KindMother** valide la cohérence et persiste la donnée

**Note :** Border Guard et KindMother n'ont jamais communiqué directement.

### 16.2 Définitions persistées (optionnel)

**Scénario :** Border Guard souhaite persister ses définitions

1. **Border Guard** crée une nouvelle définition de frontière
2. **Border Guard** exprime via **BondingBrother** : "Je souhaite persister cette définition"
3. **BondingBrother** traduit en WriteIntent de type "system_configuration"
4. **KindMother** persiste la définition comme donnée système
5. Au redémarrage, **Border Guard** charge les définitions via **BondingBrother**

---

## 17. Statut contractuel

Ce document est **contractuel, normatif, et de statut CONTRAT**. Il établit les règles de non-interaction et de complémentarité que Border Guard et KindMother doivent respecter.

Toute implémentation doit respecter ce contrat. Toute violation entraîne un comportement non conforme.

---

**Version :** 1.0  
**Date :** 2026-01-28  
**Statut :** CONTRAT — Normatif  
**Dépendances :**
- Border Guard - Documentation Fondatrice v1.5 (Section 3.1)
- KindMother - Documentation Fondatrice v1.2
- Miyukini Conceptual References - Lois Autonomie Systeme v1.1

---

## 18. Mini log de génération

### Décision éditoriale E1 : Relation de complémentarité

**Décision prise :** La relation est de complémentarité : Border Guard gouverne les frontières, KindMother gouverne les données. Cette direction respecte la Documentation Fondatrice de Border Guard Section 3.1 qui définit "KindMother traite les données une fois qu'elles sont 'à l'intérieur' ; Border Guard définit les conditions pour qu'elles y entrent."

**Application :** Tout le document est structuré autour de cette complémentarité sans chevauchement.

### Décision éditoriale E2 : Pas d'interaction directe

**Décision prise :** Border Guard et KindMother n'interagissent pas directement. Toute interaction passe par BondingBrother.

**Application :** Section 7 et Section 8 établissent ce protocole de non-interaction directe.

### Warning W1 : Risque de persistance directe

**Warning rencontré :** Risque que Border Guard soit tenté de persister directement ses définitions.

**Décision prise :** Les interdictions absolues (Section 5) clarifient que Border Guard ne persiste jamais directement (conformément à INV-BG-2).

**Correction effectuée :** INV-BG-KM-NEVER-1 et INV-BG-KM-4 confirment cette interdiction.

### Vérification de cohérence

**Vérification effectuée :**
- ✅ Cohérence avec Border Guard - Documentation Fondatrice : Confirmée (complémentarité, pas de persistance directe)
- ✅ Cohérence avec KindMother - Documentation Fondatrice : Confirmée (souveraineté des données, pas de définition de frontière)
- ✅ Cohérence avec INV-BG-2 : Confirmée (aucune persistance directe)
- ✅ Conformité LOI-1 : Confirmée (aucune dépendance externe)
- ✅ Conformité LOI-2 : Confirmée (fonctionnement indépendant en offline)
- ✅ Conformité LOI-3 : Confirmée (souveraineté locale des deux domaines)
- ✅ Séparation des domaines : Confirmée (INV-BG-KM-1, INV-BG-KM-2)

**Conclusion :** Aucune contradiction détectée. Le document est cohérent et non ambigu.

---

*Aucune autre erreur, warning, ou ambiguïté rencontrée lors de la rédaction de ce document.*
