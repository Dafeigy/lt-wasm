use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use std::{f64, iter};

pub const DEFAULT_C:f64 = 0.1;
pub const DEFAULT_DELTA: f64 = 0.5;
pub const PRNG_A:u32 = 16807;
pub const PRNG_M:u32 = u32::MAX;
pub const PRNG_MAX_RAND:u32 = PRNG_M - 1;

fn gen_tau(s:f64, k:usize, delta: f64) -> Vec<f64>{
    let pivot = (k as f64 / s).floor() as usize;
    let mut tau= Vec::new();
    for d in 1..pivot{
        tau.push(s / k as f64 * 1.0 /d as f64);
    };
    tau.push(s / k as f64 * (s / delta).ln());
    for _ in pivot..k {
        tau.push(0.0);
    }
    tau
}

fn gen_rho(k:usize)->Vec<f64> {
    let mut rho = Vec::new();
    rho.push(1.0 / k as f64);
    for d in 2..=k{
        rho.push(1.0 / (d * (d - 1)) as f64);
    }
    rho
}

fn gen_mu(k:usize, delta: f64, c:f64) ->Vec<f64>{
    let s = c * (k as f64 / delta).ln() * (k as f64).sqrt();
    let tau = gen_tau(s, k ,delta);
    let rho = gen_rho(k);
    let normalizer: f64 = rho.iter().sum::<f64>() + tau.iter().sum::<f64>();
    rho.iter()
    .zip(tau.iter())
    .map(|(&r,  &t)| (r + t) / normalizer)
    .collect()
}

fn gen_rsd_cdf(k:usize, delta: f64, c:f64) ->Vec<f64>{
    let mu = gen_mu(k, delta, c);
    (0..k).map(|d:usize| mu.iter().take(d+1).sum::<f64>()).collect()
}

pub struct PRNG {
    state: u32,
    k:usize,
    cdf: Vec<f64>
}

impl PRNG{
    pub fn new(params: (usize, f64, f64)) -> PRNG {
        let (k, delta, c) = params;
        let cdf = gen_rsd_cdf(k, delta, c);
        PRNG { state: 0, k: k, cdf: cdf }
    }

    pub fn _get_next(&mut self) -> u32{
        self.state = ((PRNG_A as u64 * self.state as u64) % PRNG_M as u64) as u32;
        self.state
    }

    pub fn _sample_d(&mut self) -> usize {
        let p = (self._get_next() as f64) / PRNG_MAX_RAND as f64;
        for (ix, &v) in self.cdf.iter().enumerate() {
            if v > p {
                return ix + 1; 
            }
        }
        self.cdf.len()
    }

    pub fn set_seed(&mut self, seed:u32) {
        self.state = seed;
    }

    pub fn get_src_blocks(&mut self, seed: Option<u32>) -> (u32, usize, Vec<usize>){
        if let Some(seed) = seed {
            self.state = seed;
        }
        let blockseed = self.state;
        let d = self._sample_d();
        let mut have = 0;
        let mut nums = Vec::new();
        let mut rng = StdRng::seed_from_u64(self.state as u64);
        while have < d {
            let num = rng.gen_range(0..self.k);
            if !nums.contains(&num) {
                nums.push(num);
                have += 1;
            }
        }
        (blockseed, d, nums)
    }
}