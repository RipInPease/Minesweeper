#[derive(Clone, Debug)]
struct Tile {
    safe: bool,
    surrounding_bombs: u8,
    flagged: bool,
    surrounding_flags: u8,
}

impl Tile {
    fn default () -> Tile {
        Tile{safe: true, surrounding_bombs: 0, flagged: false, surrounding_flags: 0}
    }
}


fn main() {
    init_board(4, 4, 1);
}

fn init_board (width: usize, height: usize, bombs: usize) -> Vec<Vec<Tile>>{
    use Babylib::Vec2d;
    use rand::random_range;
    let density = bombs as f32 / ((width as f32) * (height as f32));
    let mut tiles = Vec2d::new::<Tile>(width.into(), height.into(),Tile::default());

    let mut current_bombs = 0;
    while current_bombs < bombs {
        for x in (0..width) {
        for y in (0..height) {
            if density > random_range(0.0..1.0) && tiles[x][y].safe == true && current_bombs < bombs {
                tiles[x][y].safe = false;
                current_bombs += 1;
            }
        }
        }
    }
    
    tiles
}

