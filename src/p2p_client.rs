use std::net::TcpStream;
use std::io::{Read,Write};

fn write_stream(mut stream: TcpStream){
    loop {
        // Declares new string to write to, reads user input, stores it as bytes in array
        let mut host_input = String::new();
        print!("Client> ");
        let _ = std::io::stdout().flush();
        std::io::stdin().read_line(&mut host_input).expect("Failed to read line.");
        let message = host_input.as_bytes();
        stream.write(message).expect("failed to write message.");
    }
}


fn read_stream(mut stream: TcpStream){
    loop {
        let mut _net_buffer = [0u8; 1024];
        stream.read(&mut _net_buffer).expect("HOLY SHIT WHAT THE FUCK HAPPEND");
        let incoming_message = String::from_utf8_lossy(&_net_buffer[..]);
        println!("\n");
        print!("Server> {}", incoming_message);
    }
}

fn outbound_handler(stream: TcpStream){
    let write_clone: TcpStream = stream.try_clone().expect("Write clone broke");
    let read_clone: TcpStream = stream.try_clone().expect("Read clone broke");
    std::thread::spawn(|| read_stream(read_clone));
    write_stream(write_clone);
}
//Entry Point
fn main(){
    //connects to local host
    let connection=TcpStream::connect("127.0.0.1:443");
    match connection{
        Ok(connection)=>{
            let outbound_connection = connection.local_addr().unwrap();
            println!("[+] Connected to {} from {}", connection.peer_addr().unwrap(), outbound_connection);
            //Loop for terminal chat window
            outbound_handler(connection);
        }
        Err(err)=>{
            eprintln!("[-] Error with connection: {:?}", err);
        }
    }
}