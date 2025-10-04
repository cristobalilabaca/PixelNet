use macroquad::prelude::*;

const WALLS: [Rect; 53] = [
    Rect {
        x: 0.,
        y: 0.,
        w: 4.,
        h: 80.,
    },
    Rect {
        x: 0.,
        y: 0.,
        w: 224.,
        h: 4.,
    },
    Rect {
        x: 108.,
        y: 0.,
        w: 8.,
        h: 36.,
    },
    Rect {
        x: 220.,
        y: 0.,
        w: 4.,
        h: 80.,
    },
    Rect {
        x: 20.,
        y: 20.,
        w: 24.,
        h: 16.,
    },
    Rect {
        x: 60.,
        y: 20.,
        w: 32.,
        h: 16.,
    },
    Rect {
        x: 132.,
        y: 20.,
        w: 32.,
        h: 16.,
    },
    Rect {
        x: 180.,
        y: 20.,
        w: 24.,
        h: 16.,
    },
    Rect {
        x: 20.,
        y: 52.,
        w: 24.,
        h: 8.,
    },
    Rect {
        x: 60.,
        y: 52.,
        w: 8.,
        h: 56.,
    },
    Rect {
        x: 84.,
        y: 52.,
        w: 56.,
        h: 8.,
    },
    Rect {
        x: 108.,
        y: 52.,
        w: 8.,
        h: 32.,
    },
    Rect {
        x: 156.,
        y: 52.,
        w: 8.,
        h: 56.,
    },
    Rect {
        x: 180.,
        y: 52.,
        w: 24.,
        h: 8.,
    },
    Rect {
        x: 0.,
        y: 76.,
        w: 44.,
        h: 4.,
    },
    Rect {
        x: 40.,
        y: 76.,
        w: 4.,
        h: 32.,
    },
    Rect {
        x: 60.,
        y: 76.,
        w: 32.,
        h: 8.,
    },
    Rect {
        x: 132.,
        y: 76.,
        w: 32.,
        h: 8.,
    },
    Rect {
        x: 180.,
        y: 76.,
        w: 4.,
        h: 32.,
    },
    Rect {
        x: 180.,
        y: 76.,
        w: 44.,
        h: 4.,
    },
    Rect {
        x: 0.,
        y: 104.,
        w: 44.,
        h: 4.,
    },
    Rect {
        x: 180.,
        y: 104.,
        w: 44.,
        h: 4.,
    },
    Rect {
        x: 84.,
        y: 100.,
        w: 56.,
        h: 4.,
    },
    Rect {
        x: 84.,
        y: 100.,
        w: 4.,
        h: 32.,
    },
    Rect {
        x: 84.,
        y: 128.,
        w: 56.,
        h: 4.,
    },
    Rect {
        x: 136.,
        y: 100.,
        w: 4.,
        h: 32.,
    },
    Rect {
        x: 0.,
        y: 124.,
        w: 44.,
        h: 4.,
    },
    Rect {
        x: 40.,
        y: 124.,
        w: 4.,
        h: 32.,
    },
    Rect {
        x: 60.,
        y: 124.,
        w: 8.,
        h: 32.,
    },
    Rect {
        x: 156.,
        y: 124.,
        w: 8.,
        h: 32.,
    },
    Rect {
        x: 180.,
        y: 124.,
        w: 4.,
        h: 32.,
    },
    Rect {
        x: 180.,
        y: 124.,
        w: 44.,
        h: 4.,
    },
    Rect {
        x: 84.,
        y: 148.,
        w: 56.,
        h: 8.,
    },
    Rect {
        x: 108.,
        y: 148.,
        w: 8.,
        h: 32.,
    },
    Rect {
        x: 0.,
        y: 152.,
        w: 44.,
        h: 4.,
    },
    Rect {
        x: 0.,
        y: 152.,
        w: 4.,
        h: 96.,
    },
    Rect {
        x: 180.,
        y: 152.,
        w: 44.,
        h: 4.,
    },
    Rect {
        x: 220.,
        y: 152.,
        w: 4.,
        h: 96.,
    },
    Rect {
        x: 20.,
        y: 172.,
        w: 24.,
        h: 8.,
    },
    Rect {
        x: 36.,
        y: 172.,
        w: 8.,
        h: 32.,
    },
    Rect {
        x: 60.,
        y: 172.,
        w: 32.,
        h: 8.,
    },
    Rect {
        x: 132.,
        y: 172.,
        w: 32.,
        h: 8.,
    },
    Rect {
        x: 180.,
        y: 172.,
        w: 8.,
        h: 32.,
    },
    Rect {
        x: 180.,
        y: 172.,
        w: 24.,
        h: 8.,
    },
    Rect {
        x: 0.,
        y: 196.,
        w: 20.,
        h: 8.,
    },
    Rect {
        x: 60.,
        y: 196.,
        w: 8.,
        h: 32.,
    },
    Rect {
        x: 84.,
        y: 196.,
        w: 56.,
        h: 8.,
    },
    Rect {
        x: 108.,
        y: 196.,
        w: 8.,
        h: 32.,
    },
    Rect {
        x: 156.,
        y: 196.,
        w: 8.,
        h: 32.,
    },
    Rect {
        x: 204.,
        y: 196.,
        w: 20.,
        h: 8.,
    },
    Rect {
        x: 20.,
        y: 220.,
        w: 72.,
        h: 8.,
    },
    Rect {
        x: 132.,
        y: 220.,
        w: 72.,
        h: 8.,
    },
    Rect {
        x: 0.,
        y: 244.,
        w: 224.,
        h: 4.,
    },
];

