pub struct Config{
    cfg: Vec<Arg>,  // 参数列表
}

struct Arg{
    short: char,        // 短参数
    long: String,       // 长参数
    val: String,    // 参数值
    desc: String,   // 参数描述
}

impl Config{
    pub new (a: Vec<String>) -> Self{
        if a.len() < 3 {
            println!("参数不正确！");
        }
    }

    pub usage(){
        for i in cfg.iter(){
            println!("    -{}", i.arg,)
        }
    }

}