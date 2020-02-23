use rustyblocks::game_logic::field::{GameField};
use rustyblocks::game_logic::field::render::Renderable;
use rustyblocks::game_logic::field::rl::LearnHelper;
use rustyblocks::game_logic::log::{GameLog};
use insta::{assert_debug_snapshot,assert_snapshot};

const BLA: &'static str = "{\"log\":[{\"PayloadRolled\":{\"from\":1,\"block\":2}},{\"PayloadPlaced\":{\"from\":1,\"block\":2,\"orientation\":0,\"x\":1,\"y\":0}},{\"PayloadRolled\":{\"from\":0,\"block\":1}},{\"PayloadPlaced\":{\"from\":0,\"block\":1,\"orientation\":0,\"x\":4,\"y\":0}},{\"PayloadRolled\":{\"from\":1,\"block\":1}},{\"PayloadPlaced\":{\"from\":1,\"block\":1,\"orientation\":0,\"x\":2,\"y\":1}},{\"PayloadRolled\":{\"from\":0,\"block\":0}},{\"PayloadPlaced\":{\"from\":0,\"block\":0,\"orientation\":1,\"x\":6,\"y\":0}},{\"PayloadRolled\":{\"from\":1,\"block\":1}},{\"PayloadPlaced\":{\"from\":1,\"block\":1,\"orientation\":0,\"x\":2,\"y\":3}},{\"PayloadRolled\":{\"from\":0,\"block\":1}},{\"PayloadPlaced\":{\"from\":0,\"block\":1,\"orientation\":0,\"x\":4,\"y\":2}},{\"PayloadRolled\":{\"from\":1,\"block\":0}},{\"PayloadPlaced\":{\"from\":1,\"block\":0,\"orientation\":1,\"x\":3,\"y\":5}},{\"PayloadRolled\":{\"from\":0,\"block\":4}},{\"PayloadPlaced\":{\"from\":0,\"block\":4,\"orientation\":4,\"x\":4,\"y\":4}},{\"PayloadRolled\":{\"from\":1,\"block\":0}},{\"PayloadPlaced\":{\"from\":1,\"block\":0,\"orientation\":1,\"x\":3,\"y\":9}},{\"PayloadRolled\":{\"from\":0,\"block\":3}},{\"PayloadPlaced\":{\"from\":0,\"block\":3,\"orientation\":1,\"x\":4,\"y\":7}},{\"PayloadRolled\":{\"from\":1,\"block\":4}},{\"PayloadPlaced\":{\"from\":1,\"block\":4,\"orientation\":7,\"x\":0,\"y\":1}},{\"PayloadRolled\":{\"from\":0,\"block\":4}},{\"PayloadPlaced\":{\"from\":0,\"block\":4,\"orientation\":3,\"x\":5,\"y\":7}},{\"PayloadRolled\":{\"from\":1,\"block\":4}},{\"PayloadPlaced\":{\"from\":1,\"block\":4,\"orientation\":0,\"x\":0,\"y\":5}},{\"PayloadRolled\":{\"from\":0,\"block\":2}},{\"PayloadPlaced\":{\"from\":0,\"block\":2,\"orientation\":0,\"x\":5,\"y\":8}},{\"PayloadRolled\":{\"from\":1,\"block\":0}},{\"PayloadPlaced\":{\"from\":1,\"block\":0,\"orientation\":1,\"x\":3,\"y\":13}},{\"PayloadRolled\":{\"from\":0,\"block\":3}},{\"PayloadPlaced\":{\"from\":0,\"block\":3,\"orientation\":3,\"x\":4,\"y\":9}},{\"PayloadRolled\":{\"from\":1,\"block\":2}},{\"PayloadPlaced\":{\"from\":1,\"block\":2,\"orientation\":1,\"x\":1,\"y\":7}},{\"PayloadRolled\":{\"from\":0,\"block\":4}},{\"PayloadPlaced\":{\"from\":0,\"block\":4,\"orientation\":4,\"x\":7,\"y\":0}},{\"PayloadRolled\":{\"from\":1,\"block\":4}},{\"PayloadPlaced\":{\"from\":1,\"block\":4,\"orientation\":7,\"x\":0,\"y\":7}},{\"PayloadRolled\":{\"from\":0,\"block\":0}},{\"PayloadPlaced\":{\"from\":0,\"block\":0,\"orientation\":1,\"x\":4,\"y\":11}},{\"PayloadRolled\":{\"from\":1,\"block\":1}},{\"PayloadPlaced\":{\"from\":1,\"block\":1,\"orientation\":0,\"x\":8,\"y\":1}},{\"PayloadRolled\":{\"from\":0,\"block\":1}},{\"PayloadPlaced\":{\"from\":0,\"block\":1,\"orientation\":0,\"x\":7,\"y\":3}},{\"PayloadRolled\":{\"from\":1,\"block\":0}},{\"PayloadPlaced\":{\"from\":1,\"block\":0,\"orientation\":1,\"x\":2,\"y\":8}},{\"PayloadRolled\":{\"from\":0,\"block\":3}},{\"PayloadPlaced\":{\"from\":0,\"block\":3,\"orientation\":1,\"x\":8,\"y\":6}},{\"PayloadRolled\":{\"from\":1,\"block\":4}},{\"PayloadPlaced\":{\"from\":1,\"block\":4,\"orientation\":1,\"x\":0,\"y\":10}},{\"PayloadRolled\":{\"from\":0,\"block\":4}},{\"PayloadPlaced\":{\"from\":0,\"block\":4,\"orientation\":7,\"x\":7,\"y\":5}},{\"PayloadRolled\":{\"from\":1,\"block\":4}},{\"PayloadPlaced\":{\"from\":1,\"block\":4,\"orientation\":3,\"x\":0,\"y\":13}},{\"PayloadRolled\":{\"from\":0,\"block\":1}},{\"PayloadPlaced\":{\"from\":0,\"block\":1,\"orientation\":0,\"x\":7,\"y\":8}},{\"PayloadSkipped\":{\"from\":1,\"block\":4,\"reason\":1}},{\"PayloadRolled\":{\"from\":0,\"block\":3}},{\"PayloadPlaced\":{\"from\":0,\"block\":3,\"orientation\":3,\"x\":6,\"y\":9}},{\"PayloadRolled\":{\"from\":1,\"block\":2}},{\"PayloadPlaced\":{\"from\":1,\"block\":2,\"orientation\":2,\"x\":6,\"y\":12}},{\"PayloadRolled\":{\"from\":0,\"block\":4}},{\"PayloadPlaced\":{\"from\":0,\"block\":4,\"orientation\":4,\"x\":5,\"y\":13}},{\"PayloadRolled\":{\"from\":1,\"block\":1}},{\"PayloadPlaced\":{\"from\":1,\"block\":1,\"orientation\":0,\"x\":4,\"y\":15}},{\"PayloadRolled\":{\"from\":0,\"block\":2}},{\"PayloadPlaced\":{\"from\":0,\"block\":2,\"orientation\":3,\"x\":7,\"y\":15}},{\"PayloadSkipped\":{\"from\":1,\"block\":4,\"reason\":1}},{\"PayloadSkipped\":{\"from\":0,\"block\":4,\"reason\":1}},{\"PayloadRolled\":{\"from\":1,\"block\":2}},{\"PayloadPlaced\":{\"from\":1,\"block\":2,\"orientation\":3,\"x\":2,\"y\":14}},{\"PayloadRolled\":{\"from\":0,\"block\":3}},{\"PayloadPlaced\":{\"from\":0,\"block\":3,\"orientation\":1,\"x\":5,\"y\":18}},{\"PayloadRolled\":{\"from\":1,\"block\":0}},{\"PayloadPlaced\":{\"from\":1,\"block\":0,\"orientation\":1,\"x\":0,\"y\":14}},{\"PayloadRolled\":{\"from\":0,\"block\":2}},{\"PayloadPlaced\":{\"from\":0,\"block\":2,\"orientation\":0,\"x\":3,\"y\":17}},{\"PayloadSkipped\":{\"from\":1,\"block\":0,\"reason\":1}},{\"PayloadRolled\":{\"from\":0,\"block\":0}},{\"PayloadPlaced\":{\"from\":0,\"block\":0,\"orientation\":1,\"x\":1,\"y\":15}},{\"PayloadRolled\":{\"from\":1,\"block\":1}},{\"PayloadPlaced\":{\"from\":1,\"block\":1,\"orientation\":0,\"x\":6,\"y\":18}},{\"PayloadRolled\":{\"from\":0,\"block\":1}},{\"PayloadPlaced\":{\"from\":0,\"block\":1,\"orientation\":0,\"x\":8,\"y\":10}},{\"PayloadRolled\":{\"from\":1,\"block\":3}},{\"PayloadPlaced\":{\"from\":1,\"block\":3,\"orientation\":1,\"x\":8,\"y\":14}},{\"PayloadRolled\":{\"from\":0,\"block\":2}},{\"PayloadPlaced\":{\"from\":0,\"block\":2,\"orientation\":2,\"x\":2,\"y\":19}},{\"PayloadSkipped\":{\"from\":1,\"block\":0,\"reason\":1}},{\"PayloadRolled\":{\"from\":0,\"block\":0}},{\"PayloadConsidering\":{\"play_index\":0}},{\"PayloadPlaced\":{\"from\":0,\"block\":0,\"orientation\":1,\"x\":8,\"y\":15}},{\"PayloadSkipped\":{\"from\":1,\"block\":0,\"reason\":1}},{\"PayloadSkipped\":{\"from\":0,\"block\":4,\"reason\":1}},{\"PayloadRolled\":{\"from\":1,\"block\":3}},{\"PayloadSkipped\":{\"from\":1,\"block\":3,\"reason\":0}},{\"PayloadRolled\":{\"from\":0,\"block\":2}},{\"PayloadSkipped\":{\"from\":0,\"block\":2,\"reason\":0}}]}";

#[test]
fn score_via_replay() {
  let log: GameLog = serde_json::from_str(BLA).unwrap();
  let mut field = GameField::new(2);
  field.restore_from_log(&log, true);
  assert_snapshot!("restored_field", &field.to_field_string());
  assert_debug_snapshot!("SCORES", field.scores);
  field.place_block_using_play(0);
  assert!(field.game_over == true);
  assert_snapshot!("full_reward", field.calculate_full_reward(0, false).to_string().as_str());
}