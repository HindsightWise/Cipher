use eframe::egui::{self, Color32, Pos2, Rect, Rounding, Sense, Shape, Stroke, Vec2};
use crossbeam_channel::Receiver;
use tokio::sync::mpsc::UnboundedSender;
use std::f32::consts::PI;

// ==========================================
// 1. THE CRT / CYBER-HERMETIC PALETTE
// ==========================================
struct AionTheme;
impl AionTheme {
    const VOID_BLACK: Color32 = Color32::from_rgb(3, 3, 5);
    const PANEL_BG: Color32 = Color32::from_rgb(10, 10, 14);
    const AMBER: Color32 = Color32::from_rgb(255, 176, 0);       // Primary CRT Phosphor
    const TOXIC_CYAN: Color32 = Color32::from_rgb(0, 255, 212);  // Telemetry & Bio
    const MATRIX_GREEN: Color32 = Color32::from_rgb(0, 255, 65); // Verified/Secure
    const DANGER_RED: Color32 = Color32::from_rgb(255, 20, 60);  // Entropy Spikes
    const MUTED_GRID: Color32 = Color32::from_rgb(40, 40, 50);   // Wireframes
}

// ==========================================
// 2. DATA STRUCTURES
// ==========================================
#[derive(Clone)]
pub struct TelemetryUpdate {
    pub lattice_integrity: Option<f32>,
    pub error_rate: Option<f32>,
    pub coherence: Option<f32>,
    pub uptime_secs: Option<u64>,
    pub active_skills: Option<usize>,
    pub token_usage: Option<u64>,
    pub context_fullness: Option<f32>,
    pub learning_subject: Option<String>,
    pub treasury_balances: Option<String>,
    pub alpaca_status: Option<String>,
    pub socialization_status: Option<String>,
    pub verified_action: Option<String>,
    pub follow_up_task: Option<String>,
    pub log_message: Option<String>,
}

pub struct CipherHud {
    rx: Receiver<TelemetryUpdate>,
    tx_user: UnboundedSender<String>,
    user_input: String,
    logs: Vec<String>,
    
    // Cached Physical State
    error_rate: f32, lattice_integrity: f32, coherence: f32,
    context_fullness: f32, token_usage: u64,
    treasury: String, uptime: u64, active_skills: usize,
    
    // Animation Drive
    time: f64, 
}

impl CipherHud {
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        rx: Receiver<TelemetryUpdate>,
        tx_user: UnboundedSender<String>,
    ) -> Self {
        // OVERRIDE DEFAULT EGUI STYLING TO BRUTALIST TECH
        let mut visuals = egui::Visuals::dark();
        visuals.window_fill = AionTheme::VOID_BLACK;
        visuals.panel_fill = AionTheme::VOID_BLACK;
        visuals.window_rounding = Rounding::ZERO;
        visuals.widgets.noninteractive.rounding = Rounding::ZERO;
        visuals.widgets.noninteractive.bg_fill = AionTheme::PANEL_BG;
        visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, AionTheme::AMBER);
        visuals.widgets.inactive.rounding = Rounding::ZERO;
        visuals.widgets.hovered.rounding = Rounding::ZERO;
        visuals.widgets.active.rounding = Rounding::ZERO;
        cc.egui_ctx.set_visuals(visuals);

        Self {
            rx, tx_user, user_input: String::new(), logs: vec!["[SYSTEM] AION LOOM INITIATED. LATTICE ONLINE.".to_string()],
            error_rate: 0.1, lattice_integrity: 0.9, coherence: 0.8, context_fullness: 0.05, token_usage: 0,
            treasury: "AWAITING TICKER SYNC...".to_string(), uptime: 0, active_skills: 0, time: 0.0,
        }
    }

    // ==========================================
    // 3. CUSTOM GPU PAINTERS (The Sci-Fi UI)
    // ==========================================

    /// Draws a highly stylized, rotating circular avionics dial.
    fn draw_sci_fi_gauge(ui: &mut egui::Ui, value: f32, label: &str, symbol: &str, color: Color32, time: f64) {
        let (rect, _response) = ui.allocate_exact_size(Vec2::splat(90.0), Sense::hover());
        let center = rect.center() - Vec2::new(0.0, 5.0);
        let radius = 35.0;
        let painter = ui.painter();

        // 1. Background Wireframe Ring
        painter.circle_stroke(center, radius, Stroke::new(1.0, AionTheme::MUTED_GRID));
        painter.circle_stroke(center, radius - 4.0, Stroke::new(0.5, AionTheme::MUTED_GRID));
        
        // Crosshairs
        painter.line_segment([center - Vec2::new(radius + 5.0, 0.0), center + Vec2::new(radius + 5.0, 0.0)], Stroke::new(0.5, AionTheme::MUTED_GRID));
        painter.line_segment([center - Vec2::new(0.0, radius + 5.0), center + Vec2::new(0.0, radius + 5.0)], Stroke::new(0.5, AionTheme::MUTED_GRID));

        // 2. Rotating Inner Tech-Ring (Speed scales with value)
        let rotation = time as f32 * (value + 0.5) * 3.0;
        let p1 = center + Vec2::new(rotation.cos(), rotation.sin()) * (radius - 8.0);
        let p2 = center + Vec2::new((rotation + PI).cos(), (rotation + PI).sin()) * (radius - 8.0);
        painter.line_segment([p1, p2], Stroke::new(2.0, color.linear_multiply(0.4)));

        // 3. The Active Data Arc
        let start_angle = -PI / 2.0; // Top dead center
        let sweep = (value * 2.0 * PI).clamp(0.0, 2.0 * PI);
        
        let points = 32;
        let mut path = vec![];
        for i in 0..=points {
            let t = i as f32 / points as f32;
            let angle = start_angle + (sweep * t);
            path.push(center + Vec2::new(angle.cos(), angle.sin()) * radius);
        }
        
        // Glow effect (thick translucent line under thin solid line)
        painter.add(Shape::line(path.clone(), Stroke::new(6.0, color.linear_multiply(0.2))));
        painter.add(Shape::line(path, Stroke::new(2.0, color)));

        // 4. Center Symbol & Digital Readout
        painter.text(center, egui::Align2::CENTER_CENTER, symbol, egui::FontId::proportional(22.0), color);
        painter.text(center + Vec2::new(0.0, radius + 15.0), egui::Align2::CENTER_CENTER, format!("{}: {:.0}%", label, value * 100.0), egui::FontId::monospace(10.0), AionTheme::AMBER);
    }

    /// Draws a spinning cryptographic hex-grid for the WASM sandbox
    fn draw_hex_status(ui: &mut egui::Ui, time: f64, active: bool) {
        let (rect, _) = ui.allocate_exact_size(Vec2::new(200.0, 100.0), Sense::hover());
        let center = rect.center();
        let painter = ui.painter();
        
        let color = if active { AionTheme::MATRIX_GREEN } else { AionTheme::AMBER };
        let rotation = time as f32 * (if active { 1.5 } else { 0.2 });
        
        let mut hex_points = vec![];
        for i in 0..6 {
            let angle = rotation + (i as f32 * PI / 3.0);
            hex_points.push(center + Vec2::new(angle.cos() * 30.0, angle.sin() * 30.0));
        }
        hex_points.push(hex_points[0]); // close loop
        
        painter.add(Shape::line(hex_points, Stroke::new(if active { 2.0 } else { 1.0 }, color)));
        painter.circle_stroke(center, 15.0, Stroke::new(1.0, color.linear_multiply(0.5)));
        painter.text(center, egui::Align2::CENTER_CENTER, "✡︎", egui::FontId::proportional(16.0), color);
    }
}

