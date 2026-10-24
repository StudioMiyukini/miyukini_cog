//! Handlers HTTP de l'API REST Alicia.
//!
//! Chaque sous-module expose les fonctions handler pour un groupe de routes.

// @id service.alicia.api.handlers
// @role http_handlers
// @layer 7
// @do handle_api_requests
// @human Handlers REST Alicia : state, devices, rooms, health, history.

pub mod capabilities;
pub mod devices;
pub mod health;
pub mod history;
pub mod rooms;
pub mod state;
