use num_bigint::{BigInt, Sign};
use data_encoding::HEXLOWER;
use std::borrow::Borrow;

use crate::Block;

// 난이도 조정 (24비트 = 3바이트의 0)
const TARGET_BITS: u32 = 24;
const MAX_NONCE: i64 = i64::MAX;

pub struct ProofOfWork {
    block: Block,
    target: BigInt,
}

impl ProofOfWork {
    pub fn new_proof_of_work(block: Block) -> ProofOfWork {
        // 난이도 설정: 1 << (256 - TARGET_BITS)
        // 이는 해시의 앞부분에 TARGET_BITS만큼의 0이 있어야 함을 의미
        let mut target = BigInt::from(1);
        for _ in 0..(256 - TARGET_BITS) {
            target = target << 1;
        }
        
        ProofOfWork {
            block,
            target,
        }
    }
    
    fn prepare_data(&self, nonce: i64) -> Vec<u8> {
        // 블록 데이터와 nonce를 결합
        let mut data = Vec::new();
        
        // 이전 블록 해시
        data.extend(self.block.get_pre_block_hash().as_bytes());
        
        // 트랜잭션 해시
        data.extend(self.block.hash_transactions());
        
        // 타임스탬프
        data.extend(&self.block.get_timestamp().to_le_bytes());
        
        // 난이도
        data.extend(&self.target.to_bytes_be().1);
        
        // Nonce
        data.extend(&nonce.to_le_bytes());
        
        data
    }
    
    pub fn run(&self) -> (i64, String) {
        let mut nonce = 0;
        let mut hash = Vec::new();
        
        println!("Mining the block");

        while nonce < MAX_NONCE {
            let data = self.prepare_data(nonce);
            hash = crate::sha256_digest(data.as_slice());
            let hash_int = BigInt::from_bytes_be(Sign::Plus, hash.as_slice());
            
            if hash_int.lt(self.target.borrow()) {
                println!("{}", HEXLOWER.encode(hash.as_slice()));
                break;
            } else {
                nonce += 1;
                
                // 진행 상황 표시 (1000번마다)
                if nonce % 1000 == 0 {
                    print!(".");
                    std::io::Write::flush(&mut std::io::stdout()).unwrap();
                }
            }
        }
        println!();
        return (nonce, HEXLOWER.encode(hash.as_slice()));
    }
}