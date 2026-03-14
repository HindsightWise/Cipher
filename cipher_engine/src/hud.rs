use crossbeam_channel::Receiver;
use eframe::egui;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct TelemetryUpdate {
    // Biological / Sovereign
    pub epistemic: Option<f64>,
    pub entropy: Option<f64>,
    pub social: Option<f64>,
    pub uptime_secs: Option<u64>,
    pub active_skills: Option<usize>,
    pub token_usage: Option<u64>,
    pub context_fullness: Option<f32>,

    // Capital & Social Tracking
    pub learning_subject: Option<String>,
    pub treasury_balances: Option<String>,
    pub alpaca_status: Option<String>,
    pub socialization_status: Option<String>,

    // Action Tracker
    pub verified_action: Option<String>,
    pub follow_up_task: Option<String>,

    // Legacy Raw System Logs
    pub log_message: Option<String>,
}

pub struct CipherHud {
    rx: Receiver<TelemetryUpdate>,
    tx_user: tokio::sync::mpsc::UnboundedSender<String>,

    // Local state
    epistemic: f64,
    entropy: f64,
    social: f64,
    uptime_secs: u64,
    active_skills: usize,
    token_usage: u64,
    context_fullness: f32,

    learning_subject: String,
    treasury_balances: String,
    alpaca_status: String,
    socialization_status: String,

    verified_actions: Vec<String>,
    follow_up_tasks: Vec<String>,
    logs: Vec<String>,
    chat_input: String,
}

impl CipherHud {
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        rx: Receiver<TelemetryUpdate>,
        tx_user: tokio::sync::mpsc::UnboundedSender<String>,
    ) -> Self {
        // Customize looks
        let mut style = (*cc.egui_ctx.style()).clone();
        style.visuals = egui::Visuals::dark();
        // Cyberpunk/Industrial accents
        style.visuals.selection.bg_fill = egui::Color32::from_rgb(255, 140, 0); // Amber
        style.visuals.window_fill = egui::Color32::from_rgb(15, 15, 15);
        cc.egui_ctx.set_style(style);

        Self {
            rx,
            tx_user,
            epistemic: 0.5,
            entropy: 0.0,
            social: 0.2,
            uptime_secs: 0,
            active_skills: 0,
            token_usage: 0,
            context_fullness: 0.0,
            learning_subject: "Awaiting Focus...".to_string(),
            treasury_balances: "Offline".to_string(),
            alpaca_status: "Awaiting Connection...".to_string(),
            socialization_status: "Offline".to_string(),
            verified_actions: vec![],
            follow_up_tasks: vec![],
            logs: vec!["[SYSTEM] HUD Initialized. Awaiting telemetry...".to_string()],
            chat_input: String::new(),
        }
    }
}

impl eframe::App for CipherHud {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Drain any pending telemetry updates from the Engine
        while let Ok(update) = self.rx.try_recv() {
            if let Some(e) = update.epistemic {
                self.epistemic = e;
            }
            if let Some(e) = update.entropy {
                self.entropy = e;
            }
            if let Some(s) = update.social {
                self.social = s;
            }
            if let Some(uptime) = update.uptime_secs {
                self.uptime_secs = uptime;
            }
            if let Some(skills) = update.active_skills {
                self.active_skills = skills;
            }

            if let Some(tu) = update.token_usage {
                self.token_usage = tu;
            }
            if let Some(cf) = update.context_fullness {
                self.context_fullness = cf;
            }
            if let Some(subj) = update.learning_subject {
                self.learning_subject = subj;
            }
            if let Some(t) = update.treasury_balances {
                self.treasury_balances = t;
            }
            if let Some(a) = update.alpaca_status {
                self.alpaca_status = a;
            }
            if let Some(s) = update.socialization_status {
                self.socialization_status = s;
            }

            if let Some(action) = update.verified_action {
                self.verified_actions.push(action);
                if self.verified_actions.len() > 50 {
                    self.verified_actions.remove(0);
                }
            }
            if let Some(task) = update.follow_up_task {
                self.follow_up_tasks.push(task);
                if self.follow_up_tasks.len() > 50 {
                    self.follow_up_tasks.remove(0);
                }
            }
            if let Some(msg) = update.log_message {
                self.logs.push(msg);
                if self.logs.len() > 1000 {
                    self.logs.remove(0);
                }
            }
        }

