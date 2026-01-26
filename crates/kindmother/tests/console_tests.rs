//! Tests console simples pour KindMother
//!
//! Ces tests vérifient que :
//! - KindMother démarre correctement
//! - Les appels CoreDataAPI sont rejetés proprement
//! - Les erreurs sont explicites

use kindmother::core::{
    AuthorityContext, DomainContext, InstanceContext, InstanceType, KindMother, WriteIntent,
};
use kindmother::api::CoreDataAPI;
use kindmother::errors::KMError;
use kindmother::state::KMState;

#[test]
fn test_km_creation() {
    let km = KindMother::new();
    assert_eq!(km.state(), KMState::Booting);
}

#[test]
fn test_km_start_rejected() {
    let mut km = KindMother::new();
    let result = km.start();
    assert!(result.is_err());
    
    if let Err(KMError::NotImplemented { .. }) = result {
        // Attendu
    } else {
        panic!("Le démarrage devrait être rejeté avec NotImplemented");
    }
}

#[test]
fn test_coredataapi_rejects_write_intent() {
    let km = KindMother::new();
    let mut api = CoreDataAPI::new(km.state());
    
    let authority = AuthorityContext::new(
        "test-caller".to_string(),
        vec!["write".to_string()],
    );
    let instance = InstanceContext::new(
        "test-instance".to_string(),
        InstanceType::Daughter,
    );
    let domain = DomainContext::new(
        "test-domain".to_string(),
        "Identity".to_string(),
    );
    
    let intent = WriteIntent::new(
        "test-intent-1".to_string(),
        "create_entity".to_string(),
    );
    
    let result = api.submit_write_intent(intent, authority, instance, domain);
    assert!(result.is_err(), "Le WriteIntent devrait être rejeté");
}

#[test]
fn test_coredataapi_rejects_read_entity() {
    let km = KindMother::new();
    let mut api = CoreDataAPI::new(km.state());
    
    let authority = AuthorityContext::new(
        "test-caller".to_string(),
        vec!["read".to_string()],
    );
    let instance = InstanceContext::new(
        "test-instance".to_string(),
        InstanceType::Daughter,
    );
    let domain = DomainContext::new(
        "test-domain".to_string(),
        "Identity".to_string(),
    );
    
    let result = api.read_entity("test-entity-1", authority, instance, domain);
    assert!(result.is_err(), "La lecture devrait être rejetée");
}

#[test]
fn test_state_transitions() {
    let mut km = KindMother::new();
    assert_eq!(km.state(), KMState::Booting);
    
    km.set_state(KMState::Healthy);
    assert_eq!(km.state(), KMState::Healthy);
    assert!(km.state().allows_operations());
    
    km.set_state(KMState::Quarantined);
    assert_eq!(km.state(), KMState::Quarantined);
    assert!(km.state().is_quarantined());
    assert!(!km.state().allows_operations());
}
