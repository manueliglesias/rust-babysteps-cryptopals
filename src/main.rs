mod util {
    pub fn hex_to_base64(input: &str) -> String {
        const TABLE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let mut result = String::new();
        const MOD: usize = 6;
        let mut bla: u32 = 0;
        for (i, x) in input.chars().enumerate() {
            bla = bla << 4;
            bla += x.to_digit(16).unwrap_or_default();
            if (i + 1) % MOD == 0 {
                let c1: u8 = ((bla & 0b00000000111111000000000000000000) >> 18) as u8;
                let c2: u8 = ((bla & 0b00000000000000111111000000000000) >> 12) as u8;
                let c3: u8 = ((bla & 0b000000000000000000000111111000000) >> 6) as u8;
                let c4: u8 = (bla & 0b000000000000000000000000000111111) as u8;
                bla = 0;

                result
                    .push_str(&TABLE[(c1 as usize % TABLE.len())..(c1 as usize % TABLE.len()) + 1]);
                result
                    .push_str(&TABLE[(c2 as usize % TABLE.len())..(c2 as usize % TABLE.len()) + 1]);
                result
                    .push_str(&TABLE[(c3 as usize % TABLE.len())..(c3 as usize % TABLE.len()) + 1]);
                result
                    .push_str(&TABLE[(c4 as usize % TABLE.len())..(c4 as usize % TABLE.len()) + 1]);
            }
        }
        // padding?
        result
    }

    pub fn decode_hex(s: &str) -> String {
        assert_eq!(s.len() % 2, 0);
        let x = (0..s.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap_or_default())
            .collect::<Vec<_>>();
        String::from_utf8(x).unwrap_or_default()
    }

    pub fn encode_hex(bytes: &[u8]) -> String {
        use std::fmt::Write;
        let mut s = String::with_capacity(bytes.len() * 2);
        for &b in bytes {
            write!(&mut s, "{:02x}", b).unwrap_or_default();
        }
        s
    }

    pub fn xor(op1: &str, op2: &str) -> String {
        use std::char;
        assert_eq!(op1.len(), op2.len(), "lengths are different");
        let mut result = String::from("");
        let mut op2_iter = op2.chars();
        for (_, x) in op1.chars().enumerate() {
            let c1 = x.to_digit(16).unwrap_or_default();
            let c2 = op2_iter
                .next()
                .unwrap_or_default()
                .to_digit(16)
                .unwrap_or_default();
            result.push(char::from_digit(c1 ^ c2, 16).unwrap_or_default())
        }
        result
    }

    pub fn get_score(the_str: &str) -> u32 {
        use std::collections::HashMap;
        let scores: HashMap<char, u32> = [
            (' ', 1300),
            ('e', 1270),
            ('t', 905),
            ('a', 816),
            ('o', 750),
            ('i', 696),
            ('n', 674),
            ('s', 632),
            ('h', 609),
            ('r', 598),
            ('d', 425),
            ('l', 402),
            ('u', 275),
        ]
        .iter()
        .cloned()
        .collect();
        let score = the_str.chars().fold(0 as u32, |acc, c| {
            let s = scores.get(&c);
            match s {
                Some(s) => acc + s,
                None => acc,
            }
        });
        score
    }

    pub fn hamming_distance(op1: &str, op2: &str) -> u32 {
        assert_eq!(op1.len(), op2.len());

        let op1 = op1.as_bytes();
        let op2 = op2.as_bytes();

        let result = op1
            .iter()
            .enumerate()
            .fold(0, |acc, (i, e)| acc + (e ^ op2[i]).count_ones());

        // let mut result = 0u32;

        // for i in 0..op1.len() {
        //     let x = (op1[i] ^ op2[i]).count_ones();
        //     result = result + x;

        //     // let mut b = op1[i] ^ op2[i];
        //     // for _ in 0..8 {
        //     //     result = result + (b & 1) as u32;

        //     //     b = b >> 1
        //     // }
        // }
        result
    }
}

