// Registre d'espaces de noms MSCM — déclare les nœuds de namespace parents
// des ids logiques (service.*, toolkit.*, cog) qui s'étalent sur
// plusieurs crates. Fichier de documentation uniquement : hors mod tree,
// non compilé par cargo, lu seulement par la CLI MSCM pour l'intégrité.

/// @id service
/// @do regrouper_service
//@human / @role orchestration
/// @layer domain
/// @human Services applicatifs COG (Alicia, Jay*, média…).

/// @id service.alicia
/// @do regrouper_service_alicia
//@human / @role orchestration
/// @layer domain
/// @human Service Alicia : assistant conversationnel et ses sous-systèmes.

/// @id service.alicia.api
/// @do regrouper_service_alicia_api
//@human / @role orchestration
/// @layer infra
/// @human Alicia — API et points d'entrée HTTP.

/// @id service.alicia.cores_gate
/// @do regrouper_service_alicia_cores_gate
//@human / @role securite
/// @layer infra
/// @human Alicia — porte d'accès aux cœurs (contrôle d'accès).

/// @id service.alicia.dispatcher
/// @do regrouper_service_alicia_dispatcher
//@human / @role orchestration
/// @layer domain
/// @human Alicia — répartiteur de requêtes vers les compétences.

/// @id service.alicia.memory
/// @do regrouper_service_alicia_memory
//@human / @role donnee
/// @layer domain
/// @human Alicia — mémoire et persistance du contexte.

/// @id service.alicia.skills
/// @do regrouper_service_alicia_skills
//@human / @role orchestration
/// @layer domain
/// @human Alicia — compétences (skills) exécutables.

/// @id service.jaycontact
/// @do regrouper_service_jaycontact
//@human / @role orchestration
/// @layer domain
/// @human Service Jay — contacts.

/// @id service.jaydocs
/// @do regrouper_service_jaydocs
//@human / @role orchestration
/// @layer domain
/// @human Service Jay — documents.

/// @id service.jaykoa
/// @do regrouper_service_jaykoa
//@human / @role orchestration
/// @layer domain
/// @human Service Jay — koa.

/// @id service.jaykonta
/// @do regrouper_service_jaykonta
//@human / @role orchestration
/// @layer domain
/// @human Service Jay — comptabilité (konta).

/// @id service.jaymail
/// @do regrouper_service_jaymail
//@human / @role orchestration
/// @layer domain
/// @human Service Jay — messagerie e-mail.

/// @id service.jaymessage
/// @do regrouper_service_jaymessage
//@human / @role orchestration
/// @layer domain
/// @human Service Jay — messagerie instantanée.

/// @id service.jaymeteo
/// @do regrouper_service_jaymeteo
//@human / @role orchestration
/// @layer domain
/// @human Service Jay — météo.

/// @id service.jayrdv
/// @do regrouper_service_jayrdv
//@human / @role orchestration
/// @layer domain
/// @human Service Jay — rendez-vous.

/// @id service.jaysheets
/// @do regrouper_service_jaysheets
//@human / @role orchestration
/// @layer domain
/// @human Service Jay — feuilles de calcul.

/// @id service.jayslides
/// @do regrouper_service_jayslides
//@human / @role orchestration
/// @layer domain
/// @human Service Jay — présentations.

/// @id service.media
/// @do regrouper_service_media
//@human / @role donnee
/// @layer domain
/// @human Service média : gestion des fichiers et contenus.

/// @id toolkit
/// @do regrouper_toolkit
//@human / @role outil
/// @layer outil
/// @human Boîte à outils transverse : modules réutilisables par les services.

/// @id toolkit.accounting
/// @do regrouper_toolkit_accounting
//@human / @role outil
/// @layer outil
/// @human Boîte à outils — module « accounting ».

/// @id toolkit.alicia
/// @do regrouper_toolkit_alicia
//@human / @role outil
/// @layer outil
/// @human Boîte à outils — module « alicia ».

/// @id toolkit.calc
/// @do regrouper_toolkit_calc
//@human / @role outil
/// @layer outil
/// @human Boîte à outils — module « calc ».

/// @id toolkit.communication
/// @do regrouper_toolkit_communication
//@human / @role outil
/// @layer outil
/// @human Boîte à outils — module « communication ».

/// @id toolkit.content
/// @do regrouper_toolkit_content
//@human / @role outil
/// @layer outil
/// @human Boîte à outils — module « content ».

/// @id toolkit.data
/// @do regrouper_toolkit_data
//@human / @role outil
/// @layer outil
/// @human Boîte à outils — module « data ».

/// @id toolkit.discovery
/// @do regrouper_toolkit_discovery
//@human / @role outil
/// @layer outil
/// @human Boîte à outils — module « discovery ».

/// @id toolkit.export
/// @do regrouper_toolkit_export
//@human / @role outil
/// @layer outil
/// @human Boîte à outils — module « export ».

/// @id toolkit.hr
/// @do regrouper_toolkit_hr
//@human / @role outil
/// @layer outil
/// @human Boîte à outils — module « hr ».

/// @id toolkit.identity
/// @do regrouper_toolkit_identity
//@human / @role outil
/// @layer outil
/// @human Boîte à outils — module « identity ».

/// @id toolkit.jobs
/// @do regrouper_toolkit_jobs
//@human / @role outil
/// @layer outil
/// @human Boîte à outils — module « jobs ».

/// @id toolkit.locale
/// @do regrouper_toolkit_locale
//@human / @role outil
/// @layer outil
/// @human Boîte à outils — module « locale ».

/// @id toolkit.moderation
/// @do regrouper_toolkit_moderation
//@human / @role outil
/// @layer outil
/// @human Boîte à outils — module « moderation ».

/// @id toolkit.narrative
/// @do regrouper_toolkit_narrative
//@human / @role outil
/// @layer outil
/// @human Boîte à outils — module « narrative ».

/// @id toolkit.pos
/// @do regrouper_toolkit_pos
//@human / @role outil
/// @layer outil
/// @human Boîte à outils — module « pos ».

/// @id toolkit.security
/// @do regrouper_toolkit_security
//@human / @role outil
/// @layer outil
/// @human Boîte à outils — module « security ».

/// @id toolkit.social
/// @do regrouper_toolkit_social
//@human / @role outil
/// @layer outil
/// @human Boîte à outils — module « social ».

/// @id toolkit.text
/// @do regrouper_toolkit_text
//@human / @role outil
/// @layer outil
/// @human Boîte à outils — module « text ».

/// @id toolkit.validate
/// @do regrouper_toolkit_validate
//@human / @role outil
/// @layer outil
/// @human Boîte à outils — module « validate ».

/// @id origin
/// @do regrouper_origin
/// @role orchestration
/// @layer domain
/// @human Origin : relais et suivi (tracker) — infrastructure de coordination.

/// @id origin.relay
/// @do regrouper_origin_relay
/// @role orchestration
/// @layer infra
/// @human Origin — relais de messages.

/// @id origin.tracker
/// @do regrouper_origin_tracker
/// @role orchestration
/// @layer infra
/// @human Origin — suivi (tracker) des sessions/pairs.
