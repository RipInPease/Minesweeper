#[derive(Clone)]
#[derive(Debug)]
struct Tile {
    safe: bool,
    surrounding_bombs: u8,
    flagged: bool,
    surrounding_flags: u8,
}



fn main() {
    let mut tiles: Vec<Vec<Tile>> = Vec::new();
    let width = 2;
    let height = 2;

    init_board(2, 2, 27);
    
}

fn init_board (width: u8, height: u8, bombs: usize){
    use rand::random_range;
    let density = bombs as f32 / ((width as f32) * (height as f32));
    let x = random_range(0.0..1.0);

    let tiles = vec![vec![Tile{safe: false, surrounding_bombs: 0, flagged: false, surrounding_flags: 0}; width as usize]; height as usize];

    for y in (0..height) {
        for x in (0..width) {
            println!("{:?}", tiles[x as usize][y as usize]);
        }
    }
}

