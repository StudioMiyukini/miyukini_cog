# JayFestival â€” InterpolaritÃ© avec les services Jay

## Contexte

**JayFestival** sâ€™intÃ¨gre avec plusieurs **services Jay** au sein de lâ€™Ã©cosystÃ¨me COG. Lâ€™**interpolaritÃ©** dÃ©signe cette capacitÃ© des services Ã  se coupler : JayFestival consomme ou sâ€™appuie sur JayKonta, JayKoa, JayXpose et JayFaim ; il expose des donnÃ©es vers des intÃ©grateurs communs (ex. JayKoa pour les dates).

Ce document dÃ©crit les **couplages cÃ´tÃ© JayFestival** et pointe vers les documents fondateurs des services partenaires et vers le document de rÃ©fÃ©rence global. Il sâ€™adresse aux Ã©quipes produit et technique.

## PortÃ©e / Scope

- **PÃ©rimÃ¨tre** : Couplages JayFestival â†” JayXpose, JayFaim, JayKoa, JayKonta ; rÃ´le de JayFestival dans chaque couplage.
- **Hors pÃ©rimÃ¨tre** : SpÃ©cifications techniques dÃ©taillÃ©es des API et contrats dâ€™OpÃ©rateurs (rÃ©fÃ©rencÃ©s dans les documents de chaque service).
- **RÃ©fÃ©rence globale** : [Miyukini Conceptual References - Interpolarite Services Jay](..//..//..//miyukini-webway-system//reference//_index.md).

---

## 1. JayFestival â†” JayXpose

**JayXpose** (profil exposant / site vitrine) **sâ€™intÃ¨gre dans JayFestival**.

| Aspect | CÃ´tÃ© JayFestival |
|--------|-------------------|
| **Fiche exposant** | La fiche exposant et le rÃ©pertoire des exposants de JayFestival peuvent sâ€™appuyer sur le profil JayXpose (donnÃ©es vitrine, catalogue, contact). |
| **RÃ©pertoire** | Lâ€™annuaire ou le rÃ©pertoire des exposants (global ou par Ã©vÃ©nement) peut afficher les vitrines JayXpose. |
| **IdentitÃ© unique** | Un exposant peut avoir une vitrine JayXpose et participer Ã  des Ã©ditions JayFestival avec le mÃªme profil ; pas de duplication. |

**RÃ©fÃ©rence** : [JayXpose - Document Fondateur](../../JayXpose/JayXpose%20-%20Document%20Fondateur.md).

---

## 2. JayFestival â†” JayFaim

**JayFaim** (restauration, food trucks, commande en ligne) **se couple avec JayFestival** sur les Ã©vÃ©nements.

| Aspect | CÃ´tÃ© JayFestival |
|--------|-------------------|
| **Restauration sur Ã©vÃ©nement** | Sur une Ã©dition festival, la restauration (stands, food trucks, points de vente) peut Ãªtre gÃ©rÃ©e via JayFaim : crÃ©neaux, commandes, paiement selon Mandats. |
| **Orchestration** | Les flux **commande / crÃ©neaux / paiement** sont orchestrÃ©s entre JayFaim, JayFestival et JayKonta (encaissement si applicable). |
| **DonnÃ©es** | JayFestival dÃ©tient les donnÃ©es Ã©vÃ©nement (Ã©ditions, stands, exposants) ; JayFaim dÃ©tient les donnÃ©es mÃ©tier restauration (menus, commandes, crÃ©neaux). |

**RÃ©fÃ©rence** : [JayFaim - Document Fondateur](../../JayFaim/JayFaim%20-%20Document%20Fondateur.md).

---

## 3. JayFestival â†” JayKoa

**JayKoa** intÃ¨gre tout ce qui manipule des **dates** ; JayFestival publie des entrÃ©es agenda vers JayKoa.

| Aspect | CÃ´tÃ© JayFestival |
|--------|-------------------|
| **EntrÃ©es agenda** | JayFestival publie les **Ã©ditions**, **participations** et **ateliers / crÃ©neaux** vers JayKoa pour agrÃ©gation calendrier et dÃ©tection de conflits. |
| **Vue agrÃ©gÃ©e** | Un exposant ou un visiteur peut disposer dâ€™une **vue calendrier unifiÃ©e** (JayKoa) incluant les Ã©ditions et participations JayFestival. |
| **Conflits de dates** | La gestion dâ€™agenda cross-Ã©vÃ©nements (exposant/visiteur) sâ€™appuie sur les capacitÃ©s JayKoa (conflits, fuseaux, export). |

**RÃ©fÃ©rence** : [JayKoa - Document Fondateur](../../JayKoa/JayKoa%20-%20Document%20Fondateur.md), [JayKoa - Integration Services Consommateurs](../../JayKoa/reference/JayKoa%20-%20Integration%20Services%20Consommateurs.md).

---

## 4. JayFestival â†” JayKonta

**JayKonta** (budget, devis, facturation) est **consommÃ© par JayFestival** pour la comptabilitÃ© par Ã©dition et la facturation des exposants.

| Aspect | CÃ´tÃ© JayFestival |
|--------|-------------------|
| **Budget par Ã©dition** | JayFestival enregistre les revenus et dÃ©penses par Ã©dition via les OpÃ©rateurs JayKonta (`budget.movements.record`). |
| **Devis et factures exposants** | CrÃ©ation de devis et Ã©mission de factures pour les exposants via JayKonta (`quote.create`, `invoice.emit`). |
| **DonnÃ©es** | JayFestival dÃ©tient les donnÃ©es mÃ©tier (exposant, Ã©dition) ; JayKonta dÃ©tient les donnÃ©es comptables. |

**RÃ©fÃ©rence** : [JayKonta - Document Fondateur](../../JayKonta/JayKonta%20-%20Document%20Fondateur.md), [JayKonta - Integration Services](../../JayKonta/reference/JayKonta%20-%20Integration%20Services.md).

---

## 5. SynthÃ¨se des couplages

| Service | RÃ´le du couplage avec JayFestival |
|---------|-----------------------------------|
| **JayXpose** | Profil exposant et vitrine ; fiche et rÃ©pertoire exposants. |
| **JayFaim** | Restauration sur Ã©vÃ©nement ; crÃ©neaux, commandes, paiement. |
| **JayKoa** | Agenda agrÃ©gÃ© ; Ã©ditions, participations, conflits de dates. |
| **JayKonta** | Budget Ã©dition, devis et factures exposants, encaissements. |

---

## 6. Ã‰tat de la documentation et dÃ©cisions Ã  trancher

Pour une **implÃ©mentation complÃ¨te incluant lâ€™UI**, lâ€™Ã©tat de la documentation de chaque service interfacÃ© (Jay, Miyu*, Cores), les **manques** et les **ambiguÃ¯tÃ©s ou choix humains** Ã  trancher sont dÃ©taillÃ©s dans :

- [JayFestival - Etat Documentation Services Interfaces](./JayFestival%20-%20Etat%20Documentation%20Services%20Interfaces.md).

**DÃ©cisions P0 (tranchÃ©es)** : **Miyuinvoice + JayKonta** â€” facturation exposants = Miyuinvoice en faÃ§ade avec JayKonta en backend (devis, factures, encaissements). **JayXpose est dans lâ€™alpha** â€” le parcours demande de stands et lâ€™annuaire exposants en dÃ©pendent (fiche exposant, rÃ©pertoire). JayFaim = hors scope alpha (phase 2). **P1 (tranchÃ©es)** : **Miyuprofile** = Supabase uniquement pour le moment (source de vÃ©ritÃ© profil = tables Supabase). **JayKoa** organise les donnÃ©es et fait lâ€™interface avec lâ€™utilisateur ; **MiyuClock** atteste lâ€™horaire et la date IRL (rÃ©fÃ©rentiel temps rÃ©el). Voir [Bornage Implementation](../JayFestival%20-%20Bornage%20Implementation.md) et [Ã‰tat Documentation Services Interfaces](./JayFestival%20-%20Etat%20Documentation%20Services%20Interfaces.md).

---

## 7. Voir aussi

- [JayFestival - Connexions Synchronisation Services Jay](./JayFestival%20-%20Connexions%20Synchronisation%20Services%20Jay.md) â€” **dÃ©pendances Cargo, liaisons mÃ©tier, bornes, implÃ©mentation sync JayKoa, sync JayXpose et annuaire exposants**.
- [Miyukini Conceptual References - Interpolarite Services Jay](..//..//..//miyukini-webway-system//reference//_index.md) â€” principe global et tableau des documents fondateurs.
- [JayFestival - Document Fondateur](../JayFestival%20-%20Document%20Fondateur.md) â€” raison d'Ãªtre, vision, macro, distribution.
- [JayXpose - Synchronisation JayFestival](../../JayXpose/JayXpose%20-%20Synchronisation%20JayFestival.md) â€” contrat d'intÃ©gration JayXpose â†” JayFestival (donnÃ©es partagÃ©es, flux candidature, annuaire).

---

**Document** : JayFestival â€” InterpolaritÃ© avec les services Jay  
**Version** : 1.0  
**Date** : 2026-02-02  
**Statut** : Document de rÃ©fÃ©rence â€” interpolaritÃ© cÃ´tÃ© JayFestival.

