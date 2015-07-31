use std::cmp;

extern crate time;

fn main() {

    let regular_start = time::get_time().nsec;
    let regular = 12387*93247;
    let regular_end = time::get_time().nsec;

    let fast_start = time::get_time().nsec;
    let fast = multiply(&12387, &93247);
    let fast_end = time::get_time().nsec;

    println!("Regular Result: {}", regular);
    println!("Fast Result: {}", fast);

    let regular_duration = regular_end - regular_start;
    let fast_duration = fast_end - fast_start;

    match regular_duration > fast_duration {
        true => println!("Regular was faster by: {} nanoseconds.", regular_duration - fast_duration),
        false => println!("Fast was faster by: {} nanoseconds.", fast_duration - regular_duration)
    }

}

fn multiply(x: &isize, y: &isize) -> isize {
    let mut max = cmp::max(*x, *y);
    let mut bits = 0;
    let mut mask = 0;

    while max > 0 {
        bits += 1;
        max >>= 1;
    }

    let n = bits;
    //let l = ((bits as f64)/2.0).ceil() as isize;
    let r = ((bits as f64)/2.0).floor() as isize;

    if n == 1 || n == 0 { return (*x)*(*y) }

    bits = r;
    while bits > 0 {
        mask = (mask << 1) | 1;
        bits -= 1;
    }

    let x_l = *x >> r;
    let x_r = *x & mask;

    let y_l = *y >> r;
    let y_r = *y & mask;

    let xx = x_l + x_r;
    let yy = y_l + y_r;

    let p_1 = multiply(&x_l, &y_l);
    let p_2 = multiply(&x_r, &y_r);
    let p_3 = multiply(&xx, &yy);

    return (1 << 2*r)*p_1 + (1 << r)*(p_3 - p_2 - p_1) + p_2;
}
