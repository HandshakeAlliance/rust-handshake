use crate::mining::block_template::BlockTemplate;
use crate::protocol::consensus::consensus_verify_pow;
use crate::types::{Buffer, Hash, Uint256};
use crate::Transaction;
use cryptoxide::blake2b::Blake2b;
use cryptoxide::digest::Digest;

use sp800_185::KMac;

//FROM parity Bitcoin -> https://github.com/paritytech/parity-bitcoin/blob/master/miner/src/cpu_miner.rs
/// Instead of serializing `BlockHeader` from scratch over and over again,
/// let's keep it serialized in memory and replace needed bytes
struct BlockHeaderBytes {
    data: Buffer,
}

impl BlockHeaderBytes {
    /// Creates new instance of block header bytes.
    fn new(version: u32, previous_header_hash: Hash, bits: u32) -> Self {
        let merkle_root_hash = Hash::default();
        let witness_root_hash = Hash::default();
        let tree_root_hash = Hash::default();
        let reserved_root_hash = Hash::default();

        let time = 0u64;
        let nonce = Uint256::default();

        //TODO just make this a block, and then have a helper function called to_buffer()
        let mut buffer = Buffer::new();
        buffer.write_u32(version);
        buffer.write_hash(previous_header_hash);
        buffer.write_hash(merkle_root_hash);
        buffer.write_hash(witness_root_hash);
        buffer.write_hash(tree_root_hash);
        buffer.write_hash(reserved_root_hash);
        buffer.write_u64(time);
        buffer.write_u32(bits);
        buffer.write_u256(nonce);

        BlockHeaderBytes { data: buffer }
    }

    // /// Set merkle root hash
    fn set_merkle_root_hash(&mut self, hash: &Hash) {
        let merkle_bytes: &mut [u8] = &mut self.data[4 + 32..4 + 32 + 32];
        merkle_bytes.copy_from_slice(&hash.to_array());
    }

    /// Set block header time
    // TODO needs testing.
    fn set_time(&mut self, time: u64) {
        //TODO there has to be an easier way to reference this data I would think.
        //Perhaps constants?
        let mut time_bytes: &mut [u8] = &mut self.data[4 + 32 + 32 + 32 + 32 + 32..4 + 32];
        time_bytes.copy_from_slice(&time.to_le_bytes());
    }

    /// Set block header nonce
    fn set_nonce(&mut self, nonce: &Uint256) {
        let mut nonce_bytes: &mut [u8] = &mut self.data[4 + 32 + 32 + 32 + 32 + 32 + 8 + 4..];
        nonce_bytes.copy_from_slice(&nonce.to_le_bytes());
    }

    /// Returns block header hash
    fn hash(&self) -> Hash {
        //https://github.com/handshake-org/hsd/blob/master/lib/mining/mine.js#L28-L29
        //TODO this needs to go into the consensus file
        const NONCE_POSITION: usize = 208;
        let data = &self.data[0..NONCE_POSITION];
        let nonce = &self.data[NONCE_POSITION..self.data.len()];

        let mut key = [0; 32];

        let mut kmac = KMac::new_kmac256(nonce, &[]);
        kmac.update(&data);
        kmac.finalize(&mut key);

        let mut hasher = Blake2b::new_keyed(32, &key);
        hasher.input(&data);

        let mut hash = [0; 32];

        hasher.result(&mut hash);

        Hash::from(hash)
    }
}

// /// This trait should be implemented by coinbase transaction.
// pub trait CoinbaseTransactionBuilder {
//     /// Should be used to increase number of hash possibities for miner
//     fn set_extranonce(&mut self, extranonce: &[u8]);
//     /// Returns transaction hash
//     fn hash(&self) -> Hash;
//     // /// Coverts transaction into raw bytes
//     fn finish(self) -> Transaction;
// }

//pub struct CoinbaseTransactionBuilder {
//    //TODO make this a custom hash type, and value type.
//    // pub fn new(hash: Hash, value: u64) -> Self {

//    //     // let transaction = Transaction {
//    //     // }

//    // }

//}

/// Cpu miner solution.
pub struct Solution {
    /// Block header nonce.
    pub nonce: Uint256,
    /// Coinbase transaction extra nonce (modyfiable by miner).
    // pub extranonce: U256,
    pub extranonce: Uint256,
    /// Block header time.
    pub time: u64,
    // /// Coinbase transaction (extranonce is already set).
    pub coinbase_transaction: Transaction,
}

