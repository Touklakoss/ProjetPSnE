#[allow(dead_code)]

pub mod bubulle{

    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::{Duration, Instant};
pub struct Bubulle {
    handle: Arc<Mutex<Option<thread::JoinHandle<()>>>>,
    size: f64,
    pub number_point: Arc<Mutex<i32>>,
    pub membership: String,
    point_generation: i32,
    color: String
}


impl Bubulle {
        
        // Constructeur
        pub fn new(size: f64, membership: &str, point_generation: i32, color: &str, number_point: Arc<Mutex<i32>>) -> Self {

            
            let handle = Arc::new(Mutex::new(None));

            // Clonez la référence Arc pour passer dans le thread
            let handle_clone = Arc::clone(&handle);
            let number_point_clone = Arc::clone(&number_point);

            // Créez un thread pour exécuter increment_point
            let _thread_handle = thread::spawn(move || {
                Bubulle::increment_point(handle_clone, &number_point_clone, point_generation);
            });

            Bubulle {
                size,
                number_point,
                membership: membership.to_string(),
                point_generation,
                color: color.to_string(),
                handle,
            }
        }

        pub fn reduce_point(&self, points_to_reduce: i32) {
            let mut number_point_locked = self.number_point.lock().expect("Failed to lock number_point");
            *number_point_locked -= points_to_reduce;
        }

    fn increment_point(handle: Arc<Mutex<Option<thread::JoinHandle<()>>>>, number_point: &Arc<Mutex<i32>>, point_generation: i32) {
        let interval = Duration::from_secs(1);
        let mut last_increment_time = Instant::now();
    
        loop {
            let now = Instant::now();
            if now.duration_since(last_increment_time) >= interval {
                let mut handle_locked = handle.lock().expect("Failed to lock handle");
                if let Some(thread_handle) = handle_locked.take() {
                    thread_handle.join().expect("Failed to join thread");
                }
    
                let mut number_point_locked = number_point.lock().expect("Failed to lock number_point");
                *number_point_locked += point_generation;
               
                let new_handle = thread::spawn(move || {
                    
                });
    
                *handle_locked = Some(new_handle);
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
        let number_point_locked = self.number_point.lock().expect("Failed to lock number_point");
        println!("Taille : {:.2}", self.size);
        println!("Nombre de points : {}", *number_point_locked);
        println!("Appartenance : {}", self.membership);
        println!("Génération de point : {}", self.point_generation);
        println!("Couleur : {}", self.color);
    }

}
    
    
   
}
