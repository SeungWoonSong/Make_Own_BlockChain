extern crate chrono;
extern crate crypto;
extern crate data_encoding;
extern crate num_bigint;
extern crate serde;

use crypto::digest::Digest;
use crypto::sha2::Sha256;
use chrono::Utc;

// 모듈 선언
pub mod block;
pub mod proof_of_work;
pub mod transaction;

// 타입 재내보내기
pub use block::Block;
pub use proof_of_work::ProofOfWork;
pub use transaction::{Transaction, TXInput, TXOutput};

// 현재 타임스탬프를 반환하는 함수
pub fn current_timestamp() -> i64 {
    Utc::now().timestamp()
}

// SHA-256 해시 함수
pub fn sha256_digest(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.input(data);
    let mut result = vec![0; hasher.output_bytes()];
    hasher.result(&mut result);
    result
}
