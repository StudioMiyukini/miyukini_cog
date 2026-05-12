//! `dav_xml_kit` — templates XML DAV via `quick-xml`.
//!
//! Implémentation en PR-4 (P3.c). API attendue :
//! - `render_multistatus(responses) -> Vec<u8>`
//! - `parse_propfind(xml) -> PropfindRequest`
//! - Mode strict (rejette entités externes pour anti-XXE)
