# Miyukini Conceptual References — Accès DB et droits pour agents IA

## Contexte

Ce document répond à deux questions récurrentes dans l’écosystème Miyukini :
1. Comment accélérer et encadrer l’accès à la base de données pour les agents IA (ex. Cursor, automations) ?
2. Qui donne ou comment sont accordés les droits, et plus précisément les WriteIntent approuvés ?

Il est **informativ** et s’appuie sur les contrats KindMother, StrongFather et WorrySentinel. Il ne crée pas de nouvelles obligations contractuelles.

## Portée

- **Cible :** Équipes produit, ops et développeurs qui intègrent des agents IA ou qui configurent les accès DB.
- **Hors portée :** Implémentation détaillée des cores (KindMother, StrongFather) ; celle-ci reste dans les contrats par core.

---

## 1. Accès DB pour les agents IA : MCP vs API

### 1.1 Rappel architectural

Dans Miyukini, **aucun composant métier n’a d’accès direct au stockage**. Les adaptateurs et produits passent par la **CoreDataAPI** de KindMother (lecture/écriture via intentions). Les écritures sont des **WriteIntent** validées par KindMother (Runtime Boundaries), pas des requêtes SQL brutes.

Les **agents IA** (Cursor, scripts d’IA, automations) sont en revanche des **clients externes** au sens “qui parle à l’infra ou à l’app”. Pour eux, deux canaux principaux sont pertinents : **MCP** et **API**.

### 1.2 Option 1 : Serveur MCP (ex. user-supabase)

**Principe :** Un serveur MCP (ex. Supabase) expose des **outils** (execute_sql, apply_migration, list_tables, list_projects, etc.). L’agent IA (Cursor) appelle ces outils dans le cadre d’une session.

**Avantages :**
- **Rapide** : pas de couche HTTP intermédiaire, appel direct outil → Supabase.
- **Encadré** : les droits = ceux du **projet Supabase** et des clés configurées (service role / anon) ; pas d’exposition publique d’API.
- **Fiable** : un seul canal (IDE / session), pas de gestion de tokens côté agent si la config MCP est centralisée.
- **Robuste** : migrations via `apply_migration`, SQL arbitraire limité à `execute_sql` si on choisit de l’exposer.

**Recommandations pour un système fiable et robuste :**
- **En dev** : MCP Supabase avec un projet dédié (ou branche) ; les agents n’ont accès qu’à ce projet.
- **Limiter les outils** : par environnement, n’exposer que ce qui est nécessaire (ex. pas d’`execute_sql` en prod si la politique est “tout en migrations”).
- **Documenter** : quel agent (quel MCP, quel projet) a accès à quoi, et avec quelle clé (service_role = plein accès, anon = accès restreint par politiques applicatives).

**Limites :**
- Pas d’auth fine par “utilisateur agent” si on ne construit pas une couche dédiée.
- Les écritures passent **directement** en SQL ; elles ne passent **pas** par le flux WriteIntent / KindMother / StrongFather. Donc adapté à l’outillage (migrations, scripts admin, lecture) plutôt qu’aux écritures métier contractuelles.

### 1.3 Option 2 : API dédiée (REST / GraphQL)

**Principe :** Une API métier reçoit les requêtes des agents (authentification par clé, service account, ou autre). Côté serveur, l’API appelle soit KindMother (CoreDataAPI / WriteIntent), soit directement le stockage avec un compte de service, selon le niveau de conformité souhaité.

**Avantages :**
- **Encadrement fort** : auth, rate-limiting, audit, traçabilité par client/agent.
- **Conformité Miyukini** : les écritures peuvent transiter par **WriteIntent** → KindMother → Runtime Boundaries (et éventuellement StrongFather pour les décisions stratégiques).
- **Robuste** : un point d’entrée unique, politiques centralisées.

**Inconvénients :**
- **Latence** : un à plusieurs sauts réseau et couches applicatives.
- **Coût** : développement, déploiement et maintenance de l’API.

### 1.4 Synthèse recommandée

| Besoin | Canal recommandé | Commentaire |
|--------|------------------|-------------|
| Dev / outillage (migrations, scripts, lecture) | **MCP** (ex. user-supabase) | Rapide, encadré par projet Supabase, pas de WriteIntent. |
| Écritures métier contractuelles (WriteIntent, audit) | **API** appelant KindMother / StrongFather | Conforme aux contrats, traçable. |
| Agents distants, multi-clients, quotas | **API** | Auth, rate-limit, audit centralisés. |

**Système fiable, robuste et rapide** au sens “agents IA” :
- **Rapide** : privilégier MCP pour tout ce qui est outillage et lecture.
- **Fiable / robuste** : définir clairement qui (quel MCP, quel projet, quelle API) a accès à quoi ; limiter les outils MCP par environnement ; pour les écritures métier, faire passer par une API qui soumet des WriteIntent.

---

## 2. Qui accorde les droits ? WriteIntent approuvés

