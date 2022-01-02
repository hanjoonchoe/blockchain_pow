use std::any::Any;
use std::collections::{HashMap, LinkedList};
use hex_literal::hex;
use sha2::{Sha256, Sha512, Digest};
use std::time::{SystemTime, UNIX_EPOCH};
extern crate serde;
extern crate serde_json;

pub trait UnitUBlock {
    fn compute_hash(&self, proof: String) -> String;
    fn change_nonce(&self, tmp_nonce: i32);
    fn get_hash(&self) -> String;
    fn set_hash(&self, hash: String) -> String;
}

pub struct Block {
    pub index : i32,
    pub transactions : LinkedList<String>,
    pub timestamp : String,
    pub hash : String,
    pub nonce : i32
}

impl UnitUBlock for Block {
    fn compute_hash(&self, block: Block) -> String {
        let mut hasher = Sha256::new();
        let serialized = serde_json::to_string(&block).unwrap();
        hasher.update(serialized);
        let result = hasher.finalize();
        return result;
    }

    fn change_nonce(&mut self, tmp_nonce: i32) {
        /// nonce를 바꿈
        self.nonce = tmp_nonce;
    }

    fn get_hash(&self) -> &String {
        /// 해당 블럭의 해시값을 가져옴
        &self.hash;
    }

    fn set_hash(&mut self, hash: String) {
        self.hash = hash;
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

    fn proof_of_work(&self, mut block: Block) -> String {
        /// 최한준
        /// 주어진 조건에 부합하는 해쉬값이 존재하는지 확인하는 함수.
        /// 예) difficulty가 4이면 sha256 해쉬값의 앞 네자리 수가 무조건 0000으로 시작.

        let computed_hash = block.compute_hash();
        while computed_hash.starts_with("0"*4){
            tmp_nonce += 1;
            block.change_nonce(tmp_nonce);
        }
        return computed_hash;
        todo!()
    }

    fn add_block(&self, mut block: Block, proof: String) -> bool {
        /// 최한준
        /// 블록을 추가하는 함수.

        previous_hash = block.get_hash();
        ///if previous_hash != block.last_block.hash{
        ///    return False;
        ///}
        if !self.is_valid_proof(block, proof){
            return False;
        }
        block.set_hash(proof);
        todo!()
    }

    fn is_valid_proof(&self, block: Block, proof: String) -> bool {
        /// 최한준
        /// 블록을 검증하는 함수. 조건에 부합하는지(시작이 0000인지 등).
        ///
        let is_valid = proof.starts_with("0"*4)& (proof==block.compute_hash());
        return is_valid;
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
