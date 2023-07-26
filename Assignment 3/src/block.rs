use crate::queue::{Task, WorkQueue};
use digest::consts::U32;
use sha2::digest::generic_array::GenericArray;
use sha2::{Digest, Sha256};
use std::fmt::Write;
use std::sync;

type Hash = GenericArray<u8, U32>;

#[derive(Debug, Clone)]
pub struct Block {
    prev_hash: Hash,
    generation: u64,
    difficulty: u8,
    data: String,
    proof: Option<u64>,
}

impl Block {
    pub fn initial(difficulty: u8) -> Block {
        // TODO: create and return a new initial block
        Block {
            prev_hash: Hash::default(),
            generation: 0,
            difficulty: difficulty,
            data: String::from(""),
            proof: None,
        }
    }

    pub fn next(previous: &Block, data: String) -> Block {
        // TODO: create and return a block that could follow `previous` in the chain
        Block {
            prev_hash: previous.hash(),
            generation: previous.generation + 1,
            difficulty: previous.difficulty,
            data: data,
            proof: None,
        }
    }

    pub fn hash_string_for_proof(&self, proof: u64) -> String {
        // TODO: return the hash string this block would have if we set the proof to `proof`.
        
        // the previous hash, encoded to a string in lower-case hexadecimal
        let previous_hash = self.prev_hash.iter().map(|b| format!("{:02x}", b)).collect::<String>();

        let hash_string = format!("{}:{}:{}:{}:{}", previous_hash, self.generation, self.difficulty, self.data, proof);

        hash_string
    }

    pub fn hash_string(&self) -> String {
        // self.proof.unwrap() panics if block not mined
        let p = self.proof.unwrap();
        self.hash_string_for_proof(p)
    }

    pub fn hash_for_proof(&self, proof: u64) -> Hash {
        // TODO: return the block's hash as it would be if we set the proof to `proof`.
        let mut hasher = Sha256::new();
        hasher.update(self.hash_string_for_proof(proof));

        let result = hasher.finalize();

        result
    }

    pub fn hash(&self) -> Hash {
        // self.proof.unwrap() panics if block not mined
        let p = self.proof.unwrap();
        self.hash_for_proof(p)
    }

    pub fn set_proof(self: &mut Block, proof: u64) {
        self.proof = Some(proof);
    }

    pub fn is_valid_for_proof(&self, proof: u64) -> bool {
        // TODO: would this block be valid if we set the proof to `proof`?
        let n_bytes = self.difficulty / 8;
        let n_bits = self.difficulty % 8;
        let hash = self.hash_for_proof(proof);

        // Check each of the last n_bytes bytes are 0u8.
        for i in 0..n_bytes {
            if hash[usize::from(32 - n_bytes + i)] != 0u8 {
                return false;
            }
        }

        // Check that the next byte (from the end) is divisible by 1<<n_bits (one left-shifted n_bits times is equal to 2^n_bits).
        if hash[usize::from(32 - n_bytes - 1)] % (1 << n_bits) != 0u8 {
            return false;
        }

        return true;
    }

    pub fn is_valid(&self) -> bool {
        if self.proof.is_none() {
            return false;
        }
        self.is_valid_for_proof(self.proof.unwrap())
    }

    // Mine in a very simple way: check sequentially until a valid hash is found.
    // This doesn't *need* to be used in any way, but could be used to do some mining
    // before your .mine is complete. Results should be the same as .mine (but slower).
    pub fn mine_serial(self: &mut Block) {
        let mut p = 0u64;
        while !self.is_valid_for_proof(p) {
            p += 1;
        }
        self.proof = Some(p);
    }

    pub fn mine_range(self: &Block, workers: usize, start: u64, end: u64, chunks: u64) -> u64 {
        // TODO: With `workers` threads, check proof values in the given range, breaking up into `chunks` tasks in a work queue. Return the first valid proof found.
        // HINTS:
        // - Create and use a queue::WorkQueue.
        // - Use sync::Arc to wrap a clone of self for sharing.

        let mut work_queue = WorkQueue::new(workers);

        let block = sync::Arc::new(self.clone());

        let mut range = (end - start) / chunks;
        let mut block_num = chunks;

        if range == 0 {
            range = 1;
            block_num = end;
        }

        let mut start_num = start;
        let mut end_num = range;

        for _ in 0..block_num {
            if end_num > end {
                end_num = end;
            }
            if start_num > end {
                break;
            }
            let task = MiningTask::new(block.clone(), start_num, end_num);
            let _ = work_queue.enqueue(task);
            start_num += range;
            end_num += range;
        }

        // dequeue tasks and check if valid
        for _ in 0..block_num {
            let result = work_queue.recv();
            work_queue.shutdown();
            if result != u64::MAX {
                return result;
            }
        }

        u64::MAX
    }

    pub fn mine_for_proof(self: &Block, workers: usize) -> u64 {
        let range_start: u64 = 0;
        let range_end: u64 = 8 * (1 << self.difficulty); // 8 * 2^(bits that must be zero)
        let chunks: u64 = 2345;
        self.mine_range(workers, range_start, range_end, chunks)
    }

    pub fn mine(self: &mut Block, workers: usize) {
        self.proof = Some(self.mine_for_proof(workers));
    }
}

struct MiningTask {
    block: sync::Arc<Block>,
    // TODO: more fields as needed
    start: u64,
    end: u64,
}

impl MiningTask {
    // TODO: implement MiningTask::new(???) -> MiningTask
    pub fn new(block: sync::Arc<Block>, start: u64, end: u64) -> MiningTask {
        MiningTask {
            block: block,
            start: start,
            end: end,
        }
    }
}

impl Task for MiningTask {
    type Output = u64;

    fn run(&self) -> Option<u64> {
        // TODO: what does it mean to .run?
        for i in self.start..self.end {
            if self.block.is_valid_for_proof(i) {
                return Some(i);
            }
        }
        None
    }
}
