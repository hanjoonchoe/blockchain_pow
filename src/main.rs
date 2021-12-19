use std::any::Any;
use std::collections::{HashMap, LinkedList};

pub trait UnitUBlock {
    fn compute_hash(&self) -> String;
}

pub struct Block {
    pub index : i32,
    pub transactions : LinkedList<String>,
    pub timestamp : String,
    pub previous_hash : String,
    pub nonce : i32
}

impl UnitUBlock for Block {
    fn compute_hash(&self) -> String {
        todo!()
    }
}

pub trait Chain {
    fn create_genesis_block(&self);
    fn last_block(&self);
    fn proof_of_work(&self, block: Block) -> String;
    fn add_block(&self, block: Block, proof: String) -> bool;
    fn is_valid_proof(&self, block: Block, block_hash: String) -> bool;
    fn add_new_transaction(&self, transaction: String);
    fn mine(&self) -> i32;
}

pub struct BlockChain {
    pub unconfirmed_transactions : LinkedList<String>,
    pub chain : LinkedList<Block>,
    pub difficulty : i32
}

impl Chain for BlockChain {
    fn create_genesis_block(&self) {
        /// 제네시스 블록(최초의 블록)을 생성하는 함수.
        ///
        todo!()
    }

    fn last_block(&self) {
        /// 현 시점에서 가장 마지막 블록을 반환하는 함수.
        ///
        todo!()
    }

    fn proof_of_work(&self, block: Block) -> String {
        /// 주어진 조건에 부합하는 해쉬값이 존재하는지 확인하는 함수.
        /// 예) difficulty가 4이면 sha256 해쉬값의 앞 네자리 수가 무조건 0000으로 시작.
        ///
        todo!()
    }

    fn add_block(&self, block: Block, proof: String) -> bool {
        /// 블록을 추가하는 함수.
        ///
        todo!()
    }

    fn is_valid_proof(&self, block: Block, block_hash: String) -> bool {
        /// 블록을 검증하는 함수. 조건에 부합하는지(시작이 0000인지 등).
        ///
        todo!()
    }

    fn add_new_transaction(&self, transaction: String) {
        /// 거래 내역을 새로 추가하는 함수.
        ///
        todo!()
    }

    fn mine(&self) -> i32 {
        /// 전체 파이프라인을 한번 실행하는 함수
        ///
        todo!()
    }
}

fn main() {
    todo!()
}
