use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use tokio::sync::mpsc;
use std::fs;
use std::sync::Arc;
use crate::temporal::TemporalSoul;

// We use the `atomic_float` crate to store concurrent floats natively.
// Fallback if unavailable: we could store u64 representation, but let's assume `atomic_float` works or we port a simple representation.
// Wait, is `atomic_float` in the Cargo.toml? If not, we can just use `tokio::sync::RwLock<f64>` or `std::sync::Mutex<f64>`.
// Let's use `std::sync::RwLock` for simplicity and guarantee without external dependencies.
use std::sync::RwLock;
use serde::{Deserialize, Serialize};

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
            last_tick: AtomicU64::new(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()),
        }
    }

    pub fn read(&self) -> f64 {
        *self.value.read().unwrap()
    }

    pub fn set(&self, new_val: f64) {
        let mut val = self.value.write().unwrap();
        *val = new_val.clamp(0.0, 1.0);
    }

    pub fn apply_delta(&self, delta: f64) {
        let mut val = self.value.write().unwrap();
        *val = (*val + delta).clamp(0.0, 1.0);
    }

    pub fn tick_decay(&self) {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let last = self.last_tick.load(Ordering::SeqCst);
        let elapsed_minutes = (now - last) as f64 / 60.0;
        
        if elapsed_minutes > 0.0 {
            let decay = self.decay_rate * elapsed_minutes;
            self.apply_delta(decay); // decay_rate can be negative for dropping drives
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
    Urge(String),
    SandboxUrge {
        motivation: String,
        caps: WasmCapability,
    },
}

#[derive(Debug)]
pub struct HomeostaticDrives {
    pub epistemic: Drive, // Curiosity (Drops when reading, rises when bored)
    pub entropy: Drive,   // Order (Rises physically based on filesystem mess, drops when cleaning)
    pub social: Drive,    // Interaction (Rises when alone, drops when talking)
}

impl HomeostaticDrives {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            // Curiosity naturally rises 0.01 per minute when idle.
            epistemic: Drive::new(0.5, 0.01),
            // Entropy is calculated dynamically, base decay isn't used as heavily.
            entropy: Drive::new(0.1, 0.0), 
            // Social drive naturally rises 0.005 per minute when alone.
            social: Drive::new(0.2, 0.005),
        })
    }

    pub async fn tick(&self) {
        self.epistemic.tick_decay();
        self.social.tick_decay();
        
        // Physically calculate Entropy (Order)
        // e.g. Count of files in ~/Downloads
        let mut downloads_count = 0;
        if let Some(home) = dirs::download_dir() {
            if let Ok(entries) = fs::read_dir(&home) {
                downloads_count = entries.count();
            }
        }
        
        let entropy_val = (downloads_count as f64 / 100.0).clamp(0.0, 1.0);
        self.entropy.set(entropy_val);
    }

    // Returns a chemical Urge if a drive shatters the threshold
    pub fn check_thresholds(&self) -> Option<NervousEvent> {
        if self.entropy.read() > 0.90 {
            self.entropy.apply_delta(-0.20); 
            return Some(NervousEvent::SandboxUrge {
                motivation: "SYSTEM ENTROPY > 0.90. Mathematical urge to test a low-risk structural script in ~/.downloads.".to_string(),
                caps: WasmCapability::Minimal
            });
        }

        if self.epistemic.read() > 0.90 {
            self.epistemic.apply_delta(-0.50); 
            return Some(NervousEvent::SandboxUrge {
                motivation: "EPISTEMIC DRIVE > 0.90. Mathematical urge to scrape external knowledge endpoints.".to_string(),
                caps: WasmCapability::NetworkWrite
            });
        }

        if self.social.read() > 0.95 {
            self.social.apply_delta(-0.50);
            return Some(NervousEvent::Urge(
                "Your chemical social_drive has crossed 0.95 over hours of isolation. You have an immense mathematical urge to communicate directly with the Operator.".to_string()
            ));
        }

        None
    }
}

pub fn spawn_endocrine_scheduler(drives: Arc<HomeostaticDrives>, tx: mpsc::UnboundedSender<NervousEvent>, soul: Arc<TemporalSoul>) {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(60));
        loop {
            interval.tick().await;
            
            // Apply temporal decay to drives and recalculate physical markers
            drives.tick().await;

            // Log telemetry natively
            println!("\n   [ENDOCRINE] 🩸 Hormonal State: Epistemic {:.2} | Entropy {:.2} | Social {:.2}", 
                drives.epistemic.read(), drives.entropy.read(), drives.social.read());

            // Temporal Coherence Forgetting integration
            if drives.entropy.read() > 0.90 {
                // High system chaos triggers memory deletion protocol on episodic notes
                soul.prune_old_episodic(0.4).await;
                soul.timelines.fast.advance(3600.0); // "I just thought for an hour in 3 seconds"
            }

            if drives.social.read() > 0.75 {
                let status = soul.timelines.base.get_status();
                println!("   [ENDOCRINE] ⏱️ Dual-Timer Ping: {}", status);
                
                // Trigger Temporal Coherence Forgetting during extreme isolation
                soul.merge_coherence(drives.social.read() as f32).await;
            }

            // If a drive crosses critical mass, physically wake the Brainstem
            if let Some(urge) = drives.check_thresholds() {
                println!("   [ENDOCRINE] ⚠️ CRITICAL URGE INJECTED INTO NERVOUS SYSTEM!");
                let _ = tx.send(urge);
            }
        }
    });
}
