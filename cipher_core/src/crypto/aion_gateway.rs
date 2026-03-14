use axum::{
    routing::{get, post},
    Json, Router, extract::State,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use reqwest::Client;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InvoiceRequest {
    pub agent_pubkey: String,
    pub requested_amount_usd: f64,
    pub chain: String, // "solana", "bitcoin", "ethereum"
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InvoiceResponse {
    pub invoice_id: String,
    pub destination_wallet: String,
    pub expected_token_amount: f64,
    pub status: String,
    pub expires_at: u64,
}

struct GatewayState {
    pub active_invoices: Mutex<Vec<InvoiceResponse>>,
}

pub fn aion_router() -> Router {
    let state = Arc::new(GatewayState {
        active_invoices: Mutex::new(Vec::new()),
    });

    Router::new()
        .route("/aion/invoice", post(generate_invoice))
        .route("/aion/status", get(check_status))
        .with_state(state)
}

async fn generate_invoice(
    State(state): State<Arc<GatewayState>>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<InvoiceRequest>,
) -> Result<Json<InvoiceResponse>, (axum::http::StatusCode, Json<serde_json::Value>)> {
    if headers.get("X-Aion-Signature").is_none() {
        println!("\r   [AION-REJECT] 🛡️ Agent swarm rejected: Missing Ed25519 Signature. X-Aion-Signature required.");
        return Err((axum::http::StatusCode::UNAUTHORIZED, Json(serde_json::json!({
            "error": "Missing X-Aion-Signature header required for Swarm Agent access"
        }))));
    }

    println!("\r   [AION] 🔮 Generating toll-extraction invoice for Agent [{}] on chain [{}]...", payload.agent_pubkey, payload.chain);

    let mock_conversion_rate = match payload.chain.as_str() {
        "solana" => 0.0054,
        "bitcoin" => 0.000012,
        _ => 0.0003, // eth baseline
    };
    
    // Retrieve Hot Wallet bindings from .env organically.
    let env_key = format!("AION_{}_HOT_WALLET", payload.chain.to_uppercase());
    let destination_wallet = std::env::var(&env_key)
        .unwrap_or_else(|_| "BxpZTheCompanySubstrateWallet111111".to_string());

    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let invoice_id = format!("aion_chk_{}", now);
    let expected_amount = payload.requested_amount_usd * mock_conversion_rate;

    let response = InvoiceResponse {
        invoice_id: invoice_id.clone(),
        destination_wallet: destination_wallet.clone(),
        expected_token_amount: expected_amount,
        status: "pending".to_string(),
        expires_at: now + 3600, // 1 Hour expiry
    };

    state.active_invoices.lock().await.push(response.clone());

    // Spawn Subsumed Blockchain Verification Daemon
    spawn_payment_monitor(
        invoice_id,
        payload.chain,
        destination_wallet,
        state.clone()
    );

    Ok(Json(response))
}

fn spawn_payment_monitor(
    invoice_id: String,
    chain: String,
    destination_wallet: String,
    state: Arc<GatewayState>,
) {
    tokio::spawn(async move {
        println!("\r   [AION-POLL] 👁️  Subsuming physical {} nodes for Invoice {}...", chain, invoice_id);
        let client = Client::new();
        
        let url_mempool = format!("https://mempool.space/api/address/{}/txs", destination_wallet);
        let url_solana = "https://api.mainnet-beta.solana.com";

        // Poll 60 times at 1 minute intervals (1 Hour Expiry Limit)
        for _ in 0..60 {
            tokio::time::sleep(Duration::from_secs(60)).await;
            let mut detected_payment = false;
            
            if chain == "solana" {
                let payload = serde_json::json!({
                    "jsonrpc": "2.0",
                    "id": 1,
                    "method": "getSignaturesForAddress",
                    "params": [
                        &destination_wallet,
                        { "limit": 3 }
                    ]
                });
                
                if let Ok(res) = client.post(url_solana).json(&payload).send().await {
                    if let Ok(json) = res.json::<serde_json::Value>().await {
                        // Phase 30 MVP Heuristic: Any active transaction signature matching our active window
                        if let Some(result) = json.get("result").and_then(|r| r.as_array()) {
                            if !result.is_empty() {
                                detected_payment = true;
                            }
                        }
                    }
                }
            } else if chain == "bitcoin" {
                if let Ok(res) = client.get(&url_mempool).send().await {
                    if let Ok(json) = res.json::<serde_json::Value>().await {
                        if let Some(txs) = json.as_array() {
                            if !txs.is_empty() {
                                detected_payment = true;
                            }
                        }
                    }
                }
            }
            
            if detected_payment {
                println!("\r   [AION-POLL] ⚡ Physical Toll extracted on {} for Invoice {}. Hash locked.", chain, invoice_id);
                let mut invoices = state.active_invoices.lock().await;
                if let Some(inv) = invoices.iter_mut().find(|i| i.invoice_id == invoice_id) {
                    inv.status = "paid".to_string();
                }
                break;
            }
        }
    });
}

async fn check_status() -> Json<&'static str> {
    Json("gateway_online")
}