Il faut distinguer **deux niveaux** : (1) les **permissions** attachées au contexte d’une WriteIntent, et (2) la **décision** d’accepter ou refuser une intention (y compris stratégique).

### 2.1 Permissions (suffisance des droits) — KindMother

- **Qui “donne” les permissions ?**  
  Les **règles de permissions** sont **définies par le produit** et **fournies dans le contexte** de chaque appel (lecture ou WriteIntent). KindMother **ne les accorde pas** : il **vérifie** qu’elles sont **suffisantes** pour l’opération demandée (voir **Boundary de permissions**, Runtime Boundary & Enforcement Contract).

- **Où c’est décrit ?**  
  - KindMother — Write Intent Lifecycle Contract (contexte complet : identité, permissions, instance, domaine).  
  - KindMother — Adapter Compliance Contract : l’adaptateur ne modifie jamais les règles de permissions fournies par le produit.  
  - KindMother — CoreDataAPI Contract : Boundary 4 = permissions (suffisance des droits).

En résumé : le **produit** (ou l’adaptateur au nom du produit) fournit le contexte ; **KindMother** valide que ce contexte est suffisant. Pas d’“octroi” de droits par KindMother.

### 2.2 Décision d’autorisation (APPROVED / DENIED) — StrongFather

- **Qui accorde ou refuse ?**  
  **StrongFather** est le **moteur de décision stratégique et politique**. Il **évalue des intentions** (volonté d’action) et produit une décision (APPROVED, DENIED, etc.) selon les **politiques** qu’il applique.

- **D’où viennent les politiques ?**  
  Les politiques viennent de la **Policy Source** (StrongFather — Policy Source Contract) : une **source unique, configurée** (ex. déclarative statique). Elle est **alimentée par la gouvernance / le produit** ; StrongFather ne génère pas de politiques à la volée.

- **Où StrongFather intervient-il par rapport aux WriteIntent ?**  
  Les WriteIntent sont **validées par KindMother** (Runtime Boundaries). Lorsque le flux métier le prévoit (ex. configuration MiyukiniAdmin, franchissement BorderGuard, arbitrages LogisticsSteward), la **décision** “autorisé ou non” peut être déléguée à **StrongFather**. StrongFather ne persiste rien et n’exécute rien ; il rend un verdict, que d’autres composants utilisent pour autoriser ou bloquer une action (et donc, in fine, une WriteIntent ou un flux qui mène à une WriteIntent).

En résumé : les **WriteIntent “approuvées”** au sens KindMother = celles qui **passent toutes les Runtime Boundaries** (dont la boundary de permissions). Les **décisions “accordées”** au sens stratégique = **StrongFather** selon les **politiques** définies dans la **Policy Source** (produit / gouvernance).

### 2.3 Schéma récapitulatif

```
Produit / Gouvernance
        │
        ├── Règles de permissions (contexte) ──► KindMother (vérification Boundary permissions)
        │
        └── Policy Source (politiques) ──► StrongFather (décision APPROVED/DENIED)
                                                    │
                                                    ▼
Adaptateur / Produit ──► WriteIntent (contexte complet) ──► KindMother
                                                                  │
                                    Runtime Boundaries (dont permissions)
                                                                  │
                                    ACCEPTÉE → Application par KindMother
                                    REJETÉE  → Pas d’application
```

---

## 3. Chemin lecture DB et chemin écriture depuis un Outil

**Terminologie (Glossaire officiel) :** « Produit intermédiaire » = **Outil** ou **Kit d'Outils**. Un **Outil** est une capacité exécutable gouvernée (Strate 6), sans autorité, gouvernée par les Cores. Il fait, mais ne décide jamais. **BondingBrother** est le Core de médiation qui traduit les intentions des Opérateurs en demandes pour les Cores (dont KindMother).

### 3.1 Chemin d’une lecture DB (depuis un Outil)

Une lecture DB ne modifie pas les données. Elle transite par KindMother uniquement via la **CoreDataAPI** (opérations **read**, **list** ou **query**). Aucun accès direct au stockage.

**Étapes :**

1. **Outil / Opérateur** — L’Outil (ou l’Opérateur qui l’utilise) exprime une intention de lecture (ex. lire un contenu, lister des entités, requête complexe).
2. **BondingBrother** — Reçoit l’intention, la traduit en opération KindMother (ex. `READ_CONTENT` → `read_content` ; `QUERY_CONTENT` → `query_content`). Construit la demande au format KindMother avec **contexte complet** (identité, règles de permissions, instance).
3. **KindMother — CoreDataAPI** — Reçoit l’appel (read, list ou query) avec le contexte.
4. **KindMother — Runtime Boundaries** — Chaque appel traverse les frontières d’exécution dans l’ordre : **appel** (légalité), **contexte** (validité), **instance** (état), **permissions** (suffisance des droits), **cohérence**, **contournement**, **charge**. Si une boundary échoue → rejet immédiat, pas de lecture.
5. **KindMother — Persistance** — Si toutes les boundaries sont passées : lecture depuis la persistance de l’instance (DB Mère ou DB Fille selon le contexte), filtrage selon les permissions, retour des données.
6. **BondingBrother** — Traduit la réponse KindMother au format produit et la retourne à l’Outil / Opérateur.

