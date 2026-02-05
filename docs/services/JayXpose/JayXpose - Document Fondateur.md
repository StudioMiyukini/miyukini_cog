# JayXpose — Document fondateur

## Contexte

**JayXpose** est le service Miyukini dédié au **profil exposant** et au **site vitrine** pour artisans, artistes et petites marques. Il permet de constituer une vitrine en ligne (catalogue, contact, portfolio, lien vers réservation ou boutique) et de **s’intégrer dans JayFestival** : la fiche exposant et le répertoire des exposants de JayFestival peuvent s’appuyer sur JayXpose ; un exposant peut avoir une vitrine JayXpose et participer à des éditions JayFestival avec le même profil.

Ce document est le **document fondateur** du service : il en fixe la raison d’être, la portée, les principes directeurs et l’intégration avec JayFestival. Il s’adresse aux équipes produit, technique et aux parties prenantes.

## Portée / Scope

- **Périmètre** : Définition du service JayXpose, positionnement, intégration avec JayFestival, vitrine autonome.
- **Hors périmètre** : Spécifications techniques détaillées, contrats d’API, implémentation (référencés dans d’autres documents).
- **Références** : Glossaire Miyukini, document fondateur JayFestival, [Interpolarité des services Jay](../../reference/Miyukini%20Conceptual%20References%20-%20Interpolarite%20Services%20Jay.md).

---

## 1. Raison d’être

### 1.1 Proposition de valeur

**JayXpose** permet à des **exposants** (artisans, artistes, petites marques) de :

- **Constituer un profil exposant** : identité, catalogue produits ou réalisations, contact, portfolio.
- **Exposer une vitrine en ligne** : site vitrine accessible au public, avec lien possible vers réservation (JayRDV) ou boutique (Miyustore).
- **Participer à des événements JayFestival** avec le **même profil** : la fiche exposant dans JayFestival peut s’appuyer sur les données JayXpose ; un exposant a une seule identité vitrine et peut la réutiliser pour plusieurs éditions.

La **vitrine autonome** permet d’utiliser JayXpose sans lien avec un festival : site vitrine seul, sans candidature ni édition.

### 1.2 Positionnement

| Mode | Description |
|------|-------------|
| **Intégré JayFestival** | Profil JayXpose alimente la fiche exposant et le répertoire des exposants de JayFestival ; un exposant peut participer à plusieurs éditions avec le même profil. |
| **Vitrine autonome** | Site vitrine seul, sans événement festival ; utile pour artisans ou marques qui veulent une présence en ligne sans s’inscrire à un festival. |

---

## 2. Principes directeurs

| Principe | Description |
|----------|-------------|
| **Gouvernance** | Le service fonctionne sous gouvernance COG : StrongFather, KindMother, Master Butler, WorrySentinel. |
| **Réutilisabilité** | S’appuyer sur les Kits d’outils Miyukini existants (Miyauth, Miyuprofile, Miyucms, Miyumedia, Miyucontacts, etc.) et définir les Opérateurs et Kits spécifiques « vitrine exposant ». |
| **Interpolarité** | Conçu pour s’intégrer dans JayFestival ; les couplages sont explicites et gouvernés (Mandats de Permission, niveaux de sécurité). |

---

## 3. Intégration et interpolarité

### 3.1 JayXpose dans JayFestival

- La **fiche exposant** de JayFestival peut s’appuyer sur le profil JayXpose (données vitrine, catalogue, contact).
- Le **répertoire des exposants** (annuaire plateforme ou par événement) peut afficher les vitrines JayXpose.
- Un **exposant** peut avoir une vitrine JayXpose et **participer à des éditions JayFestival** avec le même profil ; pas de duplication d’identité ni de contenu vitrine.

### 3.2 Vitrine autonome

- JayXpose peut être utilisé **sans JayFestival** : site vitrine seul, sans candidature ni édition.
- Les données (profil, contenu vitrine) sont gouvernées ; la résidence et le niveau de sécurité sont définis par le contrat du service et le contexte (JayFestival vs. autonome).

### 3.3 Référence interpolarité

Voir [Miyukini Conceptual References - Interpolarite Services Jay](../../reference/Miyukini%20Conceptual%20References%20-%20Interpolarite%20Services%20Jay.md) pour le principe global et les couplages entre services Jay.

---

## 4. Niveaux de sécurité (orientation)

Les données du profil exposant (identité, catalogue, contact) sont au moins niveau **Sensitive (2)** lorsqu’elles sont personnelles ou commerciales. La résidence (COG de référence) et les règles d’accès sont à préciser dans un document dédié (niveaux de sécurité, politique de résidence), aligné avec le Glossaire et la Politique de résidence des données sensibles.

---

## 5. Prochaines étapes (orientation)

1. **Fonder** : Valider ce document fondateur et le diffuser.
2. **Spécifier** : Documenter les Opérateurs et Kits JayXpose (profil, vitrine, catalogue, liaison JayFestival).
3. **Intégration** : Formaliser le contrat d’intégration avec JayFestival (fiche exposant, répertoire).
4. **Implémentation** : Développer les Opérateurs et Kits en s’appuyant sur les Cores.

---

## 6. Références

| Document | Rôle |
|----------|------|
| [Miyukini Conceptual References — Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) | Terminologie (Opérateur, Mandat, COG, Niveaux de sécurité). |
| [JayFestival - Document Fondateur](../JayFestival/JayFestival%20-%20Document%20Fondateur.md) | Service dans lequel JayXpose s’intègre (fiche exposant, répertoire). |
| [Miyukini Conceptual References - Interpolarite Services Jay](../../reference/Miyukini%20Conceptual%20References%20-%20Interpolarite%20Services%20Jay.md) | Principe d’interpolarité et couplage JayXpose ↔ JayFestival. |

---

**Document** : JayXpose — Document fondateur  
**Version** : 1.0  
**Statut** : Document de référence — non contractuel pour l’implémentation.
