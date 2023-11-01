#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

/**
 * @file game_core.rs
 * @author Antoine Largeaud
 * @brief Fichier source du module game_core
 * @date 16-10-2023
 * @package game_core
 */

use std::thread;
use std::sync::mpsc;

// --- Private types ---

/**
 * @brief Liste des états possibles de la MAE
 */

#[derive(Clone, Copy)]
enum stateMachine_state
{
    // État initial
    FORGET = 0,
    // Etat affichant le menu principal
    MAIN_MENU,
    // Etat quand l'host attend la connexion
    WAITING_CONNECTION,
    // Etat quand le client essaye de se connecter
    TRY_CONNECTION,
    // Etat lorsque l'host et le client sont connectés 
    LOBBY,
    // Etat lorsque la partie est lancée
    GAMELOOP,
    // Fin de la MAE
    DEAD,
    // Nombre d'états
    NB_STATE,
}

impl PartialEq for stateMachine_state {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (stateMachine_state::FORGET, stateMachine_state::FORGET) => true,
            (stateMachine_state::MAIN_MENU, stateMachine_state::MAIN_MENU) => true,
            (stateMachine_state::WAITING_CONNECTION, stateMachine_state::WAITING_CONNECTION) => true,
            (stateMachine_state::TRY_CONNECTION, stateMachine_state::TRY_CONNECTION) => true,
            (stateMachine_state::LOBBY, stateMachine_state::LOBBY) => true,
            (stateMachine_state::GAMELOOP, stateMachine_state::GAMELOOP) => true,
            (stateMachine_state::DEAD, stateMachine_state::DEAD) => true,
            _ => false,
        }
    }
}

use stateMachine_state::*;

/**
 * @brief Liste des évenements possibles de la MAE
 */
#[derive(Clone, Copy)]
enum stateMachine_events
{
    // Le joueur veut host la partie
    SELECTED_HOST_STATUS_IS_HOST = 0,
    // Le joueur veut rejoindre la partie
    SELECTED_HOST_STATUS_IS_CLIENT,
    // Les joueurs sont connectés
    CONNECTION_IS_ESTABLISHED,
    // La connexion n'a pas pu être établie (NA)
    CONNECTION_TIMED_OUT,
    // Le joueur host à lancé la partie
    GAME_STARTED,
    // Une condition de fin de partie à été déclenchée
    GAME_HAS_ENDED,
    // Le joueur annule la tentative de connexion
    CANCEL,
    // Le joueur demande à retourné au main_menu
    ASK_TO_GO_BACK_TO_MAIN_MENU,
    // Etat de fin de la MAE
    DEATH,
    // Nombre d'énvenements
    NB_EVENTS,
}

/**
 * @brief Liste des actions possibles
 **/
 #[derive(Clone, Copy)]
enum stateMachine_actions{
    // Ne fait rien
    NOP = 0,
    // Indique à displayer qu'il faut afficher le main_menu
    DISPLAY_MAIN_MENU,
    // Indique à displayer qu'il faut afficher l'écran de connexion pour l'host
    DISPLAY_WAITING_CONNECTION,
    // Indique à displayer qu'il faut afficher l'écran de connexion pour le client
    DISPLAY_TRY_CONNECTION,
    // Indique à displayer qu'il faut afficher le lobby
    DISPLAY_LOBBY,
    // Indique à displayer qu'il faut afficher l'écran de jeu 
    DISPLAY_GAMELOOP,
    // Nombre d'actions
    NB_ACTIONS,
 }

/**
 * @brief structure d'une transition entre deux états
 */

#[derive(Clone, Copy)]
struct stateMachine_transition{
    next_state: stateMachine_state,
    action: stateMachine_actions,
}

//Défini le type de fonction correspondant
type StateMachineActionFunction = fn(&mpsc::Sender<stateMachine_events>);

/**
 * @brief tableau de pointeur de fonction pour connaître l'action à effectuer
 */

const game_core_action_handler: [Option<StateMachineActionFunction>; stateMachine_actions::NB_ACTIONS as usize] = [
    Some(game_core_nop_handler),
    Some(game_core_display_main_menu_handler),
    Some(game_core_display_waiting_connection_handler),
    Some(game_core_display_try_connection_handler),
    Some(game_core_display_lobby_handler),
    Some(game_core_display_gameloop_handler),
];

fn game_core_nop_handler(tx: &mpsc::Sender<stateMachine_events>)
{
    
}

fn game_core_display_main_menu_handler(tx: &mpsc::Sender<stateMachine_events>)
{
    println!("MAIN_MENU\n");
    tx.send(stateMachine_events::SELECTED_HOST_STATUS_IS_CLIENT);
}

fn game_core_display_waiting_connection_handler(tx: &mpsc::Sender<stateMachine_events>)
{
    println!("WAIT_CONNECTION\n");
}

fn game_core_display_try_connection_handler(tx: &mpsc::Sender<stateMachine_events>)
{
    println!("TRY_CONNECTION\n");
    tx.send(stateMachine_events::CANCEL);
}

