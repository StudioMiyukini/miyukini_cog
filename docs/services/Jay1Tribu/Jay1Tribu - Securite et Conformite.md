# Jay1Tribu â€” SÃ©curitÃ© et ConformitÃ©

## Contexte

**Jay1Tribu** est un Service Inter-COG dont les donnÃ©es (messages, fichiers, images) sont sensibles. Ce document dÃ©crit les exigences de **sÃ©curitÃ©** (chiffrement, contrÃ´les d'accÃ¨s, modÃ¨le de menaces) et de **conformitÃ©** (gouvernance Cores, Lois d'Autonomie, bonnes pratiques).

## PortÃ©e / Scope

- **Applicable Ã  :** Conception sÃ©curitÃ©, chiffrement, contrÃ´les d'accÃ¨s, audit, conformitÃ©.
- **Audience :** Ã‰quipes sÃ©curitÃ©, architectes, dÃ©veloppeurs, auditeurs.
- **Statut :** Document normatif de rÃ©fÃ©rence sÃ©curitÃ©.

### Hors pÃ©rimÃ¨tre

- Choix prÃ©cis d'algorithmes et de bibliothÃ¨ques (Ã  figer en implÃ©mentation avec WorrySentinel).

---

## 1. Classification des donnÃ©es

| Classe | Exemples | Niveau de sensibilitÃ© | RÃ¨gle |
|--------|----------|------------------------|-------|
| **Contenu de message** | Texte, piÃ¨ces jointes, images | Ã‰levÃ© | Transit cryptÃ© obligatoire ; au repos selon politique WorrySentinel / KindMother (chiffrement disque ou DB). |
| **MÃ©tadonnÃ©es de conversation** | Identifiants salon, tribu, participants, horodatages | Moyen | Transit cryptÃ© ; exposition minimale (seuls les participants et le MWS pour le routage, sans lecture du contenu). |
| **Liste d'amis** | Identifiants COG, pseudos | Moyen | Local uniquement ; pas de partage avec des tiers. |
| **ParamÃ¨tres tribu** | RÃ´les, rÃ¨gles d'invitation | Moyen | Local + Ã©change Inter-COG cryptÃ© pour synchronisation entre membres. |
| **PrÃ©sence** | En ligne / hors ligne | Faible | Fournie par le MWS ; Jay1Tribu consomme sans la dupliquer. |

**Principe :** Toute donnÃ©e Ã©changÃ©e entre COGs est considÃ©rÃ©e comme sensible jusqu'Ã  preuve contraire ; le dÃ©faut est le chiffrement.

---

## 2. Chiffrement

### 2.1 En transit

| Exigence | Description |
|----------|-------------|
| **Obligation** | L'ensemble des donnÃ©es qui transitent entre COGs (messages, fichiers, images, mÃ©tadonnÃ©es sensibles) est cryptÃ©. Aucune exception. |
| **PÃ©rimÃ¨tre** | Transport via le MWS : le canal (TLS ou Ã©quivalent) et/ou le payload applicatif (chiffrement de bout en bout) doivent garantir la confidentialitÃ© et l'intÃ©gritÃ©. |
| **SpÃ©cification technique** | Les mÃ©canismes (chiffrement de bout en bout, Ã©change de clÃ©s, gestion des identitÃ©s) seront dÃ©finis dans une spÃ©cification technique et validÃ©s avec WorrySentinel et Border Guard. |

### 2.2 Au repos

| Exigence | Description |
|----------|-------------|
| **AutoritÃ©** | La classification et le chiffrement au repos dans chaque COG relÃ¨vent de **WorrySentinel** et **KindMother** (niveaux de sÃ©curitÃ©, politique de rÃ©sidence des donnÃ©es). |
| **CohÃ©rence** | Si le COG applique le chiffrement de base de donnÃ©es (ex. SQLCipher via kindmother-db-key), les donnÃ©es Jay1Tribu sont incluses dans ce pÃ©rimÃ¨tre. |
| **Pas de stockage central** | Puisqu'il n'y a pas d'archives centrales, la question du chiffrement au repos ne se pose que localement (chaque COG). |

---

## 3. ContrÃ´les d'accÃ¨s et permissions

| Niveau | MÃ©canisme | RÃ´le |
|--------|-----------|------|
| **DÃ©cision** | StrongFather | Autorise ou refuse toute action (envoi, crÃ©ation tribu, invitation, attribution de rÃ´les). |
| **CapacitÃ©s** | Master Butler | Registre des permissions : qui peut crÃ©er un salon, inviter, Ãªtre Chef de tribu, envoyer des fichiers, etc. |
| **FrontiÃ¨res** | Border Guard | DÃ©finit qui peut communiquer avec qui (COGs de confiance, rÃ¨gles Inter-COG). |
| **Persistance** | KindMother | Valide les WriteIntent ; refuse toute Ã©criture non autorisÃ©e ou non conforme. |
| **SÃ©curitÃ©** | WorrySentinel | Niveaux de sÃ©curitÃ© des contenus, rÃ¨gles de rÃ©tention, politique de chiffrement. |

