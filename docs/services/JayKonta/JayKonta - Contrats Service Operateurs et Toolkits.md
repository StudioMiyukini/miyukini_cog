# JayKonta - Contrats Service Operateurs et Toolkits

## Contexte

Ce document formalise les contrats normatifs du service JayKonta :
- contrats de service
- contrats operateurs
- contrats toolkits
- contrats d'integration
- contrats de securite et audit

Tous les contrats sont alignes sur les references JayKonta existantes.

## Portee

- Perimetre : contrats fonctionnels, regles d'appel, obligations de securite, codes d'erreur fonctionnels
- Hors perimetre : schemas techniques d'API HTTP detaillees, definitions SQL finales

## Convention contractuelle

- Prefixe contrat service : `CK-SVC-*`
- Prefixe contrat operateur : `CK-OP-*`
- Prefixe contrat toolkit : `CK-TK-*`
- Prefixe integration : `CK-INT-*`
- Prefixe securite : `CK-SEC-*`
- Prefixe audit : `CK-AUD-*`

## 1. Contrats de service

### CK-SVC-01 - Unicite de service COG

- Rule : JayKonta est un seul service COG avec deux points d'entree.
- Input context : Purse ou Account.
- Output expected : perimetre capacitaire adapte au point d'entree.
- Reject when : tentative de melange non mandate entre contextes Purse et Account.

### CK-SVC-02 - Separation Purse et Account

- Rule : les donnees, permissions et parcours sont isoles par contexte.
- Input context : user identity plus entry point.
- Output expected : scope data filtre par contexte.
- Reject when : acces Account depuis mandat Purse ou inverse sans autorisation explicite.

### CK-SVC-03 - Source de verite comptable

- Rule : les donnees comptables canoniques sont detenues par JayKonta.
- Input context : appels services consommateurs.
- Output expected : reference comptable stable (id quote, id invoice, id movement).
- Reject when : service consommateur tente d'ecrire en direct hors contrat.

## 2. Contrats operateurs

### Operateur OP-PURSE

#### CK-OP-01 - Consultation dashboard Purse

- Input : mandat Purse valide.
- Process : aggregation mouvements, objectifs, alertes.
- Output : synthese budget personnelle.
- Security : niveau 2 minimum.

#### CK-OP-02 - Enregistrement mouvement Purse

- Input : date, montant, type, categorie, libelle.
- Process : validation champs, controle mandat, write intent.
- Output : movement_id, statut `recorded`.
- Reject when : montant invalide, categorie interdite, mandat absent.

#### CK-OP-03 - Gestion budget occasionnel

- Input : budget_id optionnel, movement_id ou payload budget.
- Process : creation/affectation/solde.
- Output : etat budget mis a jour.

### Operateur OP-ACCOUNT

#### CK-OP-11 - Creation devis

- Input : counterparty, lignes, taxes, conditions.
- Process : validation legale minimale, numerotation, write intent.
- Output : quote_id, statut `draft|sent`.

#### CK-OP-12 - Conversion devis vers facture

- Input : quote_id.
- Process : controle statut quote, generation facture.
- Output : invoice_id, lien quote_invoice.
- Reject when : quote non eligible.

#### CK-OP-13 - Emission facture

- Input : facture payload ou quote convertie.
- Process : controle mandat, controle conformite, generation artefact.
- Output : invoice_id, statut `issued`, artifact_id.

#### CK-OP-14 - Enregistrement paiement

- Input : invoice_id, montant, date, moyen, reference opaque.
- Process : controle coherences, update statut.
- Output : payment_id, invoice_status.
- Reject when : reference paiement sensible en clair.

#### CK-OP-15 - Rapport legal

- Input : periode, type rapport, filtre entite.
- Process : aggregation plus regles de confidentialite.
- Output : report_id, data_set.

## 3. Contrats toolkits

### Toolkit TK-AUTH-CONTEXT

#### CK-TK-01

- Purpose : authentification et resolution contexte Purse/Account.
- Contract : retourne `identity`, `entry_point`, `permissions`.

### Toolkit TK-LEDGER

#### CK-TK-11

- Purpose : ecriture et lecture mouvements.
- Contract : operations atomiques `record`, `list`, `correct`, `void`.
- Guarantee : trace write intent obligatoire.

