use std::process::exit;

use crate::audio::Sound;
use macroquad::audio::PlaySoundParams;
use macroquad::prelude::*;
pub use miniquad::KeyCode;
use macroquad::audio;
use macroquad::audio::set_sound_volume;
use macroquad::audio::stop_sound;

#[derive(Clone)]
struct Car {
    x: f32,
    y: f32,
    radius: f32,
    color: Color,
    dir: f32,
    move_dir: f32,
    vx: f32,
    vy: f32,
    speed: f32,
    turn_speed: f32,
    max_speed: f32,
    accel: f32,
    friction: f32,
    keys: Vec<KeyCode>,
    name: String,
    can_lap: bool,
    lap: usize,
    string_pos: f32,
    spinning_out: u32,
}

enum WallType {
    Top,
    Bottom,
    Left,
    Right,
}

struct Wall {
    start: f32,
    end: f32,
    level: f32,
    dir: WallType,
}

#[derive(PartialEq, Debug, Clone)]
enum GameState {
    MainMenu,
    Game,
}

//Collision detection function = is the distance from the center of circle a to the center of circle b 
//less/equal than the radius of both circles combined?
impl Car {
    fn collides(&self, wall: &Wall) -> bool {
        match wall.dir {
            WallType::Top | WallType::Bottom => {
                if self.x + self.radius >= wall.start && self.x - self.radius <= wall.end
                    && self.y + self.radius >= wall.level && self.y - self.radius <= wall.level {
                        true
                } else {
                    false
                }
            }
            _ => {
                if self.x + self.radius >= wall.level && self.x - self.radius <= wall.level
                    && self.y + self.radius >= wall.start && self.y - self.radius <= wall.end {
                        true
                } else {
                    false
                }
            }
        }
    }
}

