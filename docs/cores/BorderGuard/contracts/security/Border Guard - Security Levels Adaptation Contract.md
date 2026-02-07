# Border Guard - Security Levels Adaptation Contract

## 1. Contexte

Ce document définit comment **Border Guard adapte son comportement** selon les cinq niveaux de sécurité Miyukini (0-4). Il spécifie formellement les règles d'adaptation des frontières, la rigueur de classification, les seuils de détection, et les comportements en dégradation selon le profil de risque déclaré.

**Document fondateur :** [Border Guard - Documentation Fondatrice](../../foundation/Border%20Guard%20-%20Documentation%20Fondatrice.md)

**Référence principale :** [Miyukini Conceptual References - Security Levels](../../../../reference/Miyukini%20Conceptual%20References%20-%20Security%20Levels.md) (Section 7.3)

**Statut contractuel :** Ce document est **contractuel, normatif, et non négociable**. Il dérive directement de la Documentation Fondatrice et des Security Levels.

---

## 2. Portée / Scope

- **Applicable à :** Toutes les définitions de frontières et classifications de Border Guard
- **Responsable :** Border Guard (adaptation des frontières selon niveau sécurité)
- **Consommateurs :** Tous les Opérateurs déclarant un niveau de sécurité
- **Ne couvre pas :** L'implémentation technique des contrôles (responsabilité des adaptateurs)

---

## 3. Principe fondamental

### 3.1 La sécurité est un paramètre de gouvernance

**Un Opérateur :**
- ✅ Déclare son profil de risque (niveau 0-4)
- ❌ N'implémente jamais sa propre sécurité de frontière
- ✅ Subit le niveau imposé par l'écosystème

**Border Guard adapte automatiquement :**
- La rigueur des frontières
- Les critères de classification de confiance
- Les seuils de détection de menaces
- Les règles de franchissement

### 3.2 Indépendance niveau de sécurité / niveau de confiance

| Concept | Défini par | Nature |
|---------|------------|--------|
| **Niveau de sécurité (0-4)** | Opérateur | Profil de risque déclaré |
| **Niveau de confiance (T0-T4)** | Caring Nanny | État d'intégrité du système |
| **Niveau de confiance source** | Border Guard | Classification (TRUSTED/VERIFIED/UNKNOWN/HOSTILE) |

**Ces trois concepts sont indépendants mais interconnectés.**

---

## 4. Les cinq niveaux de sécurité et Border Guard

### 4.1 Niveau 0 — PUBLIC / DISPLAY

**Contexte :** Site vitrine, données publiques, WebApp sans état critique.

**Philosophie :** "Si ça casse, ce n'est pas grave."

| Aspect | Comportement Border Guard |
|--------|---------------------------|
| **Frontières** | Assouplies |
| **Classification** | Simplifiée |
| **Détection hostile** | Seuil haut (tolérant) |
| **Dégradation** | Douce |
| **TTL VERIFIED** | Long (heures) |
| **Réévaluation TRUSTED** | Rare |

**Règles de frontière :**
- ✅ Franchissement libre sous conditions minimales
- ✅ Validation structurelle uniquement
- ❌ Pas de vérification stricte de contexte
- ✅ Traçabilité minimale

**Classification :**
- Critères `VERIFIED` : Assouplis
- Distribution `TRUSTED` : Largement distribuée
- Seuil `HOSTILE` : Patterns d'attaque évidents uniquement

### 4.2 Niveau 1 — STANDARD / CMS

**Contexte :** CMS, backoffice simple, contenu éditorial.

**Philosophie :** "On protège l'accès, pas le système."

| Aspect | Comportement Border Guard |
|--------|---------------------------|
| **Frontières** | Standard |
| **Classification** | Normale |
| **Détection hostile** | Seuil standard |
| **Dégradation** | Normale |
| **TTL VERIFIED** | Standard (minutes) |
| **Réévaluation TRUSTED** | Mensuelle |

**Règles de frontière :**
- ✅ Franchissement contrôlé
- ✅ Authentification simple requise pour zones protégées
- ✅ Contrôle d'intégrité périodique
- ✅ Traçabilité normale

