use num_bigint::BigUint;
use num_traits::One;

fn n_cmul(a: &mut (BigUint, BigUint), b: &(BigUint, BigUint)) {
    let tmp = &a.1 * &b.1;
    a.1 += &a.0;
    a.1 *= &b.0 + &b.1;
    a.0 *= &b.0;
    a.1 -= &a.0;
    a.0 += tmp;
}

fn n_csq(x: &mut (BigUint, BigUint)) {
    let tmp = &x.1 * &x.1;
    x.1 += &x.0;
    x.1 = x.1.pow(2);
    x.0 = x.0.pow(2);
    x.1 -= &x.0;
    x.0 += tmp;
}

// 300k,420k
pub fn iter(n: u32) -> BigUint {
    let mut a: BigUint = BigUint::ZERO;
    let mut b: BigUint = BigUint::one();
    let mut c = true;
    for _ in 0..n {
        if c {
            b += &a;
        } else {
            a += &b;
        }
        c = !c;
    }
    if c {
        a
    } else {
        b
    }
}

// 10M,16M
pub fn cm(mut n: u32) -> BigUint {
    let mut base = (BigUint::ZERO, BigUint::one());
    let mut ret = (BigUint::one(), BigUint::ZERO);
    while n > 0 {
        if n % 2 > 0 {
            n_cmul(&mut ret, &base);
        }
        n /= 2;
        if n > 0 {
            n_csq(&mut base);
        }
    }
    ret.1
}
