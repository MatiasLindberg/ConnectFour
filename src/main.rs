use macroquad::prelude::*;

const SIZE_Y: usize = 6;
const SIZE_X: usize = 7;
const AI_DEPTH: usize = 6;

#[derive(PartialEq, Clone)]
enum Owned {
    PLAYER,
    AI,
    NOBODY,
}

#[derive(PartialEq, Clone)]
struct Token {
    pos: (f32, f32),
    owned: Owned,
    color: Color,
}

fn set_up_game(tokens: &mut Vec<Vec<Token>>) {
    tokens.clear();
    for j in 0..SIZE_X {
        let mut tmp_tok: Vec<Token> = Vec::new();
        for i in 0..SIZE_Y {
            let tmp: Token = Token {
                pos: ((j * 100 + 80) as f32, (i * 100 + 180) as f32),
                owned: Owned::NOBODY,
                color: BLACK,
            };
            tmp_tok.push(tmp);
        }
        tokens.push(tmp_tok);
    }
}

fn drop_token(token_row: &mut Vec<Token>, player: bool) -> bool {
    if token_row[0].owned != Owned::NOBODY {
        return false;
    }
    let mut pos: usize = 1;
    while pos < SIZE_Y && token_row[pos].owned == Owned::NOBODY {
        pos += 1;
    }
    if player {
        token_row[pos - 1].owned = Owned::PLAYER;
        token_row[pos - 1].color = RED;
    } else {
        token_row[pos - 1].owned = Owned::AI;
        token_row[pos - 1].color = YELLOW;
    }
    true
}

fn drop_back(token_row: &mut Vec<Token>) {
    let mut pos: usize = 0;
    while token_row[pos].owned == Owned::NOBODY {
        pos += 1;
    }
    token_row[pos].owned = Owned::NOBODY;
    token_row[pos].color = BLACK;
}

fn check_victory(tokens: &Vec<Vec<Token>>, owner: Owned) -> bool {
    for y in 0..SIZE_Y {
        for x in 0..SIZE_X {
            if tokens[x][y].owned == owner {
                if x + 3 < SIZE_X
                    && tokens[x + 1][y].owned == owner
                    && tokens[x + 2][y].owned == owner
                    && tokens[x + 3][y].owned == owner
                {
                    return true;
                }

                if y + 3 < SIZE_Y
                    && tokens[x][y + 1].owned == owner
                    && tokens[x][y + 2].owned == owner
                    && tokens[x][y + 3].owned == owner
                {
                    return true;
                }

                if x + 3 < SIZE_X
                    && y + 3 < SIZE_Y
                    && tokens[x + 1][y + 1].owned == owner
                    && tokens[x + 2][y + 2].owned == owner
                    && tokens[x + 3][y + 3].owned == owner
                {
                    return true;
                }

                if x >= 3
                    && y + 3 < SIZE_Y
                    && tokens[x - 1][y + 1].owned == owner
                    && tokens[x - 2][y + 2].owned == owner
                    && tokens[x - 3][y + 3].owned == owner
                {
                    return true;
                }
            }
        }
    }
    false
}

fn possible_drops(tokens: &Vec<Vec<Token>>) -> Vec<usize> {
    (0..SIZE_X)
        .filter(|&col| tokens[col][0].owned == Owned::NOBODY)
        .collect()
}

fn eval(tokens: &Vec<Vec<Token>>) -> i32 {
    if check_victory(tokens, Owned::AI) {
        100
    } else if check_victory(tokens, Owned::PLAYER) {
        -100
    } else if possible_drops(tokens).is_empty() {
        -10
    } else {
        0
    }
}

fn minmax(tokens: &mut Vec<Vec<Token>>, depth: usize, alpha: i32, beta: i32, ai: bool) -> i32 {
    match eval(tokens) {
        100 => return 100,
        -100 => return -100,
        -10 => return -10,
        _ => {
            if depth >= AI_DEPTH {
                return 0;
            }
        }
    }

    let mut alpha: i32 = alpha;
    let mut beta: i32 = beta;

    if ai {
        let mut max_eval = i32::MIN;
        for col in possible_drops(tokens) {
            drop_token(&mut tokens[col], false);
            let eval = minmax(tokens, depth + 1, alpha, beta, false);
            drop_back(&mut tokens[col]);
            max_eval = std::cmp::max(max_eval, eval);
            alpha = std::cmp::max(alpha, eval);
            if beta <= alpha {
                break;
            }
        }
        max_eval
    } else {
        let mut min_eval = i32::MAX;
        for col in possible_drops(tokens) {
            drop_token(&mut tokens[col], true);
            let eval = minmax(tokens, depth + 1, alpha, beta, true);
            drop_back(&mut tokens[col]);
            min_eval = std::cmp::min(min_eval, eval);
            beta = std::cmp::min(beta, eval);
            if beta <= alpha {
                break;
            }
        }
        min_eval
    }
}

fn ai_move(tokens: &mut Vec<Vec<Token>>) {
    let mut best_score: i32 = i32::MIN;
    let mut best_col: usize = 0;

    for col in possible_drops(tokens) {
        drop_token(&mut tokens[col], false);
        let score = minmax(tokens, 1, i32::MIN, i32::MAX, false);
        drop_back(&mut tokens[col]);

        if score > best_score {
            best_score = score;
            best_col = col;
        }
    }
    drop_token(&mut tokens[best_col], false);
}

#[macroquad::main("MyGame")]
async fn main() {
    let t: std::time::SystemTime = std::time::SystemTime::now();
    let mut wins: (u32, u32) = (0, 0);
    let mut ended: bool = false;
    let mut tokens: Vec<Vec<Token>> = Vec::new();
    let mut drop_pos: usize = 3;
    set_up_game(&mut tokens);
    request_new_screen_size(760.0, 760.0);

    loop {
        if is_key_pressed(KeyCode::Escape) {
            println!("Exiting game");
            return;
        } else if is_key_pressed(KeyCode::Space) {
            println!(
                "Player has won {} games and AI has won {} games",
                wins.0, wins.1
            );
        }
        if ended {
            if is_key_pressed(KeyCode::Backspace) {
                println!("Starting new game!");
                set_up_game(&mut tokens);
                ended = false;
            }
        } else {
            rand::srand(t.elapsed().map(|d| d.as_micros()).unwrap_or(0) as u64);
            if is_key_pressed(KeyCode::Enter) {
                if drop_token(&mut tokens[drop_pos], true) {
                    if check_victory(&tokens, Owned::PLAYER) {
                        println!("You won!");
                        wins.0 += 1;
                        ended = true;
                    } else {
                        ai_move(&mut tokens);
                        if check_victory(&tokens, Owned::AI) {
                            println!("AI won!");
                            wins.1 += 1;
                            ended = true;
                        } else if possible_drops(&tokens).is_empty() {
                            println!("Game ended in tie!");
                            ended = true;
                        }
                    }
                } else {
                    println!("Try something else!");
                }
            } else if is_key_pressed(KeyCode::Left) {
                drop_pos = (drop_pos + SIZE_X - 1) % SIZE_X;
            } else if is_key_pressed(KeyCode::Right) {
                drop_pos = (drop_pos + SIZE_X + 1) % SIZE_X;
            }
        }

        clear_background(BLACK);
        draw_rectangle(30.0, 130.0, 700.0, 600.0, DARKGRAY);
        draw_circle(80.0 + (drop_pos as f32) * 100.0, 70.0, 45.0, RED);

        for line in &tokens {
            for tok in line {
                draw_circle(tok.pos.0, tok.pos.1, 45.0, tok.color);
            }
        }
        next_frame().await
    }
}
