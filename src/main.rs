use std::iter::Iterator;
use std::ops::Range;

struct Sieve {
   base: usize,
   is_prime: Vec<bool>,
}

impl Sieve {
    pub fn new(range: Range<usize>) -> Sieve {
        //println!("sieve from {} to {}", range.start, range.end);
        Sieve {
            base: range.start,
            is_prime: vec![true; range.end-range.start]
        }
    }
    pub fn strike_out(&mut self, prime: usize){
        let mut offset = (prime - self.base % prime) % prime;
        //print!("{} divides ", prime);
        while offset < self.is_prime.len() {
            //print!("{} ", self.base + offset);
            self.is_prime[offset] = false;
            offset += prime;
        }
    }
    pub fn read_holes(&self, out: &mut Vec<usize>) {
        for (i, &val) in self.is_prime.iter().enumerate() {
            if val {
                out.push(self.base + i);
            }
        }
    }
}

struct Primes {
    prime_list: Vec<usize>,
    highest_checked: usize,
    sieve_chunk: usize,
}

struct PrimesIterator<'a> {
    place: usize,
    primes: &'a mut Primes
}

impl<'a> Iterator for PrimesIterator<'a> {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        if self.place >= self.primes.prime_list.len() {
            let end = std::cmp::min(self.primes.highest_checked + self.primes.sieve_chunk,
                          self.primes.prime_list.last().unwrap()*self.primes.prime_list.last().unwrap());
            let mut sieve = Sieve::new((self.primes.highest_checked+1)..end);
            for prime in self.primes.prime_list.iter() {
                sieve.strike_out(*prime);
            }
            sieve.read_holes(&mut self.primes.prime_list);
            self.primes.highest_checked = end - 1;
        }
        let ret = self.primes.prime_list[self.place];
        self.place += 1;
        Some(ret)
    }
}

impl Primes {
    pub fn new() -> Primes {
        let found = vec![2,3,5,7];
        Primes {
            prime_list: found,
            highest_checked: 7,
            sieve_chunk: 1024
        }
    }
    pub fn iter<'a>(&'a mut self) -> PrimesIterator<'a> {
        PrimesIterator {
            place: 0,
            primes: self
        }
    }
}

#[cfg(test)]
mod test {
    use super::Primes;

    #[test]
    fn high_prime() {
        //104729 is the 10000th prime according to wolfram alpha
        assert_eq!(Primes::new().iter().take(10000).last().unwrap(), 104729);
    }
}

fn prime_table(n: usize) {
    assert!(n>0);
    let primes: Vec<usize> = Primes::new().iter().take(n).collect();
    let widest_column = (primes[n-1] * primes[n-1]).to_string().len();
    print!("{:width$}|", "X", width = widest_column);
    for col in primes.iter() {
        print!("{:width$}|", col, width = widest_column);
    }
    print!("\n");
    for row in primes.iter() {
        print!("{:width$}|", row, width = widest_column);
        for col in primes.iter() {
            print!("{:width$}|", row*col, width = widest_column);
        }
        print!("\n");
    }
}

fn main() {
    prime_table(10);
}
