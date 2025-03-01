extern crate bincode;
use sled::IVec;

use crate::proof_of_work::ProofOfWork;
use crate::transaction::Transaction;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Block {
    timestamp: i64,
    pre_block_hash: String,
    hash: String,
    transactions: Vec<Transaction>,
    nonce: i64,
    height: usize,
}

impl Block {
    pub fn new_block(pre_block_hash: String, transactions: &[Transaction], height: usize) -> Block {
        let mut block = Block {
            timestamp: crate::current_timestamp(),
            pre_block_hash,
            hash: String::new(),
            transactions: transactions.to_vec(),
            nonce: 0,
            height,
        };
        
        let pow = ProofOfWork::new_proof_of_work(block.clone());
        let (nonce, hash) = pow.run();
        
        block.nonce = nonce;
        block.hash = hash;
        return block;
    }

    pub fn deserialize(bytes: &[u8]) -> Block {
        bincode::deserialize(bytes).unwrap()
    }

    pub fn serialize(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap().to_vec()
    }
    
    // 트랜잭션 목록을 가져오는 함수
    pub fn get_transactions(&self) -> &[Transaction] {
        self.transactions.as_slice()
    }
    
    // 이전 블록의 해시를 가져오는 함수
    pub fn get_pre_block_hash(&self) -> String {
        self.pre_block_hash.clone()
    }
    
    // 현재 블록의 해시를 가져오는 함수
    pub fn get_hash(&self) -> &str {
        self.hash.as_str()
    }
    
    // 현재 블록의 해시를 바이트 배열로 가져오는 함수
    pub fn get_hash_bytes(&self) -> Vec<u8> {
        self.hash.as_bytes().to_vec()
    }
    
    // 블록의 타임스탬프를 가져오는 함수
    pub fn get_timestamp(&self) -> i64 {
        self.timestamp
    }
    
    // 블록의 높이를 가져오는 함수
    pub fn get_height(&self) -> usize {
        self.height
    }
    
    // 트랜잭션들을 해싱하는 함수
    pub fn hash_transactions(&self) -> Vec<u8> {
        let mut txhashs = vec![];
        for transaction in &self.transactions {
            txhashs.extend(transaction.get_id());
        }
        crate::sha256_digest(txhashs.as_slice())
    }
    
    // 제네시스 블록(첫 번째 블록)을 생성하는 함수
    pub fn generate_genesis_block(transaction: &Transaction) -> Block {
        let transactions = vec![transaction.clone()];
        return Block::new_block(String::from("None"), &transactions, 0);
    }
}

impl From<Block> for IVec {
    fn from(b: Block) -> Self {
    let bytes = bincode::serialize(&b).unwrap();
    Self::from(bytes)
    }
}