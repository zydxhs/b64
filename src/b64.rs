// 把字节流解码为字符串
pub fn encode(buf: &[u8]) -> String {
    let num = buf.len();
    let mut result = String::new();
    result.reserve((num + 3) / 3 * 4);

    for i in 0..(num / 3) {
        let item = [buf[i * 3], buf[i * 3 + 1], buf[i * 3 + 2]];
        let a4 = a3_to_a4(&item);

        result.push(byte_to_char(a4[0]));
        result.push(byte_to_char(a4[1]));
        result.push(byte_to_char(a4[2]));
        result.push(byte_to_char(a4[3]));
    }

    // 处理余下的1/2字节
    let mut item = [0, 0, 0];
    let mm = num % 3;
    if mm == 1 {
        item = [buf[num - 1], 0, 0];
    } else if mm == 2 {
        item = [buf[num - 2], buf[num - 1], 0];
    }
    let a4 = a3_to_a4(&item);
    result.push(byte_to_char(a4[0]));
    result.push(byte_to_char(a4[1]));
    if a4[2] == 0u8 {
        result.push('=');
    } else {
        result.push(byte_to_char(a4[2]));
    }

    if a4[3] == 0u8 {
        result.push('=');
    } else {
        result.push(byte_to_char(a4[3]));
    }

    result
}

// 把字符串解码为字节流
pub fn decode(buf: &String) -> Vec<u8> {
    let num = buf.as_bytes().len();
    let mut result: Vec<u8> = Vec::new();
    result.reserve(num / 4 * 3);
    let mut a4 = [0u8; 4];

    // 处理前 num - 4 个字符
    for (i, item) in buf[..num - 4].char_indices() {
        let j = i % 4;
        a4[j] = char_to_byte(item);
        if j == 3 {
            let a3 = a4_to_a3(&a4);
            result.push(a3[0]);
            result.push(a3[1]);
            result.push(a3[2]);
        }
    }
    // 处理最后4个字符
    for (i, item) in buf[num - 4..].char_indices() {
        a4[i] = char_to_byte(item);
    }
    let a3 = a4_to_a3(&a4);
    if a3[0] != 0 {
        result.push(a3[0]);
        if a3[1] != 0 {
            result.push(a3[1]);
            if a3[2] != 0 {
                result.push(a3[2]);
            }
        }
    }
    result
}

// b64 字符对照表
static B64_STR: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

// 把字节值转为B64字符
fn byte_to_char(key: u8) -> char {
    // 代码1
    // let mut it = B64_STR.char_indices();
    // loop {
    //     match it.next() {
    //         Some((i, ch)) => {
    //             if i as u8 == key {
    //                 return ch;
    //             }
    //         }
    //         None => {}
    //     };
    // }

    // 代码2
    // let mut it = B64_STR.char_indices();
    // loop {
    //     if let Some((i, ch)) = it.next() {
    //         if i as u8 == key {
    //             return ch;
    //         }
    //     }
    // }

    // 代码3
    // let mut it = B64_STR.char_indices();
    // loop {
    //     if let Some((i, ch)) = it.next() {
    //         if i as u8 == key {
    //             break ch;
    //         }
    //     }
    // }

    // 代码4
    let mut it = B64_STR.char_indices();
    loop {
        match it.next() {
            Some((i, ch)) if i as u8 == key => break ch,
            _ => {}
        }
    }

    // 代码5
    // let mut c = ' ';
    // for (i, ch) in B64_STR.chars().enumerate() {
    //     if i as u8 == key {
    //         c = ch;
    //         break;
    //     }
    // }
    // c
}

// 把字符转为字节值
fn char_to_byte(ch: char) -> u8 {
    if ch == '=' {
        return 0 as u8;
    }
    B64_STR.find(ch).unwrap() as u8
}

// 3字节数组转换为4字节数组
fn a3_to_a4(a3: &[u8; 3]) -> [u8; 4] {
    let mut a4: [u8; 4] = [0u8; 4];
    a4[0] = a3[0] >> 2;
    a4[1] = (a3[0] << 6) >> 2 | a3[1] >> 4;
    a4[2] = (a3[1] << 4) >> 2 | a3[2] >> 6;
    a4[3] = (a3[2] << 2) >> 2;
    a4
}

// 4字节数组转换为3字节数组
fn a4_to_a3(a4: &[u8; 4]) -> [u8; 3] {
    let mut a3 = [0u8; 3];
    a3[0] = a4[0] << 2 | a4[1] >> 4;
    a3[1] = a4[1] << 4 | a4[2] >> 2;
    a3[2] = a4[2] << 6 | a4[3];
    a3
}