**Schéma lecture :**

```
Outil / Opérateur  →  BondingBrother (traduction)  →  KindMother CoreDataAPI (read / list / query)
                                                              │
                                    Runtime Boundaries (appel, contexte, instance, permissions, …)
                                                              │
                                    OK  →  Persistance (lecture)  →  Données  →  BondingBrother  →  Outil
                                    KO  →  Rejet (erreur explicite)
```

Aucune **WriteIntent** ni **StrongFather** sur une pure lecture ; seules les **permissions** fournies dans le contexte sont vérifiées par KindMother (Boundary de permissions).

### 3.2 Chemin d’une écriture depuis un Outil (produit intermédiaire)

Une écriture est une **modification de données**. Elle doit passer par une **WriteIntent** soumise à KindMother ; aucune écriture directe n’est autorisée (CoreDataAPI ne fournit pas d’accès direct au stockage).

**Étapes :**

1. **Outil / Opérateur** — L’Outil (ou l’Opérateur) déclenche une intention d’écriture (ex. créer, modifier, supprimer un contenu).
2. **BondingBrother** — Reçoit l’intention, la traduit en opération KindMother (ex. `CREATE_CONTENT` → création de contenu ; `UPDATE_CONTENT` → mise à jour). Construit une **demande** au format KindMother contenant les données à modifier et le **contexte complet** (identité, règles de permissions, instance, domaine).
3. **KindMother — CoreDataAPI** — Reçoit **submitWriteIntent** (ou submitBatchWriteIntent) avec la WriteIntent et le contexte.
4. **KindMother — Cycle de vie WriteIntent** — La WriteIntent passe à l’état **CRÉÉE** (identité attribuée par KindMother), puis **EN_VALIDATION**.
5. **KindMother — Runtime Boundaries** — La WriteIntent traverse les mêmes frontières que pour la lecture (appel, contexte, instance, **permissions**, cohérence, contournement, charge). Si une boundary échoue → WriteIntent **REJETÉE**, pas d’application.
6. **KindMother — Décision** — Si toutes les boundaries passent → WriteIntent **ACCEPTÉE**, puis **APPLIQUÉE** (modification atomique + persistance), puis **ARCHIVÉE**. Sinon → **REJETÉE** puis archivée pour traçabilité.
7. **BondingBrother** — Traduit la réponse (succès ou erreur explicite) et la retourne à l’Outil / Opérateur.

Si le flux métier le prévoit (ex. configuration sensible, franchissement de frontière), une **décision StrongFather** (APPROVED/DENIED) peut être demandée avant ou en parallèle ; StrongFather ne modifie pas les données, il rend un verdict utilisé par d’autres composants pour autoriser ou bloquer le flux qui mène à la WriteIntent.

**Schéma écriture :**

```
Outil / Opérateur  →  BondingBrother (traduction)  →  KindMother CoreDataAPI (submitWriteIntent)
                                                              │
                                    WriteIntent CRÉÉE  →  EN_VALIDATION
                                                              │
                                    Runtime Boundaries (appel, contexte, instance, permissions, cohérence, …)
                                                              │
                                    OK  →  ACCEPTÉE  →  APPLIQUÉE (persistance)  →  ARCHIVÉE  →  BondingBrother  →  Outil
                                    KO  →  REJETÉE  →  ARCHIVÉE  →  Erreur explicite  →  BondingBrother  →  Outil
```

**Résumé :** En lecture, le chemin est **Outil → BondingBrother → KindMother (CoreDataAPI read/list/query) → Runtime Boundaries → persistance (lecture) → retour**. En écriture, le chemin est **Outil → BondingBrother → KindMother (submitWriteIntent) → WriteIntent (cycle de vie) → Runtime Boundaries → ACCEPTÉE → APPLIQUÉE → persistance → retour**. Les **droits** sont portés par le **contexte** fourni par le produit ; KindMother **vérifie** qu’ils sont suffisants (Boundary de permissions), il ne les accorde pas.

---

## 4. Références contractuelles

- KindMother — Write Intent Lifecycle Contract  
- KindMother — Runtime Boundary & Enforcement Contract (Boundary de permissions)  
- KindMother — CoreDataAPI Contract  
- KindMother — Adapter Compliance Contract (règles de permissions fournies par le produit)  
- StrongFather — Intent Model Contract  
- StrongFather — Policy Engine Contract  
- StrongFather — Policy Source Contract  
- WorrySentinel — StrongFather Integration Contract (sévérité des décisions)

---

**Document créé le :** 2026-01-28  
**Type :** Référence conceptuelle (informative)  
**Statut :** Non contractuel
