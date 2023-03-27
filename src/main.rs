use blockchainlib::*;


fn main() {
   let mut block=Block::new(0,0,vec![0;32],"Genesis block".to_owned(),0x00ffffffffffffffffffffffffffffff);
       
    
   
    
    block.hash=block.hash();

    println!("{:?}",&block);
   
    block.mine();  /* Computationally Expensive */

    println!("{:?}",&block);
}
