const PRIME_MAX: usize = 1_000_000;
const PRIME_CNT: usize = 78498;

const fn sieve() -> [u32; PRIME_CNT] {
    let mut is_prime = [true; PRIME_MAX + 1];
    let mut primes: [u32; PRIME_CNT] = [0; PRIME_CNT];
    let mut crr_p = 0;
    is_prime[0] = false;
    is_prime[1] = false;

    let mut i: usize = 2;
    while i <= PRIME_MAX {
        if !is_prime[i] {
            i += 1;
            continue;
        }

        primes[crr_p] = i as u32;
        crr_p += 1;

        let mut j = 2 * i;
        while j <= PRIME_MAX {
            is_prime[j] = false;
            j += i;
        }

        i += 1;
    }

    primes
}

/// Contains all primes between 0 and [PRIME_MAX]
#[allow(long_running_const_eval)]
const PRIMES: [u32; PRIME_CNT] = sieve();
const LAST_PRIME: u32 = *{
    let this = PRIMES.last();
    match this {
        Some(val) => val,
        None => panic!(),
    }
};
const MAX_CHECKABLE: u64 = (LAST_PRIME as u64).pow(2);

/// Does not use rayon for fairness
pub fn is_prime(k: u64) -> Option<bool> {
    if k > MAX_CHECKABLE {
        None
    } else if k <= LAST_PRIME as u64 {
        let k = k as u32;
        Some(PRIMES.binary_search(&k).is_ok())
    } else {
        let upper_bound =
            match PRIMES.binary_search(&((k as f64).sqrt().ceil() as u32)) {
                Ok(i) => i,
                Err(i) => i,
            };
        Some(
            PRIMES[..=upper_bound]
                .iter()
                .find(|&&p| k % (p as u64) == 0)
                .is_none(),
        )
    }
}
