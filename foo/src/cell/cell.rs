/**
 * @file cell.rs
 * @author Pierre-Alexandre WITTLING
 * @brief Fichier source du module cell
 * @date 16/10/2023
 * @package cell
 */



pub mod bubulle{
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::{Duration, Instant};
    mod player;


//@brief Structure Cell, contient Thread, size (i32), point_available (Mutec(i32)), ownerships (player), point_generation (i32), id (i32) entre 1 et 10
    pub struct Cell{
        handle: Arc<Mutex<Option<thread::JoinHandle<()>>>>,
        size: i32,
        pub point_available: Arc<Mutex<i32>>,
        pub ownership: player,
        point_generation: i32,
        id: i32,
    }

    impl cell{

        //Constructeur
        pub fn new(size: i32, ownerships: player, point_generation: i32, point_available: Arc<Mutex<i32>>) -> self{
            
            let handle = Arc::new(Mutex::new(None));

            // Clone la référence Arc pour passer dans le thread
            let handle_clone = Arc::clone(&handle);
            let point_available_clone = Arc::clone(&point_available);

            // Création d'un thread pour exécuter increment_point()
            let _thread_handle = thread::spawn(move || {
                Bubulle::increment_point(handle_clone, &point_available_clone, point_generation);
            });
            
            Cell {
                size,
                point_available,
                ownership,
                point_generation,
                handle,
                id,
            }
        }

        /**
         * @breif increment les points de Cell tout les secondes
         * @param handle thread pour increment en fond les points de la structure cell correspondante
         * @param point_available les points diponiblent qui vont etre augmenter de la cell correspondante
         * @param point_generation de combien les points vont etre incrementé 
         */

        fn increment_pointhandle: Arc<Mutex<Option<thread::JoinHandle<()>>>>, point_available: &Arc<Mutex<i32>>, point_generation: i32) {
            let interval = Duration::from_secs(1);
            let mut last_increment_time = Instant::now();
        
            loop {
                let now = Instant::now();
                if now.duration_since(last_increment_time) >= interval {
                    let mut handle_locked = handle.lock().expect("Failed to lock handle");
                    if let Some(thread_handle) = handle_locked.take() {
                        thread_handle.join().expect("Failed to join thread");
                    }
        
                    let mut point_available_lock = point_available.lock().expect("Failed to lock point_available");
                    *point_available_lock += point_generation;
                   
                    let new_handle = thread::spawn(move || {
                        
                    });
        
                    *handle_locked = Some(new_handle);
                    last_increment_time = now;
                }
        
                thread::sleep(Duration::from_millis(100));
            }
        }

        /**
         * @brief attaque une autre cell, a partir de la cell selectionné
         * @param &self, la cell attaquante
         * @param cell, la cell attaqué
         */
        fn attacking(&self, cell: Cell){
            cell.point_available -= (self.point_available/2);
            self.point_available = self.point_available/2;

            if(cell.point_available < 0){
                change_ownership(&self.ownership, cell: Cell);
            }
        }

        /**
         * @brief change l'appartenance de la cell
         * @param player, le joueur qui obtient la cell
         * @param cell, la cell qui change d'appartenance
         */
        fn change_ownership(player: Player, cell: Cell){
            cell.ownership = player
        }
    }

}