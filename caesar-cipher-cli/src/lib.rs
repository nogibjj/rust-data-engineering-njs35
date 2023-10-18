/*
This code defines two functions: encrypt and decrypt.
The encrypt function takes a plaintext string and a shift value, and returns the ciphertext string. The decrypt function takes a ciphertext string and a shift value,
and returns the plaintext string.

*/

pub fn encrypt(text: &str, shift: u8) -> String {
    let mut result = String::new();
    for c in text.chars() {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
            let offset = (c as u8 - base + shift) % 26;
            result.push((base + offset) as char);
        } else {
            result.push(c);
        }
    }
    result
}

pub fn decrypt(text: &str, shift: u8) -> String {
    encrypt(text, 26 - shift)
}

pub fn fixed_encrypt(input: &str) -> String {
    let mut result = String::new();

    for ch in input.chars() {
        match ch {
            'a' => result.push('4'),
            'b' => result.push('8'),
            'e' => result.push('3'),
            'g' => result.push('9'),
            'i' => result.push('1'),
            'l' => result.push('1'),
            'o' => result.push('0'),
            's' => result.push('5'),
            't' => result.push('7'),
            'z' => result.push('2'),
            'A' => result.push('4'),
            'B' => result.push('8'),
            'E' => result.push('3'),
            'G' => result.push('9'),
            'I' => result.push('1'),
            'L' => result.push('1'),
            'O' => result.push('0'),
            'S' => result.push('5'),
            'T' => result.push('7'),
            'Z' => result.push('2'),
            _ => result.push(ch),
        }
    }

    result
}

pub fn fixed_decrypt(input: &str) -> String {
    let mut result = String::new();

    for ch in input.chars() {
        match ch {
            '4' => result.push('a'),
            '8' => result.push('b'),
            '3' => result.push('e'),
            '9' => result.push('g'),
            '1' => result.push('i'),
            '0' => result.push('o'),
            '5' => result.push('s'),
            '7' => result.push('t'),
            '2' => result.push('z'),
            _ => result.push(ch),
        }
    }

    result
}