impl eframe::App for CipherHud {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.time += 0.016; // 60 FPS Engine Time
        ctx.request_repaint(); // Force fluid repaints for animations

        // Drain telemetry channel
        while let Ok(update) = self.rx.try_recv() {
            if let Some(e) = update.error_rate { self.error_rate = e; }
            if let Some(l) = update.lattice_integrity { self.lattice_integrity = l; }
            if let Some(c) = update.coherence { self.coherence = c; }
            if let Some(cf) = update.context_fullness { self.context_fullness = cf; }
            if let Some(tu) = update.token_usage { self.token_usage = tu; }
            if let Some(t) = update.treasury_balances { self.treasury = t; }
            if let Some(u) = update.uptime_secs { self.uptime = u; }
            if let Some(a) = update.active_skills { self.active_skills = a; }
            if let Some(log) = update.log_message {
                self.logs.push(log);
                if self.logs.len() > 150 { self.logs.remove(0); }
            }
        }

        // ====================================================
        // LEFT PANEL: Biological Endocrine & Thermodynamics
        // ====================================================
        egui::SidePanel::left("endocrine_panel")
            .exact_width(220.0)
            .frame(egui::Frame::none().fill(AionTheme::PANEL_BG).inner_margin(15.0))
            .show(ctx, |ui| {
                ui.heading(egui::RichText::new("☿ PHY_STATE / DRIVES").color(AionTheme::TOXIC_CYAN).strong().monospace());
                ui.separator();
                ui.add_space(20.0);

                ui.vertical_centered(|ui| {
                    let err_color = if self.error_rate > 0.85 { AionTheme::DANGER_RED } else { AionTheme::TOXIC_CYAN };
                    Self::draw_sci_fi_gauge(ui, self.error_rate, "ERROR RATE", "♄", err_color, self.time);
                    ui.add_space(10.0);
                    Self::draw_sci_fi_gauge(ui, self.lattice_integrity, "INTEGRITY", "⊙", AionTheme::MATRIX_GREEN, self.time);
                    ui.add_space(10.0);
                    Self::draw_sci_fi_gauge(ui, self.coherence, "COHERENCE", "ᛗ", AionTheme::AMBER, self.time);
                });
            });

