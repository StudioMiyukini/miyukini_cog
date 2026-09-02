//! @id     jaycloud_kits_dav_xml_kit
//! @do     fournir_kits_dav_xml_kit
//! @role   orchestration
//! @layer  core
//! @human  jaycloud — module « kits dav xml kit ».

//! `dav_xml_kit` — templates XML DAV via `quick-xml`.
//!
//! Implémentation en PR-4 (P3.c). API attendue :
//! - `render_multistatus(responses) -> Vec<u8>`
//! - `parse_propfind(xml) -> PropfindRequest`
//! - Mode strict (rejette entités externes pour anti-XXE)
