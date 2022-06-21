// Rusty_engine test project

use rusty_engine::prelude::*;
use rand::prelude::*;

struct GameState {
    health_amount: u8,
    objects_collected: u8,
    lost: bool,
    timer: Timer,
}

const PLAYER_SPEED: f32 = 350.0;
const BACKGROUND_SPEED: f32 = 400.0;

fn main() {
    let mut game = Game::new();

    // Game Setup
    // Create the player sprite
    //let player1 = game.add_sprite("player1", SpritePreset::RacingCarRed);
    
    // Schnauzer version
    //let player1 = game.add_sprite("player1", "sprite/schnauzer/standing-right.png");
    let player1 = game.add_sprite("player1", "sprite/schnauzer/dog-with-balloons-white.png");
    player1.scale = 0.5;

    player1.translation.x = -500.0;
    player1.layer = 10.0;
    player1.collision = true;


    // Populate objects
    for mut i in 1..10 {
        let object = game.add_sprite(format!("object{}", i), "sprite/schnauzer/treat-green.png");
        //let object = game.add_sprite(format!("object{}", i), "sprite/schnauzer/bone-yellow.png");
        object.scale = 0.1; // object size
        object.collision = true;
        object.rotation = thread_rng().gen_range(0.0..6.2);
        object.translation.x = -600.0 + 150.0 * i as f32;
        object.translation.y = thread_rng().gen_range(-300.0..300.0);
    }
    // Start some background music
    
    game.audio_manager
        .play_music(MusicPreset::WhimsicalPopsicle, 0.2);

    let timer_message = game.add_text("timer_message", "Aika: 60.0");
    timer_message.translation = Vec2::new(10.0, 320.0);

    // Create the points message
    let points_message = game.add_text("points_message", "Herkkuja: 0");
    points_message.translation = Vec2::new(550.0, 320.0);
    
    // add game logic
    game.add_logic(game_logic);
    // define initial game state with the struct and run game
    game.run(GameState {
        health_amount: 5,
        objects_collected: 0,
        lost: false,
        timer: Timer::from_seconds(60.0, false),
    });
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    // Handle keyboard input
    let mut direction = 0.0;
    if engine.keyboard_state.pressed_any(&[KeyCode::W, KeyCode::Up]) {
        direction += 1.0;
    }
    if engine.keyboard_state.pressed_any(&[KeyCode::S, KeyCode::Down]) {
        direction -= 1.0;
    }
    if engine.keyboard_state.just_pressed(KeyCode::Z) {
        if engine.audio_manager.music_playing() {
            engine.audio_manager.stop_music();
        } else {
            engine.audio_manager
                .play_music(MusicPreset::WhimsicalPopsicle, 0.2);
        }
    }


    // Move player
    let player1 = engine.sprites.get_mut("player1").unwrap();
    player1.translation.y += direction * PLAYER_SPEED * engine.delta_f32;
    player1.rotation = direction * 0.15;

    // Check up and down edges
    if player1.translation.y < -360.0 {
        player1.translation.y = 359.0;
    }
    if player1.translation.y > 360.0 {
        player1.translation.y = -359.0;
    }

    // Move objects and generate to new location once out of range
    for sprite in engine.sprites.values_mut() {
        if sprite.label.starts_with("object") {
            sprite.translation.x -= BACKGROUND_SPEED * engine.delta_f32;
            sprite.rotation += 0.01;
            if sprite.translation.x < -675.0 {
                sprite.translation.x += 1500.0;
                sprite.translation.y = thread_rng().gen_range(-300.0..300.0);
            }
        }
    }
    
    // Collision management
    for event in engine.collision_events.drain(..) {
        // We don't care if obstacles collide with each other or collisions end
        if !event.pair.either_contains("player1") || event.state.is_end() {
            continue;
        }
        // debug message
        //println!("{} and {} collided!", event.pair.0, event.pair.1);
        if event.pair.0 == "player1" {
            //println!("player1 first");
            //engine.sprites.remove(&event.pair.1);
            let treat = engine.sprites.get_mut(&event.pair.1).unwrap();
            treat.translation.x = 825.0;
            treat.translation.y = thread_rng().gen_range(-300.0..300.0);  
        }
        if event.pair.1 == "player1" {
            //println!("player1 second");
            //engine.sprites.remove(&event.pair.0);
            let treat = engine.sprites.get_mut(&event.pair.0).unwrap();
            treat.translation.x = 825.0;
            treat.translation.y = thread_rng().gen_range(-300.0..300.0);  
        }
        // Update points
        let points_message = engine.texts.get_mut("points_message").unwrap();
        if game_state.objects_collected < 100 {
            game_state.objects_collected += 1;
            points_message.value = format!("Herkkuja: {}", game_state.objects_collected);
            engine.audio_manager.play_sfx(SfxPreset::Impact3, 0.5);
        }
   }

    // Update timer
    game_state.timer.tick(engine.delta);
    let elapsed_time = game_state.timer.elapsed_secs() as i64;
    let timer_message = engine.texts.get_mut("timer_message").unwrap();
    timer_message.value = format!("Aika: {}", 60-elapsed_time);
 
}