### Toolkit TK-QUOTES

#### CK-TK-21

- Purpose : gestion cycle devis.
- Contract : `create`, `send`, `accept`, `reject`, `convert`.

### Toolkit TK-INVOICES

#### CK-TK-31

- Purpose : cycle facture.
- Contract : `emit`, `send`, `remind`, `mark_partial`, `mark_paid`.

### Toolkit TK-PAYMENTS

#### CK-TK-41

- Purpose : enregistrement et rapprochement paiements.
- Contract : `record`, `link_invoice`, `status`.
- Security : aucune donnee carte/RIB en clair.

### Toolkit TK-REPORTING

#### CK-TK-51

- Purpose : tableaux de bord et rapports.
- Contract : `dashboard`, `balance`, `legal`, `export`.

### Toolkit TK-BUDGET-PURSE

#### CK-TK-61

- Purpose : budgets occasionnels et objectifs Purse.
- Contract : `create_budget`, `assign_movement`, `goal_progress`, `alert_trigger`.

## 4. Contrats d'integration

### CK-INT-01 - JayFestival vers JayKonta

- Allowed operations : `quote.create`, `invoice.emit`, `budget.movements.record`, `report.by_edition`.
- Ownership split :
- JayFestival garde donnees metier (edition, exposant)
- JayKonta garde donnees comptables (devis, factures, mouvements)
- Required refs : edition_ref, exhibitor_ref, actor_ref.

### CK-INT-02 - JayRDV vers JayKonta

- Allowed operations : `quote.create`, `invoice.emit`, `payment.record`, `report.by_professional`.
- Ownership split :
- JayRDV garde donnees metier (rdv, professionnel)
- JayKonta garde donnees comptables.

### CK-INT-03 - JayKoa reminders

- Allowed operations : `deadline.reminder.publish`.
- Constraint : pas de copie canonique des donnees financieres dans JayKoa.

## 5. Contrats securite

### CK-SEC-01 - Classification de donnees

- Purse : niveau 2 minimum.
- Account : niveau 2 a 3 selon classe.
- Niveau 3 : residence centralisee obligatoire.

### CK-SEC-02 - Mandat obligatoire ecriture

- Toute operation create/update/delete requiert mandat valide.
- Refus si mandat absent, expire ou insuffisant.

### CK-SEC-03 - Chiffrement et secrets

- Transit chiffre obligatoire.
- Au repos renforce pour classes 2+.
- Secrets paiement tokenises uniquement.

### CK-SEC-04 - Federation inter-COG

- Identite != autorite.
- Visa requis pour session externe.
- Bridge transporte sans gouverner.

## 6. Contrats audit

### CK-AUD-01 - Journalisation des ecritures

- Event requis : actor, operation, timestamp, scope, objet, resultat.
- Retention : selon politique service.

### CK-AUD-02 - Journalisation des exports

- Event requis : report_id, format, scope, recipient, justification.

### CK-AUD-03 - Traçabilite de conversion devis->facture

- Event requis : quote_id, invoice_id, actor, timestamp.

## 7. Codes de refus fonctionnels

- `ERR-MANDAT-ABSENT`
- `ERR-MANDAT-INSUFFISANT`
- `ERR-CONTEXTE-INVALID`
- `ERR-DATA-CLASS-BLOCKED`
- `ERR-QUOTE-NOT-CONVERTIBLE`
- `ERR-INVOICE-NOT-FOUND`
- `ERR-PAYMENT-REFERENCE-INVALID`
- `ERR-EXPORT-SCOPE-DENIED`

## 8. Critere d'acceptation des contrats

- chaque contrat est rattache a au moins un besoin Purse ou Account
- aucune operation sensible sans controle mandat et audit
- split ownership explicite pour chaque integration
- mapping securite coherent avec niveaux JayKonta

## References

- docs/services/JayKonta/JayKonta - Document Fondateur.md
- docs/services/JayKonta/reference/JayKonta - Niveaux Securite et Protection Donnees.md
- docs/services/JayKonta/reference/JayKonta - Integration Services.md
- docs/services/JayKonta/publics/Account/Account - Analyse des besoins.md
- docs/services/JayKonta/publics/Purse/Purse - Analyse des besoins.md

## Statut

- Version : 1.0
- Date : 2026-02-07
- Statut : Normatif
