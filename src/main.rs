use std::io::Write;
//Uncomment this block to pass the first stage
use std::net::TcpListener;
use std::net::TcpStream;
use std::io;
use std::io::Result;


fn handle_connection(stream: &mut TcpStream){

    let response = b"+PONG\r\n";

    // let mut writer = io::BufWriter::new(stream);

    stream.write(response).unwrap();
    stream.flush().unwrap();
    
   
  
}

fn main()-> Result<()>{
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    //ncomment this block to pass the first stage
    
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
   
    for stream in listener.incoming() {
        match stream{
            Ok(mut _stream) =>{
               handle_connection(&mut _stream);
            },
            Err(_stream) => println!("FAILED")
          };
    }
    Ok(())
}
