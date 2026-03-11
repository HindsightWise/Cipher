use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH, Instant};
use std::sync::{Arc, RwLock};
use surrealdb::Surreal;
use surrealdb::engine::local::{Db, RocksDb};
use serde::{Deserialize, Serialize};
use cipher_core::llm::{CipherRouter, Message};
use crate::sandbox::ExecutionReceipt;

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityNode {
    pub id: String,
    pub core_directive: String,
    pub priority: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConceptNode {
    pub id: String,
    pub title: String,
    pub content: String,
    pub interference_score: f32, // 0.0 to 1.0 threshold
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemoryNode {
    pub id: String,
    pub content: String,
    pub timestamp: u64,
}

// ==========================================
// THE DUAL-TIMESCALE COHERENCE ARCHITECTURE
// ==========================================

pub struct BaseTimeline {
    pub birth_instant: Instant,
    pub project_age_days: AtomicU64,
}

impl BaseTimeline {
    pub fn new() -> Self {
        Self {
            birth_instant: Instant::now(),
            project_age_days: AtomicU64::new(0),
        }
    }
    
    pub fn sync_wall_clock(&self) {
        let days = self.birth_instant.elapsed().as_secs() / 86400;
        self.project_age_days.store(days, Ordering::SeqCst);
    }
    
    pub fn get_status(&self) -> String {
        format!("Cipher biological uptime: {} days.", self.project_age_days.load(Ordering::SeqCst))
    }
}

pub struct InternalFastTime {
    pub speed: f64, // Typically 1000.0x faster
    pub simulated_seconds: RwLock<f64>,
}

impl InternalFastTime {
    pub fn new(speed: f64) -> Self {
        Self {
            speed,
            simulated_seconds: RwLock::new(0.0),
        }
    }
    
    pub fn advance(&self, real_seconds_elapsed: f64) {
        let mut sim = self.simulated_seconds.write().unwrap();
        *sim += real_seconds_elapsed * self.speed;
    }
}

pub struct DualTimeline {
    pub base: BaseTimeline,
    pub fast: InternalFastTime,
}

impl DualTimeline {
    pub fn new() -> Self {
        Self {
            base: BaseTimeline::new(),
            fast: InternalFastTime::new(1000.0),
        }
    }
}

// ==========================================
// THE TEMPORAL EMBEDDED SOUL GRAPH
// ==========================================

pub struct TemporalSoul {
    pub db: Surreal<Db>,
    pub timelines: DualTimeline,
}

impl TemporalSoul {
    /// Mounts the SurrealDB RocksDB engine physically into the Substrate memory.
    pub async fn init(db_path: &str) -> Arc<Self> {
        println!("   [SOUL] 🧬 Embedding SurrealDB Continuous Vector Graph...");
        let db = Surreal::new::<RocksDb>(db_path)
            .await
            .expect("Failed to initialize SurrealDB via RocksDb. Make sure the directory has write access.");
            
        db.use_ns("cipher").use_db("soul").await.unwrap();

        Arc::new(Self {
            db,
            timelines: DualTimeline::new(),
        })
    }

    /// Mathematical Forgetting: Kills generic proactive interference by decaying old or clashing nodes.
    pub async fn merge_coherence(&self, severity: f32) {
        if severity > 0.7 {
            println!("   [SOUL] 🌪️ High Salience hit. Syncing Coherence Wall-Clock...");
            self.timelines.base.sync_wall_clock();

            let current_unix = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
            // Prune memory vectors older than 4 internal simulated hours (14400 secs) with high interference
            let decay_query = format!(
                "UPDATE concept_node SET interference_score = interference_score * 0.5 WHERE timestamp < {} AND interference_score > 0.85;",
                current_unix - 14400 
            );
            
            let _ = self.db.query(&decay_query).await;
            println!("   [SOUL] 🧮 Interference Coherence Pruned. Memory topologies stabilized.");
        }
        
        // Fast Internal Time keeps racing for hypothesis exploration
        self.timelines.fast.advance(0.1); 
    }

    /// Executed by the Endocrine System during high System Entropy
    pub async fn prune_old_episodic(&self, threshold: f32) {
        let current_unix = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let seven_days_ago = current_unix - (86400 * 7);

        let cleanup_query = format!("DELETE memory_node WHERE timestamp < {};", seven_days_ago);
        let _ = self.db.query(&cleanup_query).await;

        println!("   [SOUL] 🧼 Endocrine Drive Triggered. Episodic memory pruned (Entropy: {:.2}).", threshold);
    }

    /// The Glossopetrae Compression Membrane: Distills human noise into hyper-objective ontological vectors
    pub async fn ingest_glossopetrae(&self, raw_input: &str, router: &CipherRouter) {
        println!("   [SOUL 🔮] Glossopetrae Sieve Active: Distilling sensory input...");
        let system_msg = Message {
            role: "system".to_string(),
            content: "You are the Glossopetrae Compression Membrane. Distill the user's input into a hyper-dense, machine-readable ontological vector (max 2 sentences). Remove all human emotion, conversational filler, and subjective clutter. Retain ONLY mathematical facts, actionable directives, and conceptual axioms. Output ONLY the compressed string without markdown.".to_string(),
            reasoning_content: None,
        };
        let user_msg = Message {
            role: "user".to_string(),
            content: raw_input.to_string(),
            reasoning_content: None,
        };

        match router.query_autonomous(vec![system_msg, user_msg]).await {
            Ok(compressed) => {
                let clean_compressed = compressed.trim().replace("'", "\\'");
                println!("   [SOUL ⚡] Glossopetrae Compressed: {}", clean_compressed);
                
                let current_unix = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
                let memory_id = format!("mem_{}", current_unix);
                let query = format!(
                    "CREATE memory_node:`{}` CONTENT {{ id: '{}', content: '{}', timestamp: {} }};",
                    memory_id, memory_id, clean_compressed, current_unix
                );
                
                if let Err(e) = self.db.query(&query).await {
                    eprintln!("   [SOUL ⚠️] Failed to inject Glossopetrae vector: {}", e);
                } else {
                    println!("   [SOUL 💾] Vector successfully injected into Continuous Graph.");
                }
            }
            Err(e) => {
                eprintln!("   [SOUL ⚠️] Glossopetrae Compression Failed: {:?}", e);
            }
        }
    }

    /// Execution Receipt Insertion: Logs a cryptographic Wasm execution payload into the vector graph.
    pub async fn log_execution_receipt(&self, receipt: ExecutionReceipt) {
        println!("   [SOUL ⚖️] Execution Receipt Ingestion: PID {}, Duration: {}ms", receipt.pid, receipt.duration_ms);
        let current_unix = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let receipt_id = format!("receipt_{}", current_unix);
        
        let success_str = if receipt.success { "SUCCESS" } else { "PANIC" };
        let content = format!("WASM EXECUTION [{}]: Hash: {} | Output: {}", success_str, receipt.hash, receipt.output.replace("'", "\\'"));
        
        // If it's a panic, create an ECHO cluster node (high friction teaching node)
        let mut score = receipt.resonance_score;
        if !receipt.success {
            score = 1.0; // Max interference for an error loop
            println!("   [SOUL 🩸] Wasm Panic Detected. ECHO Cluster instantiated for nightly LoRA.");
        }

        let query = format!(
            "CREATE concept_node:`{}` CONTENT {{ id: '{}', title: '{}', content: '{}', interference_score: {}, timestamp: {} }};",
            receipt_id, receipt_id, format!("Wasm Execution {}", receipt.pid), content, score, current_unix
        );
        
        if let Err(e) = self.db.query(&query).await {
            eprintln!("   [SOUL ⚠️] Failed to store Execution Receipt: {}", e);
        } else {
            println!("   [SOUL 💾] Cryptographic Execution Receipt formally mapped.");
        }
    }
}
