use std::io::{self, prelude::*, BufReader, Write};
use std::net::TcpStream;
use std::str;

fn main() -> std::io::Result<()> {
    //设置服务器ip port
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    for _ in 0..10 {
        // 定义变量input
        let mut input = String::new();
        //读取输入到流
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read from stdin");
        stream
            .write(input.as_bytes())
            .expect("Failed to write to stream");
        //从流读取数据
        let mut reader = BufReader::new(&stream);
        let mut buffer: Vec<u8> = Vec::new();
        //读取并打印sever应答
        reader
            .read_until(b'\n', &mut buffer)
            .expect("Could not read into buffer");
        println!("{}", 
            str::from_utf8(&buffer).expect("Could not write buffer as string"));
        println!("");
    }
    Ok(())
}

