# MiyuWebwayTracker — Reference Outils

## Contexte

Ce document liste les **Outils MWS** composant le Kit **MiyuWebwayTracker** (`toolkit.webway.tracker`). Chaque outil est une capacité atomique gouvernée ; les décisions (accepter, rejeter, filtrer) relèvent des Cores (Border Guard, WorrySentinel).

**Référence :** [MiyuWebwayTracker - Documentation Fondatrice](./MiyuWebwayTracker%20-%20Documentation%20Fondatrice.md)

## Portée / Scope

- Liste exhaustive des Outils MWS du Kit MiyuWebwayTracker : ToolId, action, niveau de sécurité.
- Invariants : le Kit exécute validations, vérifications et filtrages ; les politiques (accepter, rejeter) relèvent des Cores.

---

## Liste des outils MWS

| ToolId | Action | Niveau sécurité | Note |
|--------|--------|-----------------|------|
| `mws.declaration.validate` | Valider le format d'une déclaration reçue | 2 | Exécution seule |
| `mws.declaration.verify` | Vérifier la signature d'une déclaration reçue | 2 | Exécution seule |
| `mws.transport.receive` | Recevoir un message sur un endpoint | 2–3 | Exécution seule ; port 21000 officiel |
| `mws.transport.send` | Envoyer un message (réponse, liste statuts) | 2 | Exécution seule ; adresse fournie |
| `mws.discovery.response.build` | Construire une réponse de découverte | 2 | Liste filtrée ; critère fourni par Cores |
| `mws.discovery.response.send` | Envoyer la réponse au demandeur | 2 | Exécution seule |
| `mws.cog_list.get` | Lire la liste locale de COGs | 2 | Lecture |
| `mws.cog_list.update` | Mettre à jour une entrée dans la liste locale | 2 | Écriture liste locale |
| `mws.cog_list.merge` | Fusionner une liste reçue avec la liste locale | 2 | Règle de fusion fournie par Cores |
| `mws.cog_list.filter` | Filtrer la liste selon critère | 2 | Critère fourni par Border Guard, WorrySentinel |
| `mws.port.check` | Vérifier si un port est exclus MWS | 1–2 | Exécution seule ; déterministe |
| `mws.address.tracker_default` | Résoudre l'adresse Tracker (port 21000) | 1–2 | Exécution seule ; déterministe |

---

**Invariant :** Le Kit ne décide pas d'accepter ou rejeter ; il exécute validations, vérifications et filtrages ; politiques = Cores.