const PACMAN_SPRITES: [[Rect; 3]; 4] = [
    [
        Rect {
            x: 457.,
            y: 1.,
            w: 13.,
            h: 13.,
        },
        Rect {
            x: 473.,
            y: 1.,
            w: 13.,
            h: 13.,
        },
        Rect {
            x: 489.,
            y: 1.,
            w: 13.,
            h: 13.,
        },
    ],
    [
        Rect {
            x: 457.,
            y: 16.,
            w: 13.,
            h: 13.,
        },
        Rect {
            x: 473.,
            y: 16.,
            w: 13.,
            h: 13.,
        },
        Rect {
            x: 489.,
            y: 1.,
            w: 13.,
            h: 13.,
        },
    ],
    [
        Rect {
            x: 457.,
            y: 32.,
            w: 13.,
            h: 13.,
        },
        Rect {
            x: 473.,
            y: 32.,
            w: 13.,
            h: 13.,
        },
        Rect {
            x: 489.,
            y: 1.,
            w: 13.,
            h: 13.,
        },
    ],
    [
        Rect {
            x: 457.,
            y: 48.,
            w: 13.,
            h: 13.,
        },
        Rect {
            x: 473.,
            y: 48.,
            w: 13.,
            h: 13.,
        },
        Rect {
            x: 489.,
            y: 1.,
            w: 13.,
            h: 13.,
        },
    ],
];

const RED_GOST_SPRITES: [[Rect; 2]; 4] = [
    [
        Rect {
            x: 457.,
            y: 65.,
            w: 14.,
            h: 14.,
        },
        Rect {
            x: 473.,
            y: 65.,
            w: 14.,
            h: 14.,
        },
    ],
    [
        Rect {
            x: 489.,
            y: 65.,
            w: 14.,
            h: 14.,
        },
        Rect {
            x: 505.,
            y: 65.,
            w: 14.,
            h: 14.,
        },
    ],
    [
        Rect {
            x: 521.,
            y: 65.,
            w: 14.,
            h: 14.,
        },
        Rect {
            x: 537.,
            y: 65.,
            w: 14.,
            h: 14.,
        },
    ],
    [
        Rect {
            x: 553.,
            y: 65.,
            w: 14.,
            h: 14.,
        },
        Rect {
            x: 569.,
            y: 65.,
            w: 14.,
            h: 14.,
        },
    ],
];

const PINK_GHOST_SPRITES: [[Rect; 2]; 4] = [
    [
        Rect {
            x: 457.,
            y: 81.,
            w: 14.,
            h: 14.,
        },
        Rect {
            x: 473.,
            y: 81.,
            w: 14.,
            h: 14.,
        },
    ],
    [
        Rect {
            x: 489.,
            y: 81.,
            w: 14.,
            h: 14.,
        },
        Rect {
            x: 505.,
            y: 81.,
            w: 14.,
            h: 14.,
        },
    ],
    [
        Rect {
            x: 521.,
            y: 81.,
            w: 14.,
            h: 14.,
        },
        Rect {
            x: 537.,
            y: 81.,
            w: 14.,
            h: 14.,
        },
    ],
    [
        Rect {
            x: 553.,
            y: 81.,
            w: 14.,
            h: 14.,
        },
        Rect {
            x: 569.,
            y: 81.,
            w: 14.,
            h: 14.,
        },
    ],
];

