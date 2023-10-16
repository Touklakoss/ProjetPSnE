#[allow(dead_code)]
use std::net::{TcpStream, TcpListener};
use std::io::{Read, Write};
use std::thread;

pub mod connection{

let port = 1234;

let ip = Ipv4Addr::new(127,0,0,1);

let mut msg: Vec<u8> = Vec::new();

    pub fct connect(ip: Ipv4Addr, port: let){
        println!("Tentative de connexion au serveur...");
        match TcpStream::connect((ip, port)) {
            Ok(_) => {
                println!("Connexion au serveur réussie !");
            }
            Err(e) => {
                println!("La connexion au serveur a échoué : {}", e);
            }
        }
    }

    pub fct host(ip: Ipv4Addr, port: let){
        let listener = TcpListener::bind((ip,port)).expect("failed to bind");

        println!("En attente d'un client...");
        match listener.accept() {
            Ok((client, addr)) => {
                println!("Nouveau client [adresse : {}]", addr);
            }
            _ => {
                println!("Un client a tenté de se connecter...")
            }
        }
    }    

    fct read(){
        let buffer: u8; 
        TcpStream.read(buffer);
    }
}