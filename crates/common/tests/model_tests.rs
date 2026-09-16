use common::models::*;
use common::ApiResponse;
use ipnetwork::IpNetwork;
use uuid::Uuid;
use validator::Validate;

#[test]
fn test_user_role_serialization() {
    let role = UserRole::Admin;
    let json = serde_json::to_string(&role).unwrap();
    assert_eq!(json, "\"admin\"");

    let deserialized: UserRole = serde_json::from_str("\"analyst\"").unwrap();
    assert_eq!(deserialized, UserRole::Analyst);
}

#[test]
fn test_user_dto_validation() {
    let invalid_dto = CreateUserDto {
        username: "ab".to_string(), // too short
        email: "not-an-email".to_string(),
        password: "123".to_string(), // too short
        role: Some(UserRole::Viewer),
    };
    assert!(invalid_dto.validate().is_err());

    let valid_dto = CreateUserDto {
        username: "secops_lead".to_string(),
        email: "lead@secnet.local".to_string(),
        password: "SuperStrongPassword123!".to_string(),
        role: Some(UserRole::Admin),
    };
    assert!(valid_dto.validate().is_ok());
}

#[test]
fn test_alert_severity_ordering() {
    assert!(AlertSeverity::Critical > AlertSeverity::High);
    assert!(AlertSeverity::High > AlertSeverity::Medium);
    assert!(AlertSeverity::Medium > AlertSeverity::Low);
}

#[test]
fn test_traffic_event_ip_network() {
    let src: IpNetwork = "192.168.1.100/32".parse().unwrap();
    let dst: IpNetwork = "10.0.0.1/32".parse().unwrap();

    let event = TrafficEvent {
        time: chrono::Utc::now(),
        id: Uuid::new_v4(),
        src_ip: src,
        dst_ip: dst,
        src_port: 54321,
        dst_port: 443,
        protocol: "TCP".to_string(),
        bytes_transferred: 1420,
        packet_count: 1,
        flags: "SYN".to_string(),
        interface_name: "eth0".to_string(),
    };

    let serialized = serde_json::to_string(&event).unwrap();
    assert!(serialized.contains("192.168.1.100/32"));
    assert!(serialized.contains("TCP"));
}

#[test]
fn test_api_response_envelope() {
    let ok_res = ApiResponse::ok("healthy");
    assert!(ok_res.success);
    assert_eq!(ok_res.data, Some("healthy"));
    assert!(ok_res.error.is_none());

    let err_res: ApiResponse<()> = ApiResponse::err("Unauthorized access");
    assert!(!err_res.success);
    assert_eq!(err_res.error, Some("Unauthorized access".to_string()));
}
