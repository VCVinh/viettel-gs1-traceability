//! Standalone CLI & HTTP Server for the Viettel × GS1 Digital Link Offline-First P2P Platform (`http://127.0.0.1:8090`).

use std::io::{Read, Write};
use std::net::TcpListener;
use viettel_gs1_traceability::ViettelAiEmployeesAutopilot;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let cmd = args.get(1).map(|s| s.as_str()).unwrap_or("demo");

    match cmd {
        "demo" => {
            let out_dir = "demo-output";
            match ViettelAiEmployeesAutopilot::execute_full_viettel_autopilot(
                out_dir,
                "viettel-founder-hmac-key-2026",
            ) {
                Ok(dossier) => {
                    println!(
                        "Viettel GS1 Offline-First P2P Demo generated successfully at: {}",
                        dossier.interactive_live_demo_html_path
                    );
                }
                Err(e) => {
                    eprintln!("Error generating demo: {}", e);
                    std::process::exit(1);
                }
            }
        }
        "serve" => {
            let bind_addr = args.get(2).map(|s| s.as_str()).unwrap_or("127.0.0.1:8090");
            let out_dir = "demo-output";
            let _ = ViettelAiEmployeesAutopilot::execute_full_viettel_autopilot(
                out_dir,
                "viettel-founder-hmac-key-2026",
            );
            let listener = TcpListener::bind(bind_addr).expect("Failed to bind port");
            println!(
                "Viettel GS1 Offline-First P2P Server listening on http://{}/viettel-demo",
                bind_addr
            );
            for stream in listener.incoming().flatten() {
                let mut stream = stream;
                let mut buf = [0u8; 4096];
                let n = stream.read(&mut buf).unwrap_or(0);
                let req = String::from_utf8_lossy(&buf[..n]);
                let first_line = req.lines().next().unwrap_or("GET / HTTP/1.1");
                let path = first_line.split_whitespace().nth(1).unwrap_or("/");

                let (status, content_type, body) = match path {
                    "/" | "/viettel-demo" => {
                        let html = std::fs::read_to_string(
                            "demo-output/viettel_gs1_offline_p2p_live_demo.html",
                        )
                        .unwrap_or_else(|_| "<h1>Viettel Demo Ready</h1>".to_string());
                        ("HTTP/1.1 200 OK", "text/html; charset=utf-8", html)
                    }
                    "/v1/viettel-autopilot" | "/api/status" | "/health" => {
                        let json = std::fs::read_to_string(
                            "demo-output/viettel_autopilot_outbox_ledger.json",
                        )
                        .unwrap_or_else(|_| r#"{"status":"ok"}"#.to_string());
                        ("HTTP/1.1 200 OK", "application/json", json)
                    }
                    _ => (
                        "HTTP/1.1 404 Not Found",
                        "application/json",
                        r#"{"error":"not_found"}"#.to_string(),
                    ),
                };

                let resp = format!(
                    "{}\r\nContent-Type: {}\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                    status,
                    content_type,
                    body.len(),
                    body
                );
                let _ = stream.write_all(resp.as_bytes());
            }
        }
        _ => {
            println!("Usage: cargo run -- [demo | serve [127.0.0.1:8090]]");
        }
    }
}
