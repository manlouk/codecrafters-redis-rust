use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Read;
use std::io::Write;
//Uncomment this block to pass the first stage
use std::net::TcpListener;
use std::net::TcpStream;
use std::io;
use std::io::Result;


fn handle_connection(stream: &mut TcpStream){

    let response = b"+PONG\r\n";

    println!("Inside handle");

    let stream_copy = stream.try_clone().unwrap();
    
    let mut writer = BufWriter::new(stream_copy);

    let mut reader = BufReader::new(stream);
    
    

    println!("{:?}",reader);

    for line in reader.lines(){
        match line{
            Ok(_line) => {
               
                let new_line: i16 = _line.replace("*", "").parse().unwrap();

              
                for i in 0..new_line+1{
                    writer.write(b"+PONG\r\n");
                
                }
                writer.flush();
                
            }
            Err(_line) => panic!("No valid input")
        }
        break;
        

    }
   
    
   
    
   
    // let mut buf = String::new();
    // // reader.read_to_string(&mut buf);

    // println!("{:?}", buf);

    // let mut writer = io::BufWriter::new(stream);

    // stream.write(response).unwrap();
    // stream.flush().unwrap();
    
   
  
}

fn main()-> Result<()>{
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    //ncomment this block to pass the first stage
    
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
   
    for stream in listener.incoming() {
        println!("{:?}", stream);
        match stream{
            Ok(mut _stream) =>{
               handle_connection(&mut _stream);
            },
            Err(_stream) => println!("FAILED")
          };
    }
    Ok(())
}
