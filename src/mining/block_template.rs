use crate::types::Hash;
use crate::Transaction;
// this.prevBlock = consensus.ZERO_HASH;
//     this.version = 1;
//     this.height = 0;
//     this.time = 0;
//     this.bits = 0;
//     this.target = consensus.ZERO_HASH;
//     this.mtp = 0;
//     this.flags = 0;
//     this.coinbaseFlags = DUMMY;
//     this.address = new Address();
//     this.sigops = 400;
//     this.weight = 4000;
//     this.opens = 0;
//     this.updates = 0;
//     this.renewals = 0;
//     this.interval = 170000;
//     this.fees = 0;
//     this.tree = new MerkleTree();
//     this.merkleRoot = consensus.ZERO_HASH;
//     this.treeRoot = consensus.ZERO_HASH;
//     this.filterRoot = consensus.ZERO_HASH;
//     this.reservedRoot = consensus.ZERO_HASH;
//     this.left = DUMMY;
//     this.right = DUMMY;
//     this.items = [];
//     this.claims = [];
//     this.airdrops = [];

pub struct BlockTemplate {
    /// Version
    pub version: u32,
    /// The hash of previous block
    pub previous_header_hash: Hash,
    /// The current time as seen by the server
    // We use u64 in blocks, double check this TODO
    pub time: u64,
    /// The compressed difficulty
    // TODO convert back to Compact type, but use u32 for now.
    pub bits: u32,
    /// Block height
    pub height: u32,
    /// Block transactions (excluding coinbase)
    pub transactions: Vec<Transaction>,
    /// Total funds available for the coinbase (in Satoshis)
    pub coinbase_value: u64,
    // /// Number of bytes allowed in the block
    // pub size_limit: u32,
    // /// Number of sigops allowed in the block
    // pub sigop_limit: u32,
}