**Classification :**
- Critères `VERIFIED` : Standard
- Distribution `TRUSTED` : Normale
- Seuil `HOSTILE` : Patterns d'attaque classiques

### 4.3 Niveau 2 — SENSITIVE DATA

**Contexte :** Données personnelles, comptes utilisateurs, profils, historique.

**Philosophie :** "On protège les données."

| Aspect | Comportement Border Guard |
|--------|---------------------------|
| **Frontières** | Renforcées |
| **Classification** | Renforcée |
| **Détection hostile** | Seuil bas (sensible) |
| **Dégradation** | Rapide |
| **TTL VERIFIED** | Court (minutes) |
| **Réévaluation TRUSTED** | Hebdomadaire |

**Règles de frontière :**
- ✅ Franchissement soumis à vérification stricte
- ✅ Signatures d'intentions pour données sensibles
- ✅ Contrôles de cohérence réguliers
- ✅ Traçabilité complète
- ✅ Détection d'anomalies comportementales

**Classification :**
- Critères `VERIFIED` : Renforcés (contexte vérifié)
- Distribution `TRUSTED` : Restreinte
- Seuil `HOSTILE` : Patterns d'attaque + comportements anormaux

### 4.4 Niveau 3 — CRITICAL SYSTEM

**Contexte :** Auth, paiement, autorisations, décisions structurantes, cores internes.

**Philosophie :** "On protège le système avant l'UX."

| Aspect | Comportement Border Guard |
|--------|---------------------------|
| **Frontières** | Strictes |
| **Classification** | Stricte avec vérifications croisées |
| **Détection hostile** | Seuil très bas |
| **Dégradation** | Blocage rapide |
| **TTL VERIFIED** | Très court |
| **Réévaluation TRUSTED** | Quotidienne |

**Règles de frontière :**
- ✅ Zero-trust strict
- ✅ Signatures obligatoires pour tout franchissement sensible
- ✅ Vérifications croisées systématiques
- ✅ Gel partiel possible en cas de doute
- ✅ Dégradation rapide si anomalie

**Classification :**
- Critères `VERIFIED` : Stricts (historique + contexte + authentification forte)
- Distribution `TRUSTED` : Minimale (cores uniquement)
- Seuil `HOSTILE` : Moindre anomalie significative

### 4.5 Niveau 4 — HARDENED / ISOLATED

**Contexte :** Environnement isolé, hardware non fiable, contexte hostile, mode survie.

**Philosophie :** "On protège l'intégrité coûte que coûte."

| Aspect | Comportement Border Guard |
|--------|---------------------------|
| **Frontières** | Maximales (isolement strict) |
| **Classification** | Ultra-stricte, zéro tolérance |
| **Détection hostile** | Minimal (aucune tolérance) |
| **Dégradation** | Blocage progressif → total |
| **TTL VERIFIED** | Minimal |
| **Réévaluation TRUSTED** | Constante |

**Règles de frontière :**
- ✅ Contrôles continus
- ✅ Attestations régulières requises
- ✅ Très peu de franchissements autorisés
- ✅ Blocage progressif puis total si anomalie
- ❌ Aucune tolérance aux anomalies

**Classification :**
- Critères `VERIFIED` : Ultra-stricts (vérification continue)
- Distribution `TRUSTED` : Quasi nulle (isolement)
- Seuil `HOSTILE` : Toute anomalie = hostilité potentielle

---

## 5. Matrice d'adaptation des frontières

### 5.1 Perméabilité par niveau

| Type de frontière | Niveau 0 | Niveau 1 | Niveau 2 | Niveau 3 | Niveau 4 |
|-------------------|----------|----------|----------|----------|----------|
| **Externe** | Ouverte | Contrôlée | Contrôlée + vérif | Stricte | Fermée |
| **Interne** | Ouverte | Standard | Contrôlée | Stricte | Ultra-stricte |
| **Intégration** | Permissive | Standard | Renforcée | Stricte | Minimale/Aucune |

### 5.2 Règles de franchissement par niveau

