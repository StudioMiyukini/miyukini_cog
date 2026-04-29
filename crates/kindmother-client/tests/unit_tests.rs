//! Tests unitaires KindMother Client.
//!
//! Tests du protocole IPC, de la sérialisation et des types d'erreur.
//!
//! @id: kindmother_client_unit_tests
//! @role: test
//! @layer: toolkit

use kindmother_client::{ClientError, Operation, Request, Response};
use serde_json::{json, Value};

// ═══════════════════════════════════════════════════════════════════════════
// Tests du protocole - Sérialisation des requêtes
// ═══════════════════════════════════════════════════════════════════════════

/// @id: KMC_P_001
/// @role: test
/// @layer: toolkit
/// @human: Sérialisation Request::Query - vérifie le format JSON d'une requête SELECT.
#[test]
fn kmc_p_001_serialize_query_request() {
    let request = Request::Query {
        operator_id: "jaykoa".to_string(),
        database: "jaykoa_db".to_string(),
        sql: "SELECT * FROM entries WHERE active = ?".to_string(),
        params: vec!["1".to_string()],
    };

    let json = serde_json::to_value(&request).unwrap();

    assert_eq!(json["type"], "Query");
    assert_eq!(json["operator_id"], "jaykoa");
    assert_eq!(json["database"], "jaykoa_db");
    assert_eq!(json["sql"], "SELECT * FROM entries WHERE active = ?");
    assert_eq!(json["params"], json!(["1"]));
}

/// @id: KMC_P_002
/// @role: test
/// @layer: toolkit
/// @human: Sérialisation Request::Execute - vérifie le format JSON d'une requête d'écriture.
#[test]
fn kmc_p_002_serialize_execute_request() {
    let request = Request::Execute {
        operator_id: "jaykonta".to_string(),
        database: "jaykonta_db".to_string(),
        sql: "INSERT INTO movements (label) VALUES (?)".to_string(),
        params: vec!["Loyer".to_string()],
        intent: "create_movement".to_string(),
    };

    let json = serde_json::to_value(&request).unwrap();

    assert_eq!(json["type"], "Execute");
    assert_eq!(json["intent"], "create_movement");
}

/// @id: KMC_P_003
/// @role: test
/// @layer: toolkit
/// @human: Sérialisation Request::Transaction - vérifie le format d'une transaction multi-opérations.
#[test]
fn kmc_p_003_serialize_transaction_request() {
    let operations = vec![
        Operation {
            sql: "INSERT INTO orders (id) VALUES (?)".to_string(),
            params: vec!["ord-001".to_string()],
        },
        Operation {
            sql: "INSERT INTO order_items (order_id, product_id) VALUES (?, ?)".to_string(),
            params: vec!["ord-001".to_string(), "prod-001".to_string()],
        },
    ];

    let request = Request::Transaction {
        operator_id: "jaykonta".to_string(),
        database: "jaykonta_db".to_string(),
        operations,
        intent: "create_order_with_items".to_string(),
    };

    let json = serde_json::to_value(&request).unwrap();

    assert_eq!(json["type"], "Transaction");
    assert_eq!(json["operations"].as_array().unwrap().len(), 2);
}

/// @id: KMC_P_004
/// @role: test
/// @layer: toolkit
/// @human: Sérialisation Request::HealthCheck - vérifie le format d'une requête de santé.
#[test]
fn kmc_p_004_serialize_health_check_request() {
    let request = Request::HealthCheck {
        database: Some("test_db".to_string()),
    };

    let json = serde_json::to_value(&request).unwrap();

    assert_eq!(json["type"], "HealthCheck");
    assert_eq!(json["database"], "test_db");
}

/// @id: KMC_P_005
/// @role: test
/// @layer: toolkit
/// @human: Sérialisation Request::DatabaseInfo - vérifie le format d'une requête d'info.
#[test]
fn kmc_p_005_serialize_database_info_request() {
    let request = Request::DatabaseInfo {
        operator_id: "jaykoa".to_string(),
        database: "jaykoa_db".to_string(),
    };

    let json = serde_json::to_value(&request).unwrap();

    assert_eq!(json["type"], "DatabaseInfo");
    assert_eq!(json["operator_id"], "jaykoa");
}

// ═══════════════════════════════════════════════════════════════════════════
// Tests du protocole - Désérialisation des réponses
// ═══════════════════════════════════════════════════════════════════════════

/// @id: KMC_P_010
/// @role: test
/// @layer: toolkit
/// @human: Désérialisation Response::QueryResult - vérifie le parsing d'un résultat SELECT.
#[test]
fn kmc_p_010_deserialize_query_result() {
    let json_str = r#"{
        "type": "QueryResult",
        "success": true,
        "rows": [{"id": "1", "name": "Product A"}, {"id": "2", "name": "Product B"}],
        "row_count": 2,
        "error": null
    }"#;

    let response: Response = serde_json::from_str(json_str).unwrap();

    match response {
        Response::QueryResult {
            success,
            rows,
            row_count,
            error,
        } => {
            assert!(success);
            assert_eq!(rows.len(), 2);
            assert_eq!(row_count, 2);
            assert!(error.is_none());
        }
        _ => panic!("Expected QueryResult"),
    }
}

