extern crate clap;
use clap::{App, Arg};

mod b64;

use std::fs::File;
use std::io::prelude::*;

#[allow(unused_variables)]
fn main() {
    let matches = App::new("b64")
        .help_message("打印帮助信息")
        .version_message("打印版本信息")
        .arg(
            Arg::with_name("encode")
                .help("编码")
                .short("e")
                .long("encode")
                .required(true)
                .conflicts_with("decode"),
        )
        .arg(
            Arg::with_name("decode")
                .help("解码")
                .short("d")
                .long("decode")
                .required(true)
                .conflicts_with("encode"),
        )
        .arg(
            Arg::with_name("input")
                .help("输入文件。没有该选项时，从命令行读取待编码字符串")
                .short("i")
                .long("input")
                .takes_value(true) // MUST be set to true in order to be an "option" argument
                // .value_name("file"),
        )
        .arg(
            Arg::with_name("output")
            .help("输出文件。没有该选项时，打印到标准输出")
                .short("o")
                .long("output")
                .takes_value(true) // MUST be set to true in order to be an "option" argument
                // .value_name("file"),
        )
        .arg(
            Arg::with_name("str")
                .help("待编码字符串。不能与 -i 参数同时使用")
                .conflicts_with("input"),
        )
        .get_matches();

    // 处理输入数据
    let mut data = Vec::new();
    if let Some(input) = matches.value_of("input") {
        // 读文件
        let mut f = File::open(input).expect("文件不存在！");
        f.read_to_end(&mut data).unwrap();
        println!("读文件长度：{}", data.len());
    } else if let Some(str) = matches.value_of("str") {
        data = str.as_bytes().to_vec();
    } else {
        println!("输入参数不正确");
        return;
    }

    if matches.is_present("encode") {
        // 编码
        let s = b64::encode(&data);
        println!("编码后 数据长度：{}", s.len());
        // 输出结果
        if let Some(output) = matches.value_of("output") {
            // 写文件
            let mut f = File::create(output).expect("创建文件失败！");
            let s = s.as_bytes();
            println!("编码后 文件长度：{}", s.len());
            f.write_all(s).expect("写文件失败！");
        } else {
            println!("编码结果：{}", s);
        }
    } else {
        // 解码
        let data = String::from_utf8(data).expect("转字符串失败！");
        println!("解码前 数据长度：{}", data.len());
        let result = b64::decode(&data);
        println!("解码后数据长度={}", result.len());

        // 输出结果
        if let Some(output) = matches.value_of("output") {
            // 写文件
            let mut f = File::create(output).expect("创建文件失败！");
            f.write_all(&result).expect("写文件失败！");
        } else {
            if let Some(str) = matches.value_of("str") {
                println!("解码结果：{}", String::from_utf8(result).unwrap());
            } else {
                let mut column = 0;
                for byte in &result {
                    print!("{:02X}", byte);
                    column += 1;
                    if column == 32 {
                        column = 0;
                        print!("\n");
                    }
                }
            }
        }
    }
}
