use std::fs;
use std::process::Command;
use walkdir::WalkDir;
use serde::Serialize;
use sha2::{Sha256, Digest};
use std::io::{Read};

#[derive(Serialize)]
struct ScanReport {
    player_uuid: String,
    status: String,
    detections: Vec<String>,
}

fn main() {
    let player_uuid = std::env::args().nth(1).unwrap_or_else(|| "unknown".to_string());
    let mut detections = Vec::new();

    // 1. Escaneamento (Exemplo de procura pelo Doomsday)
    for entry in WalkDir::new("C:\\Users").into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() && path.to_str().unwrap().contains("Doomsday") {
            detections.push(format!("Detected: {:?}", path));
        }
    }

    // 2. Envio do relatório
    let report = ScanReport {
        player_uuid,
        status: if detections.is_empty() { "CLEAN".to_string() } else { "DETECTED".to_string() },
        detections,
    };

    let client = reqwest::blocking::Client::new();
    let _ = client.post("WEBHOOK_URL_AQUI")
        .json(&report)
        .send();

    // 3. Auto-deleção (Segura)
    self_destruct();
}

fn self_destruct() {
    let bat = r#"
@echo off
timeout /t 2 >nul
del "%~f0"
"#;
    fs::write("cleanup.bat", bat).unwrap();
    Command::new("cmd").args(&["/C", "start", "cleanup.bat"]).spawn().unwrap();
    std::process::exit(0);
}