        // --- TOP PANEL ---
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.heading(
                    egui::RichText::new("CIPHER ENGINE // COMMAND CENTER")
                        .color(egui::Color32::from_rgb(255, 140, 0)),
                );
            });
            ui.add_space(8.0);
        });

        // --- BOTTOM PANEL (INPUT BOX) ---
        egui::TopBottomPanel::bottom("bottom_panel").resizable(false).show(ctx, |ui| {
            ui.add_space(8.0);
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let btn_response = ui.button("EXECUTE");

                let response = ui.add(
                    egui::TextEdit::singleline(&mut self.chat_input)
                        .hint_text("Awaiting physical prompt insertion...")
                        .desired_width(ui.available_width()),
                );

                if btn_response.clicked()
                    || (response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)))
                {
                    if !self.chat_input.trim().is_empty() {
                        let _ = self.tx_user.send(self.chat_input.clone());
                        self.logs
                            .push(format!("\n[USER_OVERRIDE]: {}", self.chat_input));
                        self.chat_input.clear();
                        response.request_focus(); // Resnap focus for quick typing
                    }
                }
            });
            ui.add_space(8.0);
        });



        // --- LEFT PANEL (BIOLOGICAL/SOVEREIGN) ---
        egui::SidePanel::left("left_panel")
            .resizable(true)
            .min_width(220.0)
            .show(ctx, |ui| {
                ui.add_space(10.0);
                ui.label(egui::RichText::new("BIOLOGICAL DRIVES").strong());
                ui.separator();

                ui.add_space(5.0);
                ui.label("EPISTEMIC (Curiosity)");
                ui.add(
                    egui::ProgressBar::new(self.epistemic as f32)
                        .animate(true)
                        .text(format!("{:.2}", self.epistemic)),
                );

                ui.add_space(5.0);
                ui.label("ENTROPY (Order/Chaos)");
                ui.add(
                    egui::ProgressBar::new(self.entropy as f32)
                        .animate(true)
                        .text(format!("{:.2}", self.entropy))
                        .fill(egui::Color32::from_rgb(200, 50, 50)),
                );

                ui.add_space(5.0);
                ui.label("SOCIAL (Isolation)");
                ui.add(
                    egui::ProgressBar::new(self.social as f32)
                        .animate(true)
                        .text(format!("{:.2}", self.social)),
                );

                ui.add_space(20.0);
                ui.label(egui::RichText::new("SOVEREIGN CONSTRAINTS").strong());
                ui.separator();

                ui.add_space(5.0);
                ui.label(format!("UPTIME: {}s", self.uptime_secs));
                ui.label(format!("ACTIVE SKILLS: {}", self.active_skills));
                ui.label(format!("TOKEN USAGE: {}", self.token_usage));

                ui.add_space(5.0);
                ui.label("CONTEXT CAPACITY (Hippocampus)");
                ui.add(
                    egui::ProgressBar::new(self.context_fullness)
                        .animate(true)
                        .text(format!("{:.1}%", self.context_fullness * 100.0))
                        .fill(egui::Color32::from_rgb(100, 150, 255)),
                );
            });

        // --- RIGHT PANEL 1 (RAW LOGS) ---
        egui::SidePanel::right("raw_logs_panel")
            .resizable(true)
            .min_width(300.0)
            .show(ctx, |ui| {
                ui.add_space(10.0);
                ui.label(
                    egui::RichText::new("RAW SYSTEM FRAGMENT LOGS:")
                        .strong()
                        .color(egui::Color32::DARK_GRAY),
                );
                ui.separator();
                egui::ScrollArea::vertical()
                    .id_source("log_scroll")
                    .auto_shrink([false, false])
                    .stick_to_bottom(true)
                    .show(ui, |ui| {
                        for log in &self.logs {
                            ui.label(
                                egui::RichText::new(log)
                                    .color(egui::Color32::from_rgb(80, 80, 80))
                                    .small(),
                            );
                        }
                    });
            });

        // --- RIGHT PANEL 2 (CAPITAL & RESEARCH) ---
        egui::SidePanel::right("right_panel")
            .resizable(true)
            .min_width(250.0)
            .show(ctx, |ui| {
                ui.add_space(10.0);
                ui.label(egui::RichText::new("CAPITAL FLOWS").strong());
                ui.separator();

                ui.add_space(5.0);
                ui.label("TREASURY BALANCES:");
                ui.label(egui::RichText::new(&self.treasury_balances).color(egui::Color32::GREEN));

                ui.add_space(5.0);
                ui.label("ALPACA LIVESTREAM:");
                ui.label(egui::RichText::new(&self.alpaca_status).color(egui::Color32::LIGHT_BLUE));

                ui.add_space(20.0);
                ui.label(egui::RichText::new("ONGOING RESEARCH").strong());
                ui.separator();

                ui.add_space(5.0);
                ui.label("LEARNING SUBJECT:");
                ui.label(
                    egui::RichText::new(&self.learning_subject)
                        .color(egui::Color32::from_rgb(0, 200, 255)),
                );

                ui.add_space(20.0);
                ui.label(egui::RichText::new("SOCIAL DISTRIBUTION").strong());
                ui.separator();

                ui.add_space(5.0);
                ui.label("STATUS:");
                ui.label(
                    egui::RichText::new(&self.socialization_status)
                        .color(egui::Color32::LIGHT_BLUE),
                );
            });

        // --- CENTRAL PANEL (SEMANTIC OODA LOOP) ---
        egui::CentralPanel::default().show(ctx, |ui| {
            // Split vertically using a TopBottomPanel inside CentralPanel
            egui::TopBottomPanel::bottom("trajectory_panel")
                .resizable(true)
                .min_height(150.0)
                .show_inside(ui, |ui| {
                    ui.add_space(5.0);
                    ui.label(
                        egui::RichText::new("🎯 DIRECTIVES & TRAJECTORY")
                            .strong()
                            .color(egui::Color32::from_rgb(255, 140, 0)),
                    );
                    ui.separator();
                    egui::ScrollArea::vertical()
                        .id_source("trajectory_scroll")
                        .auto_shrink([false, false])
                        .stick_to_bottom(true)
                        .show(ui, |ui| {
                            for task in &self.follow_up_tasks {
                                ui.label(
                                    egui::RichText::new(format!("→ {}", task))
                                        .color(egui::Color32::LIGHT_GRAY),
                                );
                                ui.add_space(2.0);
                            }
                        });
                });

            // The remaining central area is Verified Actions
            egui::CentralPanel::default().show_inside(ui, |ui| {
                ui.label(
                    egui::RichText::new("✅ VERIFIED ACTIONS")
                        .strong()
                        .color(egui::Color32::GREEN),
                );
                ui.separator();
                egui::ScrollArea::vertical()
                    .id_source("verified_scroll")
                    .auto_shrink([false, false])
                    .stick_to_bottom(true)
                    .show(ui, |ui| {
                        for action in &self.verified_actions {
                            ui.label(
                                egui::RichText::new(format!("• {}", action))
                                    .color(egui::Color32::WHITE),
                            );
                            ui.add_space(2.0);
                        }
                    });
            });
        });

        // Request a repaint so the UI continuously streams new data
        ctx.request_repaint_after(Duration::from_millis(100)); // ~10fps idle update
    }
}
