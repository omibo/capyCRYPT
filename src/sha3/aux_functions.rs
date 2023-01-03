/** NIST 800-185 compliant functions.  */
pub mod nist_800_185{

    use byteorder::{BigEndian, WriteBytesExt};

    /**
    NIST SP 800-185 2.3.3
    The bytepad(X, w) function prepends an encoding of the integer w to an input string X, then pads
    the result with zeros until it is a byte string whose length in bytes is a multiple of w.

        X: the byte string to pad
        w: the rate of the sponge
        return: z = encode(X) + X + ("0" * LCM of length of z and w)
    */
    pub fn byte_pad(input: &mut Vec<u8>, w: u32) -> Vec<u8> {
        let mut z = left_encode(w as u64);
        z.append(input);
        let padlen = w - ((z.len() as u32 % w) as u32);
        let mut padded = vec![0; padlen as usize];
        z.append(&mut padded);
        z
    }

    /**
    NIST SP 800-185 2.3.2: String Encoding.
        
    The encode_string function is used to encode bit strings in a way that may be parsed
    unambiguously from the beginning of the string.
            
        return: left_encode(len(S)) + S.
    */
    pub fn encode_string(s: &mut Vec<u8>) -> Vec<u8>{
        let mut encoded = left_encode((s.len()*8) as u64);
        encoded.append(s);
        encoded
    }

    /**
    leftEncode function is used to encode bit strings in a way that may be parsed
    unambiguously from the beginning of the string by prepending the encoding of
    the length of the string to the beginning of the string.
        
        return: left_encode(len(S)) + S.
    */
    pub fn left_encode(value: u64) -> Vec<u8> {

        if value == 0 {return vec![1, 0];}
        let mut vec = Vec::new();
        vec.write_u64::<BigEndian>(value).unwrap();
        let index = 0;
        //remove leading zeros
        while index < vec.len() && vec[index] == 0 {
            vec.remove(index);
        }
        let mut res = Vec::new();
        res.push(vec.len() as u8);
        res.extend_from_slice(&vec);
        res

    }

    /**
    rightEncode function is used to encode bit strings in a way that may be parsed
    unambiguously from the beginning of the string by prepending the encoding of
    the length of the string to the beginning of the string.
        
        return: left_encode(len(S)) + S.
    */
    pub fn right_encode(value: u64) -> Vec<u8> {

        if value == 0 {return vec![0, 1];}
        let mut b = Vec::new();
        b.write_u64::<BigEndian>(value).unwrap();
        let mut i: u8 = 1;
        while i < 8 && b[i as usize] == 0 {
            i += 1;
        }
        // Prepend number of encoded bytes
        b[0] = 9 - i;
        return b[0..(9 - i as usize)].to_vec();   
    }

}

pub mod byte_utils{
use num::BigInt;
/** Aux methods for byte operations.  */
    use rand::prelude::*;

    /** Gets 512 randomy bytes for model functions. */
    pub fn get_random_bytes() -> Vec<u8> {
        let mut rand_bytes = vec![0u8; 512];
        thread_rng().fill(&mut rand_bytes[..]);
        rand_bytes
    }

    /** XORs byte streams in place using iterators Will probably bottleneck unless impl with SIMD. */
    pub fn xor_bytes(a: &mut Vec<u8>, b: &Vec<u8>) {
        assert_eq!(a.len(), b.len());
        a.iter_mut()
        .zip(b.iter())
        .for_each(|(x1, x2)| *x1 ^= *x2);
        
    }


    pub fn bytes_to_big_int(input: &[u8]) -> BigInt {
        let mut bigint = BigInt::from(0 as u32);
        let base: BigInt = BigInt::from(2u32).pow(8u32);
    
        for &byte in input.iter().rev() {
            let b:BigInt = BigInt::from(byte);
            bigint = &bigint * &base + &b;
        }
    
        bigint
    }

    pub fn get_date_and_time_as_string() -> String {
        let local = chrono::Local::now();
        local.format("%Y-%m-%d %H:%M:%S").to_string()
    }
}