# Odoo PLM — Guide Intégration COG

## Contexte

Ce document fournit un **guide pratique** pour intégrer les fonctionnalités PLM (ECO, révisions BoM, approbations, versioning) dans l'architecture Miyukini COG, avec exemples de code pseudo-Rust et explications des patterns COG.

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Architecture d'intégration COG pour PlmService
- Patterns WriteIntent et Mandates (ECO, Apply Changes, approbations)
- Exemples de code pseudo-Rust
- Gestion des gouvernances (StrongFather, KindMother, TAMR, Ever Buddy)

---

## 1. Architecture d'Intégration

### 1.1 Vue d'Ensemble

```
PlmUI → BondingBrother → EcoOperator → StrongFather (décision)
                                       → KindMother (WriteIntent ECO, BoM)
                                       → Master Butler (permissions)
                                       → WorrySentinel (sécurité)
                                       → EcoApprovalOperator (approbations)
                                       → BomRevisionOperator (révisions)
                                       → EcoDocumentOperator (documents)
                                       → Ever Buddy (rebase, versions)
```

### 1.2 Flux Typiques

**Création ECO et Start Revision :**
1. Intention utilisateur (PlmUI) → BondingBrother
2. EcoOperator reçoit l'intention `CreateEcoIntent` / `StartRevisionIntent`
3. StrongFather : décision de création / révision
4. Master Butler : vérification permissions
5. KindMother : WriteIntent ECO ; BomRevisionOperator + KindMother : création révision BoM
6. Retour ECO avec révision créée et stages affichés

**Apply Changes :**
1. Intention Apply Changes (PlmUI) → EcoOperator
2. EcoApprovalOperator : toutes les approbations requises obtenues ?
3. StrongFather : décision d'appliquer
4. KindMother : WriteIntent — archiver ancienne BoM, promouvoir révision en production, incrémenter version
5. EcoDocumentOperator : synchronisation documents ECO → BoM production
6. ECO passé en stage clôture

**Approbation :**
1. Demande d'approbation (EcoOperator → EcoApprovalOperator)
2. TAMR : point d'intervention humaine (approbateur)
3. StrongFather : enregistrement de la décision (approuvé / refusé)
4. Si toutes approbations OK → déblocage Apply Changes

---

## 2. Patterns d'Intégration

### 2.1 Création ECO

**Pattern :** WriteIntent + Mandate

```rust
// Pseudo-code Rust
pub struct CreateEcoIntent {
    pub description: String,
    pub eco_type_id: Uuid,
    pub apply_on: ApplyOn,  // BillOfMaterials | ProductOnly
    pub product_id: Uuid,
    pub bom_id: Option<Uuid>,
    pub responsible_id: Option<Uuid>,
    pub effective: Effective,  // AsSoonAsPossible | AtDate(DateTime<Utc>)
    pub tag_ids: Vec<Uuid>,
}

impl EcoOperator {
    pub async fn create_eco(
        &self,
        intent: CreateEcoIntent,
        mandate: Mandate,
    ) -> Result<Eco, EcoError> {
        mandate.validate_operators(&[self.id()])?;
        mandate.validate_flows(&["eco.create"])?;

        let decision = self.strong_father
            .decide(DecisionRequest {
                action: "create_eco",
                context: &intent,
            })
            .await?;

        if !decision.allowed {
            return Err(EcoError::DecisionDenied);
        }

        let permission = self.master_butler
            .check_permission(PermissionRequest {
                operator: self.id(),
                capability: "eco.create",
                resource: None,
            })
            .await?;

        if !permission.granted {
            return Err(EcoError::PermissionDenied);
        }

        let write_intent = EcoWriteIntent {
            description: intent.description,
            eco_type_id: intent.eco_type_id,
            apply_on: intent.apply_on,
            product_id: intent.product_id,
            bom_id: intent.bom_id,
            responsible_id: intent.responsible_id,
            effective: intent.effective,
            tag_ids: intent.tag_ids,
        };

        let eco = self.kind_mother
            .persist_eco(write_intent)
            .await?;

        Ok(eco)
    }
}
```

### 2.2 Start Revision

**Pattern :** WriteIntent (révision BoM) + Mandate

```rust
pub struct StartRevisionIntent {
    pub eco_id: Uuid,
}

impl EcoOperator {
    pub async fn start_revision(
        &self,
        intent: StartRevisionIntent,
        mandate: Mandate,
    ) -> Result<(Eco, BomRevision), EcoError> {
        mandate.validate_flows(&["eco.start_revision"])?;

        let eco = self.kind_mother.get_eco(intent.eco_id).await?;
        if eco.apply_on != ApplyOn::BillOfMaterials {
            return Err(EcoError::ApplyOnMustBeBom);
        }

        let decision = self.strong_father
            .decide(DecisionRequest {
                action: "start_revision",
                context: &intent,
            })
            .await?;

        if !decision.allowed {
            return Err(EcoError::DecisionDenied);
        }

        let revision_intent = BomRevisionWriteIntent {
            eco_id: intent.eco_id,
            source_bom_id: eco.bom_id.ok_or(EcoError::NoBom)?,
        };

        let revision = self.bom_revision_operator
            .create_revision(revision_intent, mandate.clone())
            .await?;

        let eco_updated = self.kind_mother
            .persist_eco_revision_started(intent.eco_id, revision.id())
            .await?;

        Ok((eco_updated, revision))
    }
}
```

### 2.3 Apply Changes

**Pattern :** Approbations + StrongFather + KindMother (bascule BoM)

