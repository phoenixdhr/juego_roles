use std::collections::HashMap;
use csv::{ReaderBuilder, StringRecord};

// Ruta del archivo CSV con las situaciones del juego.
const PATH: &str = "./history.csv";

// Estructura que representa una situación en el juego.
#[derive(Debug)]
struct Situation {
    tipo: String,        // Tipo de la situación (SITUACION o OPCION).
    tag: String,         // Etiqueta única para identificar la situación.
    texto: String,       // Texto descriptivo de la situación u opción.
    vida: i32,           // Puntos de vida que afectan al jugador (positivos o negativos).
    options: Vec<Situation>,  // Vector que contiene las opciones disponibles para esta situación.
}

impl Situation {
    // Constructor para crear una nueva instancia de Situation a partir de una línea del CSV.
    fn new(line: StringRecord) -> Situation {
        let new_situation = Situation {
            tipo: line.get(0).unwrap().trim().to_string(),
            tag: line.get(1).unwrap().trim().to_string(),
            texto: line.get(2).unwrap().trim().to_string(),
            vida: line.get(3).unwrap().trim().parse().unwrap_or(0), // Si no se puede parsear, se asigna 0.
            options: Vec::new(), // Se inicializa con un vector vacío para almacenar las opciones.
        };

        new_situation
    }
}

fn main() {
    // HashMap para almacenar las situaciones del juego, donde la clave es la etiqueta de la situación.
    let mut storage_game: HashMap<String, Situation> = HashMap::new();

    // Etiqueta de la situación actual, inicializada en "INICIO".
    let mut actual_tag_situation: &str = "INICIO";

    // Variable para almacenar la última etiqueta de situación almacenada en el HashMap.
    let mut last_tag_storage = "".to_string();

    // Puntos de vida del jugador, inicializados en 100.
    let mut vida = 100;

    // Creación del lector CSV para leer el archivo de situaciones.
    let mut all_line_situations = ReaderBuilder::new().delimiter(b';').from_path(PATH).unwrap();

    // Bucle para procesar cada línea del CSV y crear las instancias de Situation correspondientes.
    for line_wrap in all_line_situations.records() {
        let line = line_wrap.unwrap();
        let situation = Situation::new(line);

        if situation.tipo == "SITUACION" {
            last_tag_storage = situation.tag.clone(); // Almacenar la etiqueta de la situación actual.
            storage_game.insert(situation.tag.clone(), situation); // Agregar la situación al HashMap.
        } else if situation.tipo == "OPCION" {
            // Si es una opción, agregarla al vector de opciones de la última situación almacenada.
            storage_game.get_mut(&last_tag_storage).unwrap().options.push(situation);
        }
    }

    // Bucle principal del juego que se ejecuta mientras la vida del jugador sea mayor que 0.
    while vida > 0 {
        println!("VIDA: {}  TAG {}", vida, actual_tag_situation);

        // Obtener la situación actual del HashMap según la etiqueta actual.
        if let Some(actual_situaton) = storage_game.get(actual_tag_situation) {
            let mut vec_option = &actual_situaton.options;
            println!("{}", actual_situaton.texto);
            vida = vida + actual_situaton.vida; // Actualizar los puntos de vida del jugador.

            if vec_option.len() > 0 {
                // Si hay opciones disponibles, mostrarlas al jugador.
                for (i, option) in vec_option.into_iter().enumerate() {
                    println!("[{}] {}", i, option.texto);
                }
                let mut option_player_string = String::new();
                std::io::stdin().read_line(&mut option_player_string).unwrap();
                let option_player_num = option_player_string.trim().parse().unwrap_or(99);

                if let Some(opcion_elegida) = vec_option.get(option_player_num) {
                    // Cambiar la etiqueta de situación actual según la opción elegida.
                    actual_tag_situation = &opcion_elegida.tag;
                } else {
                    println!("LA OPCION NO ES VALIDA GIL");
                }
            }

            if actual_situaton.vida != 0 {
                println!("Punto a Favor/Contra = {}", actual_situaton.vida);
                if vida < 0 {
                    println!("PERDISTE {}", vida);
                }
            }
        } else {
            // Si no se encuentra la etiqueta actual en el HashMap, significa que el jugador ganó.
            println!("GANASTE!!");
            break;
        }
    }
}