#[macroquad::main("MyGame")]
async fn main() {
    let screen_width= 1200.0;
    let screen_height = 800.0;
    let dev_mode = false;
    let mut sprite_counter = 0;
    let mut main_sprite_counter = 0;
    let mut game_state = GameState::MainMenu;
    let mut start_button= 2;
    let mut quit_button= 4;
    let mut controls_timer = 0;
    let mut win_state = false;
    let mut win_car:Vec<String> = vec![];
    let mut countdown = true;
    let mut play_title_audio = true;
    let mut play_game_audio = true;
    let mut play_win_audio = true;

    let mut cars: Vec<Car> = Vec::new();
    let car1 = Car {
        x: 125.0,
        y: 240.0,
        radius: 15.0,
        color: Color::from_hex(0xff70d4),
        dir: -89.0,
        move_dir: 0.0,
        vx: 0.0,
        vy: 0.0,
        speed: 0.0,
        turn_speed: 100.0,
        max_speed: 200.0,
        accel: 200.0,
        friction: 0.95,
        keys: vec!(KeyCode::W, KeyCode::S, KeyCode::A, KeyCode::D),
        name: "Pinky".to_string(),
        can_lap: false,
        lap: 1,
        string_pos: 330.,
        spinning_out: 0,
    };
    cars.push(car1);
    let car2 = Car {
        x: 175.0,
        y: 275.0,
        radius: 15.0,
        color: Color::from_hex(0x3CA7D5),
        dir: -89.0,
        move_dir: 0.0,
        vx: 0.0,
        vy: 0.0,
        speed: 0.0,
        turn_speed: 100.0,
        max_speed: 200.0,
        accel: 200.0,
        friction: 0.95,
        keys: vec!(KeyCode::Up, KeyCode::Down, KeyCode::Left, KeyCode::Right),
        name: "Bluey".to_string(),
        can_lap: false,
        lap: 1,
        string_pos: 650.,
        spinning_out: 0,
    };
    cars.push(car2);
    let walls: Vec<Wall> = vec![
        Wall { start: 405.0, end: 495.0, level: 350.0, dir: WallType::Top },
        Wall { start: 105.0, end: 395.0, level: 100.0, dir: WallType::Top },
        Wall { start: 505.0, end: 945.0, level: 100.0, dir: WallType::Top },
        Wall { start: 705.0, end: 945.0, level: 300.0, dir: WallType::Top },
        Wall { start: 405.0, end: 495.0, level: 200.0, dir: WallType::Top },
        Wall { start: 805.0, end: 1095.0, level: 450.0, dir: WallType::Top },
        Wall { start: 905.0, end: 995.0, level: 600.0, dir: WallType::Top },
        Wall { start: 205.0, end: 845.0, level: 600.0, dir: WallType::Top },

        Wall { start: 405.0, end: 495.0, level: 250.0, dir: WallType::Bottom },
        Wall { start: 205.0, end: 295.0, level: 200.0, dir: WallType::Bottom },
        Wall { start: 305.0, end: 595.0, level: 450.0, dir: WallType::Bottom },
        Wall { start: 605.0, end: 945.0, level: 200.0, dir: WallType::Bottom },
        Wall { start: 805.0, end: 945.0, level: 400.0, dir: WallType::Bottom },
        Wall { start: 705.0, end: 845.0, level: 550.0, dir: WallType::Bottom },
        Wall { start: 905.0, end: 995.0, level: 550.0, dir: WallType::Bottom },
        Wall { start: 105.0, end: 1095.0, level: 700.0, dir: WallType::Bottom },

        Wall { start: 205.0, end: 445.0, level: 300.0, dir: WallType::Left },
        Wall { start: 255.0, end: 345.0, level: 500.0, dir: WallType::Left },
        Wall { start: 105.0, end: 195.0, level: 500.0, dir: WallType::Left },
        Wall { start: 305.0, end: 545.0, level: 700.0, dir: WallType::Left },
        Wall { start: 555.0, end: 595.0, level: 850.0, dir: WallType::Left },
        Wall { start: 555.0, end: 595.0, level: 1000.0, dir: WallType::Left },
        Wall { start: 105.0, end: 695.0, level: 100.0, dir: WallType::Left },

        Wall { start: 105.0, end: 195.0, level: 400.0, dir: WallType::Right },
        Wall { start: 255.0, end: 345.0, level: 400.0, dir: WallType::Right },
        Wall { start: 205.0, end: 445.0, level: 600.0, dir: WallType::Right },
        Wall { start: 405.0, end: 445.0, level: 800.0, dir: WallType::Right },
        Wall { start: 555.0, end: 595.0, level: 900.0, dir: WallType::Right },
        Wall { start: 455.0, end: 695.0, level: 1100.0, dir: WallType::Right },
        Wall { start: 205.0, end: 595.0, level: 200.0, dir: WallType::Right }
    ];

    let checkpoint = Rect::new(700., 300., 250., 100.);
    let goal = Rect::new(100., 100., 200., 100.);

    //TEXTURES

    //Background texture
    request_new_screen_size(screen_width, screen_height);
    //set_fullscreen(true);
    //let background_texture: Texture2D = load_texture("src/race.png").await.unwrap();

    //Car 1 texture - NOTE: Car Rotation is in RADIANS
    //let car_texture: Texture2D = load_texture("src/car.png").await.unwrap();
    //car_texture.set_filter(FilterMode::Linear);
    let car_sprites = vec![
        vec![
            load_texture("src/car-right/car1.png").await.unwrap(),
            load_texture("src/car-right/car2.png").await.unwrap(),
            load_texture("src/car-right/car3.png").await.unwrap(),
            load_texture("src/car-right/car4.png").await.unwrap(),
        ],
        vec![
            load_texture("src/car-left/car1.png").await.unwrap(),
            load_texture("src/car-left/car2.png").await.unwrap(),
            load_texture("src/car-left/car3.png").await.unwrap(),
            load_texture("src/car-left/car4.png").await.unwrap(),
        ]
    ];
    for vec in &car_sprites {
        for sprite in vec {
            sprite.set_filter(FilterMode::Linear);
        }
    }

    // Starry background sprites
    let background_sprites = vec![
        load_texture("src/background/1.png").await.unwrap(),load_texture("src/background/2.png").await.unwrap(),
        load_texture("src/background/3.png").await.unwrap(),load_texture("src/background/4.png").await.unwrap(),
        load_texture("src/background/5.png").await.unwrap(),load_texture("src/background/6.png").await.unwrap(),
        load_texture("src/background/7.png").await.unwrap(),load_texture("src/background/8.png").await.unwrap(),
        load_texture("src/background/9.png").await.unwrap(),load_texture("src/background/10.png").await.unwrap(),
        load_texture("src/background/11.png").await.unwrap(),load_texture("src/background/12.png").await.unwrap(),
        load_texture("src/background/13.png").await.unwrap(),load_texture("src/background/14.png").await.unwrap(),
        load_texture("src/background/16.png").await.unwrap(),
        load_texture("src/background/17.png").await.unwrap(),load_texture("src/background/18.png").await.unwrap(),
        load_texture("src/background/19.png").await.unwrap(),load_texture("src/background/20.png").await.unwrap(),
        load_texture("src/background/22.png").await.unwrap(),
        load_texture("src/background/23.png").await.unwrap(),load_texture("src/background/24.png").await.unwrap()
    ];
    
    for sprite in &background_sprites {
        sprite.set_filter(FilterMode::Linear);
    }

    // track sprites
    let track_sprites = vec![
        load_texture("src/track/track1.png").await.unwrap(),
        load_texture("src/track/track2.png").await.unwrap(),
        load_texture("src/track/track3.png").await.unwrap(),
        load_texture("src/track/track4.png").await.unwrap(),
        load_texture("src/track/track5.png").await.unwrap(),
        load_texture("src/track/track6.png").await.unwrap(),
        load_texture("src/track/track7.png").await.unwrap(),
        load_texture("src/track/track8.png").await.unwrap(),
        load_texture("src/track/track9.png").await.unwrap(),
        load_texture("src/track/track10.png").await.unwrap(),
        load_texture("src/track/track11.png").await.unwrap(),
        load_texture("src/track/track12.png").await.unwrap(),
        load_texture("src/track/track13.png").await.unwrap(),
        load_texture("src/track/track14.png").await.unwrap(),
    ];
    for sprite in &track_sprites {
        sprite.set_filter(FilterMode::Linear);
    }
    let track_coords = vec![
        vec![100.0, 100.0, 300.0, 100.0],
        vec![300.0, 100.0, 100.0, 350.0],
        vec![400.0, 200.0, 100.0, 50.0],
        vec![300.0, 350.0, 300.0, 100.0],
        vec![500.0, 100.0, 100.0, 350.0],
        vec![500.0, 100.0, 450.0, 100.0],
        vec![950.0, 100.0, 150.0, 300.0],
        vec![700.0, 300.0, 250.0, 100.0],
        vec![700.0, 300.0, 100.0, 250.0],
        vec![700.0, 450.0, 400.0, 100.0],
        vec![850.0, 550.0, 50.0, 50.0],
        vec![1000.0, 450.0, 100.0, 250.0],
        vec![100.0, 600.0, 1000.0, 100.0],
        vec![100.0, 100.0, 100.0, 600.0],
    ];

    // ice texture and coords
    let ice_texture = load_texture("src/track/ice.png").await.unwrap();
    ice_texture.set_filter(FilterMode::Linear);
    let ice_coords = vec![
        vec![105.0, 305.0],
        vec![135.0, 505.0],
        vec![105.0, 635.0],
        vec![235.0, 605.0],
        vec![435.0, 635.0],
        vec![635.0, 605.0],
    ];

    //Pause menu image
    let pause = load_texture("src/pause_menu/1.png").await.unwrap();
    pause.set_filter(FilterMode::Linear);

    // Starry background sprites
    let main_sprites = vec![
        load_texture("src/title/0.png").await.unwrap(),
        load_texture("src/title/1.png").await.unwrap(),load_texture("src/title/2.png").await.unwrap(),
        load_texture("src/title/3.png").await.unwrap(),load_texture("src/title/4.png").await.unwrap(),
        load_texture("src/title/5.png").await.unwrap(),load_texture("src/title/6.png").await.unwrap(),
        load_texture("src/title/7.png").await.unwrap(),load_texture("src/title/8.png").await.unwrap(),
        load_texture("src/title/9.png").await.unwrap(),load_texture("src/title/10.png").await.unwrap(),
        load_texture("src/title/11.png").await.unwrap(),load_texture("src/title/12.png").await.unwrap(),
        load_texture("src/title/13.png").await.unwrap(),load_texture("src/title/14.png").await.unwrap(),
        load_texture("src/title/15.png").await.unwrap(),load_texture("src/title/16.png").await.unwrap(),
        load_texture("src/title/17.png").await.unwrap(),load_texture("src/title/18.png").await.unwrap(),
        load_texture("src/title/19.png").await.unwrap(),load_texture("src/title/20.png").await.unwrap(),
        load_texture("src/title/21.png").await.unwrap(),load_texture("src/title/22.png").await.unwrap(),
        load_texture("src/title/23.png").await.unwrap(),load_texture("src/title/24.png").await.unwrap(),
        load_texture("src/title/25.png").await.unwrap(),load_texture("src/title/26.png").await.unwrap(),
        load_texture("src/title/27.png").await.unwrap(),load_texture("src/title/28.png").await.unwrap(),
        load_texture("src/title/29.png").await.unwrap(),load_texture("src/title/30.png").await.unwrap(),
        load_texture("src/title/31.png").await.unwrap(),load_texture("src/title/32.png").await.unwrap(),
        load_texture("src/title/33.png").await.unwrap(),load_texture("src/title/34.png").await.unwrap(),
        load_texture("src/title/35.png").await.unwrap(),load_texture("src/title/36.png").await.unwrap(),
        load_texture("src/title/37.png").await.unwrap(),load_texture("src/title/38.png").await.unwrap(),
        load_texture("src/title/39.png").await.unwrap(),load_texture("src/title/40.png").await.unwrap(),
        load_texture("src/title/41.png").await.unwrap(),load_texture("src/title/42.png").await.unwrap(),
        load_texture("src/title/43.png").await.unwrap(),load_texture("src/title/44.png").await.unwrap(),
        load_texture("src/title/45.png").await.unwrap(),load_texture("src/title/46.png").await.unwrap(),
        load_texture("src/title/47.png").await.unwrap(),load_texture("src/title/48.png").await.unwrap(),
        load_texture("src/title/49.png").await.unwrap(),load_texture("src/title/50.png").await.unwrap(),
        load_texture("src/title/51.png").await.unwrap(),load_texture("src/title/52.png").await.unwrap(),
        load_texture("src/title/53.png").await.unwrap(),load_texture("src/title/54.png").await.unwrap(),
        load_texture("src/title/55.png").await.unwrap(),load_texture("src/title/56.png").await.unwrap(),
        load_texture("src/title/57.png").await.unwrap(),load_texture("src/title/58.png").await.unwrap(),
        load_texture("src/title/59.png").await.unwrap(),load_texture("src/title/60.png").await.unwrap(),
        load_texture("src/title/61.png").await.unwrap(),load_texture("src/title/62.png").await.unwrap(),
        load_texture("src/title/63.png").await.unwrap(),load_texture("src/title/64.png").await.unwrap(),
        load_texture("src/title/65.png").await.unwrap(),load_texture("src/title/66.png").await.unwrap(),
        load_texture("src/title/67.png").await.unwrap(),load_texture("src/title/68.png").await.unwrap(),
        load_texture("src/title/69.png").await.unwrap(),load_texture("src/title/70.png").await.unwrap(),
        load_texture("src/title/71.png").await.unwrap(),load_texture("src/title/72.png").await.unwrap(),
        load_texture("src/title/73.png").await.unwrap(),load_texture("src/title/74.png").await.unwrap(),
        load_texture("src/title/75.png").await.unwrap(),load_texture("src/title/76.png").await.unwrap(),
        load_texture("src/title/77.png").await.unwrap(),load_texture("src/title/78.png").await.unwrap(),
        load_texture("src/title/79.png").await.unwrap(),load_texture("src/title/80.png").await.unwrap(),
        load_texture("src/title/81.png").await.unwrap(),load_texture("src/title/82.png").await.unwrap(),
        load_texture("src/title/83.png").await.unwrap(),load_texture("src/title/84.png").await.unwrap(),
        load_texture("src/title/85.png").await.unwrap(),load_texture("src/title/86.png").await.unwrap(),
        load_texture("src/title/87.png").await.unwrap(),load_texture("src/title/88.png").await.unwrap(),
        load_texture("src/title/89.png").await.unwrap(),load_texture("src/title/90.png").await.unwrap(),
        load_texture("src/title/91.png").await.unwrap(),load_texture("src/title/92.png").await.unwrap(),
        load_texture("src/title/93.png").await.unwrap(),load_texture("src/title/94.png").await.unwrap(),
        load_texture("src/title/95.png").await.unwrap(),load_texture("src/title/96.png").await.unwrap(),
        load_texture("src/title/97.png").await.unwrap(),load_texture("src/title/98.png").await.unwrap(),
        load_texture("src/title/99.png").await.unwrap()
    ];
    
    for sprite in &main_sprites {
        sprite.set_filter(FilterMode::Linear);
    }

    //Control menu image
    let controls = load_texture("src/pause_menu/controls.png").await.unwrap();
    controls.set_filter(FilterMode::Linear);

    //Start menu textures
    let title_cards = vec![
        load_texture("src/title_cards/Title.png").await.unwrap(),load_texture("src/title_cards/Start-Clicked.png").await.unwrap(),
        load_texture("src/title_cards/Start.png").await.unwrap(),load_texture("src/title_cards/Quit-Clicked.png").await.unwrap(),
        load_texture("src/title_cards/Quit.png").await.unwrap()
         ];
    
    for sprite in &title_cards {
        sprite.set_filter(FilterMode::Linear);
    }

    let goaltape = load_texture("src/goaltape/finish.png").await.unwrap();
    goaltape.set_filter(FilterMode::Linear);

    let countdown_anime = vec![
        load_texture("src/countdown/3.png").await.unwrap(),load_texture("src/countdown/2.png").await.unwrap(),
        load_texture("src/countdown/1.png").await.unwrap()
    ];

    for sprite in &countdown_anime {
        sprite.set_filter(FilterMode::Linear);
    }

    let win_screen = vec![
        load_texture("src/win_screen/0.png").await.unwrap(),
        load_texture("src/win_screen/1.png").await.unwrap(),load_texture("src/win_screen/2.png").await.unwrap(),
        load_texture("src/win_screen/3.png").await.unwrap(),load_texture("src/win_screen/4.png").await.unwrap(),
        load_texture("src/win_screen/5.png").await.unwrap(),load_texture("src/win_screen/6.png").await.unwrap(),
        load_texture("src/win_screen/7.png").await.unwrap(),load_texture("src/win_screen/8.png").await.unwrap(),
        load_texture("src/win_screen/9.png").await.unwrap(),load_texture("src/win_screen/10.png").await.unwrap(),
        load_texture("src/win_screen/11.png").await.unwrap(),load_texture("src/win_screen/12.png").await.unwrap(),
        load_texture("src/win_screen/13.png").await.unwrap(),load_texture("src/win_screen/14.png").await.unwrap(),
        load_texture("src/win_screen/15.png").await.unwrap(),load_texture("src/win_screen/16.png").await.unwrap(),
        load_texture("src/win_screen/17.png").await.unwrap(),load_texture("src/win_screen/18.png").await.unwrap(),
        load_texture("src/win_screen/19.png").await.unwrap(),load_texture("src/win_screen/20.png").await.unwrap(),
        load_texture("src/win_screen/21.png").await.unwrap(),load_texture("src/win_screen/22.png").await.unwrap(),
        load_texture("src/win_screen/23.png").await.unwrap(),load_texture("src/win_screen/24.png").await.unwrap(),
        load_texture("src/win_screen/25.png").await.unwrap(),load_texture("src/win_screen/26.png").await.unwrap(),
        load_texture("src/win_screen/27.png").await.unwrap(),load_texture("src/win_screen/28.png").await.unwrap(),
        load_texture("src/win_screen/29.png").await.unwrap(),load_texture("src/win_screen/30.png").await.unwrap(),
    ];

    for sprite in &win_screen {
        sprite.set_filter(FilterMode::Linear);
    }
    
    // Import mario font
    let font_hold = load_ttf_font("src/font/1.ttf").await.unwrap();
    let font = Some(&font_hold);

      //Helper variable for mute function
      let mut songcount = 0;
      let intro_bgm = audio::load_sound("src/soundtracks/race-intro.wav").await.unwrap();
      let countdown_bgm = audio::load_sound("src/soundtracks/countdown_audio.wav").await.unwrap();
      let game_bgm = audio::load_sound("src/soundtracks/rainbow_road.wav").await.unwrap();  
      let win_bgm = audio::load_sound("src/soundtracks/win_music.wav").await.unwrap();  

    loop {
        let delta_time = get_frame_time();

        if game_state == GameState::MainMenu {
            
            // Play background music
            if play_title_audio {
            audio::play_sound(&intro_bgm, PlaySoundParams {looped: true, volume: 1.,});
            play_title_audio = false;
            }


            // Check if either button is pressed, highlight corresponding menu button
            if is_key_pressed(KeyCode::Left) {
                start_button = 1;
                quit_button = 4;
            } else if is_key_pressed(KeyCode::Right) {
                start_button = 2;
                quit_button = 3;
            }

            // Draw menu background
            draw_texture_ex(
                &main_sprites[(main_sprite_counter / 12) % 99],
                0.,
                0.,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(screen_width, screen_height)),
                    ..Default::default()}
            );

            draw_text_ex("Use arrow keys and enter to navigate menu", 10., 20., TextParams {font_size: 15, font, ..Default::default()});

            // Draw title
            draw_texture_ex(
                &title_cards[0],
                114.4,
                -25.,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(screen_width/1.3, screen_height/1.3)),
                    ..Default::default()}
            );

            // Draw appropriate start and quit buttons
            draw_texture_ex(
                &title_cards[start_button],
                280.,
                screen_height/1.6,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(270., 200.)),
                    ..Default::default()}
            );
            draw_texture_ex(
                &title_cards[quit_button],
                650.,
                screen_height/1.6,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(270., 200.)),
                    ..Default::default()}
            );

            // Update frame count for sprites
            main_sprite_counter += 1;

            // Menu actions
            if is_key_pressed(KeyCode::Enter) && start_button == 1 {
                game_state = GameState::Game;
                loop {
                    // Draw menu background
                    draw_texture_ex(
                        &main_sprites[(main_sprite_counter/12) % 99],
                        0.,
                        0.,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(vec2(screen_width, screen_height)),
                            ..Default::default()}
                    );

                    main_sprite_counter += 1;

                    draw_texture_ex(
                        &controls,
                        screen_width/4.,
                        screen_height/4.,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(vec2(screen_width/2., screen_height/2.)),
                            ..Default::default()}
                    );

                    // Make it so you have to look at the controls for a set amount of time
                    controls_timer += 1;

                    if is_key_pressed(KeyCode::Enter) && controls_timer >= 50 {
                        stop_sound(&intro_bgm);
                        break;
                    }

                    next_frame().await
                }
            } else if is_key_pressed(KeyCode::Enter) && quit_button == 3 {
                exit(0)
            }
            next_frame().await
        
        // Play Game State
        } else if game_state == GameState::Game {

            draw(&background_sprites, &track_sprites, &track_coords, &ice_texture, &ice_coords, sprite_counter, screen_width, screen_height, dev_mode, &walls, &cars, &car_sprites, &goaltape);
            //pause info text
            draw_text_ex("Press p to pause", 10., 20., TextParams {font_size: 15, font, ..Default::default()});

            if countdown == true {
                
                // Play background music
                audio::play_sound(&countdown_bgm, PlaySoundParams {looped: false, volume: 1.,});
                
                let mut countdown_timer = 0;

                loop {
                    draw(&background_sprites, &track_sprites, &track_coords, &ice_texture, &ice_coords, sprite_counter, screen_width, screen_height, dev_mode, &walls, &cars, &car_sprites, &goaltape);

                    //pause info text
                    draw_text_ex("Press p to pause", 10., 20., TextParams {font_size: 15, font, ..Default::default()});

                    draw_texture_ex(
                        &countdown_anime[(countdown_timer/68) % 3],
                        screen_width/3.,
                        screen_height/3.,
                        BROWN,
                        DrawTextureParams {
                            dest_size: Some(vec2(screen_width/3., screen_height/3.)),
                            ..Default::default()}
                    );

                    countdown_timer += 1;
                    if countdown_timer == 204 {
                        countdown = false;
                        break
                    }
                    next_frame().await
                }
            }

            // Play background music
            if play_game_audio {
                audio::play_sound(&game_bgm, PlaySoundParams {looped: true, volume: 1.,});
                play_game_audio = false;
            }

            for car in &mut cars {
                // UPDATE

                draw_text_ex(format!("{}:  lap  {}", car.name, car.lap).as_str(), car.string_pos, 20., TextParams {font_size: 15, font, ..Default::default()});
 
                if car.spinning_out > 0 {
                    car.move_dir = car.vy.atan2(car.vx).to_degrees();
                    car.dir += 540.0 / 40.0;
                    car.vx = 100.0 * car.move_dir.to_radians().cos();
                    car.vy = 100.0 * car.move_dir.to_radians().sin();

                    car.spinning_out -= 1;
                } else {
                    //Car movement check, collision check, and rotation
                    if is_key_down(car.keys[0]) {
                        car.vx += car.accel * car.dir.to_radians().cos() * delta_time;
                        car.vy += car.accel * car.dir.to_radians().sin() * delta_time;
                    } else if is_key_down(car.keys[1]) {
                        car.vx -= car.accel * car.dir.to_radians().cos() * delta_time;
                        car.vy -= car.accel * car.dir.to_radians().sin() * delta_time;
                    } else {
                        car.vx *= car.friction;
                        car.vy *= car.friction;
                    }
                    car.speed = (car.vx * car.vx + car.vy * car.vy * delta_time).sqrt();
                    if car.speed > car.max_speed {
                        car.move_dir = car.vy.atan2(car.vx).to_degrees();
                        car.vx = car.max_speed * car.move_dir.to_radians().cos();
                        car.vy = car.max_speed * car.move_dir.to_radians().sin();
                    }

                    if is_key_down(car.keys[2]) {
                        car.dir -= car.turn_speed * delta_time;
                    }
                    if is_key_down(car.keys[3]) {
                        car.dir += car.turn_speed * delta_time;
                    }

                    // ice collision
                    for ice in &ice_coords {
                        if vec2(car.x, car.y).distance(vec2(ice[0] + 30.0, ice[1] + 30.0)) <= 30.0 {
                            car.spinning_out = 40;
                        }
                    }
                }

                car.move_dir = car.vy.atan2(car.vx).to_degrees();
                car.x += car.vx * delta_time;
                car.y += car.vy * delta_time;

                // Track Collision
                for wall in &walls {
                    if car.collides(wall) {
                        match wall.dir {
                            WallType::Top => { car.vy *= -0.8; car.y += 1.; },
                            WallType::Bottom => { car.vy *= -0.8; car.y -= 1.; }
                            WallType::Left => { car.vx *= -0.8; car.x += 1.; },
                            WallType::Right => { car.vx *= -0.8; car.x -= 1.; },
                        }
                    }
                }

                // Track Collision for arc - inner circle
                let distance = vec2(car.x, car.y).distance(vec2(950.0, 250.0));
                if distance <= car.radius + 50.0 && car.x >= 950.0 {
                    let collision = vec2(car.x - 950.0, car.y - 250.0);
                    let collision_norm = vec2(collision.x / distance, collision.y / distance);
                    let rel_vel = vec2(car.vx * -1., car.vy * -1.);
                    let speed = rel_vel.x * collision_norm.x + rel_vel.y * collision_norm.y;

                    if speed >= 0.0 {
                        car.vx += speed * collision_norm.x * 7.;
                        car.vy += speed * collision_norm.y * 7.;
                    }
                }
                
                // Track Collision for arc - outer circle
                let distance = vec2(car.x, car.y).distance(vec2(950.0, 250.0));
                if distance >= 149.0 - car.radius && distance <= 151.0 + car.radius && car.x >= 950.0 {
                    let collision = vec2(car.x - 950.0, car.y - 250.0);
                    let collision_norm = vec2(collision.x / distance, collision.y / distance);
                    let rel_vel = vec2(car.vx, car.vy);
                    let speed = rel_vel.x * collision_norm.x + rel_vel.y * collision_norm.y;

                    if speed >= 0.0 {
                        car.vx -= speed * collision_norm.x * 7.;
                        car.vy -= speed * collision_norm.y * 7.;
                    }
                }

                //Set minimums and maximums on car boundaries and acceleration
                if car.x < car.radius {
                    car.x = car.radius;
                } else if car.x > screen_width - car.radius {
                    car.x = screen_width - car.radius;
                }
                if car.y < car.radius {
                    car.y = car.radius;
                } else if car.y > screen_height - car.radius {
                    car.y = screen_height - car.radius;
                }
                car.dir %= 360.0;

                //Check collision with checkpoint and goal
                if checkpoint.contains(vec2(car.x,car.y)) {
                    car.can_lap = true;
                } else if goal.contains(vec2(car.x, car.y)) && car.lap == 3 && car.can_lap {
                    win_state = true;
                    win_car.push(car.name.clone());
                }else if goal.contains(vec2(car.x, car.y)) && car.can_lap {
                    car.can_lap = false;
                    car.lap += 1;
                } else if goal.contains(vec2(car.x, car.y)) && car.lap == 3{
                    draw_text_ex(format!("{}'s final lap!", car.name).as_str(), car.string_pos, 50., TextParams {font_size: 20, font, ..Default::default()});
                }
                sprite_counter += 1;
            }

            // Car Collision
            let distance = vec2(cars[0].x, cars[0].y).distance(vec2(cars[1].x, cars[1].y));
            if distance <= cars[0].radius + cars[1].radius {
                let collision = vec2(cars[1].x - cars[0].x, cars[1].y - cars[0].y);
                let collision_norm = vec2(collision.x / distance, collision.y / distance);
                let rel_vel = vec2(cars[0].vx - cars[1].vx, cars[0].vy - cars[1].vy);
                let speed = rel_vel.x * collision_norm.x + rel_vel.y * collision_norm.y;
                let multiplier = 2.0;

                if speed >= 0.0 {
                    cars[0].vx -= speed * collision_norm.x * multiplier;
                    cars[0].vy -= speed * collision_norm.y * multiplier;
                    cars[1].vx += speed * collision_norm.x * multiplier;
                    cars[1].vy += speed * collision_norm.y * multiplier;
                }
            }

            //AUDIO

            //Mute audio check
            if is_key_pressed(KeyCode::M) {
                if songcount%2 == 0 {
                    set_sound_volume(&game_bgm, 0.);
                    songcount += 1;
                } else {
                    set_sound_volume(&game_bgm, 1.);
                    songcount += 1;
                }
            }

            if is_key_pressed(KeyCode::I) {
                cars[0].spinning_out = 40;
                cars[1].spinning_out = 40;
            }

            //Load pause menu
            if is_key_pressed(KeyCode::P) || win_state {
                let hold = pause_menu(&background_sprites, &track_sprites, &track_coords, &ice_texture, &ice_coords, sprite_counter, screen_width, screen_height, dev_mode, &walls, &mut cars.clone(), &car_sprites, &pause, songcount, &game_bgm, win_state, win_car.clone(), &win_screen, &goaltape, font, &mut play_win_audio, &win_bgm).await;
                sprite_counter = hold.0;
                game_state = hold.1.clone();
                win_state = hold.2;
                countdown = hold.3;
                cars = hold.4;
                play_game_audio = hold.5;
                play_title_audio = hold.6;
                play_win_audio = hold.7;
            }


            next_frame().await
        }
    }
}

