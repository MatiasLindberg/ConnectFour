use macroquad::prelude::*;

const SIZE_Y: usize = 6;
const SIZE_X: usize = 7;
const AI_DEPTH: usize = 5;

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
    let pos = token_row
        .iter()
        .rposition(|token| token.owned == Owned::NOBODY)
        .unwrap();

    token_row[pos].owned = if player { Owned::PLAYER } else { Owned::AI };
    token_row[pos].color = if player { RED } else { YELLOW };

    true
}

fn drop_back(token_row: &mut Vec<Token>) {
    if let Some(pos) = token_row
        .iter()
        .position(|token| token.owned != Owned::NOBODY)
    {
        token_row[pos].owned = Owned::NOBODY;
        token_row[pos].color = BLACK;
    }
}

fn win_positions(tokens: &Vec<Vec<Token>>, owner: Owned) -> Vec<(f32, f32)> {
    for y in 0..SIZE_Y {
        for x in 0..SIZE_X {
            if tokens[x][y].owned == owner {
                if x + 3 < SIZE_X
                    && tokens[x + 1][y].owned == owner
                    && tokens[x + 2][y].owned == owner
                    && tokens[x + 3][y].owned == owner
                {
                    let tmp: Vec<(f32, f32)> = vec![
                        tokens[x][y].pos,
                        tokens[x + 1][y].pos,
                        tokens[x + 2][y].pos,
                        tokens[x + 3][y].pos,
                    ];
                    return tmp;
                }

                if y + 3 < SIZE_Y
                    && tokens[x][y + 1].owned == owner
                    && tokens[x][y + 2].owned == owner
                    && tokens[x][y + 3].owned == owner
                {
                    let tmp: Vec<(f32, f32)> = vec![
                        tokens[x][y].pos,
                        tokens[x][y + 1].pos,
                        tokens[x][y + 2].pos,
                        tokens[x][y + 3].pos,
                    ];
                    return tmp;
                }

                if x + 3 < SIZE_X
                    && y + 3 < SIZE_Y
                    && tokens[x + 1][y + 1].owned == owner
                    && tokens[x + 2][y + 2].owned == owner
                    && tokens[x + 3][y + 3].owned == owner
                {
                    let tmp: Vec<(f32, f32)> = vec![
                        tokens[x][y].pos,
                        tokens[x + 1][y + 1].pos,
                        tokens[x + 2][y + 2].pos,
                        tokens[x + 3][y + 3].pos,
                    ];
                    return tmp;
                }

                if x >= 3
                    && y + 3 < SIZE_Y
                    && tokens[x - 1][y + 1].owned == owner
                    && tokens[x - 2][y + 2].owned == owner
                    && tokens[x - 3][y + 3].owned == owner
                {
                    let tmp: Vec<(f32, f32)> = vec![
                        tokens[x][y].pos,
                        tokens[x - 1][y + 1].pos,
                        tokens[x - 2][y + 2].pos,
                        tokens[x - 3][y + 3].pos,
                    ];
                    return tmp;
                }
            }
        }
    }
    vec![]
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
    tokens
        .iter()
        .enumerate()
        .filter(|(_, col)| col[0].owned == Owned::NOBODY)
        .map(|(index, _)| index)
        .collect()
}

fn eval(tokens: &Vec<Vec<Token>>, depth: usize) -> i32 {
    // Tries to find fastest win or slowest lost/tie
    if possible_drops(tokens).is_empty() {
        50 - depth as i32
    } else if check_victory(tokens, Owned::AI) {
        100 + depth as i32
    } else if check_victory(tokens, Owned::PLAYER) {
        -100 - depth as i32
    } else {
        70
    }
}

fn minmax(tokens: &mut Vec<Vec<Token>>, depth: usize, alpha: i32, beta: i32, ai: bool) -> i32 {
    match eval(tokens, depth) {
        score if score >= 100 => return score,
        score if score <= -100 => return score,
        score if score <= 50 => return score,
        _ => {
            if depth <= 0 {
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
            let eval = minmax(tokens, depth - 1, alpha, beta, false);
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
            let eval = minmax(tokens, depth - 1, alpha, beta, true);
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
        let score = minmax(tokens, AI_DEPTH, i32::MIN, i32::MAX, false);
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
    let mut win_pos: Vec<(f32, f32)> = Vec::new();
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
                win_pos = vec![];
            }
        } else {
            if is_key_pressed(KeyCode::Enter) {
                if drop_token(&mut tokens[drop_pos], true) {
                    if check_victory(&tokens, Owned::PLAYER) {
                        println!("You won!");
                        wins.0 += 1;
                        ended = true;
                        win_pos = win_positions(&tokens, Owned::PLAYER);
                    } else {
                        ai_move(&mut tokens);
                        if check_victory(&tokens, Owned::AI) {
                            println!("AI won!");
                            wins.1 += 1;
                            ended = true;
                            win_pos = win_positions(&tokens, Owned::AI);
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
        for x in &win_pos {
            draw_circle(x.0, x.1, 10.0, BLUE);
        }
        next_frame().await
    }
}
