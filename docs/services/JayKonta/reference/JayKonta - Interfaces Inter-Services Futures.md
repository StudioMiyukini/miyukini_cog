# JayKonta - Interfaces Inter-Services Futures

## Contexte

Ce document definit le cadre d'implementation des interfaces inter-services autour de JayKonta.
Objectif: fournir une base stable pour brancher les services metier futurs sans casser les contrats existants.

Ce document complete:
- `docs/services/JayKonta/JayKonta - Contrats Service Operateurs et Toolkits.md`
- `docs/services/JayKonta/reference/JayKonta - Integration Services.md`
- `docs/services/JayKonta/JayKonta - Bornage Implementation.md`

## Portee

- Inclus:
- contrats d'interface inter-services (request/event)
- format des payloads et metadonnees minimales
- versioning, idempotence, audit, securite
- matrice d'interfaces CK-INT-01, CK-INT-02, CK-INT-03
- regles pour futurs connecteurs

- Exclu:
- schema SQL final par table
- details UI
- orchestration reseau multi-COG complete

## Principes directeurs

- `INTF-01`: separation stricte metier vs comptable.
- Les services consommateurs gardent leurs donnees metier.
- JayKonta reste source de verite comptable.

- `INTF-02`: chaque appel/evenement doit etre tracable.
- correlation_id, actor_ref, contract_id obligatoires.

- `INTF-03`: ecriture idempotente obligatoire.
- Toute operation create/update sensible doit accepter un `idempotency_key`.

- `INTF-04`: schema explicite et versionne.
- Aucune evolution cassante sans nouveau `schema_version`.

- `INTF-05`: securite avant fonctionnalite.
- Mandat valide, scope controle, niveau de donnees respecte.

## Types d'interfaces

- `sync_request_response`
- Usage: operations transactionnelles (quote.create, invoice.emit, payment.record).
- Retour: statut + reference stable JayKonta.

- `async_event_ingest`
- Usage: notifications et synchronisations (reminder.publish, report.requested).
- Retour: ack technique + trace d'audit.

- `query_read_model`
- Usage: lecture de KPI/rapports agreges.
- Retour: donnees non canoniques derivees de la base JayKonta.

## Enveloppe commune (obligatoire)

Tous les flux doivent transporter l'enveloppe suivante:

```json
{
  "meta": {
    "contract_id": "CK-INT-01",
    "operation": "invoice.emit",
    "schema_version": "1.0.0",
    "correlation_id": "c4e5d8f2-...",
    "idempotency_key": "idmp-2026-03-14-001",
    "actor_ref": "jayfestival:org-admin",
    "source_service": "jayfestival",
    "occurred_at": "2026-03-14T09:12:00Z",
    "security_level": 2
  },
  "payload": {}
}
```

## Regles de metadonnees

- `contract_id`: obligatoire, ex: `CK-INT-01`.
- `operation`: obligatoire, operation exacte du contrat.
- `schema_version`: obligatoire, semver.
- `correlation_id`: obligatoire, unique par conversation inter-services.
- `idempotency_key`: obligatoire pour toute ecriture.
- `actor_ref`: obligatoire, acteur applicatif ou humain.
- `source_service`: obligatoire, service emetteur.
- `occurred_at`: obligatoire, horodatage ISO-8601 UTC.
- `security_level`: obligatoire, entier >= 2 pour JayKonta.

## Matrice des interfaces contractuelles

| Contrat | Service source | Operation | Type | Ecriture JayKonta |
|---------|----------------|-----------|------|-------------------|
| CK-INT-01 | JayFestival | quote.create | sync_request_response | quotes |
| CK-INT-01 | JayFestival | invoice.emit | sync_request_response | invoices |
| CK-INT-01 | JayFestival | budget.movements.record | sync_request_response | ledger_movements |
| CK-INT-01 | JayFestival | report.by_edition | async_event_ingest | audit_log |
| CK-INT-02 | JayRDV | quote.create | sync_request_response | quotes |
| CK-INT-02 | JayRDV | invoice.emit | sync_request_response | invoices |
| CK-INT-02 | JayRDV | payment.record | sync_request_response | payments + invoices |
| CK-INT-02 | JayRDV | report.by_professional | async_event_ingest | audit_log |
| CK-INT-03 | JayKoa | deadline.reminder.publish | async_event_ingest | reminders + audit_log |

## Interfaces detaillees

### CK-INT-01 - JayFestival -> JayKonta

#### quote.create

Payload minimal:

```json
{
  "quote_id": "q-2026-001",
  "scope": "account",
  "context_ref": "edition:summer-2026",
  "counterparty_ref": "exhibitor:alfa-food",
  "total": 2800.0,
  "currency": "EUR"
}
```

Reponse attendue:

```json
{
  "status": "ok",
  "quote_id": "q-2026-001",
  "audit_ref": "aud-..."
}
```

#### invoice.emit

Payload minimal:

```json
{
  "invoice_id": "inv-2026-001",
  "scope": "account",
  "context_ref": "edition:summer-2026",
  "counterparty_ref": "exhibitor:alfa-food",
  "quote_id": "q-2026-001",
  "total": 2800.0,
  "currency": "EUR",
  "due_at": "2026-03-31T23:59:59Z"
}
```

#### budget.movements.record

Payload minimal:

```json
{
  "movement_id": "mov-2026-001",
  "scope": "account",
  "context_ref": "edition:summer-2026",
  "category": "edition_income",
  "amount": 2800.0,
  "currency": "EUR",
  "movement_date": "2026-03-14T09:12:00Z"
}
```

#### report.by_edition

Payload minimal:

```json
{
  "edition_ref": "edition:summer-2026",
  "scope": "account"
}
```