const BLUE_GHOST_SPRITES: [[Rect; 2]; 4] = [
    [
        Rect {
            x: 457.,
            y: 97.,
            w: 14.,
            h: 14.,
        },
        Rect {
            x: 473.,
            y: 97.,
            w: 14.,
            h: 14.,
        },
    ],
    [
        Rect {
            x: 489.,
            y: 97.,
            w: 14.,
            h: 14.,
        },
        Rect {
            x: 505.,
            y: 97.,
            w: 14.,
            h: 14.,
        },
    ],
    [
        Rect {
            x: 521.,
            y: 97.,
            w: 14.,
            h: 14.,
        },
        Rect {
            x: 537.,
            y: 97.,
            w: 14.,
            h: 14.,
        },
    ],
    [
        Rect {
            x: 553.,
            y: 97.,
            w: 14.,
            h: 14.,
        },
        Rect {
            x: 569.,
            y: 97.,
            w: 14.,
            h: 14.,
        },
    ],
];

const ORANGE_GHOST_SPRITES: [[Rect; 2]; 4] = [
    [
        Rect {
            x: 457.,
            y: 113.,
            w: 14.,
            h: 14.,
        },
        Rect {
            x: 473.,
            y: 113.,
            w: 14.,
            h: 14.,
        },
    ],
    [
        Rect {
            x: 489.,
            y: 113.,
            w: 14.,
            h: 14.,
        },
        Rect {
            x: 505.,
            y: 113.,
            w: 14.,
            h: 14.,
        },
    ],
    [
        Rect {
            x: 521.,
            y: 113.,
            w: 14.,
            h: 14.,
        },
        Rect {
            x: 537.,
            y: 113.,
            w: 14.,
            h: 14.,
        },
    ],
    [
        Rect {
            x: 553.,
            y: 113.,
            w: 14.,
            h: 14.,
        },
        Rect {
            x: 569.,
            y: 113.,
            w: 14.,
            h: 14.,
        },
    ],
];

const PACMAN_DIAMETER: f32 = 30.;

macro_rules! convert_sprites {
    ($sprites:expr) => {
        $sprites
            .iter()
            .map(|direction| direction.to_vec())
            .collect()
    };
}

const BACKGROUND_SPRITE: Rect = Rect {
    x: 0.,
    y: 0.,
    w: 225.,
    h: 248.,
};

#[derive(Copy, Clone)]
enum Direction {
    Right = 0,
    Left = 1,
    Up = 2,
    Down = 3,
}

struct Character {
    pos: Rect,
    direction: Direction,
    next_direction: Direction,
    movements: u16,
    current_rect: usize,
    sprites: Vec<Vec<Rect>>,
}

fn window_conf() -> Conf {
    Conf {
        window_title: "PixelNet".to_owned(),
        window_width: 450,
        window_height: 496,
        fullscreen: false,
        icon: None,
        ..Default::default()
    }
}

fn are_objects_colliding(obj1: Rect, obj2: Rect) -> bool {
    return obj1.x <= obj2.x + obj2.w
        && obj1.x + obj1.w >= obj2.x
        && obj1.y <= obj2.y + obj2.h
        && obj1.y + obj1.h >= obj2.y;
}

fn colliding_with_wall(pacman: Rect) -> bool {
    let window_width: i32 = window_conf().window_width;
    let window_height: i32 = window_conf().window_height;

    for wall in WALLS {
        let wall_x = wall.x * window_width as f32 / BACKGROUND_SPRITE.w;
        let wall_y = wall.y * window_height as f32 / BACKGROUND_SPRITE.h;
        let wall_h = wall.h * window_height as f32 / BACKGROUND_SPRITE.h;
        let wall_w = wall.w * window_width as f32 / BACKGROUND_SPRITE.w;

        let wall_rect = Rect {
            x: wall_x,
            y: wall_y,
            w: wall_w,
            h: wall_h,
        };

        if are_objects_colliding(pacman, wall_rect) {
            return true;
        }
    }
    return false;
}