pub mod challenges {
    use super::util;
    pub fn challenge1() {
        let input = &"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        // SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t
        // I'm killing your brain like a poisonous mushroom
        let output = util::hex_to_base64(input);
        println!("{}", input);
        println!("{}", util::decode_hex(input));
        println!("{}", output);
    }
    pub fn challenge2() {
        let op1 = &"1c0111001f010100061a024b53535009181c";
        let op2 = &"686974207468652062756c6c277320657965";
        // 746865206b696420646f6e277420706c6179
        // the kid don't play
        let output = util::xor(op1, op2);
        println!("{}", output);
        println!("{}", util::decode_hex(&output));
    }
    pub fn challenge3() {
        let op1 = &"1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
        // Cooking MC's like a pound of bacon
        let len = op1.len();
        let (mut max_score, mut winner) = (0u32, None as Option<String>);
        println!("{} {}", op1, util::decode_hex(op1));
        for _ in 0..op1.len() {
            for x in 32..=122 {
                use super::util::get_score;
                use std::char;
                let c = char::from_u32(x).unwrap_or_default().to_string();
                let op2 = str::repeat(c.as_str(), len / 2);
                let xored = util::xor(&op1, &util::encode_hex(&op2.as_bytes()));
                let xored_str = util::decode_hex(&xored);
                let score = get_score(&xored_str);
                if score > max_score {
                    max_score = score;
                    winner = Some(xored_str);
                }
            }
        }
        println!("{}", winner.unwrap_or_default());
    }
    pub fn challenge4() {
        use std::env::current_dir;
        use std::fs::File;
        use std::io::{BufRead, BufReader};
        use std::path::PathBuf;

        let mut path: PathBuf = current_dir().unwrap_or_default();
        path.push("4.txt");
        // Now that the party is jumping

        let file = File::open(&path).unwrap();

        let file = BufReader::new(file);

        let mut candidates = Vec::new();

        use super::util::get_score;

        // TODO: parallelize

        for line in file.lines() {
            let line = line.unwrap_or_default();
            let len = line.len();

            let (mut max_score, mut winner) = (0u32, None as Option<String>);

            for _ in 0..len {
                for x in 32..=122 {
                    use std::char;
                    let c = char::from_u32(x).unwrap_or_default().to_string();
                    let op2 = str::repeat(c.as_str(), len / 2);
                    let xored = util::xor(&line, &util::encode_hex(&op2.as_bytes()));
                    let xored_str = util::decode_hex(&xored);
                    let score = get_score(&xored_str);
                    if score > max_score {
                        max_score = score;
                        winner = Some(xored_str);
                    }
                }
            }

            let winner = winner.unwrap_or_default();
            let len = winner.len();
            if len > 0 {
                candidates.push(winner);
            }
        }

        let winner = candidates
            .into_iter()
            .max_by(|x, y| get_score(&x).cmp(&get_score(y)))
            .unwrap_or_default();

        println!("{}", winner);
    }
    pub fn challenge5() {
        let input = &"Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        const KEY: &str = &"ICE";
        // 0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272
        // a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f

        let mut result = String::from("");

        for (i, c) in input.chars().enumerate() {
            let key_index = i % KEY.len();
            let key_letter = &KEY[key_index..key_index + 1];

            let l = util::xor(
                &util::encode_hex(c.to_string().as_bytes()),
                &util::encode_hex(&key_letter.as_bytes()),
            );

            // println!("{} {} {}", c, key_letter, l);

            result.push_str(&l);
        }

        println!("{}", result);
    }
    pub fn challenge6() {
        let op1 = &"this is a test";
        let op2 = &"wokka wokka!!!";

        let result = util::hamming_distance(&op1, &op2);

        println!("{}", result);
    }
}

fn main() {
    challenges::challenge1();
    // challenges::challenge2();
    // challenges::challenge3();
    // challenges::challenge4();
    // challenges::challenge5();
    // challenges::challenge6();
}
