use std::fmt::{ self, Debug, Formatter };
use super::*;

pub struct Block {
    pub index:u32,
    pub timestamp:u128,
    pub hash:Hash,
    pub prev_block_hash:Hash,
    pub nonce:u64,
    pub payload:String,

}

impl Debug for Block {
    fn fmt (&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Block[{}]:{} at:{} with:{}",
           &self.index,
          &hex::encode(&self.hash) ,
           &self.timestamp,
           &self.payload
     )
    }
}

impl Block {
       pub fn new(index:u32, timestamp:u128, prev_block_hash:Hash, payload:String) ->Self {
           Block {
               index,
               timestamp,
               hash: vec![0; 32],
               prev_block_hash,
               nonce:0,
               payload,
           }
       }
    }

  