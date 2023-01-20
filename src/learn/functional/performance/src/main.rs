fn main() {
    let buffer: &mut [i32] = &mut [3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31];
    let coefficients: [i64; 12] = [2, 4, 6, 7, 8, 10, 13, 25, 28, 39, 11, 47];
    let qlp_shift: i16 = 2;

    // println!("{:?} {}", &buffer, buffer.len());

    for i in 12..buffer.len() {
        // when i = 15: 12 ~ 15
        let prediction = coefficients
            .iter()
            .zip(&buffer[i - 12..i]) // when i = 12 : (i-12)..i -> 0~11, when i = 13 -> 1~12, when i = 14 -> 2~13
            //    .for_each(|(&c, &s)| println!("{}, {}", c, s));
            // [(coef, buf)] -> [(2, 3), (4, 5) ... (47, 25)] : len = 12, 13, 14 (3 = 15-12)
            .map(|(&c, &s)| c * s as i64)
            .sum::<i64>()
            >> qlp_shift;

        let delta = buffer[i];
        buffer[i] = prediction as i32 + delta;
    }

    println!("{:?} {}", &buffer, buffer.len());
    // [3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 964, 12075, 145285] 15
}
