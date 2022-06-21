use rusty_engine::prelude::*;

struct GameState {}

fn main() {
    let mut game = Game::new();

    let a = game.add_sprite("51", "sprite/rolling/block_narrow.png"); a.translation = Vec2::new(-253.0, -240.0); a.rotation = 12.56637192; a.scale = 1.00000000; a.layer = 0.01000000; a.collision = true;
    let a = game.add_sprite("52", "sprite/rolling/block_narrow.png"); a.translation = Vec2::new(-23.0, -140.0); a.rotation = 12.56637192; a.scale = 1.00000000; a.layer = 0.02000000; a.collision = true;
    let a = game.add_sprite("53", "sprite/rolling/block_narrow.png"); a.translation = Vec2::new(202.3, -40.0); a.rotation = 12.56637192; a.scale = 1.00000000; a.layer = 0.03000000; a.collision = true;
    let a = game.add_sprite("54", "sprite/rolling/block_narrow.png"); a.translation = Vec2::new(440.7, 40.0); a.rotation = 12.56637192; a.scale = 1.00000000; a.layer = 0.04000000; a.collision = true;
    let a = game.add_sprite("55", "sprite/rolling/block_narrow.png"); a.translation = Vec2::new(566.6, 40.0); a.rotation = 12.56637192; a.scale = 1.00000000; a.layer = 0.05000000; a.collision = true;
    let a = game.add_sprite("56", "sprite/schnauzer/standing-right.png"); a.translation = Vec2::new(-360.0, -360.0); a.rotation = 0.0; a.scale = 0.50000000; a.layer = 10.0; a.collision = true;


    game.add_logic(logic);
    game.run(GameState {});
}

fn logic(engine: &mut Engine, game_state: &mut GameState) {
    // Game Logic Goes Here
}