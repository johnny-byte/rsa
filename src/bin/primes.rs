use num_bigint::{BigInt, BigUint, RandBigInt, ToBigInt, ToBigUint};
use num_traits::{one, zero};

fn new_gcd(a: &BigUint, b: &BigUint) -> BigUint {
    use std::mem::replace;
    let mut r = a % b;
    #[allow(unused_assignments)]
    let mut a = a.clone();
    let mut b = b.clone();
    while r != zero() {
        a = replace(&mut b, r);
        r = &a % &b;
    }
    b.clone()
}

pub fn gcd(a: &BigUint, b: &BigUint) -> BigUint {
    return new_gcd(a, b);
}

#[test]
fn gcd_test() {
    let mut rng = rand::thread_rng();
    for _ in 0..100 {
        let a = rng.gen_biguint(64);

        let b = rng.gen_biguint(64);
        assert_eq!(new_gcd(&a, &b), gcd(&a, &b));
    }
}

#[test]
fn gcd_time_test() {
    use std::time::Instant;
    let mut rng = rand::thread_rng();
    let n = 100_000;
    let t1 = Instant::now();
    for _ in 0..n {
        let a = rng.gen_biguint(64);
        let b = rng.gen_biguint(64);
        let _ = gcd(&a, &b);
    }
    let t2 = Instant::now();
    eprintln!("gcd duration={:?}", t2 - t1);

    let t1 = Instant::now();
    for _ in 0..n {
        let a = rng.gen_biguint(64);
        let b = rng.gen_biguint(64);
        let _ = new_gcd(&a, &b);
    }
    let t2 = Instant::now();
    eprintln!("new_gcd duration={:?}", t2 - t1);
    panic!();
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
    if gcd(&n, &m) != one() || n >= m {
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
    while gcd(&n, &t) != one() {
        t = rng.gen_biguint_range(&lower, &n);
    }
    return t;
}

pub fn get_primes(n: usize, threads_amount: usize, bit_size: u64) -> Vec<BigUint> {
    use mpsc::TryRecvError::{Disconnected, Empty};
    use std::{collections::HashSet, sync::mpsc, thread};

    let (primes_sender, primes_receiver) = mpsc::channel();
    let mut handles = Vec::with_capacity(threads_amount);

    for _ in 0..threads_amount {
        let primes_sender = primes_sender.clone();
        let (say_stop, should_stop) = mpsc::channel();

        let handle = thread::spawn(move || {
            let mut rng = rand::thread_rng();

            let mut num = rng.gen_biguint(bit_size);
            let step = 2.to_biguint().unwrap();

            if &num % 2.to_biguint().unwrap() == zero() {
                num += one::<BigUint>();
            }

            loop {
                match should_stop.try_recv() {
                    Ok(()) => return,
                    Err(e) => match e {
                        Disconnected => panic!("Parent thread closed before child thread"),
                        Empty => {}
                    },
                };

                if fast_prime_test(num.clone()) {
                    if primes_sender.send(num.clone()).is_err() {
                        return;
                    }
                }

                num += &step;
            }
        });

        handles.push((handle, say_stop));
    }

    let mut primes = HashSet::new();
    for received in primes_receiver {
        if hard_prime_test(received.clone()) {
            primes.insert(received);
            if primes.len() == n {
                break;
            }
        }
    }
    for (_, say_stop) in handles.iter() {
        if say_stop.send(()).is_err() {
            continue;
        }
    }
    for (handler, _) in handles {
        handler.join().unwrap();
    }

    return primes.into_iter().collect();
}

pub fn powmod(b: BigUint, e: BigUint, m: BigUint) -> BigUint {
    fn f(b: &BigUint, e: &BigUint, m: &BigUint, r: Option<&BigUint>) -> BigUint {
        let __used_to_make_lifitime;
        let r = match r {
            Some(x) => x,
            None => {
                __used_to_make_lifitime = one();
                &__used_to_make_lifitime
            }
        };
        if *e == zero() {
            return r.clone();
        }
        if e & one::<BigUint>() == one() {
            return f(&((b * b) % m), &(e >> 1), m, Some(&((r * b) % m)));
        } else {
            return f(&((b * b) % m), &(e >> 1), m, Some(r));
        }
    }
    return f(&b, &e, &m, None);
}

type Number = BigUint;

pub fn fast_prime_test(n: Number) -> bool {
    use rand::prelude::*;

    if n == one() || n == zero() {
        return false;
    }

    let mut rng = rand::thread_rng();
    for _ in 0..16 {
        let a: Number = rng.gen_range(one()..n.clone());
        if gcd(&a, &n) != one() {
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
        if gcd(&a, &n) != one() {
            return false;
        }
        if powmod(a.clone(), &n - one::<BigUint>(), n.clone()) != one() {
            return false;
        }
    }
    true
}
