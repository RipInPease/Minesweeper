#[derive(Clone, Debug)]
struct Tile {
    safe: bool,
    surrounding_bombs: u8,
    flagged: bool,
    surrounding_flags: u8,
}

impl Default for Tile {
    fn default () -> Tile {
        Tile{safe: true, surrounding_bombs: 0, flagged: false, surrounding_flags: 0}
    }
}


fn main() {
    let width = 2;
    let height = 2;
    let mut tiles = Babylib::Vec2d::new::<Tile>(width, height);

    println!("{:?}", tiles);
}

fn init_board (width: u8, height: u8, bombs: usize){
    use rand::random_range;
    let density = bombs as f32 / ((width as f32) * (height as f32));
    let x = random_range(0.0..1.0);

  
}

