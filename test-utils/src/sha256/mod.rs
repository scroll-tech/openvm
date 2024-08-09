use openssl::sha;
use p3_symmetric::CryptographicHasher;

/// The SHA2-256 hash function.
#[derive(Copy, Clone, Debug)]
pub struct Sha256;

impl CryptographicHasher<u8, [u8; 32]> for Sha256 {
    fn hash_iter<I>(&self, input: I) -> [u8; 32]
    where
        I: IntoIterator<Item = u8>,
    {
        let input = input.into_iter().collect::<Vec<_>>();
        self.hash_iter_slices([input.as_slice()])
    }

    fn hash_iter_slices<'a, I>(&self, input: I) -> [u8; 32]
    where
        I: IntoIterator<Item = &'a [u8]>,
    {
        let mut hasher = sha::Sha256::new();
        for chunk in input.into_iter() {
            hasher.update(chunk);
        }
        hasher.finish()
    }
}
