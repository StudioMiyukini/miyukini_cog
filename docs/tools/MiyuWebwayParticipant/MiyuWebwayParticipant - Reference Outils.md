# MiyuWebwayParticipant — Reference Outils

## Contexte

Ce document liste les **Outils MWS** composant le Kit **MiyuWebwayParticipant** (`toolkit.webway.participant`). Chaque outil est une capacité atomique gouvernée ; les décisions (annoncer, découvrir, politique) relèvent des Cores (StrongFather, Border Guard, WorrySentinel).

**Référence :** [MiyuWebwayParticipant - Documentation Fondatrice](./MiyuWebwayParticipant%20-%20Documentation%20Fondatrice.md)

## Portée / Scope

- Liste exhaustive des Outils MWS du Kit MiyuWebwayParticipant : ToolId, action, niveau de sécurité.
- Invariants : le Kit exécute les intentions fournies par les Cores ; il ne décide pas.

---

## Liste des outils MWS

| ToolId | Action | Niveau sécurité | Note |
|--------|--------|-----------------|------|
| `mws.declaration.build` | Construire un message de déclaration conforme MWS | 2 | Exécution seule ; pas de décision |
| `mws.declaration.sign` | Signer une déclaration | 2–3 | Exécution seule ; clé gouvernée |
| `mws.declaration.validate` | Valider le format d'une déclaration | 2 | Exécution seule |
| `mws.declaration.verify` | Vérifier la signature d'une déclaration | 2 | Exécution seule |
| `mws.transport.send` | Envoyer un message vers une adresse | 2 | Exécution seule ; adresse fournie par Cores |
| `mws.discovery.request.build` | Construire une requête de découverte | 2 | Exécution seule |
| `mws.discovery.request.send` | Envoyer une requête de découverte | 2 | Exécution seule ; Trackers fournis par Cores |
| `mws.cog_list.get` | Lire la liste locale de COGs | 2 | Lecture |
| `mws.cog_list.update` | Mettre à jour une entrée dans la liste locale | 2 | Écriture liste locale |
| `mws.cog_list.merge` | Fusionner une liste reçue avec la liste locale | 2 | Règle de fusion fournie par Cores |
| `mws.port.check` | Vérifier si un port est exclus MWS | 1–2 | Exécution seule ; déterministe |
| `mws.address.tracker_default` | Résoudre l'adresse Tracker (port 21000) | 1–2 | Exécution seule ; déterministe |

---

**Invariant :** Le Kit ne décide pas ; il exécute les intentions fournies par les Cores via BondingBrother.