| Règle | Niveau 0 | Niveau 1 | Niveau 2 | Niveau 3 | Niveau 4 |
|-------|----------|----------|----------|----------|----------|
| **Auth requise** | ❌ | ✅ Simple | ✅ Renforcée | ✅ Forte | ✅ Maximale |
| **Contexte validé** | ❌ | ❌ | ✅ | ✅ Strict | ✅ Continu |
| **Signature** | ❌ | ❌ | ✅ Optionnelle | ✅ Obligatoire | ✅ Cryptographique |
| **Vérification croisée** | ❌ | ❌ | ❌ | ✅ | ✅ Constante |
| **Attestation** | ❌ | ❌ | ❌ | ❌ | ✅ Régulière |

---

## 6. Adaptation de la classification de confiance

### 6.1 Critères VERIFIED par niveau

| Critère | Niveau 0-1 | Niveau 2 | Niveau 3 | Niveau 4 |
|---------|------------|----------|----------|----------|
| Authentification | Simple | Renforcée | Forte + MFA | Maximale |
| Contexte cohérent | Non requis | Requis | Strict | Continu |
| Historique sans incident | Non requis | Souhaité | Requis | Critique |
| Device validé | Non requis | Recommandé | Requis | Certifié |

### 6.2 Distribution TRUSTED par niveau

| Niveau de sécurité | Distribution TRUSTED |
|--------------------|---------------------|
| **0 - PUBLIC** | Large (commodité) |
| **1 - STANDARD** | Normale (équilibre) |
| **2 - SENSITIVE** | Restreinte (précaution) |
| **3 - CRITICAL** | Minimale (cores uniquement) |
| **4 - HARDENED** | Quasi nulle (isolement) |

### 6.3 Seuil de détection HOSTILE par niveau

| Niveau de sécurité | Seuil | Comportement |
|--------------------|-------|--------------|
| **0 - PUBLIC** | Haut | Patterns évidents uniquement |
| **1 - STANDARD** | Standard | Patterns classiques |
| **2 - SENSITIVE** | Bas | Patterns + comportements anormaux |
| **3 - CRITICAL** | Très bas | Moindre anomalie significative |
| **4 - HARDENED** | Zéro | Toute anomalie = hostilité potentielle |

---

## 7. Dégradation graduée selon niveau de sécurité

### 7.1 États de dégradation disponibles

| État | Description | Action Border Guard |
|------|-------------|---------------------|
| **Nominal** | Fonctionnement normal | Frontières normales |
| **Doute** | Suspicion légère | + Vérifications |
| **Suspect** | Suspicion confirmée | Frontières resserrées |
| **Critique** | Anomalie grave | Frontières minimales |
| **Compromis** | Compromission détectée | Frontières fermées |

### 7.2 États disponibles par niveau

| Niveau de sécurité | États disponibles |
|--------------------|-------------------|
| **0-1** | Nominal → Doute → Suspect |
| **2** | Nominal → Doute → Suspect → Critique → Compromis |
| **3-4** | Tous les états + blocage progressif/total |

### 7.3 Vitesse de dégradation

| Niveau de sécurité | Vitesse de dégradation |
|--------------------|------------------------|
| **0 - PUBLIC** | Lente (tolérance haute) |
| **1 - STANDARD** | Normale |
| **2 - SENSITIVE** | Rapide |
| **3 - CRITICAL** | Très rapide |
| **4 - HARDENED** | Immédiate |

### 7.4 Vitesse de restauration

| Niveau de sécurité | Restauration après dégradation |
|--------------------|-------------------------------|
| **0 - PUBLIC** | Rapide (commodité) |
| **1 - STANDARD** | Normale |
| **2 - SENSITIVE** | Progressive (prudence) |
| **3 - CRITICAL** | Lente (validation requise) |
| **4 - HARDENED** | Très lente (validation formelle) |

---

## 8. Impact sur les intégrations

### 8.1 Gouvernance des intégrations par niveau

