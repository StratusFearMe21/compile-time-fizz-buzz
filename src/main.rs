/*
    I've written a lot of comments in this code base because it was made for educational purposes
    I am NOT an "over commenter"

    If you are looking to hire me because I write such perfect, perstine, and performant code
    you can contact me in 2 ways

    Matrix: @stratusfearme21:matrix.palomagit.ml
    Discord: CaptainCankle31#6175
*/

// For how many numbers to calculate Fizz Buzz for
const FIZZBUZZ_LEN: u8 = 100;

// This is how long the array containing the Fizz Buzz string needs to be
const FIZZBUZZ_ARR_LEN: usize = fizzbuzz_const_len();

// The actual calculated Fizz Buzz string, expressed as an array of a bit integers.
const FIZZBUZZ: [u8; FIZZBUZZ_ARR_LEN] = fizzbuzz_const();

fn main() -> Result<(), std::io::Error> {
    /*
        Because our array the we calculated was encoded in UTF-8 to begin with, we can send it directly to stdout without any conversions needed.

        This was faster than using stdout().write_all(&FIZZBUZZ).unwrap().
        However, writing to stdout without a lock is technically unsafe
        but considering that we are only using 1 thread and this is the only line in the main function
        I'm fairly sure nothing will go wrong
    */
    std::fs::write("/dev/stdout", FIZZBUZZ)
}

// const fns are capable of being calculated at compile time
const fn fizzbuzz_const() -> [u8; FIZZBUZZ_ARR_LEN] {
    /*
        The result is a [u8] instead of a &str because you cannot mutate a &str
        and you can't use the String type either because it uses the heap

        Hence why I'm using a [u8] containing UTF-8 charecters
        This is a good format not only because I don't really have any other choice
        but also because I can write this directly to stdout without having to do a conversion of any kind.

        In order to initialize this variable we allocate it with UTF-8 newlines.
        10 = \n
    */
    let mut result: [u8; FIZZBUZZ_ARR_LEN] = [10; FIZZBUZZ_ARR_LEN];

    /*
        Because of the limitation that I can't use for loops in a const fn
        I'm instead using a mutable variable that is incremented on every iteration of the while loop below.
    */
    let mut iternum: u8 = 1;

    // I use this variable to keep track of where on the array I need to write to next.
    let mut writehead = 0;

    // the equivalent of "for iternum in 1..=FIZZBUZZ_LEN {"
    while iternum != FIZZBUZZ_LEN {
        // If the number we're processing is divisable by 15 (3 & 5)
        if iternum % 15 == 0 {
            // 70 is the UTF-8 charecter for the letter "F"
            result[writehead] = 70;
            //                      'I'
            result[writehead + 1] = 105;
            //                      'Z'
            result[writehead + 2] = 122;
            //                      'Z'
            result[writehead + 3] = 122;
            //                      'B'
            result[writehead + 4] = 66;
            //                      'U'
            result[writehead + 5] = 117;
            //                      'Z'
            result[writehead + 6] = 122;
            //                      'Z'
            result[writehead + 7] = 122;
            // We increment the writehead one more time than we need to so we don't have to add a newline to our array, we already have one.
            writehead += 9;
        // If the number we're processing is divisable by 5
        } else if iternum % 5 == 0 {
            //                  'B'
            result[writehead] = 66;
            //                      'U'
            result[writehead + 1] = 117;
            //                      'Z'
            result[writehead + 2] = 122;
            //                      'Z'
            result[writehead + 3] = 122;
            writehead += 5;
        // If the number we are processing is divisable by 3
        } else if iternum % 3 == 0 {
            //                  'F'
            result[writehead] = 70;
            //                      'I'
            result[writehead + 1] = 105;
            //                      'Z'
            result[writehead + 2] = 122;
            //                      'Z'
            result[writehead + 3] = 122;
            writehead += 5;
        // If the number cannot be Fizz or Buzz and it's single digit
        } else if iternum < 9 {
            // If you add 48 to a single digit number, then you will get it's UTF-8 equivalent
            result[writehead] = iternum + 48;
            writehead += 2;
        // If the number cannot be Fizz or Buzz and it's double digit
        } else if iternum < 99 {
            // A double digit number divided by 10 will get the number's 10s place as a single digit
            result[writehead] = (iternum / 10) + 48;
            // The remainder of the quotiant above will get the number's 1s place
            result[writehead + 1] = (iternum % 10) + 48;
            writehead += 3;
        // 8 bit unsigned integers cannot be more than 3 digits, therefore this is the last possiblity
        } else {
            // A 3 digit number divided by 100 will get the number's 100s place
            result[writehead] = (iternum / 100) + 48;
            // if you divide a 3 digit number divided by 10, and then divide it by 10 again
            // the remainder will be the number's 10s place
            result[writehead + 1] = ((iternum / 10) % 10) + 48;
            // if you divide a 3 digit number divided by 10, the remainder will be the numbers 1s place
            result[writehead + 2] = (iternum % 10) + 48;
            writehead += 4;
        }
        // Increment the iternum by 1 each time the loop ends
        iternum += 1;
    }
    // After we have finished calculating Fizz Buzz we return the result
    result
}

// Before we calculate Fizz Buzz we need to know how many charecters we need to allocate in the array.
const fn fizzbuzz_const_len() -> usize {
    // The result is a usize so we can use it to slice arrays
    let mut result: usize = 0;
    let mut iternum: u8 = 1;
    while iternum != FIZZBUZZ_LEN {
        if iternum % 15 == 0 {
            // "FizzBuzz\n" is 9 charecters long
            result += 9;
        } else if iternum % 5 == 0 || iternum % 3 == 0 {
            // Both "Fizz\n" and "Buzz\n" are 5 charecters long
            result += 5;
        } else if iternum < 9 {
            // a single digit number plus a new line will take 2 charecters
            result += 2;
        } else if iternum < 99 {
            // a double digit number plus a new line will take 3 charecters
            result += 3;
        } else {
            // a 3 digit number plus a new line will take 4 charecters
            result += 4;
        }
        iternum += 1;
    }
    result
}
