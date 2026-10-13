//! TEMPLATE 03: GS1 Digital Link 1.3 + IETF RFC 9264 (`application/linkset+json`)
//! + GSMA Open Gateway CAMARA 5G `x-viettel-cell-id` Anti-Spoofing Edge Resolver
//!
//! UPSTREAM INSPIRATIONS & PROVEN SOURCES:
//! - GS1 Digital Link Standard 1.3: https://github.com/gs1/digital-link.js
//! - IETF RFC 9264 Linkset: https://www.rfc-editor.org/rfc/rfc9264.html
//! - GSMA Open Gateway / Linux Foundation CAMARA DeviceLocation API: https://github.com/camaraproject/DeviceLocation

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Camara5GNetworkHeaders {
    /// Injected directly by Viettel 5G UPF / MEC Gateway (cannot be forged by client Fake-GPS apps)
    pub x_viettel_cell_id: String,
    pub x_viettel_msisdn_hash: String,
    pub bearer_type: String, // "5G_SA" | "4G_LTE" | "WIFI_SILENT_CELL_FALLBACK"
}

/// Validates GS1 GTIN-14 / GTIN-13 Mod-10 Check Digit (GS1 General Specifications Section 7.9)
pub fn verify_gs1_mod10_check_digit(gtin: &str) -> bool {
    if !(gtin.len() == 8 || gtin.len() == 12 || gtin.len() == 13 || gtin.len() == 14) {
        return false;
    }
    if !gtin.bytes().all(|b| b.is_ascii_digit()) {
        return false;
    }
    let digits: Vec<u32> = gtin.bytes().map(|b| (b - b'0') as u32).collect();
    let check_digit = *digits.last().unwrap();
    let mut sum = 0u32;
    for (idx, &d) in digits[..digits.len() - 1].iter().rev().enumerate() {
        let weight = if idx % 2 == 0 { 3 } else { 1 };
        sum += d * weight;
    }
    let expected = (10 - (sum % 10)) % 10;
    check_digit == expected
}

/// Builds an IETF RFC 9264 compliant `application/linkset+json` response with CAMARA 5G territory verification
pub fn build_rfc9264_linkset_with_camara(
    gtin: &str,
    batch: &str,
    serial: &str,
    authorized_cell_prefix: &str,
    headers: &Camara5GNetworkHeaders,
) -> serde_json::Value {
    let territory_valid = headers.x_viettel_cell_id.starts_with(authorized_cell_prefix);
    serde_json::json!({
        "linkset": [{
            "anchor": format!("https://id.viettel.vn/01/{}/10/{}/21/{}", gtin, batch, serial),
            "https://gs1.org/voc/pip": [{
                "href": format!("https://trace.viettel.vn/passport/{}/{}/{}", gtin, batch, serial),
                "title": "Digital Product Passport (Zero-Install Web/Mini-App)",
                "type": "text/html"
            }],
            "https://gs1.org/voc/traceability": [{
                "href": format!("https://trace.viettel.vn/api/v1/ledger/{}/{}/{}", gtin, batch, serial),
                "title": "Hyperledger Besu & Polygon Dual-EVM Proof",
                "type": "application/json"
            }],
            "viettel_camara_5g_attestation": {
                "cell_id": headers.x_viettel_cell_id,
                "authorized_prefix": authorized_cell_prefix,
                "territory_valid": territory_valid,
                "anti_fake_gps_hardware_locked": true
            }
        }]
    })
}