/// @id: KMC_P_011
/// @role: test
/// @layer: toolkit
/// @human: Désérialisation Response::ExecuteResult - vérifie le parsing d'un résultat d'écriture.
#[test]
fn kmc_p_011_deserialize_execute_result() {
    let json_str = r#"{
        "type": "ExecuteResult",
        "success": true,
        "rows_affected": 1,
        "last_insert_id": 42,
        "error": null
    }"#;

    let response: Response = serde_json::from_str(json_str).unwrap();

    match response {
        Response::ExecuteResult {
            success,
            rows_affected,
            last_insert_id,
            error,
        } => {
            assert!(success);
            assert_eq!(rows_affected, 1);
            assert_eq!(last_insert_id, Some(42));
            assert!(error.is_none());
        }
        _ => panic!("Expected ExecuteResult"),
    }
}

/// @id: KMC_P_012
/// @role: test
/// @layer: toolkit
/// @human: Désérialisation Response::TransactionResult - vérifie le parsing d'un résultat de transaction.
#[test]
fn kmc_p_012_deserialize_transaction_result() {
    let json_str = r#"{
        "type": "TransactionResult",
        "success": true,
        "results": [1, 3],
        "error": null
    }"#;

    let response: Response = serde_json::from_str(json_str).unwrap();

    match response {
        Response::TransactionResult {
            success,
            results,
            error,
        } => {
            assert!(success);
            assert_eq!(results, vec![1, 3]);
            assert!(error.is_none());
        }
        _ => panic!("Expected TransactionResult"),
    }
}

/// @id: KMC_P_013
/// @role: test
/// @layer: toolkit
/// @human: Désérialisation Response::HealthCheckResult - vérifie le parsing d'un résultat de santé.
#[test]
fn kmc_p_013_deserialize_health_check_result() {
    let json_str = r#"{
        "type": "HealthCheckResult",
        "healthy": true,
        "status": "OK",
        "latency_ms": 5
    }"#;

    let response: Response = serde_json::from_str(json_str).unwrap();

    match response {
        Response::HealthCheckResult {
            healthy,
            status,
            latency_ms,
        } => {
            assert!(healthy);
            assert_eq!(status, "OK");
            assert_eq!(latency_ms, 5);
        }
        _ => panic!("Expected HealthCheckResult"),
    }
}

/// @id: KMC_P_014
/// @role: test
/// @layer: toolkit
/// @human: Désérialisation Response::DatabaseInfoResult - vérifie le parsing des infos DB.
#[test]
fn kmc_p_014_deserialize_database_info_result() {
    let json_str = r#"{
        "type": "DatabaseInfoResult",
        "success": true,
        "database": "test_db",
        "size_bytes": 1024000,
        "table_count": 12,
        "encrypted": true,
        "error": null
    }"#;

    let response: Response = serde_json::from_str(json_str).unwrap();

    match response {
        Response::DatabaseInfoResult {
            success,
            database,
            size_bytes,
            table_count,
            encrypted,
            error,
        } => {
            assert!(success);
            assert_eq!(database, "test_db");
            assert_eq!(size_bytes, 1024000);
            assert_eq!(table_count, 12);
            assert!(encrypted);
            assert!(error.is_none());
        }
        _ => panic!("Expected DatabaseInfoResult"),
    }
}

