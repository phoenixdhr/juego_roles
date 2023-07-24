use std::collections::HashMap;

use csv::{ReaderBuilder, StringRecord};



const PATH:&str = "./history.csv";

#[derive(Debug)]
struct Situation {
    tipo:String,
    tag:String,
    texto:String,
    vida:i32,
    options:Vec<Situation>,
}

impl Situation {
    fn new(line: StringRecord) ->Situation {
        let new_situation = Situation{
            tipo:line.get(0).unwrap().trim().to_string(),
            tag:line.get(1).unwrap().trim().to_string(),
            texto:line.get(2).unwrap().trim().to_string(),
            vida:line.get(3).unwrap().trim().parse().unwrap_or(0),
            options:Vec::new(),
        };

        new_situation
    }
}



fn main (){

    let mut storage_game:HashMap<String,Situation>=HashMap::new();
    let mut actual_tag_situation: &str ="INICIO";
    let mut last_tag_storage ="".to_string();
    let mut vida = 100;


    let mut all_line_situations= ReaderBuilder::new().delimiter(b';').from_path(PATH).unwrap();

    for line_wrap in all_line_situations.records() {
        
        let line = line_wrap.unwrap();
        let situation = Situation::new(line);

        if situation.tipo == "SITUACION" {
            last_tag_storage = situation.tag.clone();
            storage_game.insert(situation.tag.clone(), situation);
        } else if situation.tipo == "OPCION" {
            storage_game.get_mut(&last_tag_storage).unwrap().options.push(situation);
        }
        

    }
    
    
    while vida>0 {
        
        println!("VIDA:{}  TAG {}", vida, actual_tag_situation);
        
        if let Some(actual_situaton) = storage_game.get(actual_tag_situation){

            let mut vec_option =&actual_situaton.options;
            println!("{}", actual_situaton.texto);
            vida = vida + actual_situaton.vida;

            if vec_option.len()>0 {
                for (i, option) in vec_option.into_iter().enumerate() {
                    println!("[{i}] {}",option.texto);
                }
                let mut option_player_string=String::new();
                std::io::stdin().read_line(&mut option_player_string).unwrap();
                let option_player_num =option_player_string.trim().parse().unwrap_or(99);
    
                if let Some(opcion_elegida) = vec_option.get(option_player_num)   {
        
                    actual_tag_situation=&opcion_elegida.tag;
                } else {
                    println!("LA OPCION NO ES VALIDA GIL")
    
                }
        
            }
    
            if actual_situaton.vida!=0 {
                println!("Punto a Favor/Contra = {}",actual_situaton.vida);     
                if vida<0 {
                    println!("PERDISTE {}", vida);
                }
        
            }
        } else {
            println!("GANASTE!!");
            break;
        }
        
        
        
    }

}