| Niveau de sécurité | Intégrations autorisées |
|--------------------|------------------------|
| **0 - PUBLIC** | Toutes (responsabilité Opérateur) |
| **1 - STANDARD** | Standard (vérification basique) |
| **2 - SENSITIVE** | Certifiées (contrat requis) |
| **3 - CRITICAL** | Minimales (revue approfondie) |
| **4 - HARDENED** | Aucune ou quasi-aucune (isolement) |

### 8.2 Révocation d'intégration par niveau

| Niveau de sécurité | Seuil de révocation |
|--------------------|---------------------|
| **0 - PUBLIC** | Violation grave uniquement |
| **1 - STANDARD** | Violations répétées |
| **2 - SENSITIVE** | Violation confirmée |
| **3 - CRITICAL** | Suspicion de violation |
| **4 - HARDENED** | Moindre anomalie |

---

## 9. Combinaison niveau de sécurité + niveau de confiance système

### 9.1 Matrice de comportement

La combinaison du niveau de sécurité déclaré (0-4) et du niveau de confiance système (T0-T4) détermine le comportement de Border Guard :

| Confiance système | Niveau 0-1 | Niveau 2 | Niveau 3-4 |
|-------------------|------------|----------|------------|
| **T0 (Normal)** | Frontières normales | Frontières renforcées | Frontières strictes |
| **T1 (Surveillance)** | + Vérifications | + Signatures | + Vérifications croisées |
| **T2 (Dégradé)** | Suspect | Critique | Critique + gel |
| **T3 (Minimum)** | Critique | Compromis | Blocage progressif |
| **T4 (Survie)** | Compromis | Blocage | Blocage total |

### 9.2 Exemples concrets

**Opérateur Niveau 2 en T0 :**
- Frontières renforcées
- Classification renforcée
- Traçabilité complète

**Opérateur Niveau 2 en T2 :**
- État Critique
- Frontières minimales
- Franchissements limités

**Opérateur Niveau 4 en T1 :**
- Frontières strictes + vérifications croisées constantes
- Attestations requises
- Dégradation rapide au moindre doute

---

## 10. Protocoles de sécurité et adaptation

### 10.1 Protocoles temps réel

| Protocole | Adaptation par niveau |
|-----------|----------------------|
| **RT-SEC-1** (Session éphémère) | TTL session : long (N0-1) → minimal (N4) |
| **RT-SEC-2** (Auth en couches) | Couches : réduites (N0-1) → complètes (N3-4) |
| **RT-SEC-4** (Détection anomalie) | Seuil : haut (N0-1) → zéro (N4) |

### 10.2 Protocoles asynchrones

| Protocole | Adaptation par niveau |
|-----------|----------------------|
| **AS-SEC-2** (Signature locale) | Non requise (N0-1) → cryptographique (N4) |
| **AS-SEC-3** (Revalidation) | Partielle (N0-1) → complète (N3-4) |
| **AS-SEC-5** (Dégradation graduée) | Étapes : 3 (N0-1) → 5+ (N3-4) |

---

## 11. Invariants de ce contrat

### INV-SLAC-1 : Adaptation automatique

Border Guard **adapte toujours** son comportement au niveau de sécurité déclaré. Aucune exception manuelle n'est autorisée.

### INV-SLAC-2 : Niveau par défaut

En l'absence de déclaration explicite, le niveau de sécurité est **1 (STANDARD)**.

### INV-SLAC-3 : Pas de contournement

Un Opérateur ne peut **jamais** demander un comportement de frontière plus permissif que celui de son niveau déclaré.

### INV-SLAC-4 : Dégradation monotone

La dégradation suit toujours un chemin **monotone** (jamais de saut Nominal → Compromis sans passer par les états intermédiaires), sauf en cas de compromission flagrante en niveau 4.

### INV-SLAC-5 : Traçabilité des adaptations

Toute adaptation de frontière selon le niveau de sécurité est **traçable** avec le niveau déclaré et la raison.

---

## 12. Références croisées

### Invariants associés (Documentation Fondatrice - Section 7)