/// Simple bitcoin cpu miner.
///
/// First it tries to find solution by changing block header nonce.
/// Once all nonce values have been tried, it increases extranonce.
/// Once all of them have been tried (quite unlikely on cpu ;),
/// and solution still hasn't been found it returns None.
/// It's possible to also experiment with time, but I find it pointless
/// to implement on CPU.
/// TODO extranonce == u32??
/// extranonce maybe should be a Uint256? TODO XXX TODO Update extranonce should definitely be
/// u32.... But we can switch that later.
// pub fn find_solution<T>(
pub fn find_solution(
    block: &BlockTemplate,
    // mut coinbase_transaction_builder: T,
    // max_extranonce: Uint256,
    max_extranonce: u32,
) -> Option<Solution>
// where
    // T: CoinbaseTransactionBuilder,
{
    let mut extranonce: u32 = 0;
    let mut extranonce_bytes = [0u8; 4];

    let mut header_bytes = BlockHeaderBytes::new(
        block.version,
        block.previous_header_hash.clone(),
        block.bits,
    );
    // update header with time
    header_bytes.set_time(block.time);

    while extranonce < max_extranonce {
        // extranonce.to_little_endian(&mut extranonce_bytes);
        // update coinbase transaction with new extranonce
        coinbase_transaction_builder.set_extranonce(&extranonce.to_le_bytes());

        // recalculate merkle root hash
        let coinbase_hash = coinbase_transaction_builder.hash();
        let mut merkle_tree = vec![&coinbase_hash];
        // merkle_tree.extend(block.transactions.iter().map(|tx| &tx.hash()));
        //TODO
        // let merkle_root_hash = merkle_root(&merkle_tree);
        let merkle_root_hash = Hash::default();

        // update header with new merkle root hash
        header_bytes.set_merkle_root_hash(&merkle_root_hash);

        let mut nonce = Uint256::default();

        loop {
            // Check if this should be reference or not.
            header_bytes.set_nonce(&nonce);
            let hash = header_bytes.hash();
            if consensus_verify_pow(&hash, block.bits) {
                let solution = Solution {
                    nonce,
                    extranonce,
                    time: block.time,
                    coinbase_transaction: coinbase_transaction_builder.finish(),
                };

                return Some(solution);
            }

            //Maybe rework this, seems like a lot of checking
            if nonce == Uint256::max_value() {
                break;
            } else {
                nonce.increment();
            }
        }

        extranonce.increment();
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    pub struct P2shCoinbaseTransactionBuilder {
        transaction: Transaction,
    }

    impl P2shCoinbaseTransactionBuilder {
        pub fn new(hash: &AddressHash, value: u64) -> Self {
            let script_pubkey = Builder::build_p2sh(hash).into();

            let transaction = Transaction {
                version: 0,
                inputs: vec![TransactionInput::coinbase(Bytes::default())],
                outputs: vec![TransactionOutput {
                    value: value,
                    script_pubkey: script_pubkey,
                }],
                lock_time: 0,
            };

            P2shCoinbaseTransactionBuilder {
                transaction: transaction,
            }
        }
    }

    impl CoinbaseTransactionBuilder for P2shCoinbaseTransactionBuilder {
        fn set_extranonce(&mut self, extranonce: &[u8]) {
            self.transaction.inputs[0].script_sig = extranonce.to_vec().into();
        }

        fn hash(&self) -> Hash {
            self.transaction.hash()
        }

        fn finish(self) -> Transaction {
            self.transaction
        }
    }

    #[test]
    fn test_cpu_miner_low_difficulty() {
        let block_template = BlockTemplate {
            version: 0,
            previous_header_hash: 0.into(),
            time: 0,
            bits: U256::max_value().into(),
            height: 0,
            transactions: Vec::new(),
            coinbase_value: 10,
            size_limit: 1000,
            sigop_limit: 100,
        };

        let hash = Default::default();
        let coinbase_builder = P2shCoinbaseTransactionBuilder::new(&hash, 10);
        let solution = find_solution(&block_template, coinbase_builder, U256::max_value());
        assert!(solution.is_some());
    }
}
