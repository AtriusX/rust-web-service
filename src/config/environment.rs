use std::env;
use std::sync::LazyLock;

pub static ACCESS_TOKEN_EXP_MINUTES: LazyLock<usize> = LazyLock::new(|| {
    let minutes = env::var("ACCESS_TOKEN_EXP_MINUTES")
        .unwrap_or_else(|_| "5".to_string()).parse::<u64>()
        .expect("ACCESS_TOKEN_EXP_MINUTES must be a number");

    (minutes * 60) as usize // Convert from seconds to minutes
});

pub static REFRESH_TOKEN_EXP_DAYS: LazyLock<u64> = LazyLock::new(|| {
    let minutes = env::var("REFRESH_TOKEN_EXP_DAYS")
        .unwrap_or_else(|_| "14".to_string()).parse::<u64>()
        .expect("ACCESS_TOKEN_EXP_MINUTES must be a number");

    minutes * 60 * 60 * 24 // Convert from seconds to days
});