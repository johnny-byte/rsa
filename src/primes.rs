use num_bigint::{BigInt, BigUint, RandBigInt, ToBigInt, ToBigUint};
use num_traits::{one, zero};

fn gcd(a: BigUint, b: BigUint) -> BigUint {
    let r = a % &b;
    if r != zero() {
        return gcd(b.clone(), r);
    }
    return b;
}

fn gcdext(a: BigInt, b: BigInt) -> (BigInt, BigInt, BigInt) {
    if b == zero() {
        return (a, one(), zero());
    }
    let (d, x, y) = gcdext(b.clone(), &a % &b);
    let (x, y) = (y.clone(), x - (&a / &b) * y);
    return (d, x, y);
}

pub fn mul_inv_mod(n: BigUint, m: BigUint) -> BigUint {
    if gcd(n.to_biguint().unwrap(), m.to_biguint().unwrap()) != one() || n >= m {
        //TODO: fix that n=3, m=5
        panic!(
            "Error in mul_inv_mod: n,m must be coprime and n must be less than m; n={:?},m={:?}",
            n, m
        );
    }
    let (_, _, inv) = gcdext(m.to_bigint().unwrap(), n.to_bigint().unwrap());
    if inv < zero() {
        return (inv + m.to_bigint().unwrap()).to_biguint().unwrap();
    } else {
        return inv.to_biguint().unwrap();
    }
}

pub fn get_lower_and_coprime_with(n: BigUint) -> BigUint {
    let mut rng = rand::thread_rng();
    let lower = 2.to_biguint().unwrap();
    let mut t: BigUint = rng.gen_biguint_range(&lower, &n);
    while gcd(n.clone(), t.clone()) != one() {
        t = rng.gen_biguint_range(&lower, &n);
    }
    return t;
}

//TODO: make bit size
pub fn get_primes(n: usize) -> Vec<BigUint> {
    use std::collections::HashSet;
    let mut primes = HashSet::new();
    let mut rng = rand::thread_rng();

    while primes.len() < n {
        //TODO: make bit size
        let mut num = rng.gen_biguint(256);
        if &num % 2.to_biguint().unwrap() == zero() {
            num += one::<BigUint>();
        }
        let step = 2.to_biguint().unwrap();
        loop {
            if fast_prime_test(num.clone()) {
                if hard_prime_test(num.clone()) {
                    primes.insert(num);
                    break;
                }
            }
            num += &step;
        }
    }
    return primes.into_iter().collect();
}

pub fn powmod(b: BigUint, e: BigUint, m: BigUint) -> BigUint {
    fn f(b: BigUint, e: BigUint, m: BigUint, r: Option<BigUint>) -> BigUint {
        let r = match r {
            Some(x) => x,
            None => one(),
        };
        if e == zero() {
            return r;
        }
        if &e & one::<BigUint>() == one() {
            return f(
                (&b * &b) % &m,
                e.clone() >> 1,
                m.clone(),
                Some((r.clone() * b.clone()) % m.clone()),
            );
        } else {
            return f((&b * &b) % &m, e.clone() >> 1, m.clone(), Some(r.clone()));
        }
    }
    return f(b, e, m, None);
}

type Number = BigUint;

fn prime_test() {
    let mut rng = rand::thread_rng();
}

pub fn fast_prime_test(n: Number) -> bool {
    use rand::prelude::*;

    if n == one() || n == zero() {
        return false;
    }

    let mut rng = rand::thread_rng();
    for _ in 0..16 {
        let a: Number = rng.gen_range(one()..n.clone());
        //FIXME: remove while
        if gcd(a.clone(), n.clone()) != one() {
            return false;
        }
        if powmod(a.clone(), &n - one::<BigUint>(), n.clone()) != one() {
            return false;
        }
    }
    true
}

pub fn hard_prime_test(n: Number) -> bool {
    use rand::prelude::*;

    if n == one() || n == zero() {
        return false;
    }

    let mut rng = rand::thread_rng();
    for _ in 0..128 {
        let a: Number = rng.gen_range(one()..n.clone());
        //FIXME: remove while
        if gcd(a.clone(), n.clone()) != one() {
            return false;
        }
        if powmod(a.clone(), &n - one::<BigUint>(), n.clone()) != one() {
            return false;
        }
    }
    true
}
