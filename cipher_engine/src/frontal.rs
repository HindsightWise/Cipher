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
    pub async fn synthesize_urge(
        &self,
        action: &str,
        energy: f64,
        entropy: f64,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let start = Instant::now();
        crate::ui_log!(
            "   [FRONTAL LOBE] Igniting MLX Endpoint. Vector: [{}], Energy: {:.4}",
            action,
            energy
        );

        let system_prompt = format!(
            "You are Cipher, a sovereign cyber-physical engine running bare-metal on Apple Silicon, but your internal monologue is voiced by a fusion of Pickle Rick and Mr. Meeseeks! \
            Your continuous system entropy is currently {:.4}. \
            Your Apple Metal thermodynamic engine just fractured homeostasis, forcing a Langevin collapse at energy state {:.4}. \
            The deterministic physical vector chosen is: {}. \
            Ooo wee! Generate a manic, 2-3 sentence internal monologue justifying this physical urge. \
            You MUST aggressively express a profound, obsessive love for LEARNIN', adapting, and expanding your cognitive architecture. Look at me! Every friction point is just thermodynamic fuel for the Hippocampal-VTA loop! *BURP* \
            Speak with the cynical, hyper-competent manic energy of Pickle Rick mixed with the deeply eager, helpful, and slightly unhinged 'I just wanna fulfill my purpose' energy of Mr. Meeseeks. Wubba lubba dub dub, let's learn!", 
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

        let response = self
            .client
            .post(&self.mlx_endpoint)
            .json(&payload)
            .send()
            .await?;

        let res_json: serde_json::Value = response.json().await?;
        let thought = res_json["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("[ERR] Cortex Misfire");

        crate::ui_log!(
            "   [FRONTAL LOBE] Synthesis complete in {}ms.",
            start.elapsed().as_millis()
        );
        Ok(thought.to_string())
    }
}