**RÃ¨gle :** Aucun accÃ¨s en Ã©criture ou en lecture sensible sans passage par les Cores (via BondingBrother).

---

## 4. ModÃ¨le de menaces (rÃ©sumÃ©)

| Menace | Mitigation |
|--------|------------|
| **Interception du transit** | Chiffrement systÃ©matique (TLS et/ou E2E). |
| **Archives volÃ©es ou exposÃ©es** | Pas d'archives centrales ; au repos gouvernÃ© par WorrySentinel/KindMother (chiffrement DB si activÃ©). |
| **Usurpation d'identitÃ© COG** | Authentification et identitÃ© COG gÃ©rÃ©es par le MWS et les Cores ; Jay1Tribu s'appuie sur ces garanties. |
| **AccÃ¨s non autorisÃ© Ã  un salon / tribu** | Permissions et rÃ´les gouvernÃ©s par Master Butler / StrongFather ; Border Guard pour les frontiÃ¨res. |
| **ModÃ©ration / abus** | TAMR : points d'intervention humaine (modÃ©ration, litiges, rÃ©vocation d'accÃ¨s). |
| **DÃ©ni de service** | Caring Nanny : observation de l'Ã©tat ; rÃ©duction ou suspension possible en environnement dÃ©gradÃ©. |

---

## 5. ConformitÃ©

### 5.1 Lois d'Autonomie

Jay1Tribu respecte les Lois d'Autonomie Miyukini (LOI-2, LOI-3, LOI-4, LOI-6, LOI-7) ; voir [Contraintes et Invariants](./Jay1Tribu%20-%20Contraintes%20et%20Invariants.md).

### 5.2 Contraintes non nÃ©gociables

Les contraintes C-1 Ã  C-8 et les invariants documentÃ©s dans [Jay1Tribu - Contraintes et Invariants](./Jay1Tribu%20-%20Contraintes%20et%20Invariants.md) sont impÃ©ratives. Toute Ã©volution doit les prÃ©server.

### 5.3 Audit et traÃ§abilitÃ©

- Les dÃ©cisions de gouvernance (StrongFather, KindMother) peuvent faire l'objet de logs d'audit selon la politique du COG (WorrySentinel, TAMR).
- Aucun contenu de message ne doit Ãªtre loguÃ© en clair ; seuls les Ã©vÃ©nements (envoi, rÃ©ception, crÃ©ation salon, etc.) et les identifiants techniques peuvent Ãªtre tracÃ©s, selon la politique de confidentialitÃ©.

---

## 6. Bonnes pratiques de dÃ©veloppement

| Pratique | Description |
|----------|-------------|
| **Pas de secret en clair** | ClÃ©s et secrets (chiffrement, authentification) ne doivent jamais Ãªtre stockÃ©s ou loguÃ©s en clair. |
| **DÃ©pendances Ã  jour** | BibliothÃ¨ques de chiffrement et de communication maintenues et patchÃ©es (WorrySentinel, Ever Buddy). |
| **Principe du moindre privilÃ¨ge** | Les OpÃ©rateurs et Outils n'accÃ¨dent qu'aux donnÃ©es et capacitÃ©s strictement nÃ©cessaires. |
| **Revue sÃ©curitÃ©** | Les changements affectant le chiffrement, le transport ou les permissions font l'objet d'une revue alignÃ©e avec WorrySentinel et Border Guard. |

---

## 7. RÃ©fÃ©rences

| Document | RÃ´le |
|----------|------|
| [Jay1Tribu - Contraintes et Invariants](./Jay1Tribu%20-%20Contraintes%20et%20Invariants.md) | Contraintes C-1 Ã  C-8, invariants. |
| [Jay1Tribu - Document Conceptuel](./Jay1Tribu%20-%20Document%20Conceptuel.md) | Concepts, gouvernance Cores. |
| [Security â€” Liste des Mesures de SÃ©curitÃ©](..//..//cores//WorrySentinel//_index.md) | RÃ©fÃ©rence sÃ©curitÃ© Miyukini COG et MWS. |

---

**Document** : Jay1Tribu â€” SÃ©curitÃ© et ConformitÃ©  
**Version** : 1.0  
**Date** : 2026-02-15  
**Statut** : Document normatif

