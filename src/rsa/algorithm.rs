use num_bigint::{BigUint, RandomBits, BigInt, Sign};
use num_traits::{Zero, One, Signed};
use num_integer::{Integer, gcd};
use rand::Rng;

pub fn new_prime(bits: usize) -> BigUint {
    loop {
        let mut candidate: BigUint = gen_bits(bits);

        candidate.set_bit(0, true);
        candidate.set_bit((bits - 1) as u64, true);
        
        if is_prime(&candidate) == true {
            println!("\nprime {}", candidate);
            return candidate;
        }
    }
}

fn gen_bits(bits: usize) -> BigUint {
    rand::thread_rng().sample(RandomBits::new(bits as u64))
}

fn is_prime(candidate: &BigUint) -> bool {
    let zero = BigUint::zero();
    let one = BigUint::one();
    let two = &one + &one;

    *candidate != zero
    && (candidate.is_odd() || *candidate == two)
    && miller_rabin_test(candidate)
    && fermat_test(candidate)
}

fn fermat_test(candidate: &BigUint) -> bool {
    let one = BigUint::one();

    for _ in 0..2 {
        let random = gen_bits(candidate.bits() as usize - 1);
        let result = random.modpow(&(candidate - &one), candidate);
        if result != one {
            return false;
        }
    }

    true
}

fn miller_rabin_test(candidate: &BigUint) -> bool {
    true
}

pub fn find_coprime(phi: &BigUint) -> Option<BigUint> {
    let one = BigUint::one();
    let two = &one + &one;

    let mut e = std::cmp::min(BigUint::from(65537u32), phi / &two + &one);
    while gcd(e.clone(), phi.clone()) != one {
        e += &one;
        if &e >= &phi {
            return None;
        }
    }
    Some(e)
}

pub fn inverse(phi: &BigUint, e: &BigUint) -> BigUint {
    let e = BigInt::from_biguint(Sign::Plus, e.clone());
    let phi = BigInt::from_biguint(Sign::Plus, phi.clone());
    let mut x = BigInt::zero();
    let mut y = BigInt::zero();
    exgcd(&e, &phi, &mut x, &mut y);
    if x.sign() == Sign::Minus {
        x = (&x % &phi + &phi) % &phi;
    }
    x.to_biguint().unwrap()
}

fn exgcd(a: &BigInt, b: &BigInt, x: &mut BigInt, y: &mut BigInt) {
    let zero = BigInt::zero();
    let one = BigInt::one();

    if b == &zero {
        *x = one;
        *y = zero;
    } else {
        exgcd(b, &(a % b), y, x);
        *y -= (a / b) * x.clone();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_prime_1() {
        println!("\n{:?}", new_prime(1024).to_radix_be(2));
    }

    #[test]
    fn test_inv_1() {
        let bits = 32usize;
        let one = BigUint::one();

        let p = new_prime(bits);
        let q = new_prime(bits);
        let phi = (&p - &one) * (&q - &one);

        let e = match find_coprime(&phi) {
            Some(res) => res,
            None => return,
        };
        let inv = inverse(&phi, &e);

        println!("\n{}", inv);
        assert_eq!(inv * e % phi, BigUint::one());
    }
}