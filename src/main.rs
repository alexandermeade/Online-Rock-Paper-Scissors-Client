use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::error::Error;
use std::io;
use std::io::Write;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Connect to the server at localhost:7878
    let mut stream = TcpStream::connect("127.0.0.1:7878").await?;

    println!("Connected to the server!");

    // Read input from the user
    print!("input your name> ");
    io::stdout().flush()?; // Make sure the prompt is displayed
    let mut input = String::new();
    io::stdin().read_line(&mut input)?; // Read a line from the user

    // Send the input data to the server
    stream.write_all(input.as_bytes()).await?;


    let mut buffer = vec![0; 1024];

    loop {
        let n = stream.read(&mut buffer).await?;
        let message = String::from_utf8_lossy(&buffer[..n]);
        let len:usize = if message.len() <= 0 {0} else {message.len() - 1}; 

        if message.chars().nth(len) == Some('_') { 
            let mut input = String::new();
            while input == "" {
                let client_msg = &message[0..message.len()];
                println!("server> {}", client_msg);
                print!("\ninput> ");
                io::stdout().flush()?; // Make sure the prompt is displayed
                io::stdin().read_line(&mut input)?; // Read a line from the user
                stream.write_all(input.as_bytes()).await?;
            }
            continue;
        }

        if n == 0 {
            println!("Server closed the connection.");
            break;
        }

        println!("server> {}", message);
    }
    Ok(())
}


