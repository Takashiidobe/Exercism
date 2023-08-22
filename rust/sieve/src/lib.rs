pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    use reikna::prime::prime_sieve;

    prime_sieve(upper_bound)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![2, 3, 5, 7], primes_up_to(10));
    }
}