### CK-INT-02 - JayRDV -> JayKonta

#### quote.create

Payload minimal:

```json
{
  "quote_id": "q-rdv-2026-001",
  "scope": "account",
  "context_ref": "professional:kinetic-care",
  "counterparty_ref": "customer:patient-42",
  "total": 120.0,
  "currency": "EUR"
}
```

#### invoice.emit

Payload minimal:

```json
{
  "invoice_id": "inv-rdv-2026-001",
  "scope": "account",
  "context_ref": "professional:kinetic-care",
  "counterparty_ref": "customer:patient-42",
  "quote_id": "q-rdv-2026-001",
  "total": 120.0,
  "currency": "EUR",
  "due_at": "2026-03-20T23:59:59Z"
}
```

#### payment.record

Payload minimal:

```json
{
  "payment_id": "pay-rdv-2026-001",
  "invoice_id": "inv-rdv-2026-001",
  "amount": 120.0,
  "currency": "EUR",
  "method": "card_token",
  "reference_opaque": "tok_7f9a2",
  "paid_at": "2026-03-16T10:00:00Z"
}
```

Regle critique:
- aucune reference CB/RIB en clair.
- uniquement token ou reference opaque.

#### report.by_professional

Payload minimal:

```json
{
  "professional_ref": "professional:kinetic-care",
  "scope": "account"
}
```

### CK-INT-03 - JayKoa -> JayKonta

#### deadline.reminder.publish

Payload minimal:

```json
{
  "deadline_ref": "invoice:overdue:acme-2026-03",
  "due_at": "2026-03-20T09:00:00Z",
  "label": "Relance facture Acme",
  "context_ref": "purse-goal:month-control"
}
```

Regle:
- JayKoa ne devient pas source comptable.
- il consomme uniquement des references d'echeance.

## Codes de reponse standard

- `ok`: operation acceptee et tracee.
- `accepted`: evenement recu, traitement differe.
- `rejected`: rejet metier/securite.

## Erreurs inter-services

Format standard:

```json
{
  "status": "rejected",
  "error": {
    "code": "ERR-MANDAT-INSUFFISANT",
    "message": "Permission write.account.invoice manquante",
    "retryable": false
  },
  "correlation_id": "c4e5d8f2-..."
}
```

Codes minimums:
- `ERR-MANDAT-ABSENT`
- `ERR-MANDAT-INSUFFISANT`
- `ERR-CONTEXTE-INVALID`
- `ERR-DATA-CLASS-BLOCKED`
- `ERR-QUOTE-NOT-CONVERTIBLE`
- `ERR-INVOICE-NOT-FOUND`
- `ERR-PAYMENT-REFERENCE-INVALID`
- `ERR-IDEMPOTENCY-CONFLICT`
- `ERR-SCHEMA-UNSUPPORTED`

## Idempotence

- Cle: `idempotency_key`.
- Fenetre de deduplication recommandee: 24h minimum.
- Comportement:
- meme cle + meme payload -> reponse identique (`ok` replay-safe).
- meme cle + payload different -> `ERR-IDEMPOTENCY-CONFLICT`.

## Versioning des schemas

- Version dans `meta.schema_version`.
- Strategie:
- `MAJOR`: changement cassant.
- `MINOR`: ajout backward-compatible.
- `PATCH`: correction sans impact contractuel.

- Regle d'implementation:
- JayKonta doit supporter au moins la version courante et la precedente mineure.

## Securite et gouvernance

- Mandat obligatoire pour toute operation d'ecriture.
- Validation du `scope` (`purse` vs `account`) avant traitement.
- Niveau de securite >= 2 pour donnees JayKonta.
- Chiffrement en transit obligatoire.
- Donnees de paiement: tokenisation obligatoire.

## Audit obligatoire

Pour chaque message traite, journaliser:
- `contract_id`
- `actor_ref`
- `operation`
- `scope`
- `object_ref`
- `result`
- `payload_json`
- `created_at`
- `correlation_id`

Table cible recommandee: `audit_log`.

## Contrat de compatibilite futures integrations

Pour un nouveau service `SVC-X`, les prerequis d'integration sont:

- `IFC-X-01`: mapper ses operations metier vers operations CK-INT existantes ou proposer `CK-INT-XX`.
- `IFC-X-02`: fournir `context_ref` et `counterparty_ref` opaques/stables.
- `IFC-X-03`: respecter l'enveloppe commune meta/payload.
- `IFC-X-04`: implementer `idempotency_key`.
- `IFC-X-05`: publier un jeu de tests de contrat.

## Tests de contrat recommandes

- Tests positifs:
- payload valide par operation
- idempotence replay
- audit ecrit

- Tests negatifs:
- mandat absent
- schema_version non supporte
- scope incoherent
- reference paiement non opaque

- Tests de resilience:
- retry reseau avec meme `idempotency_key`
- reception hors ordre d'evenements

## Roadmap d'implementation interfaces

- Lot 1:
- CK-INT-01/02/03 operationnels avec audit
- enveloppe commune imposee

- Lot 2:
- registre de schemas versionnes
- contract tests automatises CI

- Lot 3:
- federation multi-COG et policy-as-code Mandat
- observabilite transverse correlation_id bout-en-bout

## References

- `docs/services/JayKonta/JayKonta - Contrats Service Operateurs et Toolkits.md`
- `docs/services/JayKonta/reference/JayKonta - Integration Services.md`
- `docs/services/JayKonta/reference/JayKonta - Niveaux Securite et Protection Donnees.md`
- `docs/services/JayKonta/JayKonta - Bornage Implementation.md`

## Statut

- Version: 1.0
- Date: 2026-02-07
- Statut: Reference implementation interfaces futures