```rust
pub struct ApplyChangesIntent {
    pub eco_id: Uuid,
}

impl EcoOperator {
    pub async fn apply_changes(
        &self,
        intent: ApplyChangesIntent,
        mandate: Mandate,
    ) -> Result<Eco, EcoError> {
        mandate.validate_flows(&["eco.apply_changes"])?;

        let eco = self.kind_mother.get_eco(intent.eco_id).await?;
        let approvals_ok = self.eco_approval_operator
            .all_approvals_granted(eco.id(), eco.stage_id())
            .await?;

        if !approvals_ok {
            return Err(EcoError::ApprovalsRequired);
        }

        let decision = self.strong_father
            .decide(DecisionRequest {
                action: "apply_changes",
                context: &intent,
            })
            .await?;

        if !decision.allowed {
            return Err(EcoError::DecisionDenied);
        }

        let revision = self.bom_revision_operator
            .get_revision_by_eco(intent.eco_id)
            .await?;

        self.kind_mother
            .apply_bom_revision_to_production(ApplyBomIntent {
                eco_id: intent.eco_id,
                revision_id: revision.id(),
                previous_bom_id: revision.source_bom_id(),
            })
            .await?;

        self.eco_document_operator
            .sync_documents_to_bom(intent.eco_id, revision.target_bom_id())
            .await?;

        let eco_closed = self.kind_mother
            .move_eco_to_closing_stage(intent.eco_id)
            .await?;

        Ok(eco_closed)
    }
}
```

### 2.4 Approbation (TAMR + StrongFather)

**Pattern :** Intervention humaine (TAMR) + Décision (StrongFather)

```rust
pub struct GrantApprovalIntent {
    pub eco_id: Uuid,
    pub stage_id: Uuid,
    pub approver_id: Uuid,
    pub granted: bool,
    pub comment: Option<String>,
}

impl EcoApprovalOperator {
    pub async fn grant_approval(
        &self,
        intent: GrantApprovalIntent,
        mandate: Mandate,
    ) -> Result<Approval, ApprovalError> {
        self.tamr.require_human_intervention(InterventionRequest {
            context: "eco_approval",
            operator: intent.approver_id,
        }).await?;

        let decision = self.strong_father
            .decide(DecisionRequest {
                action: if intent.granted { "approval.grant" } else { "approval.deny" },
                context: &intent,
            })
            .await?;

        let approval = self.kind_mother
            .persist_approval(ApprovalWriteIntent {
                eco_id: intent.eco_id,
                stage_id: intent.stage_id,
                approver_id: intent.approver_id,
                granted: intent.granted,
                comment: intent.comment,
            })
            .await?;

        if intent.granted {
            self.miyu_notify
                .notify_approval_granted(intent.eco_id, intent.approver_id)
                .await?;
        }

        Ok(approval)
    }
}
```

### 2.5 Apply Rebase (Ever Buddy + KindMother)

**Pattern :** Compatibilité versions (Ever Buddy) + WriteIntent (fusion révision)

```rust
impl EcoOperator {
    pub async fn apply_rebase(
        &self,
        eco_id: Uuid,
        mandate: Mandate,
    ) -> Result<BomRevision, EcoError> {
        mandate.validate_flows(&["eco.apply_rebase"])?;

        let eco = self.kind_mother.get_eco(eco_id).await?;
        let revision = self.bom_revision_operator.get_revision_by_eco(eco_id).await?;

        let is_obsolete = self.ever_buddy
            .revision_base_is_obsolete(revision.id())
            .await?;

        if !is_obsolete {
            return Err(EcoError::RebaseNotNeeded);
        }

        let merge_intent = RebaseWriteIntent {
            revision_id: revision.id(),
            current_production_bom_id: self.kind_mother.get_current_production_bom_id(eco.product_id).await?,
        };

        let updated_revision = self.kind_mother
            .rebase_revision(merge_intent)
            .await?;

        Ok(updated_revision)
    }
}
```

---

## 3. Gestion des Gouvernances

- **StrongFather** : Toute action de création ECO, Start Revision, Apply Changes, Approbation, Apply Rebase — décision explicite avant écriture.
- **KindMother** : Toute persistance — ECO, révisions, BoM (versioning), approbations, documents ; Apply Changes = WriteIntent composite (archive ancienne BoM, promouvoir révision, sync documents).
- **Master Butler** : Vérification des capacités (eco.create, eco.apply_changes, approval.grant, etc.) avant exécution.
- **WorrySentinel** : Niveau de sécurité 1–2 ; isolation des données PLM entre équipes/entreprises.
- **Ever Buddy** : Détection base obsolète (revision_base_is_obsolete), rebase (compatibilité versions).
- **TAMR** : Uniquement pour les approbations (require_human_intervention avant enregistrement approbation).
- **BondingBrother** : Traduction des intentions PlmUI vers EcoOperator et sous-opérateurs ; pas d'autorité, uniquement médiation.

---

## 4. Récapitulatif des WriteIntents

| Action | WriteIntent / Flux KindMother |
|--------|------------------------------|
| Création ECO | EcoWriteIntent → persist_eco |
| Start Revision | BomRevisionWriteIntent → create_revision ; persist_eco_revision_started |
| Apply Changes | ApplyBomIntent → apply_bom_revision_to_production ; sync_documents_to_bom ; move_eco_to_closing_stage |
| Approbation | ApprovalWriteIntent → persist_approval |
| Apply Rebase | RebaseWriteIntent → rebase_revision |

---

**Document rédigé selon la méthodologie d'analyse Odoo et l'architecture COG Miyukini.**
