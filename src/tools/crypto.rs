use std::{
    cmp::min,
    mem,
    time::{SystemTime, UNIX_EPOCH},
};

use hmacsha1::hmac_sha1;
use qrcodegen::QrCode;

#[inline]
pub fn crypto_current() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        / 30
}

const RFC4648_INV_ALPHABET: [i8; 43] = [
    -1, -1, 26, 27, 28, 29, 30, 31, -1, -1, -1, -1, -1, 0, -1, -1, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8,
    9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
];

pub fn base32_decode(data: &[u8]) -> Option<Vec<u8>> {
    let mut len = data.len();
    for i in 1..min(6, data.len()) + 1 {
        if data[data.len() - i] != b'=' {
            break;
        }
        len -= 1;
    }
    let final_length = len * 5 / 8;
    let mut result = Vec::with_capacity((final_length + 4) / 5 * 5);
    for chunk in data.chunks(8) {
        let buf = {
            let mut buf = [0u8; 8];
            for (i, &c) in chunk.iter().enumerate() {
                match RFC4648_INV_ALPHABET.get(c.to_ascii_uppercase().wrapping_sub(b'0') as usize) {
                    Some(&-1) | None => return None,
                    Some(&value) => buf[i] = value as u8,
                };
            }
            buf
        };
        result.push((buf[0] << 3) | (buf[1] >> 2));
        result.push((buf[1] << 6) | (buf[2] << 1) | (buf[3] >> 4));
        result.push((buf[3] << 4) | (buf[4] >> 1));
        result.push((buf[4] << 7) | (buf[5] << 2) | (buf[6] >> 3));
        result.push((buf[6] << 5) | buf[7]);
    }
    result.truncate(final_length);
    Some(result)
}

pub fn crypto(current: u64, s: &[u8]) -> u32 {
    let s = base32_decode(s).unwrap();
    let code = hmac_sha1(s.as_slice(), &current.to_be_bytes());
    let offset = code[code.len() - 1] & 0x0F; // mod 16
    let mut truncated_hash: [u8; 4] = [0u8; 4];
    truncated_hash.copy_from_slice(&code[offset as usize..(offset + 4) as usize]);
    let code = unsafe { mem::transmute::<[u8; 4], i32>(truncated_hash) }; // mem copy
    let code = i32::from_be(code);
    let code = code & 0x7FFFFFFF; // abs and mod
    code as u32 % 1_000_000
}

unsafe trait Draw {
    fn shell(&self);
    fn binary(&self) -> Option<Vec<u8>>;
}

unsafe impl Draw for QrCode {
    fn shell(&self) {
        let border = 0i32;
        for y in border..self.size() {
            for x in border..self.size() {
                let c = if self.get_module(x, y) {
                    "██"
                } else {
                    "  "
                };
                print!("{}", c);
            }
            println!();
        }
    }
    fn binary(&self) -> Option<Vec<u8>> {
        if self.size() < 0 {
            None
        } else {
            let size = self.size() * self.size();
            let mut value: Vec<u8> = Vec::with_capacity(size as usize);
            for y in 0..self.size() {
                for x in 0..self.size() {
                    value.push(if self.get_module(x, y) { 1 } else { 0 });
                }
            }
            Some(value)
        }
    }
}

#[cfg(test)]
mod verify {
    use crate::tools::crypto::crypto;
    use google_authenticator::GoogleAuthenticator;
    use std::time::{SystemTime, UNIX_EPOCH};
    #[test]
    // #[ignore = "create qrcode to check"]
    fn crypto_is_equal_google() {
        let s = "wrb2h33o26id43rm2rsylb34cye543p5";
        // let qr = QrCode::encode_text(
        //     format!("otpauth://totp/lqxc?secret={}&issuer=Dashboard", s).as_str(),
        //     QrCodeEcc::High,
        // )
        // .unwrap();
        let current = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            / 30;
        let this = crypto(current, s.as_bytes());
        dbg!(this);
        let ga = GoogleAuthenticator::new();
        let code = ga.get_code(s, current).unwrap();
        assert_eq!(code.parse::<u32>().unwrap(), this);
        // qr.shell();
    }
}
