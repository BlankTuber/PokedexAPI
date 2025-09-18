use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EvolutionChain {
    pub evolution_chain_id: i32,
    pub evolution_chain_name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CreateEvolutionChain {
    pub evolution_chain_name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UpdateEvolutionChain {
    pub evolution_chain_name: Option<String>,
}