async fn pause_menu(background_sprites: &Vec<Texture2D>, track_sprites: &Vec<Texture2D>, track_coords: &Vec<Vec<f32>>, ice_texture: &Texture2D, ice_coords: &Vec<Vec<f32>>, mut sprite_counter: usize, screen_width: f32, screen_height: f32, dev_mode: bool, walls: &Vec<Wall>, cars: &mut Vec<Car>, car_sprites: &Vec<Vec<Texture2D>>, pause: &Texture2D, mut songcount:i32, game_bgm:&Sound, win_state:bool, win_car:Vec<String>, win_screen:&Vec<Texture2D>, goaltape: &Texture2D, font:Option<&Font>, play_win_audio:&mut bool, win_bgm:&Sound) -> (usize, GameState, bool, bool, Vec<Car>, bool, bool, bool){
    
    let mut win_screen_counter = 0;

    loop {
        draw(&background_sprites, &track_sprites, &track_coords, &ice_texture, &ice_coords, sprite_counter, screen_width, screen_height, dev_mode, &walls, &cars, &car_sprites, &goaltape);

        if win_state {
            if *play_win_audio {
                stop_sound(&game_bgm);
                audio::play_sound(&win_bgm, PlaySoundParams {looped: true, volume: 1.,});
                *play_win_audio = false;
            }
            draw_texture_ex(
                &win_screen[(win_screen_counter/5) % 31],
                screen_width/4.,
                screen_height/4.,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(screen_width/2., screen_height/2.)),
                    ..Default::default()
                },
            );
        } else {
            draw_texture_ex(
                &pause,
                screen_width/4.,
                screen_height/4.,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(screen_width/2., screen_height/2.)),
                    ..Default::default()
                },
            );
        }

        if win_state {
            draw_text_ex(format!("{} WINS!!!", win_car[0]).as_str(), 400., 75., TextParams {font_size: 45, font, ..Default::default()});
        }

        //Mute audio check
        if is_key_pressed(KeyCode::M) {
            if songcount%2 == 0 {
                set_sound_volume(&game_bgm, 0.);
                songcount += 1;
            } else {
                set_sound_volume(&game_bgm, 1.);
                songcount += 1;
            }
        }

        sprite_counter += 1;
        next_frame().await;
        if is_key_pressed(KeyCode::P) && win_state != true {
            return (sprite_counter, GameState::Game, false, false, cars.clone(), false, false, true)
        } else if is_key_pressed(KeyCode::Q) && win_state != true {
            cars[0].x = 125.0;
            cars[0].y = 240.0;
            cars[0].vx = 0.0;
            cars[0].vy = 0.0;
            cars[0].speed = 0.0;
            cars[0].dir = -89.0;
            cars[0].move_dir = 0.0;
            cars[0].lap = 1;
            cars[1].x = 175.0;
            cars[1].y = 275.0;
            cars[1].vx = 0.0;
            cars[1].vy = 0.0;
            cars[1].speed = 0.0;
            cars[1].dir = -89.0;
            cars[1].move_dir = 0.0;
            cars[1].lap = 1;
            stop_sound(&game_bgm);
            return (sprite_counter, GameState::MainMenu, false, true, cars.clone(), true, true, true)
        } else if is_key_pressed(KeyCode::Enter) && win_state == true {
            cars[0].x = 125.0;
            cars[0].y = 240.0;
            cars[0].vx = 0.0;
            cars[0].vy = 0.0;
            cars[0].speed = 0.0;
            cars[0].dir = -89.0;
            cars[0].move_dir = 0.0;
            cars[0].lap = 1;
            cars[1].x = 175.0;
            cars[1].y = 275.0;
            cars[1].vx = 0.0;
            cars[1].vy = 0.0;
            cars[1].speed = 0.0;
            cars[1].dir = -89.0;
            cars[1].move_dir = 0.0;
            cars[1].lap = 1;
            stop_sound(&game_bgm);
            stop_sound(&win_bgm);
            return (sprite_counter, GameState::MainMenu, false, true, cars.clone(), true, true, true)
        }
        win_screen_counter += 1;
    }
}

