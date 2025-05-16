struct Tile {
    safe: bool,
    surrounding_bombs: u8,
    flagged: bool,
    surrounding_flags: u8,
}



fn main() {
    init_board(32, 16, 99);
    let mut tiles: Vec<Vec<Tile>> = Vec::new();
}

fn init_board (width: u8, height: u8, bombs: u8){
    use rand::random_range;
    let density = bombs as f32 / ((width as f32) * (height as f32));

}

