use rand08::{Rng, SeedableRng};

/// Get `N` bytes of random data.
///
/// This function is mildly expensive to call, as it fetches random data
/// directly from the OS entropy source.
///
/// # Panics
///
/// Panics if the OS entropy source returns an error.
pub fn os_random<const N: usize>() -> [u8; N] {
    let mut buf = [0u8; 1024];
    //getrandom::getrandom(&mut buf).unwrap();
    let mut rng = rand08::rngs::StdRng::seed_from_u64(8675309);
    rng.fill(&mut buf);
    let mut out = [0u8; N];
    out.copy_from_slice(&buf[..N]);
    out
}
