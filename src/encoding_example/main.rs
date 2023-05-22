use std::io::{BufReader, Read};
use std::fs::File;
use encoding_rs_io::DecodeReaderBytesBuilder;

pub fn main() {
    // 打开文件
    let file = File::open("./file.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    // 使用 encoding_rs 进行解码
    // 这种设计允许在不同情况下灵活地选择是否使用显式编码。如果你想要强制指定一个编码，你可以提供一个具体的编码对象，否则，你可以传递None以便使用默认逻辑。
    // let _decoder = Some(encoding_rs::UTF_8); // 选择所需的编码，这里以 GBK 为例

    let mut decode_reader = DecodeReaderBytesBuilder::new()
        .encoding(None)
        .build(reader);

    // 读取文件内容并解码
    let mut buffer = String::new();
    let result = decode_reader.read_to_string(&mut buffer);

    match result {
        Ok(_) => {
            // 解码成功
            println!("Decoded content: {}", buffer);
        }
        Err(err) => {
            // 解码错误
            eprintln!("Decoding error: {:?}", err);
        }
    }
}
