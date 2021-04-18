use std::{
    collections::HashSet,
    fs::{read, read_to_string},
};

use num_bigint::{BigUint, ToBigUint};
use num_traits::{one, FromPrimitive, Num, ToPrimitive};

mod primes;

fn generate_possible_keys(p: &BigUint, q: &BigUint) -> Vec<(BigUint, BigUint)> {
    let one = &one::<BigUint>();
    let n = &(p * q);
    let f = &((p - one) * (q - one));

    let mut ds = vec![];
    let mut start = 1.to_biguint().unwrap();
    let end = f;

    while start < *end {
        if primes::gcd(&start, &f) == 1.to_biguint().unwrap() {
            ds.push(start.clone());
        }
        start += 1.to_biguint().unwrap();
    }

    let mut keys = vec![];
    for d in ds {
        let e = primes::mul_inv_mod(d.clone(), f.clone());
        // println!("e={},n={}",e,n);
        keys.push((e, n.clone()));
    }
    keys
}

fn get_primes(bit_size: u32) -> Vec<usize> {
    let mut nums = vec![0; 256];
    let mut primes = vec![];
    for i in 0..2usize.pow(bit_size) {
        nums[i] = i;
    }
    for i in 2..2usize.pow(bit_size) {
        if nums[i] != 0 {
            primes.push(nums[i]);
            for j in (i..256).step_by(nums[i]) {
                nums[j] = 0;
            }
        }
    }
    primes
}

fn get_combinations(bit_size: u32) -> Vec<(usize, usize)> {
    let primes = get_primes(bit_size);
    let mut combinations = vec![];
    for i in 0..primes.len() {
        for j in i + 1..primes.len() {
            combinations.push((primes[i], primes[j]));
        }
    }
    combinations
}

fn load_words() -> HashSet<String> {
    let mut words = HashSet::new();
    let string = read_to_string("words").unwrap();
    for word in string.split("\n") {
        words.insert(word.trim().to_string());
    }
    words
}

fn get_encoded_symbols() -> Vec<BigUint> {
    let file = read("encoded_".to_string() + "poem.txt").unwrap();
    // let file = String::from_utf8(file).unwrap();
    // file.split(" ")
    //     .filter(|&x| x != "")
    //     .map(|x| BigUint::from_str_radix(x.trim(), 10).unwrap())
    //     .collect()

    file
    .split(|&x| x == b' ')
    .filter(|x| String::from_utf8_lossy(x).trim() != "")
    .map(|x| BigUint::from_str_radix(String::from_utf8_lossy(x).trim(), 10).unwrap())
    .collect()
}

fn try_decode(private_key_e_n: (BigUint, BigUint), encoded_msg: &[BigUint]) -> Option<Vec<u8>> {
    let (e, n) = private_key_e_n;
    let mut msg = vec![];
    for byte in encoded_msg {
        let rem = primes::powmod(byte.clone(), e.clone(), n.clone());
        msg.push(rem);
    }
    for i in msg.iter() {
        if i.to_u8().is_none() {
            return None;
        }
    }
    Some(msg.iter().map(|x| x.to_u8().unwrap()).collect())
}

fn main() -> std::io::Result<()> {
    let words = load_words();
    let n = std::env::args().nth(1).unwrap().parse::<u32>().unwrap();
    let combinations = get_combinations(n);
    let encoded_symbols = get_encoded_symbols();

    let mut possible_messages = vec![];

    for (p, q) in combinations {
        let private_keys =
            generate_possible_keys(&p.to_biguint().unwrap(), &q.to_biguint().unwrap());
        for private_key in private_keys {
            // if p == 53 && q == 179 || q == 53 && p == 179 {
            //     println!("{:?}", private_key);
            // }
                println!("1");
            let decoded_words = try_decode(private_key, &encoded_symbols);

            let decoded_words = match decoded_words {
                Some(x) => {
                    let res = String::from_utf8(x);
                    if res.is_err() {
                        continue;
                    }
                    res.unwrap()
                }
                None => {
                    // println!("Pair skiped {:?}", (p, q));
                    continue;
                }
            };

            println!("2");
            let mut counter = 0;
            let mut all_words = 0;
            for word in decoded_words.split(&[' ', '\n'][..]) {
                all_words += 1;
                let w = word;
                if words.contains(w) {
                    counter += 1;
                }
            }
            println!(
                "Matching words for pair {:?} is {}%",
                (p, q),
                counter * 100 / all_words
            );

            println!("3");
            // println!("{}", decoded_words);
            if counter * 100 / all_words >= 80 {
                possible_messages.push((counter, decoded_words))
            }
        }
    }
    possible_messages.sort();
    let mut counter = 0;
    for (_, msg) in possible_messages {
        if counter == 5 {
            break;
        }
        std::fs::write("decoded_variants.txt", msg).unwrap();
        counter += 1;
    }
    Ok(())
}
