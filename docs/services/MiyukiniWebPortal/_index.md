# Miyukini Web Portal — Point d'entrée Web du COG

## Contexte

**Miyukini Web Portal** (le **Portail**) est le **Service Fondamental** qui expose les surfaces web des Services du COG aux utilisateurs externes. Il constitue l'équivalent de Miyukini Central pour le web : là où Central est le point d'entrée pour l'utilisateur du COG, le Portail est le point d'entrée pour les utilisateurs externes accédant via un navigateur.

**Règle canonique :**

> **Central = COG, Portail = Web.**

Le Portail fait partie intégrante de l'environnement versionné du COG.

## Documentation principale

| Document | Rôle |
|----------|------|
| [Miyukini Web Portal - Document Fondateur](./Miyukini%20Web%20Portal%20-%20Document%20Fondateur.md) | Vision, scope, positionnement, architecture, relation avec Central. |
| [Miyukini Web Portal - Surface Web Implementation et Gouvernance](./Miyukini%20Web%20Portal%20-%20Surface%20Web%20Implementation%20et%20Gouvernance.md) | Comment implémenter, guider, borner et normer une surface web. |

## En bref

| Aspect | Valeur |
|--------|--------|
| **Type** | Service Fondamental (Opérateur d'Interface, Strate 7) |
| **Rôle** | Point d'entrée web pour les utilisateurs externes |
| **Cible** | Utilisateurs externes (clients, visiteurs, prospects) sans COG |
| **Relation** | Équivalent de Central, mais pour le web |
| **Gouvernance** | BorderGuard + Visa + Mandat Public d'Accès |

## Services exposés via le Portail (Type 2)

Les Services de **Type 2** (à surface web externe) exposent leurs façades publiques via le Portail :

| Service | Surface exposée |
|---------|-----------------|
| **JayXpose** | Vitrine, e-shop, annuaire exposants, blog public |
| **JayFestival** | Catalogue événements, inscriptions, billets, espace visiteur |
| **JayRDV** | Page de réservation, parcours guest, annulation/modification |
| **JayKonta** | Portail client (consultation factures, paiement) |

## Architecture simplifiée

```
┌─────────────────────────────────────────────────────────────┐
│  Utilisateur externe (navigateur web)                        │
│  https://mon-commerce.miyukini.cog / https://kine-rdv.cog   │
└───────────────────────────┬─────────────────────────────────┘
                            │ HTTPS
                            ▼
┌─────────────────────────────────────────────────────────────┐
│              Miyukini Web Portal (Portail)                   │
│  · Routage vers les surfaces des Services (Type 2)          │
│  · Identification et fichage des connexions entrantes        │
│  · Mandat Public d'Accès                                     │
└───────────────────────────┬─────────────────────────────────┘
                            │ BondingBrother
                            ▼
┌─────────────────────────────────────────────────────────────┐
│  BorderGuard · StrongFather · KindMother · WorrySentinel     │
│  (Gouvernance, sécurité, persistance)                        │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│  Services (JayXpose, JayFestival, JayRDV, JayKonta, ...)     │
│  (Exposent leurs capacités au Portail)                       │
└─────────────────────────────────────────────────────────────┘
```

## Ce que le Portail N'EST PAS

- **Pas un serveur central unique** qui affiche tous les COGs — chaque COG expose **son** Portail
- **Pas un remplacement de Central** — Central reste le point d'entrée pour l'utilisateur COG
- **Pas une porte ouverte** — tout accès passe par BorderGuard, identification et Mandat Public

## Voir aussi

- [Miyukini Central - Hub de gestion des Services](../../reference/Miyukini%20Conceptual%20References%20-%20Miyukini%20Central%20Hub%20Services.md)
- [Types de Services et Espaces](../../reference/Miyukini%20Conceptual%20References%20-%20Types%20de%20Services%20et%20Espaces.md)
- [Glossaire — Façade Publique Gouvernée](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)
- [Glossaire — Utilisateur Externe](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)
