use rand::{Rng, SeedableRng};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct RandomStateXoshiro256Plus {
    state: rand_xoshiro::Xoshiro256Plus,
}

fn create_state(seed: i32) -> rand_xoshiro::Xoshiro256Plus {
    return rand_xoshiro::Xoshiro256Plus::seed_from_u64(seed as u64);
}

#[wasm_bindgen]
impl RandomStateXoshiro256Plus {
    #[wasm_bindgen(constructor)]
    pub fn new(seed: i32) -> RandomStateXoshiro256Plus {
        // set_panic_hook();
        let state = create_state(seed);
        RandomStateXoshiro256Plus {
            state,
        }
    }

    pub fn seed(&mut self, seed: i32) {
        self.state = create_state(seed);
    }

    pub fn random(&mut self) -> f64 {
        self.state.gen::<f64>()
    }

    pub fn jump(&mut self) {
        self.state.jump()
    }

    pub fn long_jump(&mut self) {
        self.state.long_jump()
    }

    pub fn clone(&self) -> RandomStateXoshiro256Plus {
        let state = self.state.clone();
        RandomStateXoshiro256Plus {
            state,
        }
    }
}
