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
    let mut reader = BufReader::new(stream_copy);
    let mut writer = BufWriter::new(stream);

    let mut buf = String::new();
    let mut eof = true;

    while eof{
        match reader.read_line(&mut buf){
            Ok(0) => {
                 println!("Zero bytes");
                 eof = false;
            },
            Ok(_)=> {
                let num = buf.chars().nth(1).unwrap().to_digit(10);
                println!("{:?}", num);
                match num {
                    Some(_num) => {
                        println!("{}",_num);
                        for i in 0.._num{
                            writer.write(response).unwrap();
                            writer.flush().unwrap();
                        } 
                    },
                    None => {
                        println!("{:?}", num);
                        ()
                    }
                }

            },
            Err(_) => {
                println!("Error");
                eof = false
            }
    }
    }
    // let buf = reader.lines().nth(0).unwrap().unwrap().chars().nth(1).unwrap().to_digit(10);

    // println!("{:?}", buf);
   
    
    // match buf{
    //     Some(result) => 
    //         {
                
    //             for i in 0..result{
    //                 writer.write_all(response).unwrap();
    //                 writer.flush().unwrap();
                   
            
    //             }

    //         },
    //     _ => ()
    // };


    // let num = buf.chars().nth(1).unwrap().to_digit(10).unwrap();
   println!("Outside");
   
    
}
   
    
   
    
   
    // let mut buf = String::new();
    // // reader.read_to_string(&mut buf);

    // println!("{:?}", buf);

    // let mut writer = io::BufWriter::new(stream);

    // stream.write(response).unwrap();
    // stream.flush().unwrap();
    
   


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