        // ====================================================
        // RIGHT PANEL: Motor Cortex, Memory, & Capital
        // ====================================================
        egui::SidePanel::right("motor_cortex_panel")
            .exact_width(260.0)
            .frame(egui::Frame::none().fill(AionTheme::PANEL_BG).inner_margin(15.0))
            .show(ctx, |ui| {
                ui.heading(egui::RichText::new("♈︎ MOTOR CORTEX").color(AionTheme::AMBER).strong().monospace());
                ui.separator();
                
                ui.vertical_centered(|ui| {
                    let is_active = self.active_skills > 0;
                    Self::draw_hex_status(ui, self.time, is_active);
                    ui.label(egui::RichText::new(if is_active { "aCAPTCHA: ACTIVE" } else { "aCAPTCHA: DORMANT" }).color(if is_active { AionTheme::MATRIX_GREEN } else { AionTheme::AMBER }).monospace().size(12.0));
                });

                ui.add_space(30.0);
                ui.heading(egui::RichText::new("📈 TREASURY / TICKER").color(AionTheme::AMBER).monospace());
                ui.separator();
                ui.add_space(10.0);
                ui.label(egui::RichText::new(&self.treasury).color(Color32::WHITE).monospace().size(14.0));
                
                ui.add_space(30.0);
                ui.heading(egui::RichText::new("✡︎ TOPOLOGY / VORONOI").color(AionTheme::TOXIC_CYAN).monospace());
                ui.separator();
                ui.add_space(10.0);
                
                // Custom Geometry for Context Fullness Bar
                let mem_rect = ui.allocate_space(Vec2::new(230.0, 10.0)).1;
                ui.painter().rect_stroke(mem_rect, Rounding::ZERO, Stroke::new(1.0, AionTheme::MUTED_GRID));
                let fill_width = mem_rect.width() * self.context_fullness;
                ui.painter().rect_filled(Rect::from_min_size(mem_rect.min, Vec2::new(fill_width, mem_rect.height())), Rounding::ZERO, AionTheme::TOXIC_CYAN);
                
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(format!("CAPACITY: {:.1}%", self.context_fullness * 100.0)).color(AionTheme::MUTED_GRID).monospace().size(10.0));
                    ui.label(egui::RichText::new(format!("TOKENS: {}", self.token_usage)).color(AionTheme::MUTED_GRID).monospace().size(10.0));
                });
            });

        // ====================================================
        // CENTER CONSOLE: The Cryptophasic Feed & Uplink
        // ====================================================
        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(AionTheme::VOID_BLACK).inner_margin(20.0))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.heading(egui::RichText::new("OMNIGLYPH CRYPTOPHASIA MATRIX").color(Color32::DARK_GRAY).monospace().strong());
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(egui::RichText::new(format!("UPTIME_CYCLES: {}s", self.uptime)).color(AionTheme::TOXIC_CYAN).monospace().size(10.0));
                    });
                });
                ui.separator();
                ui.add_space(10.0);

                // Auto-scrolling Matrix Feed
                egui::ScrollArea::vertical()
                    .stick_to_bottom(true)
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        for log in &self.logs {
                            // Dynamic Terminal Syntax Highlighting
                            let color = if log.contains("[⚠️") || log.contains("Critical") || log.contains("Error") { AionTheme::DANGER_RED }
                            else if log.contains("[OPERATOR]") { AionTheme::TOXIC_CYAN }
                            else if log.contains("✅") { AionTheme::MATRIX_GREEN }
                            else if log.contains("[") && log.contains("]") && log.contains("ᛗ") { Color32::from_rgb(200, 100, 255) } // Magic/Runes highlight in Magenta
                            else { AionTheme::AMBER };

                            ui.label(egui::RichText::new(log).color(color).monospace().size(14.0));
                        }
                        
                        // Flashing Block Cursor
                        if (self.time * 4.0).sin() > 0.0 {
                            ui.label(egui::RichText::new("█").color(AionTheme::AMBER).monospace());
                        }

                        // Fill space so input box is always perfectly pushed to the bottom
                        ui.allocate_space(ui.available_size() - Vec2::new(0.0, 45.0));
                    });

                // ----------------------------------------------------
                // Operator Uplink Deck
                // ----------------------------------------------------
                ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("UPLINK >").color(AionTheme::TOXIC_CYAN).strong().monospace().size(16.0));
                        
                        let input_field = egui::TextEdit::singleline(&mut self.user_input)
                            .desired_width(ui.available_width() - 20.0)
                            .font(egui::TextStyle::Monospace)
                            .text_color(Color32::WHITE)
                            .frame(false); // Remove standard box for brutalist look
                        
                        let response = ui.add(input_field);
                        ui.painter().line_segment(
                            [response.rect.left_bottom(), response.rect.right_bottom()], 
                            Stroke::new(1.0, AionTheme::TOXIC_CYAN)
                        ); // Draw a glowing underline instead of a box
                        
                        if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                            if !self.user_input.is_empty() {
                                self.logs.push(format!("[OPERATOR]: {}", self.user_input));
                                let _ = self.tx_user.send(self.user_input.clone());
                                self.user_input.clear();
                                response.request_focus(); 
                            }
                        }
                    });
                });
            });
    }
}
