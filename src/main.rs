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
    init_board(2, 2, 0);
}

fn init_board (width: usize, height: usize, bombs: usize){
    use Babylib::Vec2d;
    use rand::random_range;
    let density = bombs as f32 / ((width as f32) * (height as f32));
    let mut tiles = Vec2d::new::<Tile>(width.into(), height.into(),Tile::default());

    for x in (0..width) {
        for y in (0..height) {
            if density > random_range(0.0..1.0) {
                tiles[x][y].safe = false;
            }
            println!("{:?}", tiles[x][y]);
        }
    }
}

