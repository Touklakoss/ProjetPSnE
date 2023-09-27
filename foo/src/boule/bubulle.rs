#[allow(dead_code)]
pub mod bubulle{

    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::{Duration, Instant};

pub struct Bubulle {
    handle: Arc<Mutex<Option<thread::JoinHandle<()>>>>,
    size: f64,
    pub number_point: i32,
    pub membership: String,
    point_generation: i32,
    color: String
}


impl Bubulle {
        // Constructeur
        pub fn new(size: f64, membership: &str, point_generation: i32, color: &str, number_point: i32) -> Self {

            
            let bubulle = Arc::new(Mutex::new(Bubulle {
                size,
                number_point,
                membership:membership.to_string(),
                point_generation,
                color:color.to_string(),
                handle: Arc::new(Mutex::new(None)),
            }));

           
            let bubulle_clone = Arc::clone(&bubulle);

            
            let handle = thread::spawn(move || {
                Bubulle::increment_point(&bubulle_clone);
            });

            
            let bubulle_locked = bubulle.lock().expect("Failed to lock bubulle");
            *bubulle_locked.handle.lock().expect("Failed to lock handle") = Some(handle);

            Bubulle {
                size,
                number_point,
                membership: membership.to_string(),
                color: color.to_string(),
                point_generation,
                handle: Arc::clone(&bubulle_locked.handle),
            }
        }

    //To reduce point
    pub fn reduce_point(&mut self, point: i32){
        self.number_point = (self.number_point - point).abs();
    }

    fn increment_point(bubulle: &Arc<Mutex<Bubulle>>) {
        let interval = Duration::from_secs(1);
        let mut last_increment_time = Instant::now();

        loop {
            let now = Instant::now();
            if now.duration_since(last_increment_time) >= interval {
                let mut bubulle_locked = bubulle.lock().expect("Failed to lock bubulle");
                bubulle_locked.number_point += bubulle_locked.point_generation;
                println!("point: {}", bubulle_locked.number_point);
                last_increment_time = now;
            }

            thread::sleep(Duration::from_millis(100));
        }
    }
    
    fn change_membership(&mut self, membership: String){
        self.membership = membership;
    }

    fn change_color(&mut self, color: String){
        self.color = color;
    }

    pub fn display_information(&self) {
        println!("Taille : {:.2}", self.size);
        println!("Nombre de points : {}", self.number_point);
        println!("Appartenance : {}", self.membership);
        println!("Génération de point : {}", self.point_generation);
        println!("Couleur : {}", self.color);
    }

}

   
}
