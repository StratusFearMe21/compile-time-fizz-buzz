use std::io::{stdout, Write};

const FIZZBUZZ_LEN: u8 = 100;

const FIZZBUZZ_ARR_LEN: usize = fizzbuzz_const_len();

const FIZZBUZZ: [u8; FIZZBUZZ_ARR_LEN] = fizzbuzz_const();

fn main() {
    stdout().write_all(&FIZZBUZZ).unwrap();
}

const fn fizzbuzz_const() -> [u8; FIZZBUZZ_ARR_LEN] {
    let mut result: [u8; FIZZBUZZ_ARR_LEN] = [10; FIZZBUZZ_ARR_LEN];
    let mut iternum: u8 = 1;
    let mut writehead = 0;
    loop {
        if iternum % 15 == 0 {
            result[writehead] = 70;
            result[writehead + 1] = 105;
            result[writehead + 2] = 122;
            result[writehead + 3] = 122;
            result[writehead + 4] = 66;
            result[writehead + 5] = 117;
            result[writehead + 6] = 122;
            result[writehead + 7] = 122;
            writehead += 9;
        } else if iternum % 5 == 0 {
            result[writehead] = 66;
            result[writehead + 1] = 117;
            result[writehead + 2] = 122;
            result[writehead + 3] = 122;
            writehead += 5;
        } else if iternum % 3 == 0 {
            result[writehead] = 70;
            result[writehead + 1] = 105;
            result[writehead + 2] = 122;
            result[writehead + 3] = 122;
            writehead += 5;
        } else {
            if iternum < 9 {
                result[writehead] = iternum + 48;
                writehead += 2;
            } else if iternum < 99 {
                result[writehead] = (iternum / 10) + 48;
                result[writehead + 1] = (iternum % 10) + 48;
                writehead += 3;
            } else {
                result[writehead] = (iternum / 100) + 48;
                result[writehead + 1] = ((iternum / 10) % 10) + 48;
                result[writehead + 2] = (iternum % 10) + 48;
                writehead += 4;
            }
        }
        if iternum == FIZZBUZZ_LEN {
            return result;
        } else {
            iternum += 1;
        }
    }
}

const fn fizzbuzz_const_len() -> usize {
    let mut result: usize = 0;
    let mut iternum: u8 = 1;
    loop {
        if iternum % 15 == 0 {
            result += 9;
        } else if iternum % 5 == 0 {
            result += 5;
        } else if iternum % 3 == 0 {
            result += 5;
        } else {
            if iternum < 9 {
                result += 2;
            } else if iternum < 99 {
                result += 3;
            } else {
                result += 4;
            }
        }
        if iternum == FIZZBUZZ_LEN {
            return result;
        } else {
            iternum += 1;
        }
    }
}
