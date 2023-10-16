/**
 * @file player.rs
 * @author Pierre-Alexandre WITTLING
 * @brief Fichier source du module player
 * @date 16/10/2023
 * @package player
 */

 #[allow(dead_code)]
 pub mod player{
 
 // Class abstact Color
 trait Color {
     fn get_color(&self) -> &str;
 }
 
 // Implémentation pour player Host
 struct HostColor;
 impl Color for HostColor {
     fn get_color(&self) -> &str {
         "Red"
     }
 }
 
 // Implémentation pour player participant
 struct ParticipantColor;
 impl Color for ParticipantColor {
     fn get_color(&self) -> &str {
         "Blue"
     }
 }
 
 // Class Player
 pub struct Player<'a> {
     name: &'a str,
     role: &'a str,
     color: Box<dyn Color>,
 }
 
 impl<'a> Player<'a> {
     pub fn new(name: &'a str, role: &'a str) -> Self {
         let color: Box<dyn Color> = match role {
             "Host" => Box::new(HostColor),
             "Participant" => Box::new(ParticipantColor),
             _ => panic!("Invalide player role"),
         };
         
         Player { name, role, color }
     }
 
     pub fn display_information(&self) {
         println!("Nom : {}", self.name);
         println!("Rôle : {}", self.role);
         println!("Couleur : {}", self.color.get_color());
     }
 }
 
 
 }