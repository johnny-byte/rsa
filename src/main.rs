use num_bigint;
use num_bigint::BigUint;
use num_traits;
use num_traits::{One, Zero};
use rand::random;

use std::{mem::replace, usize};

// fn gen_primes(){
//     use std::collections::HashSet;

//     //сделать генерацию по блокам
//     let N = 100;
//     let primes:HashSet<u32> = [2u32,3u32].iter().cloned().collect();
//     let mut v = vec![0;100];

//     for i in 0..N{
//         v[i]=i+1;
//     }

//     println!("{:?}",v);
// }

fn gen_primes(n: usize) -> Vec<usize> {
    // use std::collections::HashSet;

    //сделать генерацию по блокам
    // let primes:HashSet<u32> = [2u32,3u32].iter().cloned().collect();
    let mut primes = Vec::new();
    let mut v = vec![0; n - 1];

    for i in 0..n - 1 {
        v[i] = i + 2;
    }

    for i in 0..n - 1 {
        if v[i] != 0 {
            let mut j: usize = i + v[i];
            while j < n - 1 {
                v[j] = 0;
                j += v[i];
            }
            primes.push(v[i]);
        }
    }

    // println!("{:?}",v);
    return primes;
}

fn gcd(a: usize, b: usize) -> usize {
    let r = a % b;
    if r != 0 {
        return gcd(b, r);
    }
    return b;
}

fn get_keys() -> ((usize, usize), (usize, usize)) {
    let primes = gen_primes(100_000);
    println!("1");
    let (p, q) = {
        let size = primes.len();
        let i1 = random::<usize>() % size;
        let mut i2 = random::<usize>() % size;
        while i2 == i1 {
            i2 = random::<usize>() % size;
        }
        (primes[i1], primes[i2])
    };

    let n = p * q;
    let f = (p - 1) * (q - 1);
    println!("2");
    let d = {
        let mut val = random::<usize>() % f;
        while gcd(f, val) != 1 {
            val = random::<usize>() % f;
        }
        val
    };
    println!("3");
    let e = {
        let mut val = random::<usize>() % 10_000;
        while (val * d) % f != 1 {
            println!("{}",(val * d) % f);
            val += 1;
        }
        val
    };

    println!("4");
    ((d, n), (e, n))
}

fn encode(public_key: (usize, usize), msg_in_bytes: &[u8]) -> Vec<BigUint> {
    let (d, n) = public_key;
    // let d = BigUint::from(d);
    // let n = BigUint::from(n);

    let size = msg_in_bytes.len();
    let mut pos = 0;

    let mut encoded_message = Vec::new();

    while pos != size {
        let t = random::<usize>() % n;
        let val = BigUint::from_radix_be(&msg_in_bytes[pos..], 256).unwrap();
        encoded_message.push(val.pow(d as u32) % n);
        pos += t;
        println!("{}",pos);
    }
    encoded_message
}

fn decode() {}

fn main() {
    // let var = 100_000;
    // println!("fib({})={}",var, fib(var));

    // let n = BigUint::from(2u32);
    // let p = BigUint::from(2048u32);
    // let primes = gen_primes(100);
    let msg = &[32,64,128];
    let (public_key,private_key) = get_keys();
    println!("{:?}", encode(public_key, msg));
    // println!("{:?}",gcd(6,61));

    // println!("{}",n.pow(1024).to_string().len());
}
