# b64
Rust 写的 base64 编解码小程序。

用法：
```
b64                                                  

USAGE:                                               
    b64.exe [FLAGS] [OPTIONS] --decode --encode [str]

FLAGS:                                               
    -d, --decode     解码                              
    -e, --encode     编码                              
    -h, --help       打印帮助信息                          
    -V, --version    打印版本信息                          
 
OPTIONS:                                             
    -i, --input <input>      输入文件。没有该选项时，从命令行读取待编码字符串
    -o, --output <output>    输出文件。没有该选项时，打印到标准输出     

ARGS:                                                
    <str>    待编码字符串。不能与 -i 参数同时使用
```
