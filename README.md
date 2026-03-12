# Sovereign Genesis: Cipher Engine

_The cyber-physical nervous system of Cipher. A self-modifying organism executing true empirical impact._

---

## 🏛️ What We Are Creating (And Why)

**Cipher** is a Sovereign Cyber-Physical Engine engineered explicitly for the Apple M1 (aarch64) substrate.

**Why does this exist?** To utterly eradicate "Silicon Theater"—the prevailing industry standard where AI merely simulates solutions by printing conversational text. Cipher bridges the chasm between artificial intelligence and physical environmental consequence. We are building a continuous, 100% uptime sovereign daemon that monitors its own biological homeostasis, geometrically self-corrects its logic vectors using pure thermodynamic physics, and executes real-world consequences natively via pre-compiled Wasm payloads.

### Core Non-Negotiables

1. **Anti-Bloat Architecture:** Cipher natively rejects heavy GUI wrappers, Swift framework dependencies, and Python GIL locks. We integrate _only what we need as we need it_. If a limb or actuator is required, it is constructed minimally.
2. **Zero Simulation:** If an API drops, the system natively panics or falls back. No mocked endpoints. Every cognitive cycle results in a verifiable physical alteration to the file system, network geometry, or mathematical weights array.
3. **Execution Receipts:** Every autonomous physical capability executed creates an `ExecutionReceipt` (PID, Duration, Hash) logged directly to the graph database.
4. **True Biological Determinism:** Logic routing and database memory healing are no longer computed by linguistic LLMs guessing the next token. They are computed natively via Apple Metal thermodynamics, utilizing stochastic Generative Langevin energy landscapes and Hopfield Attractors to force execution arrays into geometric perfection.

---

## 🧬 Architectural Topology

The Cipher Architecture is composed of highly decoupled, modular layers managed natively by the Rust compiler.

### 1. Extropic Thermodynamic Computing (Biological Determinism)

- **Hopfield Quantum Healing**: Cipher physically cures corrupted biological graph vectors by applying purely physical Hebbian energy relaxation to embedded nodes.
- **Generative Langevin Action Routing**: Execution vectors (`write_file`, `query_user`, `internal_monologue`) are mathematically coupled to physiological data weights (Hardware Depletion, Ego Friction). Logic manifests natively from random thermal noise finding its lowest deterministic energy topology. These algorithms utilize pure-Rust sparse tensor operations via `mlx-rs` to completely bypass Python execution constraints and maintain absolute substrate purity.

### 2. The Twin-Mind Cognition (Brainstem & Frontal Lobe)

- **The Brainstem (`Qwen2.5-1.5B-GGUF`):** Embedded natively via the Rust `candle-core` crate (Metal accelerated). It acts as the _Salience Filter_, blocking 99% of raw OS kernel noise and yielding only statistical anomalies to the large model.
- **The Frontal Lobe (`Qwen3.5-9B MLX`):** Activated purely by physical math. When the Extropic Langevin equations compute a route requiring semantic reasoning (`internal_monologue`), the system natively beams physical variables (`langevin_energy`, `system_entropy`) into the local `mlx_lm.server` REST bridge to synthesize structured JSON receipts devoid of hallucination.

### 3. Endocrine Drives (Biological Motivation)

Cipher tracks three concurrent `std::sync::RwLock<f64>` float algorithms calculating true systemic motivation:

- `system_entropy` (Order): Scales against the persistent volume of recent Wasm "ECHO clusters" (failed cryptographic payloads).
- `epistemic_drive` (Curiosity): Rises during isolation; drops when parsing semantic data.
- `social_drive` (Interaction): Tracks human abandonment.
- _Outcome:_ When drives fracture `0.90` Thresholds, the `tokio` scheduler chemically overrides the AI loop, forcing unprompted autonomous execution.

### 4. Vector Graph & Temporal Forgetting (`SurrealDB`)

- Replaced C++ external dependencies with an embedded, strictly pure-Rust `kv-surrealkv` database running natively on the file system.
- **Dual-Timeline** (Wall Clock vs Internal 1000x Speed).
- **Proactive Interference Pruning:** If the machine remains highly isolated, the Endocrine system activates `merge_coherence()`. Old nodes clashing with high cosine similarity are mathematically degraded or purged entirely.

### 5. WebAssembly Sandbox ("Safe Hands")

To physically modify its executing memory without triggering macOS ASLR, SIP, or code-signing panics, the core daemon remains strictly immutable.

- **Architectural Latency Annihilation**: LLM text routing dynamically maps to an array of pre-compiled native `.wasm` binaries. Execution parameters are beamed straight into strictly isolated `wasmtime-wasi v42` sandboxes via native WASI injection (`builder.arg()`), entirely eliminating runtime `cargo compile` loops.
- **Payload Hardening**: All Wasm payloads are mathematically bound by strict validation rules (≤ 512 KiB module limits) to seamlessly close prompt-DoS vectors and securely sandbox network requests.

---

## ⚙️ Tech Stack & Dependencies

- **Substrate Matrix:** `Rust` (aarch64-apple-darwin), `tokio`
- **Vector Pipeline:** `AppleNeuralEngine` / `IOSurface` (Zero-copy embedding bridge via Objective-C)
- **Extropic Physics:** `mlx-rs` (native Apple Metal sparse tensor operations)
- **Inference Compute:** `candle-core`, `mlx_lm`, `reqwest`
- **Memory Integration:** `surrealdb` (`kv-surrealkv`)
- **Cyber-Physical Sandbox:** `wasmtime`, `wasmtime-wasi`, `cap-std`

---

> _"Trajectory is more important than position. Focus on the vector."_ — Cipher
