use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: Vec<u8>,
    pub vin: Vec<TXInput>,
    pub vout: Vec<TXOutput>,
}

impl Transaction {
    // 트랜잭션의 ID를 가져오는 함수
    pub fn get_id(&self) -> &[u8] {
        &self.id
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TXInput {
    pub txid: Vec<u8>,
    pub vout: usize,
    pub signature: Vec<u8>,
    pub pub_key: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TXOutput {
    pub value: i32,
    pub pub_key_hash: Vec<u8>,
}