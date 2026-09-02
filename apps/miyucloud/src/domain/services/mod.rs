//! @id     miyucloud_domain_services_mod
//! @do     declarer_module_i18n_service
//! @role   orchestration
//! @layer  app
//! @human  miyucloud — module « domain services mod ».

pub mod i18n_service;
pub mod path_service;

// NOTE: auth_service has been moved to infrastructure/services/jwt_service.rs
// The functionality is now exposed through application/ports/auth_ports.rs (TokenServicePort)
