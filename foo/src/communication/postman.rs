/**
 * @file postman.rs
 * @author Pierre-Alexandre WITTLING
 * @brief Fichier source du module postman
 * @date 5/11/2023
 * @package communication
 */

use std::net::{TcpStream, TcpListener};
use std::io::{Read, Write};
use std::thread;
use std::net::Ipv4Addr;
use std::io;

//Classe connection
pub mod connection {

/**
 * @brief lance la connection tcp/ip
 * @param ip : ip a laquelle se connecter
 * @param port : le port correspondant
 * @return Result : le resultat de la connection
 */
    pub fn connect(ip: Ipv4Addr, port: u16) -> io::Result<TcpStream> {
        println!("Tentative de connexion au serveur...");
        let stream = TcpStream::connect((ip, port))?;
        println!("Connexion au serveur réussie !");
        Ok(stream)
    }

/**
 * @brief lance l'host pour connection tcp/ip
 * @param ip : ip a laquelle se connecter
 * @param port : le port correspondant
 * @return Result : le resultat de la connection
 */
    pub fn host(ip: Ipv4Addr, port: u16) -> io::Result<TcpListener> {
        let listener = TcpListener::bind((ip, port))?;
        println!("En attente d'un client...");
        Ok(listener)
    }

/**
 * @brief ecrit dans le port
 * @param stream : la connection correspondante
 * @param message : le message a écrire
 * @return renvoie Ok si ca c'est bien passé
 */
    pub fn write(mut stream: TcpStream, message: &str) -> io::Result<()> {
        stream.write_all(message.as_bytes())?;
        Ok(())
    }

/**
 * @brief lit dans le port
 * @param stream : la connection correspondante
 * @return renvoie Ok si ca c'est bien passé
 */
    pub fn read(mut stream: TcpStream) -> io::Result<()> {
        let handle = thread::spawn(move || {
            let mut buffer = [0; 1024];
            loop {
                let bytes_read = stream.read(&mut buffer)?;
                if bytes_read == 0 {
                    break;
                }
                let received_data = &buffer[0..bytes_read];
                
            }
        });

        handle.join().expect("Erreur lors de la gestion du thread");
        Ok(())
    }
}
