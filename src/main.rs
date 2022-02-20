use std::alloc::System;
use std::any::Any;
use std::collections::{HashMap, LinkedList};
use std::hash::Hash;
use std::ptr::null;
extern crate chrono;
use chrono::offset::Utc;
use chrono::DateTime;
use hex_literal::hex;
use sha2::{Sha256, Sha512, Digest};
use std::time::{SystemTime, UNIX_EPOCH};
// use serde_json::Value::String;
use std::string::String;

extern crate serde;
extern crate serde_json;


pub trait UnitUBlock {
    fn to_string(&self) -> String;
    fn compute_hash(&self) -> String;
    fn change_nonce(&mut self, tmp_nonce: i32);
    // fn add_transactions(&mut self, transaction: LinkedList<String>);
    // fn get_transactions(self);
    fn get_hash(&self) -> &String;
    fn set_hash(&mut self, hash: String);
}

#[derive(Clone)]
pub struct Block {
    pub index : i32,
    pub transactions : LinkedList<String>,
    pub timestamp : String,
    pub previous_hash : String,
    pub nonce : i32,
    pub hash : String,
}

impl UnitUBlock for Block {
    fn to_string(&self) -> String {
        let mut transactions_string = "".to_string();
        for value in self.transactions.iter() {
            transactions_string += value;
        }
        format!("{{index:{},transactions:{},timestamp:{},previous_hash:{},nonce:{},hash{}}}",
                self.index, transactions_string, self.timestamp, self.previous_hash, self.nonce, self.hash)
    }

    fn compute_hash(&self) -> String {
        let mut hasher = Sha256::new();
        let serialized = serde_json::to_string(&self.to_string()).unwrap();
        hasher.update(serialized);
        let result = format!("{:X}", hasher.finalize());
        return result;
    }

    fn change_nonce(&mut self, tmp_nonce: i32) {
        // nonce를 바꿈
        self.nonce = tmp_nonce;
    }

    // fn add_transactions(&mut self, transactions: LinkedList<String>) {
    //     self.transactions = transactions;
    // }
    //
    // fn get_transactions(self) -> LinkedList<String>{
    //     self.transactions;
    // }

    fn get_hash(&self) -> &String {
        /// 해당 블럭의 해시값을 가져옴
        &self.hash
    }

    fn set_hash(&mut self, hash: String) {
        self.hash = hash;
    }
}

pub trait Chain {
    fn new() -> Self;
    fn create_genesis_block(&mut self);
    fn last_block(&self) -> &Block;
    fn proof_of_work(&self, block: Block) -> String;
    fn add_block(&mut self, block: Block, proof: String) -> bool;
    fn is_valid_proof(&self, block: &Block, block_hash: &String) -> bool;
    // fn add_new_transaction(&mut self, transaction: String);
    fn mine(&mut self, transactions: LinkedList<String>) -> i32;
}

pub struct BlockChain {
    pub unconfirmed_transactions : LinkedList<String>,
    pub chain : LinkedList<Block>,
    pub difficulty : i32,
    pub hash : String
}

impl Chain for BlockChain {

    fn new() -> Self {
        let init_unconfirmed_transaction : LinkedList<String> = LinkedList::new();
        let init_chain : LinkedList<Block> = LinkedList::new();
        let mut value = Self {
            unconfirmed_transactions: init_unconfirmed_transaction,
            chain: init_chain,
            difficulty: 4,
            hash: "".to_string()
        };
        value.create_genesis_block();

        return value;
    }
    fn create_genesis_block(&mut self) {
        /// 제네시스 블록(최초의 블록)을 생성하는 함수.
        ///
        let init_list: LinkedList<String> = LinkedList::new();
        let system_time = SystemTime::now();
        let now: DateTime<Utc> = system_time.into();

        let mut genesis_block: Block = Block {
            index: 0,
            transactions: init_list,
            timestamp: now.format("%d/%m/%Y %T").to_string(),
            previous_hash: "0".to_string(),
            nonce: 0,
            hash: "".to_string()
        };
        genesis_block.hash = genesis_block.compute_hash();
        self.chain.push_back(genesis_block);
    }

    fn last_block(&self) -> &Block {
        /// 현 시점에서 가장 마지막 블록을 반환하는 함수.
        ///
        return self.chain.back().unwrap();
    }

    fn proof_of_work(&self, mut block: Block) -> String {
        /// 최한준
        /// 주어진 조건에 부합하는 해쉬값이 존재하는지 확인하는 함수.
        /// 예) difficulty가 4이면 sha256 해쉬값의 앞 네자리 수가 무조건 0000으로 시작.

        let computed_hash = block.compute_hash();
        let mut tmp_nonce = 0;
        while computed_hash.starts_with("0000"){
            tmp_nonce += 1;
            block.change_nonce(tmp_nonce);
        }
        return computed_hash;
    }

    fn add_block(&mut self, mut block: Block, proof: String) -> bool {
        let previous_hash = self.last_block().get_hash();
        if !String::eq(previous_hash, &proof) {
            return false;
        }
        if !self.is_valid_proof(&block, &proof){
            return false;
        }
        block.set_hash(proof);
        self.chain.push_back(block);

        return true;
    }

    fn is_valid_proof(&self, block: &Block, proof: &String) -> bool {
        /// 블록을 검증하는 함수. 조건에 부합하는지(시작이 0000인지 등).
        ///
        let is_valid = proof.starts_with("0000")& (proof.eq(&block.compute_hash()));
        return is_valid;
    }

    // fn add_new_transaction(&mut self, transaction: String) {
    //     /// 거래 내역을 새로 추가하는 함수.
    //     self.unconfirmed_transactions.push_back(transaction);
    // }

    fn mine(&mut self, transactions: LinkedList<String>) -> i32 {
        /// 전체 파이프라인을 한번 실행하는 함수
        ///
        if !transactions.is_empty() {
            return -1;
        }
        let last_block = self.last_block();

        let system_time = SystemTime::now();
        let now: DateTime<Utc> = system_time.into();

        let new_block: Block = Block {
            index: 0,
            transactions: transactions,
            timestamp: now.format("%d/%m/%Y %T").to_string(),
            previous_hash: last_block.get_hash().clone(),
            nonce: 0,
            hash: "".to_string()
        };
        let proof = self.proof_of_work(new_block.clone());
        // new_block.set_transactions(["확인용"]);
        self.add_block(new_block, proof);
        // self.set_transactions(["확인용"]);

        return 1;
    }
    // def mine(self):
        // proof = self.proof_of_work(new_block)
        // self.add_block(new_block,proof)
        // self.unconfirmed_transactions = []
        //
        // return new_block.index

}

fn main() {
    //Initialize block
    let mut sample_chain: BlockChain = BlockChain::new();
    let mut list1 = LinkedList::new();
    list1.push_back(String::from("hyeokmin kwon"));
    for li in list1.clone() {
        println!("{}", li);
    }
    sample_chain.mine(list1);

    for block in sample_chain.chain.iter() {
        println!("{}", block.to_string());
    }

    println!("end");
}