fn game_core_display_lobby_handler(tx: &mpsc::Sender<stateMachine_events>)
{
    println!("LOBBY\n");
}

fn game_core_display_gameloop_handler(tx: &mpsc::Sender<stateMachine_events>)
{
    println!("LOOP\n");
}

//static transition_tab = [[stateMachine_transition{next_state: FORGET, action: NOP }; NB_STATE as usize]; NB_EVENTS as usize];

use stateMachine_actions::*;
use stateMachine_events::*;

/*
* @brief Structure de game_core regroupant les différents attributs nécessaires à son 
* fonctionnement 
*/
pub struct Game_core {
    // Tableau des transitions de la MAE
    transition_tab: [[stateMachine_transition; NB_EVENTS as usize]; NB_STATE as usize],
    // Etat courant de la MAE
    current_state: stateMachine_state,
}

impl Game_core {
    pub fn new() -> Game_core {
        Game_core {
            current_state : FORGET,
            
            //Initialisation des différentes transitions de la MAE
            transition_tab : [[stateMachine_transition{next_state: FORGET, action: NOP };  NB_EVENTS as usize]; NB_STATE as usize],
        }
    }

    /*
    * @brief fonction run du module game_core
    */
    pub fn game_core_run(&mut self) {
        let (mq_transmitter, mq_receiver): (mpsc::Sender<stateMachine_events>, mpsc::Receiver<stateMachine_events>) = mpsc::channel();

        self.game_core_transition_tab_affectation();

        self.current_state = MAIN_MENU;
        game_core_display_main_menu_handler(&mq_transmitter);
        
        let mut event_received: stateMachine_events;
        let mut next_transition: stateMachine_transition;

        while(self.current_state != stateMachine_state::DEAD) {
            event_received = mq_receiver.recv().unwrap();

            next_transition = self.transition_tab[self.current_state as usize][event_received as usize];

            if let Some(action_fn) = game_core_action_handler[next_transition.action as usize] {
                action_fn(&mq_transmitter);
            }

            self.current_state = next_transition.next_state;
            println!("oui {}",self.current_state as usize);
        }
    }

    pub fn game_core_transition_tab_affectation(&mut self) {
        // L'utilisateur décide de devenir hôte
        self.transition_tab[MAIN_MENU as usize][SELECTED_HOST_STATUS_IS_HOST as usize] = stateMachine_transition {
            next_state: WAITING_CONNECTION,
            action: DISPLAY_WAITING_CONNECTION
        };
        // L'utilisateur décide de tenter de se connecter à un hôte
        self.transition_tab[MAIN_MENU as usize][SELECTED_HOST_STATUS_IS_CLIENT as usize] = stateMachine_transition {
            next_state: TRY_CONNECTION,
            action: DISPLAY_TRY_CONNECTION
        };
        // L'utilisateur ferme la connexion en temps qu'hôte
        self.transition_tab[WAITING_CONNECTION as usize][CANCEL as usize] = stateMachine_transition {
            next_state: MAIN_MENU,
            action: DISPLAY_MAIN_MENU
        };
        // La connexion avec un client à été établie
        self.transition_tab[WAITING_CONNECTION as usize][CONNECTION_IS_ESTABLISHED as usize] = stateMachine_transition {
            next_state: LOBBY,
            action: DISPLAY_LOBBY
        };
        // L'utilisateur décide d'arréter la tentative de connexion à un hôte
        self.transition_tab[TRY_CONNECTION as usize][CANCEL as usize] = stateMachine_transition {
            next_state: MAIN_MENU,
            action: DISPLAY_MAIN_MENU
        };
        // La connexion avec un hôte à été établie
        self.transition_tab[TRY_CONNECTION as usize][CONNECTION_IS_ESTABLISHED as usize] = stateMachine_transition {
            next_state: LOBBY,
            action: DISPLAY_LOBBY
        };
        // L'utilisateur demande à retourner au menu principal
        self.transition_tab[LOBBY as usize][ASK_TO_GO_BACK_TO_MAIN_MENU as usize] = stateMachine_transition {
            next_state: MAIN_MENU,
            action: DISPLAY_MAIN_MENU
        };
        // L'hôte démarre la partie
        self.transition_tab[LOBBY as usize][GAME_STARTED as usize] = stateMachine_transition {
            next_state: GAMELOOP,
            action: DISPLAY_GAMELOOP
        };
        // L'utilisateur demande à retourner au menu principal
        self.transition_tab[GAMELOOP as usize][ASK_TO_GO_BACK_TO_MAIN_MENU as usize] = stateMachine_transition {
            next_state: MAIN_MENU,
            action: DISPLAY_MAIN_MENU
        };
        // La partie c'est terminée par la victoire de l'un des deux joueurs
        self.transition_tab[GAMELOOP as usize][GAME_HAS_ENDED as usize] = stateMachine_transition {
            next_state: LOBBY,
            action: DISPLAY_LOBBY
        };
    }
}