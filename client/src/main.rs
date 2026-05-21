#![windows_subsystem = "windows"]

use chrono::{DateTime, Utc};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::env;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::Command;
use sysinfo::System;
use walkdir::WalkDir;
use winreg::enums::*;
use winreg::RegKey;

// OFUSCAÇÃO BÁSICA DE STRINGS (Simulação)
const WEBHOOK: &str = "https://discord.com/api/webhooks/1507112513622249472/_kI_l42Kls7Irrz0SKFtq5I6M4IgOdch0g9E23eo87WUzFIE9VUcZ9BKaxSTe_i56DaI";

#[derive(Serialize)]
struct ScanReport {
    player_uuid: String,
    status: String,
    hostname: String,
    detections: Vec<String>,
    timestamp: String,
}

fn main() {
    let player_uuid = env::args().nth(1).unwrap_or_else(|| "unknown".to_string());
    let mut sys = System::new_all();
    sys.refresh_all();
    let hostname = sys.host_name().unwrap_or_else(|| "unknown".to_string());
    
    let mut detections = Vec::new();

    // 1. FORENSE: Busca Recursiva e Assinaturas
    perform_file_scan(&mut detections);

    // 2. FORENSE: Registro
    perform_registry_scan(&mut detections);

    // 3. COMUNICAÇÃO SEGURA
    let status = if detections.is_empty() { "CLEAN" } else { "DETECTED" };
    let report = ScanReport {
        player_uuid,
        status: status.to_string(),
        hostname,
        detections,
        timestamp: Utc::now().to_rfc3339(),
    };

    send_report(report);

    // 4. AUTO-DELEÇÃO INFALÍVEL
    self_destruct();
}

fn perform_file_scan(detections: &mut Vec<String>) {
    let target_dirs = vec![
        env::var("APPDATA").unwrap_or_default(),
        env::var("LOCALAPPDATA").unwrap_or_default(),
        format!("{}\\Desktop", env::var("USERPROFILE").unwrap_or_default()),
        format!("{}\\Downloads", env::var("USERPROFILE").unwrap_or_default()),
    ];

    for dir in target_dirs {
        if dir.is_empty() { continue; }
        for entry in WalkDir::new(dir).max_depth(5).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() {
                // Verificação de Doomsday e Hacks por nome/assinatura
                let filename = path.file_name().unwrap_or_default().to_string_lossy().to_lowercase();
                if filename.contains("doomsday") || filename.contains("cheat") || filename.contains("injector") {
                    detections.push(format!("File: {:?}", path));
                }
            }
        }
    }
}

fn perform_registry_scan(detections: &mut Vec<String>) {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = r"Software\Microsoft\Windows\CurrentVersion\Explorer\UserAssist";
    if let Ok(key) = hkcu.open_subkey(path) {
        detections.push("Registry activity detected in UserAssist".to_string());
    }
}

fn send_report(report: ScanReport) {
    let client = reqwest::blocking::Client::new();
    let _ = client.post(WEBHOOK).json(&report).send();
}

fn self_destruct() {
    let bat_path = env::temp_dir().join("clean.bat");
    let content = format!(
        "@echo off\ntimeout /t 3 >nul\ndel /f /q \"{}\"\ndel \"%~f0\"",
        env::current_exe().unwrap().to_str().unwrap()
    );
    fs::write(&bat_path, content).unwrap();
    Command::new("cmd").args(&["/C", "start", bat_path.to_str().unwrap()]).spawn().unwrap();
    std::process::exit(0);
}