| Invariant | Énoncé | Relation |
|-----------|--------|----------|
| INV-BG-4 | Classification exhaustive | Adaptation des critères selon niveau |
| INV-BG-5 | Frontières explicites | Adaptation de la perméabilité selon niveau |
| INV-BG-6 | Règles déclaratives | Les règles d'adaptation sont déclaratives |
| INV-BG-10 | Neutralité conceptuelle | L'adaptation est conceptuelle, pas technique |

### Documents associés

| Document | Relation |
|----------|----------|
| [Border Guard - Documentation Fondatrice](../../foundation/Border%20Guard%20-%20Documentation%20Fondatrice.md) | Document source |
| [Miyukini Conceptual References - Security Levels](../../../../reference/Miyukini%20Conceptual%20References%20-%20Security%20Levels.md) | Définition des niveaux (Section 7.3) |
| [Border Guard - Trust Level Classification Contract](../boundaries/Border%20Guard%20-%20Trust%20Level%20Classification%20Contract.md) | Classification adaptée |
| [Border Guard - Crossing Rules Contract](../boundaries/Border%20Guard%20-%20Crossing%20Rules%20Contract.md) | Règles adaptées |
| [Border Guard - Threat Model Contract](./Border%20Guard%20-%20Threat%20Model%20Contract.md) | Seuils de détection adaptés |
| [Miyukini Conceptual References - Integrity Degradation System](../../../../reference/Miyukini%20Conceptual%20References%20-%20Integrity%20Degradation%20System.md) | Combinaison niveau sécurité / confiance |

### Documentation de securite centrale

| Document | Description |
|----------|-------------|
| [Security - Core Integration Map](../../../../security/architecture/Security%20-%20Core%20Integration%20Map.md) | Cartographie des roles securite des Cores, points de controle |
| [Doctrine Securite Fondamentale](../../../../reference/Miyukini%20Conceptual%20References%20-%20Doctrine%20Securite%20Fondamentale.md) | Fondation philosophique et architecturale de la securite |
| [Security - Invariants & Guarantees](../../../../security/contracts/governance/Security%20-%20Invariants%20&%20Guarantees.md) | Lois L1-L6, contraintes C1-C4, garanties par niveau |

### Role de BorderGuard dans le dispositif de securite

Selon le [Core Integration Map](../../../../security/architecture/Security%20-%20Core%20Integration%20Map.md), BorderGuard est le **Gardien des Frontieres** avec :
- Definition des frontieres : Delimite l'interne de l'externe (INV-BG-1)
- Classification de confiance : Attribue les niveaux trusted/verified/unknown/hostile (INV-BG-2)
- Regles de franchissement : Definit les conditions d'entree/sortie (INV-BG-3)
- Gouvernance des integrations : Controle les relations avec l'externe (INV-BG-4)

**Protocoles concernes :** RT-SEC-1, RT-SEC-2, RT-SEC-4, AS-SEC-2, NET-SEC-1

**Point de controle :** Couche SERVICES → CORES (entree) et CORES → SERVICES (sortie)

---

## 13. Synthèse contractuelle

### Garanties de ce contrat

Ce contrat garantit que :

1. **Adaptation automatique** — Border Guard adapte ses frontières selon le niveau déclaré
2. **Cohérence totale** — Comportement prévisible pour chaque niveau
3. **Pas de contournement** — Impossible de demander un comportement plus permissif
4. **Dégradation proportionnelle** — Vitesse de dégradation adaptée au risque
5. **Combinaison explicite** — Interaction claire entre niveau sécurité et confiance système
6. **Traçabilité** — Toute adaptation est traçable

### Phrase de synthèse

> **Border Guard adapte automatiquement la rigueur de ses frontières, ses critères de classification, et ses seuils de détection selon le niveau de sécurité déclaré (0-4), garantissant un comportement proportionnel au profil de risque sans jamais permettre de contournement.**

---

**Version :** 1.0  
**Date :** 2026-01-28  
**Statut :** Contrat — Normatif  
**Référence :** Border Guard v1.5, Security Levels v1.0 Section 7.3  
**Type :** Contrat d'adaptation de frontières selon niveau de sécurité
