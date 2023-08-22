pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut total = 0;
    for i in 1..limit {
        for factor in factors {
            if *factor == 0 {
                continue;
            }
            if i % factor == 0 {
                total += i;
                break;
            }
        }
    }
    total
}
