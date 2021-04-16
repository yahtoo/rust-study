use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time;
use std::str;

fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    // 定义buf数组,用于接收数据
    let mut buf = [0; 512];
    //循环接收数据
    for _ in 0..1000 {
        // 从流中读取数据
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 {
            return Ok(());
        }

        // 打印接收的数据
        println!("{}", str::from_utf8(&buf).expect("Could not write buffer as string"));

        // 把内容发送给stream
        stream.write(&buf[..bytes_read])?;
        //延时等待
        thread::sleep(time::Duration::from_secs(1 as u64));
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    // 设置监听服务器ip/端口
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();

    //监听连接
    for stream in listener.incoming() {
        let stream = stream.expect("failed!");
        //创建连接处理线程
        let handle = thread::spawn(move || {
            handle_client(stream)
                .unwrap_or_else(|error| eprintln!("{:?}", error));
        });
        //启动线程处理连接
        thread_vec.push(handle);
    }

    for handle in thread_vec {
        handle.join().unwrap();
    }

    Ok(())
}

