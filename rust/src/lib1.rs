use malachite::base::num::basic::traits::{One, Zero};
use malachite::Natural;

fn m_cmul(a: &mut (Natural, Natural), b: &(Natural, Natural)) {
    let tmp = &a.1 * &b.1;
    a.1 += &a.0;
    a.1 *= &b.0 + &b.1;
    a.0 *= &b.0;
    a.1 -= &a.0;
    a.0 += tmp;
}

fn m_csq(x: &mut (Natural, Natural)) {
    let tmp = &x.1 * &x.1;
    x.1 += &x.0;
    x.1 *= x.1.clone();
    x.0 *= x.0.clone();
    x.1 -= &x.0;
    x.0 += tmp;
}

// 400k
pub fn iter(n: u32) -> Natural {
    let mut a: Natural = Zero::ZERO;
    let mut b: Natural = One::ONE;
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

// 70M
pub fn cm(mut n: u32) -> Natural {
    let mut base = (Zero::ZERO, One::ONE);
    let mut ret = (One::ONE, Zero::ZERO);
    while n > 0 {
        if n % 2 > 0 {
            m_cmul(&mut ret, &base);
        }
        n /= 2;
        if n > 0 {
            m_csq(&mut base);
        }
    }
    ret.1
}
