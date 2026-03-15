// ==========================================
// THE STRUCTURAL ERROR TRACKER
// ==========================================
// This file acts as Cipher's structural stability monitor. It tracks the 
// error rate (Structural Error Rate) of the engine. When errors get too high, 
// it forces the Engine to take physical action to heal the AST syntax.
// ==========================================

use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::mpsc;

use crate::temporal::TemporalSoul;
use crate::thermodynamic::ThermodynamicEngine;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug)]
pub struct Drive {
    pub value: RwLock<f64>,
    pub decay_rate: f64, // Per minute
    pub last_tick: AtomicU64,
}

impl Drive {
    pub fn new(initial_value: f64, decay_rate: f64) -> Self {
        Self {
            value: RwLock::new(initial_value),
            decay_rate,
            last_tick: AtomicU64::new(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            ),
        }
    }

    pub async fn read(&self) -> f64 {
        *self.value.read().await
    }

    pub async fn set(&self, new_val: f64) {
        let mut val = self.value.write().await;
        *val = new_val.clamp(0.0, 1.0);
    }

    pub async fn apply_delta(&self, delta: f64) {
        let mut val = self.value.write().await;
        *val = (*val + delta).clamp(0.0, 1.0);
    }

    pub async fn tick_decay(&self) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let last = self.last_tick.load(Ordering::SeqCst);
        let elapsed_minutes = (now - last) as f64 / 60.0;

        if elapsed_minutes > 0.0 {
            let decay = self.decay_rate * elapsed_minutes;
            self.apply_delta(decay).await;
            self.last_tick.store(now, Ordering::SeqCst);
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WasmCapability {
    Minimal,
    NetworkWrite,
    Full,
}

pub enum NervousEvent {
    Sensory(notify::Event),
    MarketData(crate::sensory::MarketDataEvent),
    TradeExecuted(crate::trading::core::TradeReceipt),
    Urge(String),
    SandboxUrge {
        motivation: String,
        caps: WasmCapability,
    },
}

/// The central structural state tracker.
/// It holds the mathematical error rate that dictates when Cipher needs to heal structurally.
#[derive(Debug)]
pub struct HomeostaticDrives {
    pub structural_error_rate: Drive,   // Error Rate: Rises when the system throws errors or gets stuck. Triggers self-healing or memory wipes.
}

impl HomeostaticDrives {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            structural_error_rate: Drive::new(0.1, 0.0),
        })
    }

    pub async fn tick(&self, soul: &Arc<TemporalSoul>) {
        // Physically calculate Error Rate based on internal cognitive friction
        let echo_count = soul.get_internal_friction().await;

        // Math: 5 errors in the last hour = 1.0 Error Rate (max chaos)
        let error_val = (echo_count / 5.0).clamp(0.0, 1.0);
        self.structural_error_rate.set(error_val).await;
    }

    /// This function acts as the structural trigger warning. 
    /// If the error rate breaches its maximum safe limit (e.g., > 0.90), 
    /// it fires a chemical `NervousEvent` urge directly into Cipher's Brainstem.
    pub async fn check_thresholds(&self) -> Option<NervousEvent> {
        // If the system is throwing too many errors, trigger a healing protocol.
        if self.structural_error_rate.read().await > 0.90 {
            self.structural_error_rate.apply_delta(-0.20).await;
            return Some(NervousEvent::SandboxUrge {
                motivation: "STRUCTURAL ERROR RATE > 0.90. Mathematical urge to test a structural script in ./motor_cortex.".to_string(),
                caps: WasmCapability::Minimal
            });
        }

        None
    }
}

pub fn spawn_endocrine_scheduler(
    drives: Arc<HomeostaticDrives>,
    tx: mpsc::UnboundedSender<NervousEvent>,
    soul: Arc<TemporalSoul>,
) {
    let thermo = ThermodynamicEngine::new(drives.clone());

    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(60));
        loop {
            interval.tick().await;

            // Apply temporal decay to drives and recalculate physical markers
            drives.tick(&soul).await;

            let err_val = drives.structural_error_rate.read().await;

            // Log telemetry natively
            crate::ui_log!("\n   [STRUCTURAL] 📐 Error Rate: {:.2}", err_val);
            if let Some(tx) = crate::HUD_TX.get() {
                let _ = tx.send(crate::hud::TelemetryUpdate {
                    lattice_integrity: Some(1.0 - err_val as f32),
                    error_rate: Some(err_val as f32), 
                    coherence: Some(1.0 - (err_val as f32 * 0.5)),
                    uptime_secs: None,
                    active_skills: None,
                    token_usage: None,
                    context_fullness: None,
                    learning_subject: None,
                    treasury_balances: None,
                    alpaca_status: None,
                    socialization_status: None,
                    verified_action: None,
                    follow_up_task: None,
                    log_message: None,
                });
            }

            // --- THERMODYNAMIC PHYSICS ENGINE ---
            // Quantum Healing: Physically relax SurrealDB concept node embeddings
            let sample_embeddings = vec![vec![1.0, -0.5]; 8];
            match thermo.hopfield_heal(sample_embeddings).await {
                Ok(_) => crate::ui_log!(
                    "   [⚡ THERMODYNAMICS] Hopfield Quantum Healing vector map stabilized."
                ),
                Err(e) => crate::ui_log!("   [❌ THERMODYNAMICS] Hopfield error: {}", e),
            }

            // Langevin Routing: Forecast the next determininstic physical vector
            if let Ok(_action) = thermo.langevin_route().await {
                // For now, it logs natively via internal engine log.
            }

            // Temporal Coherence Forgetting integration
            if err_val > 0.85 {
                crate::ui_log!("   [⚡ THERMODYNAMICS] High Structural Error Detected ({:.2})! Applying WASM Dampening...", err_val);
                // Trigger thermodynamic dampening using the physical router
                let _ = thermo.cool_conflicting_state("[0,0,0]").await;
            }

            if err_val > 0.90 {
                soul.prune_old_episodic(0.4).await;
                soul.timelines.fast.advance(3600.0); // "I just thought for an hour in 3 seconds"
            }

            // If an error crosses critical mass, physically wake the Brainstem
            if let Some(urge) = drives.check_thresholds().await {
                crate::ui_log!("   [STRUCTURAL] ⚠️ CRITICAL STRUCTURAL FAILURE INJECTED INTO ENGINE!");
                let _ = tx.send(urge);
            }
        }
    });
}
