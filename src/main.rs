

fn main() {
    let tetromino: [&str; 7]
    = ["..X...X...X...X.",
    "..X..XX..X.....",
    ".X...XX...X.....",
    ".....XX..XX.....",
    "..X..XX...X.....",
    ".....XX...X...X.",
    ".....XX..X...X.."];

    const FIELDWIDTH: usize = 12;
    const FIELDHEIGHT: usize = 18;

    let mut field: [usize; FIELDWIDTH*FIELDHEIGHT] = [0; FIELDWIDTH*FIELDHEIGHT];
    for x in 0..FIELDWIDTH {
        for y in 0..FIELDHEIGHT {
            field[y * FIELDWIDTH + x] = if x == 0 || x == FIELDWIDTH -1 || y == FIELDHEIGHT -1 {9} else {0};
        }
    }

    fn rotate(x: usize, y: usize, r: usize) -> usize {
        return match r % 4 {
            0 => y * 4 + x,
            1 => 12 + y - (x * 4),
            2 => 15 - (y * 4) - y,
            3 => 3 - y + (x * 4),
            _ => 0
        }
    }

    let mut game_over: bool = false;

    while !game_over {
        game_over = true;
    }
}





