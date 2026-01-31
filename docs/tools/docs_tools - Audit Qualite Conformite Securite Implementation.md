# docs/tools — Audit qualité, conformité, sécurité et guides d'implémentation

## Contexte

Audit de l'ensemble de la documentation des Kits d'Outils (Toolkits) dans **docs/tools/** : qualité rédactionnelle, conformité aux protocoles en vigueur, niveau de sécurité documenté, et présence/qualité des guides d'implémentation.

**Périmètre :** tous les dossiers et fichiers sous `docs/tools/`.  
**Date :** 2026-01-30.  
**Référence :** [docs_tools - Audit Documentation](./docs_tools%20-%20Audit%20Documentation.md) (audit précédent, améliorations déjà appliquées).

---

## 1. Synthèse exécutive

| Critère | État | Commentaire |
|--------|------|-------------|
| **Qualité structurelle** | Bonne | Schéma Contexte / Portée / Définition canonique / Identifiant / Outils / Gouvernance / Sécurité / KindMother / Références respecté sur l’ensemble des Doc Fondatrices. |
| **Conformité protocoles** | Partielle | Tools et Toolkits + Template Governance : OK. MIP : inégal (nombreux kits sans § Alignement MIP). Nomenclature fichiers : conforme. |
| **Niveau de sécurité** | Cohérent | Niveaux 0–4 et états HEALTHY/DEGRADED/SECURITY_LOCKDOWN/MAINTENANCE utilisés de façon cohérente avec le Glossaire (WorrySentinel). |
| **Guides d’implémentation** | En cours | **Template commun** créé ; **9 kits** disposent d’un document « Reference Implementation Guidelines » : MiyuAuth, MiyuWeb, MiyuSQL, MiyuForum, MiyuPM, MiyuNotify, MiyuSearch, MiyuWebwayParticipant, MiyuWebwayTracker. Les 40 autres n’en ont pas. |

---

## 2. Qualité de la documentation

### 2.1 Points forts

- **Alignement terminologique** : usage systématique du [Glossaire](../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) (Outil, Kit d’Outils, KindMother, WriteIntent, StrongFather, Master Butler, etc.).
- **Format ToolkitId** : `toolkit.<domain>.<name>` respecté pour les **49 kits** recensés dans `_index.md`.
- **Sections prévisibles** : Contexte, Portée, Définition canonique, Identifiant et catalogue, Liste des outils composants, Gouvernance, Niveau de sécurité et états, Relation avec KindMother, Références (et parfois § 9 Alignement MIP avant Références).
- **Flux de gouvernance factorisé** : les Doc Fondatrices renvoient au document de référence [Tools et Toolkits](../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md) et indiquent une spécificité (règles alertes, permissions, WriteIntent, etc.), conformément aux recommandations de l’audit précédent.
- **Contrats allégés** : chaque kit dispose d’un Tool Governance Compliance Contract qui renvoie au [Template](../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Compliance%20Template.md) et ne liste que les obligations spécifiques.

### 2.2 Points à améliorer

| Problème | Détail | Suggestion |
|----------|--------|------------|
| **Numérotation des sections** | MiyuTreasury a « § 10. Références croisées » sans § 9 (Alignement MIP) ; incohérence avec les kits qui ont § 9 Alignement MIP puis § 10 Références. | Uniformiser : soit § 9 Références pour tous, soit § 9 Alignement MIP (ou N/A) puis § 10 Références. |
| **Lien Doc Fondatrice → Contrat** | Seuls MiyuInvoice, MiyuExpense, MiyuComptaLedger mentionnent explicitement « Les obligations de conformité détaillées sont dans [Tool Governance Compliance Contract](...). » | Étendre cette phrase à toutes les Doc Fondatrices qui ont un contrat, pour clarifier la hiérarchie (recommandation audit précédent). |
| **Référence croisée MIP** | Les kits avec § Alignement MIP ne renvoient pas tous au même chemin du protocole MIP (certains utilisent `../../protocols/...`). | Vérifier que le lien pointe vers [Miyukini Prompt Protocol - MIP v1 MSCM Index Protocol](../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md). |

---

## 3. Conformité aux protocoles en vigueur

### 3.1 Référence conceptuelle Tools et Toolkits

- **Conformité :** Oui. Tous les kits s’appuient sur les définitions canoniques (Outil, Kit d’Outils), le flux de gouvernance (BondingBrother → Master Butler → WorrySentinel → Caring Nanny → StrongFather), et les règles (WriteIntent KindMother, pas de décision métier dans les Tools).
- **Vérification :** Les Doc Fondatrices et contrats citent ou renvoient à [Miyukini Conceptual References - Tools et Toolkits](../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md).

### 3.2 Master Butler — Tool Governance Compliance Template

- **Conformité :** Oui. Chaque contrat `MiyuXXX - Tool Governance Compliance Contract.md` référence le template et ne décrit que les obligations spécifiques du kit.
- **Vérification :** 49 kits avec contrat governance (MiyuCMS, MiyuMedia, MiyuWidgets, MiyuStore, MiyuShipping, MiyuBooking, MiyuBilling ont « — » dans le tableau complétude : pas de contrat dédié ; à confirmer selon politique projet).

### 3.3 Protocole MIP v1 (MSCM Index Protocol)

- **Conformité :** Inégale.
- **Kits avec section « Alignement MIP » explicite (référence MIP v1, blocs MSCM, blocks.json/domains.json/layers.json) :** MiyuAuth, MiyuClock, MiyuBooking, MiyuWeb, MiyuMedia, MiyuSQL, MiyuShipping, MiyuStore, MiyuBilling, MiyuCMS, MiyuWidgets, MiyuPosSales ; et section courte dans MiyuNotify, MiyuValidate, MiyuLocale, MiyuJobs, MiyuExport, MiyuSearch, MiyuText, MiyuCalc (parfois une seule phrase).
- **Kits sans section Alignement MIP :** les 17 kits récemment documentés (MiyuForum, MiyuPM, MiyuPolls, MiyuFeeds, MiyuBookmarks, MiyuProfile, MiyuContacts, MiyuModerationForum, MiyuAntiSpam, MiyuSocialFeed, MiyuStory, MiyuSocialMessaging, MiyuSocialProfile, MiyuDiscovery, MiyuSocialModeration, MiyuWebwayParticipant, MiyuWebwayTracker) ainsi que MiyuHR, MiyuTreasury, MiyuComptaLedger, MiyuInvoice, MiyuExpense, MiyuComptaReports, MiyuDeclarations, et plusieurs kits PoS (MiyuPosInventory, MiyuPosAnalytics, MiyuPosLoyalty, MiyuPosKitchen, MiyuPosPayment).
- **Décision (2026-01-30) :** La section « Alignement MIP » est **obligatoire** pour tous les toolkits. Une section courte a été ajoutée à toutes les Doc Fondatrices qui n’en disposaient pas, avec lien vers le [Protocole MIP v1](../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md).

### 3.4 Nomenclature des fichiers (règles documentation)

- **Conformité :** Oui. Format `<Sujet> - <Detail>.md` ou `MiyuXXX - Documentation Fondatrice.md`, sans accents dans les noms de fichiers, cohérent avec la règle « PREFIX - SUJET DETAIL » (préfixe implicite docs/tools).
- **Arborescence :** `docs/tools/<MiyuXXX>/` avec `_index.md`, Doc Fondatrice, Reference Outils, `contracts/governance/` respectée pour les kits « complets ».

---

## 4. Niveau de sécurité

### 4.1 Cohérence avec le Glossaire

Le [Glossaire](../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) définit :
- **Niveaux de sécurité (0–4) :** 0 = Public, 1 = Standard, 2 = Sensitive, 3 = Critical, 4 = Highest.
- **États de confiance (T0–T4) :** T0 = Normal, T1 = Instable, T2 = Dégradé, T3 = Restreint, T4 = Bloqué.  
Dans les Doc Fondatrices, les « états autorisés / interdits » utilisent en général les libellés **HEALTHY**, **DEGRADED**, **SECURITY_LOCKDOWN**, **MAINTENANCE**, ce qui est cohérent avec une traduction opérationnelle des états WorrySentinel/Caring Nanny.

### 4.2 Constat

- **Niveaux de sécurité du kit :** tous les kits documentent un niveau (ou une fourchette, ex. « 1 à 2 », « 0 à 2 »). Les fourchettes sont justifiées par le type de données (public, personnel, sensible, critique).
- **États autorisés :** `HEALTHY`, `DEGRADED` systématiques.
- **États interdits :** `SECURITY_LOCKDOWN`, `MAINTENANCE` systématiques ; certains kits ajoutent « selon politique WorrySentinel » ou renvoient au Toolkit Composition Contract (MiyuAuth, MiyuWeb, MiyuClock, MiyuBooking, MiyuShipping).
- **Kits à sensibilité élevée (niveau 2–3 ou 3) :** MiyuAuth (2 ou 3), MiyuSQL (2), MiyuPM (2), MiyuSocialMessaging (2), MiyuModerationForum (2–3), MiyuSocialModeration (2–3), MiyuWebwayParticipant (2–3), MiyuWebwayTracker (2–3). Cohérent avec la nature des données (identité, données utilisateur, modération, réseau).

### 4.3 Recommandation

- Conserver la convention actuelle (niveau 0–4, états HEALTHY/DEGRADED/SECURITY_LOCKDOWN/MAINTENANCE).
- Pour les kits à niveau 3 ou 4, envisager un renvoi explicite au contrat Security (comme MiyuAuth, MiyuWeb) ou à la doctrine sécurité du projet si elle existe.

---

## 5. Guides d’implémentation

### 5.1 État des lieux

**Template commun :** [docs_tools - Reference Implementation Guidelines Template](./docs_tools%20-%20Reference%20Implementation%20Guidelines%20Template.md) — à adapter par kit dans `<MiyuXXX>/implementation/MiyuXXX - Reference Implementation Guidelines.md`.

| Kit | Reference Implementation Guidelines |
|-----|-------------------------------------|
| MiyuAuth | Oui — [MiyuAuth - Reference Implementation Guidelines](./MiyuAuth/implementation/MiyuAuth%20-%20Reference%20Implementation%20Guidelines.md) |
| MiyuWeb | Oui — [MiyuWeb - Reference Implementation Guidelines](./MiyuWeb/implementation/MiyuWeb%20-%20Reference%20Implementation%20Guidelines.md) |
| MiyuSQL | Oui — [MiyuSQL - Reference Implementation Guidelines](./MiyuSQL/implementation/MiyuSQL%20-%20Reference%20Implementation%20Guidelines.md) |
| MiyuForum | Oui — [MiyuForum - Reference Implementation Guidelines](./MiyuForum/implementation/MiyuForum%20-%20Reference%20Implementation%20Guidelines.md) |
| MiyuPM | Oui — [MiyuPM - Reference Implementation Guidelines](./MiyuPM/implementation/MiyuPM%20-%20Reference%20Implementation%20Guidelines.md) |
| MiyuNotify | Oui — [MiyuNotify - Reference Implementation Guidelines](./MiyuNotify/implementation/MiyuNotify%20-%20Reference%20Implementation%20Guidelines.md) |
| MiyuSearch | Oui — [MiyuSearch - Reference Implementation Guidelines](./MiyuSearch/implementation/MiyuSearch%20-%20Reference%20Implementation%20Guidelines.md) |
| MiyuWebwayParticipant | Oui — [MiyuWebwayParticipant - Reference Implementation Guidelines](./MiyuWebwayParticipant/implementation/MiyuWebwayParticipant%20-%20Reference%20Implementation%20Guidelines.md) |
| MiyuWebwayTracker | Oui — [MiyuWebwayTracker - Reference Implementation Guidelines](./MiyuWebwayTracker/implementation/MiyuWebwayTracker%20-%20Reference%20Implementation%20Guidelines.md) |
| MiyuBilling, MiyuBooking, MiyuCMS, MiyuMedia, MiyuShipping, MiyuStore, MiyuWidgets, MiyuInvoice, MiyuComptaLedger, MiyuExpense, MiyuTreasury | Oui — guides ajoutés 2026-01-30 |
| **Tous les autres kits (28)** | À venir — s'appuyer sur le template et la Doc Fondatrice + Reference Outils + Contrat Governance |

### 5.2 Contenu type des guides existants (MiyuWeb exemple)

- Statut du document (informatif, non normatif).
- Principes à respecter (bornes BOUND-*, pas de décision ALLOW/DENY, pas d’accès direct à la base, sanitization/CSP, dépendances).
- Liste des contrats sources (Doc Fondatrice, KindMother Integration, Tool Governance Compliance, Security and States, etc.).
- Alignement MIP/MSCM (balisage à l’implémentation).
- Références (MIP v1, contrats).

### 5.3 Recommandation

- **Option A (recommandée) :** Rédiger un **guide d’implémentation type** (template) commun à tous les toolkits, puis l’adapter par kit prioritaire (ex. MiyuForum, MiyuPM, MiyuNotify, MiyuSearch, MiyuWebwayParticipant, MiyuWebwayTracker).
- **Option B :** Pour les kits sans guide dédié, ajouter dans la Doc Fondatrice ou dans l’_index une phrase du type : « Guide d’implémentation : à venir ; en attendant, s’appuyer sur la Documentation Fondatrice, la Reference Outils et le Tool Governance Compliance Contract. »
- **Option C :** Créer des « Reference Implementation Guidelines » minimales (1–2 pages) par domaine (forum, social, webway, compta, POS, etc.) plutôt que par kit, pour limiter la duplication.

**Vérification prêt implémentation (2026-01-30) :** Le rapport [docs_tools - Verification Pret Implementation Bornes](./docs_tools%20-%20Verification%20Pret%20Implementation%20Bornes.md) recense les kits « prêts » (bornes explicites) vs « avec précautions » (pas de guide ni contrat Boundary) et recommande de s'appuyer sur le template Reference Implementation Guidelines (BOUND-*) avant toute implémentation.

---

## 6. Tableau de conformité (résumé)

| Critère | Conforme | Partiel | Non conforme | Non applicable |
|---------|----------|---------|--------------|----------------|
| Structure Doc Fondatrice (Contexte → Références) | 49 kits | — | — | — |
| Format ToolkitId `toolkit.<domain>.<name>` | 49 kits | — | — | — |
| Flux gouvernance (renvoi Tools et Toolkits + spécificité) | 49 kits | — | — | — |
| Tool Governance Compliance Contract (renvoi template + spécificités) | 42 kits | — | 7 kits sans contrat* | — |
| Section Niveau de sécurité et états | 49 kits | — | — | — |
| Relation KindMother / WriteIntent | 49 kits | — | — | — |
| Section Alignement MIP (ou N/A) | 49 kits | — | — | ✅ Uniformisé (2026-01-30) |
| Lien Doc Fondatrice → Contrat conformité | 3 kits | — | 46 kits | — |
| Reference Implementation Guidelines | 9 kits + template | — | 40 kits (à venir) | — |

\* Kits sans contrat governance dans le tableau complétude actuel : MiyuCMS, MiyuMedia, MiyuWidgets, MiyuStore, MiyuShipping, MiyuBooking, MiyuBilling (selon `_index.md` : « Prévue » Reference Outils, « — » Contrat Governance). À confirmer si politique projet exige un contrat pour tous.

---

## 7. Actions recommandées (par priorité)

### Priorité haute

1. **Décider de l’obligation MIP** : ~~Si MIP est obligatoire pour tous les toolkits, ajouter une section « Alignement MIP »…~~ **✅ Fait (2026-01-30)** : décision = obligatoire ; section « Alignement MIP » ajoutée à toutes les Doc Fondatrices qui n’en avaient pas (29 kits), avec lien vers le [Protocole MIP v1](../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md).
2. **Clarifier Doc Fondatrice ↔ Contrat **✅ Fait (2026-01-30)**** : Ajouter dans chaque Doc Fondatrice disposant d’un Tool Governance Compliance Contract la phrase : « Les obligations de conformité détaillées sont dans [MiyuXXX - Tool Governance Compliance Contract](./contracts/governance/...). »

### Priorité moyenne

3. **Uniformiser la numérotation** : Corriger les Doc Fondatrices où la section Références est numérotée 10 sans section 9 (ex. MiyuTreasury), ou standardiser sur § 9 Références / § 9 Alignement MIP + § 10 Références.
4. **Contrats governance pour kits « minimal »** : Si la politique projet l’exige, ajouter un Tool Governance Compliance Contract pour MiyuCMS, MiyuMedia, MiyuWidgets, MiyuStore, MiyuShipping, MiyuBooking, MiyuBilling (renvoi template + 1–2 obligations spécifiques).
5. **Guides d’implémentation** : **Fait (2026-01-30)** : template « Reference Implementation Guidelines » et l’appliquer en priorité aux kits critiques Étendre à d'autres kits selon priorité.

### Priorité basse

6. **Référence croisée sécurité** : Pour les kits niveau 3 ou 4, ajouter un renvoi au contrat Security ou à la doctrine sécurité si disponible.
7. **Vérifier les liens MIP** : S’assurer que tous les liens vers le protocole MIP v1 pointent vers le même chemin canonique.

---

## 8. Références

| Document | Lien |
|----------|------|
| docs/tools - Audit Documentation | [docs_tools - Audit Documentation](./docs_tools%20-%20Audit%20Documentation.md) |
| Tools et Toolkits (référence conceptuelle) | [Miyukini Conceptual References - Tools et Toolkits](../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md) |
| Glossaire (sécurité, états, niveaux) | [Miyukini Conceptual References - Glossaire](../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) |
| Master Butler - Tool Governance Compliance Template | [Master Butler - Tool Governance Compliance Template](../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Compliance%20Template.md) |
| Protocole MIP v1 | [Miyukini Prompt Protocol - MIP v1 MSCM Index Protocol](../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md) |
| Index docs/tools | [docs/tools/_index.md](./_index.md) |

---

**Date du rapport :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document d’audit — qualité, conformité, sécurité, implémentation
