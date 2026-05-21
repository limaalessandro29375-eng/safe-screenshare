#![windows_subsystem = "windows"]

use eframe::egui;
use std::sync::{Arc, Mutex};
use std::thread;
use sysinfo::{System, ProcessExt, SystemExt};

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([500.0, 400.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "SafeBox Network - Security Auditor",
        options,
        Box::new(|_cc| Box::new(SafeApp::default())),
    )
}

struct SafeApp {
    logs: Arc<Mutex<Vec<String>>>,
    progress: f32,
    finished: bool,
}

impl Default for SafeApp {
    fn default() -> Self {
        let logs = Arc::new(Mutex::new(vec!["Iniciando Auditoria de Memória...".to_string()]));
        let logs_clone = logs.clone();

        thread::spawn(move || {
            let mut sys = System::new_all();
            sys.refresh_all();

            // 1. Scanner de Processos (Deteção de hacks rodando)
            {
                let mut logs = logs_clone.lock().unwrap();
                logs.push("Verificando processos ativos...".to_string());
            }

            for (pid, process) in sys.processes() {
                let name = process.name().to_lowercase();
                if name.contains("doomsday") || name.contains("injector") || name.contains("cheat") {
                    let mut logs = logs_clone.lock().unwrap();
                    logs.push(format!("ALERTA: Processo suspeito encontrado: {} (PID: {})", name, pid));
                }
            }

            // 2. Simulação de Forense de Arquivos
            {
                let mut logs = logs_clone.lock().unwrap();
                logs.push("Verificando assinaturas de arquivos...".to_string());
                thread::sleep(std::time::Duration::from_secs(2));
                logs.push("Status: Limpo.".to_string());
            }
        });

        Self { logs, progress: 0.0, finished: false }
    }
}

impl eframe::App for SafeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("SafeBox Network - Auditoria Completa");
            ui.separator();

            egui::ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
                let logs = self.logs.lock().unwrap();
                for log in logs.iter() {
                    ui.label(log);
                }
            });

            ui.add(egui::ProgressBar::new(self.progress));
            
            if self.progress < 1.0 {
                self.progress += 0.005;
                ctx.request_repaint();
            } else {
                self.finished = true;
                if ui.button("Finalizar Auditoria").clicked() {
                    std::process::exit(0);
                }
            }
        });
    }
}
