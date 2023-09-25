pub mod bubulle{

pub struct Bubulle {
    size: f64,
    number_point: u32,
    membership: String,
    point_generation: u32
}


impl Bubulle {
    //Constructeur
    pub fn new(size: f64, number_point: u32, membership: String, point_generation: u32) -> Self{
        Bubulle {
            size,
            number_point,
            membership,
            point_generation,
        }
    }
    pub fn display_information(&self) {
        println!("Taille : {:.2}", self.size);
        println!("Nombre de points : {}", self.number_point);
        println!("Appartenance : {}", self.membership);
        println!("Génération de point : {}", self.point_generation);
    }
}


}
