use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use std::fs::File;
use std::io::{self, Read, Write};
use std::mem::size_of;
use std::slice::from_raw_parts_mut;

const DEFAULT_C: f64 = 0.1;
const DEFAULT_DELTA: f64 = 0.5;

fn split_file(mut f: File, blocksize: usize) -> io::Result<(usize, Vec<u32>)> {
    let mut f_bytes = Vec::new();
    f.read_to_end(&mut f_bytes)?;

    let mut blocks = Vec::with_capacity((f_bytes.len() + blocksize - 1) / blocksize);
    for i in 0..=f_bytes.len() - blocksize {
        let block = &f_bytes[i..i + blocksize];
        let block_data = u32::from_le_bytes(block.try_into().unwrap());
        blocks.push(block_data);
    }

    if f_bytes.len() % blocksize != 0 {
        let padding = vec![0u8; blocksize - f_bytes.len() % blocksize];
        let block_data = u32::from_le_bytes([0u8; 4].try_into().unwrap());
        blocks.push(block_data);
    }

    Ok((f_bytes.len(), blocks))
}

struct Encoder {
    file_size: usize,
    block_size: usize,
    prng: PRNG,
    blocks: Vec<u32>,
    block_index: usize,
}

impl Encoder {
    fn new(f: File, block_size: usize, seed: Option<u32>, c: f64, delta: f64) -> io::Result<Self> {
        let (file_size, blocks) = split_file(f, block_size)?;
        let seed = seed.unwrap_or_else();
        let prng = PRNG::new((blocks.len(), delta, c), seed);
        Ok(Encoder {
            file_size,
            block_size,
            prng,
            blocks,
            block_index: 0,
        })
    }

    fn next_block(&mut self) -> io::Result<Vec<u8>> {
        let blockseed = self.prng.state;
        let d = self.prng._sample_d();
        let mut ix_samples = HashSet::new();
        while ix_samples.len() < d {
            let ix = self.prng._get_next() as usize % self.blocks.len();
            ix_samples.insert(ix);
        }

        let mut block_data = 0;
        for ix in ix_samples {
            block_data ^= self.blocks[ix];
        }

        let block = [
            self.file_size,
            self.block_size,
            blockseed,
            block_data,
        ];

        let mut buffer = vec![0u8; size_of::<u32>() * 4 + self.block_size];
        buffer[0..size_of::<u32>()].copy_from_slice(&block[0].to_le_bytes());
        buffer[size_of::<u32>()..size_of::<u32>() * 2].copy_from_slice(&block[1].to_le_bytes());
        buffer[size_of::<u32>() * 2..size_of::<u32>() * 3].copy_from_slice(&block[2].to_le_bytes());
        buffer[size_of::<u32>() * 3..].copy_from_slice(&block[3].to_le_bytes());

        Ok(buffer)
    }
}

struct PRNG {
    state: u32,
    k: usize,
    cdf: Vec<f64>,
}

impl PRNG {
    fn new(params: (usize, f64, f64), seed: u32) -> Self {
        let (k, delta, c) = params;
        let cdf = sampler::gen_rsd_cdf(k, delta, c);
        PRNG {
            state: seed,
            k,
            cdf,
        }
    }

    fn _get_next(&mut self) -> f64 {
        self.state = (16807 * self.state) % 2147483647;
        self.state as f64 / 2147483647.0
    }

    fn _sample_d(&mut self) -> usize {
        let p = self._get_next();
        for (ix, &v) in self.cdf.iter().enumerate() {
            if v > p {
                return ix + 1;
            }
        }
        self.cdf.len()
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 4 {
        eprintln!("Usage: {} <input_file> <block_size> <output_file>", args[0]);
        return Ok(());
    }

    let input_file = args[1].clone();
    let block_size: usize = args[2].parse().expect("Block size must be a valid integer");
    let output_file = args[3].clone();

    let mut input = File::open(input_file)?;
    let mut output = File::create(output_file)?;

    let encoder = Encoder::new(input, block_size, None, DEFAULT_C, DEFAULT_DELTA)?;

    for block in encoder {
        output.write_all(&block?)?;
    }

    Ok(())
}
