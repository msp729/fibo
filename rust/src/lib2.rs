use rug::Complete;
use rug::Integer;

fn r_cmul(a: &mut (Integer, Integer), b: &(Integer, Integer)) {
    let tmp = (&a.1 * &b.1).complete();
    let tmp2 = (&b.0 + &b.1).complete();
    a.1 += &a.0;
    a.0 *= &b.0;
    a.1 *= tmp2;
    a.1 -= &a.0;
    a.0 += tmp;
}

fn r_csq(x: &mut (Integer, Integer)) {
    let tmp = (&x.1 * &x.1).complete();
    x.1 += &x.0;
    x.1 *= x.1.clone();
    x.0 *= x.0.clone();
    x.1 -= &x.0;
    x.0 += tmp;
}

// 800k
pub fn iter(n: u32) -> Integer {
    let mut a = Integer::ZERO;
    let mut b = Integer::ONE.clone();
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

// 90M
pub fn cm(mut n: u32) -> Integer {
    let mut base = (Integer::ZERO, Integer::ONE.clone());
    let mut ret = (Integer::ONE.clone(), Integer::ZERO);
    while n > 0 {
        if n % 2 > 0 {
            r_cmul(&mut ret, &base);
        }
        n /= 2;
        if n > 0 {
            r_csq(&mut base);
        }
    }
    ret.1
}
