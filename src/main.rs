use rand::{thread_rng, Rng};

fn main() {
    let _pass = generate_password(7, false);
    println!("pass: {}", _pass);
}

 
fn generate_password(length: usize, numbers: bool) -> String {
    const NUM: usize = 3;
    let mut rng = thread_rng();
    let alphabet_chars: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let num_chars: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890";
    let only_num_chars: &[u8] = b"1234567890";
    let char_set = if numbers { num_chars } else { alphabet_chars };
    

    let random_string: String = (0..length)
        .map(|_| {
            let index = rng.gen_range(0..char_set.len());
            char_set[index] as char
        })
        .collect();
    
    if numbers {
        let new_string: String= random_string[0..random_string.len() - NUM].to_owned();
        let random_num: String = (0..NUM).map(|_| {
            let index = rng.gen_range(0..only_num_chars.len());
            only_num_chars[index] as char
        })
        .collect();
        let password = format!("{}{}",new_string,random_num);
        return password;
    } else {
        return random_string;
    }
}

