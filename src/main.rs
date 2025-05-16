#[derive(Clone)]
#[derive(Debug)]
struct Tile {
    safe: bool,
    surrounding_bombs: u8,
    flagged: bool,
    surrounding_flags: u8,
}



fn main() {
    init_board(32, 16, 99);
    let mut tiles: Vec<Vec<Tile>> = Vec::new();
    let width = 2;
    let height = 2;

    tiles = vec![vec![Tile{safe: false, surrounding_bombs: 0, flagged: false, surrounding_flags: 0}; height]; width];
    println!("{:?}", tiles[1][0]);
    
}

fn init_board (width: u8, height: u8, bombs: u8){
    use rand::random_range;
    let density = bombs as f32 / ((width as f32) * (height as f32));

}

