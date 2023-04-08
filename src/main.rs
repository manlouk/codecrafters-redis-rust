use std::io::Read;
use std::io::Write;
//Uncomment this block to pass the first stage
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::Result;
use std::thread;

fn handle_connection(stream: &mut TcpStream){

    let response = b"+PONG\r\n";


    let mut buf = [0;512];
    // reader.read_to_string(&mut buf);




 
    loop{
        let len = stream.read(&mut buf).unwrap();
        
        match len {
            _len if _len==0 =>{
                break;
            }
            _=>{
                stream.write(response).unwrap();
               
            }
        }
    }   
   
    
}   


fn main()-> Result<()>{
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    //ncomment this block to pass the first stage
    
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
   
    for stream in listener.incoming() {
         match stream{
            Ok(mut _stream) =>{
               let handle = thread::spawn(move || handle_connection(&mut _stream));
               handle.join().unwrap();
            },
            Err(_stream) => println!("FAILED")
          };
        
    }
    Ok(())
}