/// @id: KMC_P_015
/// @role: test
/// @layer: toolkit
/// @human: Désérialisation Response::Error - vérifie le parsing d'une erreur.
#[test]
fn kmc_p_015_deserialize_error_response() {
    let json_str = r#"{
        "type": "Error",
        "code": "PERMISSION_DENIED",
        "message": "Operator not authorized for this database"
    }"#;

    let response: Response = serde_json::from_str(json_str).unwrap();

    match response {
        Response::Error { code, message } => {
            assert_eq!(code, "PERMISSION_DENIED");
            assert!(message.contains("not authorized"));
        }
        _ => panic!("Expected Error"),
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Tests des types d'erreur
// ═══════════════════════════════════════════════════════════════════════════

/// @id: KMC_E_001
/// @role: test
/// @layer: toolkit
/// @human: ClientError Display - vérifie le format d'affichage des erreurs.
#[test]
fn kmc_e_001_error_display() {
    let err = ClientError::Connection("Connection refused".to_string());
    assert!(err.to_string().contains("Connection error"));

    let err = ClientError::PermissionDenied("No access".to_string());
    assert!(err.to_string().contains("Permission denied"));

    let err = ClientError::Timeout("Request timed out".to_string());
    assert!(err.to_string().contains("Timeout"));
}

/// @id: KMC_E_002
/// @role: test
/// @layer: toolkit
/// @human: ClientError from io::Error - vérifie la conversion depuis std::io::Error.
#[test]
fn kmc_e_002_error_from_io() {
    let io_err = std::io::Error::new(std::io::ErrorKind::ConnectionRefused, "refused");
    let client_err: ClientError = io_err.into();

    match client_err {
        ClientError::Transport(msg) => assert!(msg.contains("refused")),
        _ => panic!("Expected Transport error"),
    }
}

/// @id: KMC_E_003
/// @role: test
/// @layer: toolkit
/// @human: ClientError from serde_json::Error - vérifie la conversion depuis erreur JSON.
#[test]
fn kmc_e_003_error_from_json() {
    let json_err = serde_json::from_str::<Value>("invalid json").unwrap_err();
    let client_err: ClientError = json_err.into();

    match client_err {
        ClientError::Serialization(msg) => assert!(!msg.is_empty()),
        _ => panic!("Expected Serialization error"),
    }
}

/// @id: KMC_E_004
/// @role: test
/// @layer: toolkit
/// @human: ClientError variants - vérifie que toutes les variantes sont distinctes.
#[test]
fn kmc_e_004_error_variants() {
    let errors = vec![
        ClientError::Connection("conn".to_string()),
        ClientError::Transport("transport".to_string()),
        ClientError::Service("service".to_string()),
        ClientError::Serialization("serial".to_string()),
        ClientError::PermissionDenied("perm".to_string()),
        ClientError::DatabaseNotFound("db".to_string()),
        ClientError::QuotaExceeded("quota".to_string()),
        ClientError::NotConnected,
        ClientError::InvalidRequest("invalid".to_string()),
        ClientError::Timeout("timeout".to_string()),
    ];

    // Chaque variante a un message d'erreur différent
    let messages: Vec<String> = errors.iter().map(|e| e.to_string()).collect();
    let unique_count = messages
        .iter()
        .collect::<std::collections::HashSet<_>>()
        .len();
    assert_eq!(
        unique_count,
        errors.len(),
        "All error variants should have unique messages"
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// Tests du type Operation
// ═══════════════════════════════════════════════════════════════════════════

/// @id: KMC_O_001
/// @role: test
/// @layer: toolkit
/// @human: Operation sérialisation/désérialisation roundtrip.
#[test]
fn kmc_o_001_operation_roundtrip() {
    let operation = Operation {
        sql: "UPDATE users SET active = ? WHERE id = ?".to_string(),
        params: vec!["true".to_string(), "user-123".to_string()],
    };

    let json = serde_json::to_string(&operation).unwrap();
    let parsed: Operation = serde_json::from_str(&json).unwrap();

    assert_eq!(parsed.sql, operation.sql);
    assert_eq!(parsed.params, operation.params);
}

/// @id: KMC_O_002
/// @role: test
/// @layer: toolkit
/// @human: Operation avec params vides.
#[test]
fn kmc_o_002_operation_empty_params() {
    let operation = Operation {
        sql: "SELECT COUNT(*) FROM users".to_string(),
        params: vec![],
    };

    let json = serde_json::to_value(&operation).unwrap();
    assert!(json["params"].as_array().unwrap().is_empty());
}

// ═══════════════════════════════════════════════════════════════════════════
// Tests de robustesse du protocole
// ═══════════════════════════════════════════════════════════════════════════

/// @id: KMC_R_001
/// @role: test
/// @layer: toolkit
/// @human: Requête avec SQL vide - vérifie que la sérialisation accepte un SQL vide.
#[test]
fn kmc_r_001_request_empty_sql() {
    let request = Request::Query {
        operator_id: "test".to_string(),
        database: "test_db".to_string(),
        sql: String::new(),
        params: vec![],
    };

    let json = serde_json::to_value(&request).unwrap();
    assert_eq!(json["sql"], "");
}

/// @id: KMC_R_002
/// @role: test
/// @layer: toolkit
/// @human: Réponse avec erreur et succès false - vérifie le parsing cohérent.
#[test]
fn kmc_r_002_response_with_error() {
    let json_str = r#"{
        "type": "QueryResult",
        "success": false,
        "rows": [],
        "row_count": 0,
        "error": "Table not found: unknown_table"
    }"#;

    let response: Response = serde_json::from_str(json_str).unwrap();

    match response {
        Response::QueryResult { success, error, .. } => {
            assert!(!success);
            assert!(error.is_some());
            assert!(error.unwrap().contains("Table not found"));
        }
        _ => panic!("Expected QueryResult"),
    }
}

/// @id: KMC_R_003
/// @role: test
/// @layer: toolkit
/// @human: Paramètres avec caractères spéciaux - vérifie l'échappement JSON.
#[test]
fn kmc_r_003_params_with_special_chars() {
    let request = Request::Execute {
        operator_id: "test".to_string(),
        database: "test_db".to_string(),
        sql: "INSERT INTO logs (message) VALUES (?)".to_string(),
        params: vec![r#"Error: "quote" and 'apostrophe' and \backslash"#.to_string()],
        intent: "log_message".to_string(),
    };

    let json = serde_json::to_string(&request).unwrap();
    let parsed: Request = serde_json::from_str(&json).unwrap();

    match parsed {
        Request::Execute { params, .. } => {
            assert!(params[0].contains("quote"));
            assert!(params[0].contains("apostrophe"));
            assert!(params[0].contains("backslash"));
        }
        _ => panic!("Expected Execute"),
    }
}
