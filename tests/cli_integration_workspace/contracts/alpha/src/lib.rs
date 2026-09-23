#![no_std]
use soroban_sdk::{symbol_short, Env, Symbol};

const COUNTER: Symbol = symbol_short!("counter");

pub fn bad_storage_in_loop(env: Env) {
    for _ in 0..10 {
        env.storage().instance().set(&COUNTER, &1i32);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn test_bad_storage_in_loop() {
        let env = Env::default();
        bad_storage_in_loop(env.clone());
        let count: i32 = env.storage().instance().get(&COUNTER).unwrap();
        assert_eq!(count, 1);
    }
}
