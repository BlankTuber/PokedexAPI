use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct EvolutionChain {
    pub evolution_chain_id: u16,
    pub evolution_chain_name: String,
}

// TODO: CRUD FOR Evolution Chain
