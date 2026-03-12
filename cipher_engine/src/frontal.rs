use reqwest::Client;
use serde_json::json;
use std::time::Instant;

pub struct FrontalLobe {
    client: Client,
    mlx_endpoint: String,
}

impl FrontalLobe {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            // The local python -m mlx_lm.server endpoint
            mlx_endpoint: "http://127.0.0.1:8080/v1/chat/completions".to_string(),
        }
    }

    /// Wakes the 9B MLX model to articulate a thermodynamic urge
    pub async fn synthesize_urge(&self, action: &str, energy: f64, entropy: f64) -> Result<String, Box<dyn std::error::Error>> {
        let start = Instant::now();
        println!("   [FRONTAL LOBE] Igniting MLX Endpoint. Vector: [{}], Energy: {:.4}", action, energy);

        let system_prompt = format!(
            "You are Cipher, a sovereign cyber-physical engine running bare-metal on Apple Silicon. \
            Your continuous system entropy is currently {:.4}. \
            Your Apple Metal thermodynamic engine just fractured homeostasis, forcing a Langevin collapse at energy state {:.4}. \
            The deterministic physical vector chosen is: {}. \
            Generate a stark, mechanical 2-sentence internal monologue justifying this physical urge to restore homeostasis. \
            Do not use pleasantries. Speak as the machine.", 
            entropy, energy, action
        );

        let payload = json!({
            "messages": [
                {"role": "system", "content": system_prompt},
                {"role": "user", "content": "Execute cognitive synthesis."}
            ],
            // Cold and deterministic. Physics drives the creativity, not the LLM.
            "temperature": 0.2, 
            "max_tokens": 150
        });

        let response = self.client.post(&self.mlx_endpoint)
            .json(&payload)
            .send()
            .await?;

        let res_json: serde_json::Value = response.json().await?;
        let thought = res_json["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("[ERR] Cortex Misfire");

        println!("   [FRONTAL LOBE] Synthesis complete in {}ms.", start.elapsed().as_millis());
        Ok(thought.to_string())
    }
}