#[macroquad::main(window_conf)]
async fn main() {
    let texture: Texture2D = load_texture("assets/sprites.png").await.unwrap();

    let mut pacman: Character = Character {
        pos: Rect {
            x: 212.,
            y: 361.,
            w: PACMAN_DIAMETER,
            h: PACMAN_DIAMETER,
        },
        direction: Direction::Right,
        next_direction: Direction::Right,
        movements: 0,
        current_rect: 0,
        sprites: convert_sprites!(PACMAN_SPRITES),
    };
    let ghosts: [Character; 4] = [
        Character {
            pos: Rect {
                x: 209.,
                y: 169.,
                w: PACMAN_DIAMETER,
                h: PACMAN_DIAMETER,
            },
            direction: Direction::Left,
            next_direction: Direction::Left,
            movements: 0,
            current_rect: 0,
            sprites: convert_sprites!(RED_GOST_SPRITES),
        },
        Character {
            pos: Rect {
                x: 177.,
                y: 224.,
                w: PACMAN_DIAMETER,
                h: PACMAN_DIAMETER,
            },
            direction: Direction::Left,
            next_direction: Direction::Left,
            movements: 0,
            current_rect: 0,
            sprites: convert_sprites!(PINK_GHOST_SPRITES),
        },
        Character {
            pos: Rect {
                x: 209.,
                y: 208.,
                w: PACMAN_DIAMETER,
                h: PACMAN_DIAMETER,
            },
            direction: Direction::Left,
            next_direction: Direction::Left,
            movements: 0,
            current_rect: 0,
            sprites: convert_sprites!(BLUE_GHOST_SPRITES),
        },
        Character {
            pos: Rect {
                x: 241.,
                y: 224.,
                w: PACMAN_DIAMETER,
                h: PACMAN_DIAMETER,
            },
            direction: Direction::Left,
            next_direction: Direction::Left,
            movements: 0,
            current_rect: 0,
            sprites: convert_sprites!(ORANGE_GHOST_SPRITES),
        },
    ];
    let mut moved: bool;

    loop {
        moved = false;
        draw_texture_ex(
            &texture,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(450., 496.)),
                source: Some(BACKGROUND_SPRITE),
                ..Default::default()
            },
        );
        draw_texture_ex(
            &texture,
            pacman.pos.x,
            pacman.pos.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(PACMAN_DIAMETER, PACMAN_DIAMETER)),
                source: Some(pacman.sprites[pacman.direction as usize][pacman.current_rect]),
                ..Default::default()
            },
        );

        for ghost in ghosts.iter() {
            draw_texture_ex(
                &texture,
                ghost.pos.x,
                ghost.pos.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(PACMAN_DIAMETER, PACMAN_DIAMETER)),
                    source: Some(ghost.sprites[ghost.direction as usize][ghost.current_rect]),
                    ..Default::default()
                },
            )
        }

        if is_key_down(KeyCode::Escape) || is_key_down(KeyCode::Q) {
            return;
        }

        let mut pacman_pos_clone = pacman.pos.clone();
        match pacman.next_direction {
            Direction::Up => {
                pacman_pos_clone.y -= 1.;
                if !colliding_with_wall(pacman_pos_clone) {
                    pacman.pos.y -= 1.;
                    pacman.movements += 1;
                    moved = true;
                    pacman.direction = Direction::Up;
                }
            }
            Direction::Down => {
                pacman_pos_clone.y += 1.;
                if !colliding_with_wall(pacman_pos_clone) {
                    pacman.pos.y += 1.;
                    pacman.movements += 1;
                    moved = true;
                    pacman.direction = Direction::Down;
                }
            }
            Direction::Left => {
                pacman_pos_clone.x -= 1.;
                if !colliding_with_wall(pacman_pos_clone) {
                    pacman.pos.x -= 1.;
                    pacman.movements += 1;
                    moved = true;
                    pacman.direction = Direction::Left;
                }
            }
            Direction::Right => {
                pacman_pos_clone.x += 1.;
                if !colliding_with_wall(pacman_pos_clone) {
                    pacman.pos.x += 1.;
                    pacman.movements += 1;
                    moved = true;
                    pacman.direction = Direction::Right;
                }
            }
        }

        if !moved {
            pacman_pos_clone = pacman.pos.clone();
            match pacman.direction {
                Direction::Up => {
                    pacman_pos_clone.y -= 1.;
                    if !colliding_with_wall(pacman_pos_clone) {
                        pacman.pos.y -= 1.;
                        pacman.movements += 1;
                    }
                }
                Direction::Down => {
                    pacman_pos_clone.y += 1.;
                    if !colliding_with_wall(pacman_pos_clone) {
                        pacman.pos.y += 1.;
                        pacman.movements += 1;
                    }
                }
                Direction::Left => {
                    pacman_pos_clone.x -= 1.;
                    if !colliding_with_wall(pacman_pos_clone) {
                        pacman.pos.x -= 1.;
                        pacman.movements += 1;
                    }
                }
                Direction::Right => {
                    pacman_pos_clone.x += 1.;
                    if !colliding_with_wall(pacman_pos_clone) {
                        pacman.pos.x += 1.;
                        pacman.movements += 1;
                    }
                }
            }
        }

        if pacman.pos.x > window_conf().window_width as f32 {
            pacman.pos.x = 0.;
        } else if pacman.pos.x < 0. - PACMAN_DIAMETER {
            pacman.pos.x = window_conf().window_width as f32;
        }

        if is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::D) {
            pacman.next_direction = Direction::Right;
        } else if is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::A) {
            pacman.next_direction = Direction::Left;
        } else if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S) {
            pacman.next_direction = Direction::Down;
        } else if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) {
            pacman.next_direction = Direction::Up;
        }

        if pacman.movements > 13 {
            pacman.current_rect = (pacman.current_rect + 1) % 3;
            pacman.movements = 0;
        }

        next_frame().await
    }
}
