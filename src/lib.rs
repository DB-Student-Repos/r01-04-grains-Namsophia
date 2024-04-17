pub fn square(a: u32) -> u64 {
    if a <= 0 || a > 64{
    unimplemented!("grains of rice on square {a}");
    }
    let mut grain: u64 = 1;
    for _ in 1..a {
        grain = grain * 2;
    }
    return grain;
}

pub fn total() -> u64 {
    let mut sum: u64 = 0;

    for s in 1..65 {
        sum += square(s);
    }
    return sum;
}
