//! Tools MiyuExpense — tool.expense.claim.* (create, update, list, validate, export).
//! Notes de frais : create/update/list ; validate = décision StrongFather ; export compta = StrongFather.

use crate::context::GovernedContext;
use crate::errors::MiyuexpenseError;

/// @id miyuexpense_tool_expense_claim_create
/// @role mutator
/// @layer tool
/// @do expense_claim_create_under_governance
//@human / @human Crée une note de frais à partir de données fournies ; WriteIntent KindMother.
pub fn create(ctx: &GovernedContext, _payload: &str) -> Result<String, MiyuexpenseError> {
    if !ctx.has_mandate() {
        return Err(MiyuexpenseError::NoMandate);
    }
    Ok("claim:stub".to_string())
}

/// @id miyuexpense_tool_expense_claim_update
/// @role mutator
/// @layer tool
/// @do expense_claim_update_under_governance
//@human / @human Met à jour une note de frais ; WriteIntent KindMother.
pub fn update(
    ctx: &GovernedContext,
    _claim_id: &str,
    _payload: &str,
) -> Result<(), MiyuexpenseError> {
    if !ctx.has_mandate() {
        return Err(MiyuexpenseError::NoMandate);
    }
    Ok(())
}

/// @id miyuexpense_tool_expense_claim_list
/// @role accessor
/// @layer tool
/// @do expense_claim_list_under_governance
//@human / @human Liste les notes de frais ; filtres fournis ; lecture.
pub fn list(
    ctx: &GovernedContext,
    _filters: Option<&str>,
) -> Result<Vec<String>, MiyuexpenseError> {
    if !ctx.has_mandate() {
        return Err(MiyuexpenseError::NoMandate);
    }
    Ok(Vec::new())
}

/// @id miyuexpense_tool_expense_claim_validate
/// @role mutator
/// @layer tool
/// @do expense_claim_validate_under_governance
//@human / @human Valide une note de frais (workflow) ; décision = StrongFather.
pub fn validate(ctx: &GovernedContext, _claim_id: &str) -> Result<(), MiyuexpenseError> {
    if !ctx.has_mandate() {
        return Err(MiyuexpenseError::NoMandate);
    }
    Ok(())
}

/// @id miyuexpense_tool_expense_claim_export
/// @role mutator
/// @layer tool
/// @do expense_claim_export_under_governance
//@human / @human Export des notes de frais vers compta ; autorisation = StrongFather.
pub fn export(
    ctx: &GovernedContext,
    _claim_ids: &str,
    _format: Option<&str>,
) -> Result<Vec<u8>, MiyuexpenseError> {
    if !ctx.has_mandate() {
        return Err(MiyuexpenseError::NoMandate);
    }
    Ok(Vec::new())
}
