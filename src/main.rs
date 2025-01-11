use macroquad::prelude::*;

const SIZE_Y: usize = 6;
const SIZE_X: usize = 7;

struct Token {
    pos: (f32, f32),
    free: bool,
    color: Color,
}

fn set_up_game(tokens: &mut Vec<Vec<Token>>) {
    for j in 0..SIZE_X {
        let mut tmp_tok: Vec<Token> = Vec::new();
        for i in 0..SIZE_Y {
            let tmp: Token = Token {
                pos: ((j * 100 + 80) as f32, (i * 100 + 180) as f32),
                free: true,
                color: BLACK,
            };
            tmp_tok.push(tmp);
        }
        tokens.push(tmp_tok);
    }
}

fn drop_token(token_row: &mut Vec<Token>, player: bool) -> bool {
    if token_row[0].free == false {
        return false;
    }
    let mut pos: usize = 1;
    while pos < SIZE_Y && token_row[pos].free {
        pos += 1;
    }
    token_row[pos - 1].free = false;
    if player {
        token_row[pos - 1].color = RED;
    } else {
        token_row[pos - 1].color = YELLOW;
    }
    true
}

#[macroquad::main("MyGame")]
async fn main() {
    let t: std::time::SystemTime = std::time::SystemTime::now();
    let mut tokens: Vec<Vec<Token>> = Vec::new();
    let mut drop_pos: usize = 3;
    set_up_game(&mut tokens);
    request_new_screen_size(760.0, 760.0);

    loop {
        rand::srand(t.elapsed().map(|d| d.as_millis()).unwrap_or(0) as u64);
        if is_key_pressed(KeyCode::Escape) {
            println!("Exiting game");
            return;
        } else if is_key_pressed(KeyCode::Enter) {
            if drop_token(&mut tokens[drop_pos], true) {
                while !drop_token(&mut tokens[rand::gen_range(0, SIZE_X)], false) {}
            } else {
                println!("Try something else!");
            }
        } else if is_key_pressed(KeyCode::Left) {
            drop_pos = (drop_pos + SIZE_X - 1) % SIZE_X;
        } else if is_key_pressed(KeyCode::Right) {
            drop_pos = (drop_pos + SIZE_X + 1) % SIZE_X;
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
