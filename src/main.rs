
use rand::*;

fn main() {


    let pass = generate_password(7, true);
}





fn generate_password(length: usize, numbers: bool) -> String {
    let mut rng = thread_rng();

    let alphabet_chars: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let num_chars: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890";

    if numbers {
        let random_string: String = (0..length)
        .map(|_| {
            let index = rng.gen_range(0..num_chars.len());
            alphabet_chars[index] as char
        })
        .collect();
        return random_string;
    } else {
        let random_string: String = (0..length)
        .map(|_| {
            let index = rng.gen_range(0..num_chars.len());
            alphabet_chars[index] as char
        })
        .collect();
        return random_string;

    }
}
