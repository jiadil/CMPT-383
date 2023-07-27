pub fn factors_iterator(n: u64) -> impl Iterator<Item = u64> {
    // TODO: returns an iterator of the factors of n (not including 1 and n). 
    // Hint: you can create an iterator of useful integers with "(2..=n/2)".
    (2..=n/2).filter(move |x| n % x == 0)
}

pub fn factors(n: u64) -> Vec<u64> {
    // TODO: uses factors_iterator and returns a Vec of the factors of n. 
    // Hint: collect.
    factors_iterator(n).collect()
}

pub fn is_prime(n: u64) -> bool {
    // TODO: uses factors_iterator and determines if the number is prime or not: primes will have an empty iterator of factors. 
    // Hint: look at what .next() returns for an iterator.
    factors_iterator(n).next() == None
}