fn draw(background_sprites: &Vec<Texture2D>, track_sprites: &Vec<Texture2D>, track_coords: &Vec<Vec<f32>>, ice_texture: &Texture2D, ice_coords: &Vec<Vec<f32>>, sprite_counter: usize, screen_width: f32, screen_height: f32, dev_mode: bool, walls: &Vec<Wall>, cars: &Vec<Car>, car_sprites: &Vec<Vec<Texture2D>>, goaltape: &Texture2D) -> () {
    //Animate starry background
    let thickness = 2.0;
    draw_texture_ex(
        &background_sprites[(sprite_counter/5) % 22],
        0.,
        0.,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(screen_width, screen_height)),
            ..Default::default()
        },
    );

    // tracks
    for (i, sprite) in track_sprites.iter().enumerate() {
        draw_texture_ex(
            &sprite,
            track_coords[i][0],
            track_coords[i][1],
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(track_coords[i][2], track_coords[i][3])),
                ..Default::default()
            },
        );
    }

    // ice
    for ice in ice_coords {
        draw_texture_ex(
            &ice_texture,
            ice[0],
            ice[1],
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(60.0, 60.0)),
                ..Default::default()
            },
        );
    }

    // dev lines
    if dev_mode {
        let mut x = 100.0;
        while x < screen_width {
            draw_line(x, 0.0, x, screen_height, 1.0, GRAY);
            x += 100.0;
        }
        let mut y = 100.0;
        while y < screen_height {
            draw_line(0.0, y, screen_width, y, 1.0, GRAY);
            y += 100.0;
        }
    }

    // Draw goaltape
    draw_texture_ex(
        &goaltape,
        103.,
        182.,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(94., 50.)),
            ..Default::default()}
            );
        

    // outlines
    for wall in walls {
        match wall.dir {
            WallType::Top | WallType::Bottom =>
                draw_line(wall.start-5., wall.level, wall.end+5., wall.level, thickness, BLACK),
            _ => draw_line(wall.level, wall.start-5., wall.level, wall.end+5., thickness, BLACK),
        }
    }

    for car in cars{
        //Draw cars and update position to circle collision box position
        let car_vec;
        if (car.dir >= 90.0 && car.dir <= 270.0)
        || (car.dir <= -90.0 && car.dir >= -270.0) {
            car_vec = 1;
        } else {
            car_vec = 0;
        }
        draw_texture_ex(
            &car_sprites[car_vec][(sprite_counter/20) % 4],
            car.x - 20.,
            car.y - 20.,
            car.color,
            DrawTextureParams {
                dest_size: Some(vec2(car.radius * 2.6,car.radius * 2.6)),
                rotation: (car.dir + ((180 * car_vec) as f32)).to_radians(),
                ..Default::default()
            },
        );
    }
}