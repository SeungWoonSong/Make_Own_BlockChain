use first_own_blockchain::{Block, Transaction, TXInput, TXOutput};

fn main() {
    println!("블록체인 테스트 시작...");
    
    // 제네시스 트랜잭션 생성
    let coinbase_tx = create_coinbase_transaction("Genesis Block Miner");
    println!("코인베이스 트랜잭션 생성 완료");
    
    // 제네시스 블록 생성
    let genesis_block = Block::generate_genesis_block(&coinbase_tx);
    println!("제네시스 블록 생성 완료");
    println!("블록 해시: {}", genesis_block.get_hash());
    println!("블록 높이: {}", genesis_block.get_height());
    println!("블록 타임스탬프: {}", genesis_block.get_timestamp());
    
    // 두 번째 블록 생성
    let tx = create_transaction();
    let transactions = vec![tx];
    let second_block = Block::new_block(genesis_block.get_hash().to_string(), &transactions, 1);
    println!("\n두 번째 블록 생성 완료");
    println!("블록 해시: {}", second_block.get_hash());
    println!("이전 블록 해시: {}", second_block.get_pre_block_hash());
    println!("블록 높이: {}", second_block.get_height());
    
    // 블록 직렬화 및 역직렬화 테스트
    let serialized = second_block.serialize();
    println!("\n블록 직렬화 완료, 크기: {} 바이트", serialized.len());
    
    let deserialized = Block::deserialize(&serialized);
    println!("블록 역직렬화 완료");
    println!("역직렬화된 블록 해시: {}", deserialized.get_hash());
    
    println!("\n블록체인 테스트 완료!");
}

// 코인베이스 트랜잭션 생성 (첫 번째 블록의 보상 트랜잭션)
fn create_coinbase_transaction(to_address: &str) -> Transaction {
    let txin = TXInput {
        txid: vec![],
        vout: 0,
        signature: vec![],
        pub_key: to_address.as_bytes().to_vec(),
    };
    
    let txout = TXOutput {
        value: 50, // 블록 보상
        pub_key_hash: to_address.as_bytes().to_vec(),
    };
    
    Transaction {
        id: vec![1, 2, 3, 4], // 임시 ID
        vin: vec![txin],
        vout: vec![txout],
    }
}

// 일반 트랜잭션 생성
fn create_transaction() -> Transaction {
    let txin = TXInput {
        txid: vec![1, 2, 3, 4],
        vout: 0,
        signature: vec![],
        pub_key: "sender".as_bytes().to_vec(),
    };
    
    let txout = TXOutput {
        value: 10,
        pub_key_hash: "receiver".as_bytes().to_vec(),
    };
    
    Transaction {
        id: vec![5, 6, 7, 8], // 임시 ID
        vin: vec![txin],
        vout: vec![txout],
    }
}
