extern crate crypto;

use self::crypto::digest::Digest;
use self::crypto::sha2::Sha256;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        InvalidBlock {
            description("The block was invalid")
        }
        InvalidChain {
            description("The chain was invalid")
        }
    }
}

fn calc_hash(index: &u32, timestamp: &u32, prev_hash: &str, payload: &str) -> String {
    let record = format!("{}{}{}{}",
        index,
        timestamp,
        prev_hash,
        payload
    );

    // create a Sha256 object
    let mut hasher = Sha256::new();
    hasher.input_str(&record);
    let hex = hasher.result_str();

    return hex;
}

fn is_block_valid(prev_block: &Block, new_block: &Block) -> bool {
    // check invalid conditions
    if prev_block.index + 1 != new_block.index {
        return false;
    }

    if prev_block.hash != new_block.prev_hash {
        return false;
    }

    if calc_hash(
        &new_block.index,
        &new_block.timestamp,
        &new_block.prev_hash,
        &new_block.payload ) != new_block.hash 
    {
        return false;
    }

    // otherwise return true
    return true;
}

fn is_chain_valid(current_chain: &Blockchain, new_chain: &Blockchain) -> bool {
    // compare genesis blocks to ensure same origin
    if let Some(genesis_block) = current_chain.blocks.first().cloned() {
        if let Some(new_origin) = new_chain.blocks.first().cloned() {
            let genesis_block_hash = calc_hash(
                &genesis_block.index,
                &genesis_block.timestamp, 
                &genesis_block.prev_hash, 
                &genesis_block.payload
            );

            let new_origin_hash = calc_hash(
                &new_origin.index, 
                &new_origin.timestamp, 
                &new_origin.prev_hash, 
                &new_origin.payload
            );

            if genesis_block_hash != new_origin_hash {
                println!("Genesis block mismatch!");
                return false;
            }

            // verify each block in chain is valid
            let mut prev_block = new_origin;
            let mut index = 0;

            for new_block in new_chain.blocks.iter() {
                // skip origin
                if index > 0 {
                    if is_block_valid(&prev_block, new_block) {
                        continue;
                    } else {
                        println!("Invalid block detected with index {}!", new_block.index);
                        return false;
                    }
                }
                index += 1; // increment index to skip first one
            }
            
            return true;
        } else {
            println!("Missing new chain origin!");
            return false;
        }
    } else {
        println!("Could not find genesis block");
        return false;
    }
}

#[derive(Debug,Clone)]
struct Block {
    index: u32,
    timestamp: u32,
    hash: String,
    prev_hash: String,
    payload: String,
}

impl Block {
    pub fn new(prev_block: &Block, payload: &str) -> Block {
        let index = prev_block.index + 1;
        let timestamp = prev_block.timestamp + 10;
        let prev_hash = format!("{}", prev_block.hash);
    
        Block {
            hash: calc_hash(&index, &timestamp, &prev_hash, &payload),
            index,
            timestamp,
            prev_hash,
            payload: String::from(payload),
        }
    }
}

#[derive(Debug)]
struct Blockchain {
    blocks: Vec<Block>
}

impl Blockchain {
    pub fn new(genesis_block: Block) -> Blockchain {
        Blockchain {
            blocks: vec![genesis_block],
        }
    }

    pub fn add_block(&mut self, payload: &str) -> Result<(), Error> {
        if let Some(prev_block) = self.blocks.last().cloned() {
            let new_block = Block::new(&prev_block, payload);

            if is_block_valid(&prev_block, &new_block) {
                println!("Adding block to chain");
                self.blocks.push(new_block);
                Ok(())
            } else {
                println!("Block was invalid");
                Err(Error::InvalidBlock)
            }
        } else {
            println!("Could not find previous block to compare");
            Err(Error::InvalidBlock)
        }
    }

    pub fn replace(&mut self, new_chain: Blockchain) -> Result<(), Error> {
        let local_len = self.blocks.len();
        let new_len = new_chain.blocks.len();

        if is_chain_valid(&self, &new_chain) && new_len > local_len {
            println!("Valid chain. Replacing current chain with new one.");
            self.blocks = new_chain.blocks;
            Ok(())
        } else {
            println!("Invalid replacement chain");
            Err(Error::InvalidChain)
        }
    }
}

pub fn run() {
    println!("Testing chain ...");

    let genesis = Block {
        index: 0,
        timestamp: 0,
        prev_hash: String::new(),
        payload: "Genesis block baby!".to_string(),
        hash: String::new(),
    };

    let mut blockchain: Blockchain = Blockchain::new(genesis);

    let result = blockchain.add_block("Second block baby!");

    println!("{:?}", blockchain);
    println!("{:?}", result);

}