use std::fs::{read, write};

use num_bigint::{self, BigUint, ToBigUint};
use num_traits::{self, one, Num, ToPrimitive};
mod primes;

fn encode(public_key_d_n: (BigUint, BigUint), msg: &[u8]) -> Vec<BigUint> {
    let (d, n) = public_key_d_n;
    let mut encoded_msg = vec![];
    for byte in msg {
        let rem = primes::powmod(byte.to_biguint().unwrap().clone(), d.clone(), n.clone());
        encoded_msg.push(rem);
    }
    encoded_msg
}

fn decode(private_key_e_n: (BigUint, BigUint), encoded_msg: &[BigUint]) -> Vec<BigUint> {
    let (e, n) = private_key_e_n;
    let mut msg = vec![];
    for byte in encoded_msg {
        let rem = primes::powmod(byte.clone(), e.clone(), n.clone());
        msg.push(rem);
    }
    msg
}

fn get_keys(threads_amount: usize, bit_size: u64) -> ((BigUint, BigUint), (BigUint, BigUint)) {
    let p_q = primes::get_primes(2, threads_amount, bit_size);
    let one = &one::<BigUint>();
    let (p, q) = (&p_q[0], &p_q[1]);
    let n = &(p * q);
    let f = &((p - one) * (q - one));
    let d = primes::get_lower_and_coprime_with(f.clone());
    let e = primes::mul_inv_mod(d.clone(), f.clone());
    ((d, n.clone()), (e, n.clone()))
}

fn parse_input<'a>() -> clap::ArgMatches<'a> {
    use clap::{App, Arg};
    App::new("Encrypt programm")
        .arg(
            Arg::with_name("COMMAND")
                .help("Sets the command")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("FILENAME")
                .help("Sets the input file to use")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("BIT_SIZE")
                .long("bit-size")
                .default_value("256")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("THREADS_AMOUNT")
                .long("threads-amount")
                .default_value("8")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("OUTPUT_FILE")
                .long("output")
                .short("o")
                .takes_value(true),
        )
        .get_matches()
}
//TODO: make chose output file
fn main() -> std::io::Result<()> {
    let input = parse_input();
    let cmd = input.value_of("COMMAND").unwrap();
    let file_name = input.value_of("FILENAME").unwrap();
    // println!("cmd={}", cmd);
    // println!("file_name={}", file_name);
    if ["e", "encode"].contains(&cmd) {
        let file = read(file_name)?;

        let t1 = std::time::Instant::now();
        let bit_size = input.value_of("BIT_SIZE").unwrap().parse().unwrap();

        let threads_amount = input.value_of("THREADS_AMOUNT").unwrap().parse().unwrap();
        // println!("bit_size={}", bit_size);
        // println!("threads_amount={}", threads_amount);
        let (public_key, private_key) = get_keys(threads_amount, bit_size);

        let t2 = std::time::Instant::now();
        println!("Keys generated in {:?}", t2 - t1);

        let t1 = std::time::Instant::now();

        let encoded = encode(public_key, &file);

        let mut encoded_msg = vec![];
        for val in encoded {
            let s = val.to_string();
            encoded_msg.extend(s.into_bytes());
            encoded_msg.extend(b" ");
        }

        let mut prk = vec![];
        prk.extend(private_key.0.to_string().into_bytes());
        prk.extend(b" ");
        prk.extend(private_key.1.to_string().into_bytes());

        write("encoded_".to_string() + file_name, encoded_msg)?;
        write("private_key_".to_string() + file_name, prk)?;

        let t2 = std::time::Instant::now();
        println!("Message encoded in {:?}", t2 - t1);
    } else if ["d", "decode"].contains(&cmd) {
        let t1 = std::time::Instant::now();

        let file = read("encoded_".to_string() + file_name)?;
        let key_file = read("private_key_".to_string() + file_name)?;

        let en: Vec<BigUint> = key_file
            .split(|&x| x == b' ')
            .map(|x| BigUint::from_str_radix(&String::from_utf8_lossy(x), 10).unwrap())
            .collect();
        let private_key = (en[0].clone(), en[1].clone());
        let symbols: Vec<BigUint> = file
            .split(|&x| x == b' ')
            .filter(|x| String::from_utf8_lossy(x).trim() != "")
            .map(|x| BigUint::from_str_radix(String::from_utf8_lossy(x).trim(), 10).unwrap())
            .collect();

        let decoded_msg: Vec<u8> = decode(private_key, &symbols)
            .iter()
            .map(|x| x.to_u8().unwrap())
            .collect();
        write("decoded_".to_string() + file_name, decoded_msg)?;

        let t2 = std::time::Instant::now();
        println!("Message decoded in {:?}", t2 - t1);
    } else {
        panic!("Command unrecognized; command is '{}'", cmd);
    }
    Ok(())
}
