# blockchain-rust

- <b>Hash(Puzzle) - Fingerprint</b> , 

- Nonce(Key) is arbitary not random, added as a field to each block, to make the hash of the block start with a certain number of zeros. The number of zeros is the difficulty of the blockchain. The more zeros, the more difficult it is to mine a block. The more difficult it is to mine a block, the more secure the blockchain is.

- Process of finding key is called mining.


### Mining Strategy (Algorithm)

1. Gen new NONCE
2. Hash the block with the new NONCE

==> check_hash_difficulty ? (step 3) : (step 1)

3. If the hash is valid, add the block to the blockchain