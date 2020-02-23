use std::env;
use rustyblocks::game_logic::field::{GameField};
use rustyblocks::game_logic::field::helper_structs::{GameRules};
use rustyblocks::game_logic::log::GameLog;
use rustyblocks::game_logic::field::rl::LearnHelper;
use std::fs;

fn main() {
  let args: Vec<String> = env::args().collect();
  let heuristic_type = args[1].as_str();
  let to_parse = fs::read_to_string(args[2].as_str())
    .expect("Something went wrong reading the file");
  // let to_parse = "{\"log\":[{\"PayloadRolled\":{\"from\":0,\"block\":0}},{\"PayloadConsidering\":{\"play_index\":0}},{\"PayloadPlaced\":{\"from\":0,\"block\":0,\"orientation\":0,\"x\":0,\"y\":0}},{\"PayloadRolled\":{\"from\":1,\"block\":0}},{\"PayloadConsidering\":{\"play_index\":0}},{\"PayloadPlaced\":{\"from\":1,\"block\":0,\"orientation\":1,\"x\":1,\"y\":1}},{\"PayloadConsidering\":{\"play_index\":0}},{\"PayloadRolled\":{\"from\":0,\"block\":0}},{\"PayloadConsidering\":{\"play_index\":1}},{\"PayloadConsidering\":{\"play_index\":3}},{\"PayloadPlaced\":{\"from\":0,\"block\":0,\"orientation\":1,\"x\":1,\"y\":5}},{\"PayloadRolled\":{\"from\":1,\"block\":0}},{\"PayloadConsidering\":{\"play_index\":0}},{\"PayloadPlaced\":{\"from\":1,\"block\":0,\"orientation\":1,\"x\":1,\"y\":9}},{\"PayloadConsidering\":{\"play_index\":0}},{\"PayloadRolled\":{\"from\":0,\"block\":0}},{\"PayloadConsidering\":{\"play_index\":1}},{\"PayloadConsidering\":{\"play_index\":3}},{\"PayloadConsidering\":{\"play_index\":5}},{\"PayloadConsidering\":{\"play_index\":3}},{\"PayloadPlaced\":{\"from\":0,\"block\":0,\"orientation\":1,\"x\":1,\"y\":13}},{\"PayloadRolled\":{\"from\":1,\"block\":0}},{\"PayloadConsidering\":{\"play_index\":0}},{\"PayloadPlaced\":{\"from\":1,\"block\":0,\"orientation\":1,\"x\":3,\"y\":1}},{\"PayloadConsidering\":{\"play_index\":0}},{\"PayloadRolled\":{\"from\":0,\"block\":0}},{\"PayloadConsidering\":{\"play_index\":1}},{\"PayloadConsidering\":{\"play_index\":4}},{\"PayloadConsidering\":{\"play_index\":6}},{\"PayloadPlaced\":{\"from\":0,\"block\":0,\"orientation\":1,\"x\":3,\"y\":5}},{\"PayloadRolled\":{\"from\":1,\"block\":0}},{\"PayloadConsidering\":{\"play_index\":0}},{\"PayloadPlaced\":{\"from\":1,\"block\":0,\"orientation\":1,\"x\":2,\"y\":1}},{\"PayloadConsidering\":{\"play_index\":0}},{\"PayloadRolled\":{\"from\":0,\"block\":0}},{\"PayloadConsidering\":{\"play_index\":1}},{\"PayloadConsidering\":{\"play_index\":4}},{\"PayloadConsidering\":{\"play_index\":6}},{\"PayloadPlaced\":{\"from\":0,\"block\":0,\"orientation\":1,\"x\":3,\"y\":9}},{\"PayloadRolled\":{\"from\":1,\"block\":0}},{\"PayloadConsidering\":{\"play_index\":0}},{\"PayloadPlaced\":{\"from\":1,\"block\":4,\"orientation\":4,\"x\":4,\"y\":0}},{\"PayloadConsidering\":{\"play_index\":0}},{\"PayloadRolled\":{\"from\":0,\"block\":0}},{\"PayloadConsidering\":{\"play_index\":1}},{\"PayloadConsidering\":{\"play_index\":2}},{\"PayloadConsidering\":{\"play_index\":3}},{\"PayloadConsidering\":{\"play_index\":4}},{\"PayloadConsidering\":{\"play_index\":5}},{\"PayloadPlaced\":{\"from\":0,\"block\":1,\"orientation\":0,\"x\":5,\"y\":1}},{\"PayloadRolled\":{\"from\":1,\"block\":0}},{\"PayloadConsidering\":{\"play_index\":0}},{\"PayloadPlaced\":{\"from\":1,\"block\":4,\"orientation\":5,\"x\":7,\"y\":0}},{\"PayloadConsidering\":{\"play_index\":0}},{\"PayloadRolled\":{\"from\":0,\"block\":0}},{\"PayloadConsidering\":{\"play_index\":1}},{\"PayloadConsidering\":{\"play_index\":2}},{\"PayloadConsidering\":{\"play_index\":3}},{\"PayloadConsidering\":{\"play_index\":4}},{\"PayloadConsidering\":{\"play_index\":5}},{\"PayloadPlaced\":{\"from\":0,\"block\":1,\"orientation\":0,\"x\":5,\"y\":3}},{\"PayloadRolled\":{\"from\":1,\"block\":0}},{\"PayloadConsidering\":{\"play_index\":0}},{\"PayloadPlaced\":{\"from\":1,\"block\":0,\"orientation\":1,\"x\":8,\"y\":3}},{\"PayloadConsidering\":{\"play_index\":0}},{\"PayloadRolled\":{\"from\":0,\"block\":0}},{\"PayloadConsidering\":{\"play_index\":1}},{\"PayloadConsidering\":{\"play_index\":2}},{\"PayloadConsidering\":{\"play_index\":3}},{\"PayloadConsidering\":{\"play_index\":4}},{\"PayloadConsidering\":{\"play_index\":5}},{\"PayloadPlaced\":{\"from\":0,\"block\":1,\"orientation\":0,\"x\":5,\"y\":5}},{\"PayloadRolled\":{\"from\":1,\"block\":0}},{\"PayloadConsidering\":{\"play_index\":0}},{\"PayloadPlaced\":{\"from\":1,\"block\":4,\"orientation\":1,\"x\":5,\"y\":7}},{\"PayloadConsidering\":{\"play_index\":0}},{\"PayloadRolled\":{\"from\":0,\"block\":0}},{\"PayloadConsidering\":{\"play_index\":43}},{\"PayloadConsidering\":{\"play_index\":77}},{\"PayloadConsidering\":{\"play_index\":84}},{\"PayloadConsidering\":{\"play_index\":83}},{\"PayloadConsidering\":{\"play_index\":82}},{\"PayloadConsidering\":{\"play_index\":81}},{\"PayloadConsidering\":{\"play_index\":89}},{\"PayloadConsidering\":{\"play_index\":97}},{\"PayloadConsidering\":{\"play_index\":105}},{\"PayloadConsidering\":{\"play_index\":113}},{\"PayloadConsidering\":{\"play_index\":112}},{\"PayloadConsidering\":{\"play_index\":120}},{\"PayloadPlaced\":{\"from\":0,\"block\":4,\"orientation\":3,\"x\":5,\"y\":10}},{\"PayloadRolled\":{\"from\":1,\"block\":0}},{\"PayloadConsidering\":{\"play_index\":0}},{\"PayloadPlaced\":{\"from\":1,\"block\":4,\"orientation\":1,\"x\":5,\"y\":11}},{\"PayloadConsidering\":{\"play_index\":0}},{\"PayloadRolled\":{\"from\":0,\"block\":0}},{\"PayloadPlaced\":{\"from\":0,\"block\":1,\"orientation\":0,\"x\":0,\"y\":17}},{\"PayloadRolled\":{\"from\":1,\"block\":0}},{\"PayloadConsidering\":{\"play_index\":0}},{\"PayloadPlaced\":{\"from\":1,\"block\":4,\"orientation\":3,\"x\":5,\"y\":14}},{\"PayloadConsidering\":{\"play_index\":0}},{\"PayloadRolled\":{\"from\":0,\"block\":0}},{\"PayloadConsidering\":{\"play_index\":1}},{\"PayloadConsidering\":{\"play_index\":2}},{\"PayloadConsidering\":{\"play_index\":3}},{\"PayloadPlaced\":{\"from\":0,\"block\":1,\"orientation\":0,\"x\":5,\"y\":15}},{\"PayloadRolled\":{\"from\":1,\"block\":0}},{\"PayloadConsidering\":{\"play_index\":0}},{\"PayloadPlaced\":{\"from\":1,\"block\":2,\"orientation\":3,\"x\":4,\"y\":13}},{\"PayloadConsidering\":{\"play_index\":0}},{\"PayloadRolled\":{\"from\":0,\"block\":0}},{\"PayloadPlaced\":{\"from\":0,\"block\":2,\"orientation\":1,\"x\":2,\"y\":14}},{\"PayloadRolled\":{\"from\":1,\"block\":0}},{\"PayloadConsidering\":{\"play_index\":0}},{\"PayloadPlaced\":{\"from\":1,\"block\":3,\"orientation\":2,\"x\":4,\"y\":15}},{\"PayloadConsidering\":{\"play_index\":0}},{\"PayloadRolled\":{\"from\":0,\"block\":0}},{\"PayloadConsidering\":{\"play_index\":3}},{\"PayloadConsidering\":{\"play_index\":2}},{\"PayloadConsidering\":{\"play_index\":5}},{\"PayloadConsidering\":{\"play_index\":9}},{\"PayloadPlaced\":{\"from\":0,\"block\":2,\"orientation\":3,\"x\":4,\"y\":17}},{\"PayloadRolled\":{\"from\":1,\"block\":0}},{\"PayloadConsidering\":{\"play_index\":0}},{\"PayloadPlaced\":{\"from\":1,\"block\":2,\"orientation\":1,\"x\":2,\"y\":18}},{\"PayloadConsidering\":{\"play_index\":0}},{\"PayloadRolled\":{\"from\":0,\"block\":0}},{\"PayloadConsidering\":{\"play_index\":1}},{\"PayloadPlaced\":{\"from\":0,\"block\":2,\"orientation\":2,\"x\":5,\"y\":19}},{\"PayloadRolled\":{\"from\":1,\"block\":0}},{\"PayloadConsidering\":{\"play_index\":0}},{\"PayloadPlaced\":{\"from\":1,\"block\":3,\"orientation\":3,\"x\":7,\"y\":6}},{\"PayloadConsidering\":{\"play_index\":0}},{\"PayloadRolled\":{\"from\":0,\"block\":0}},{\"PayloadPlaced\":{\"from\":0,\"block\":2,\"orientation\":1,\"x\":7,\"y\":9}},{\"PayloadRolled\":{\"from\":1,\"block\":0}},{\"PayloadConsidering\":{\"play_index\":0}},{\"PayloadPlaced\":{\"from\":1,\"block\":3,\"orientation\":1,\"x\":7,\"y\":12}},{\"PayloadConsidering\":{\"play_index\":0}},{\"PayloadRolled\":{\"from\":0,\"block\":0}},{\"PayloadConsidering\":{\"play_index\":0}},{\"PayloadConsidering\":{\"play_index\":0}},{\"PayloadConsidering\":{\"play_index\":2}},{\"PayloadConsidering\":{\"play_index\":1}},{\"PayloadConsidering\":{\"play_index\":4}},{\"PayloadPlaced\":{\"from\":0,\"block\":2,\"orientation\":3,\"x\":8,\"y\":13}},{\"PayloadRolled\":{\"from\":1,\"block\":0}},{\"PayloadConsidering\":{\"play_index\":0}},{\"PayloadPlaced\":{\"from\":1,\"block\":3,\"orientation\":3,\"x\":7,\"y\":14}},{\"PayloadConsidering\":{\"play_index\":0}},{\"PayloadRolled\":{\"from\":0,\"block\":0}},{\"PayloadConsidering\":{\"play_index\":2}},{\"PayloadConsidering\":{\"play_index\":1}},{\"PayloadConsidering\":{\"play_index\":0}},{\"PayloadConsidering\":{\"play_index\":2}},{\"PayloadPlaced\":{\"from\":0,\"block\":3,\"orientation\":3,\"x\":7,\"y\":16}},{\"PayloadRolled\":{\"from\":1,\"block\":0}},{\"PayloadSkipped\":{\"from\":1,\"block\":0,\"reason\":0}},{\"PayloadRolled\":{\"from\":0,\"block\":0}},{\"PayloadConsidering\":{\"play_index\":0}},{\"PayloadConsidering\":{\"play_index\":0}},{\"PayloadPlaced\":{\"from\":0,\"block\":4,\"orientation\":2,\"x\":7,\"y\":18}},{\"PayloadRolled\":{\"from\":1,\"block\":0}},{\"PayloadSkipped\":{\"from\":1,\"block\":0,\"reason\":0}},{\"PayloadRolled\":{\"from\":0,\"block\":0}},{\"PayloadSkipped\":{\"from\":0,\"block\":0,\"reason\":0}}]}";
  let parsed_log: GameLog = serde_json::from_str(to_parse.as_str()).unwrap();
  let mut field = GameField::new_with_rules(2, GameRules::deterministic());
  field.restore_from_log(&parsed_log, true);
  println!("__RUSTYBLOCKS__");
  if field.game_over {
    println!("0;{}\n{}", field.calculate_full_reward(0, false), field.calculate_full_reward(1, false));
  } else {
    let heuristic_acion = if heuristic_type == "random" {
      field.get_random_heuristic_play()
    } else {
      field.get_best_heuristic_play_for_params(4.95238700733393,0.115980839099051,0.0603795891552745,19.0148438477953, true, 0)
    };
    field.place_block_using_play(heuristic_acion);
    println!("1;{}", serde_json::to_string(&field.log).unwrap());
  }
}