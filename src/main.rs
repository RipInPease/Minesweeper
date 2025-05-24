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
   let tiles = init_board(4, 4, 14);
   for x in tiles {
    for y in x {
        println!("{:?}", y);
    }
   }
}


//Given x, y and bombs: Create a tilemap
fn init_board (width: usize, height: usize, bombs: usize) -> Vec<Vec<Tile>>{
    use Babylib::Vec2d;
    use rand::random_range;
    let density = bombs as f32 / ((width as f32) * (height as f32));
    let mut tiles = Vec2d::new::<Tile>(width.into(), height.into(),Tile::default());

    let mut current_bombs = 0;
    while current_bombs < bombs {
        for x in 0..width {
        for y in 0..height {
            if density > random_range(0.0..1.0) && tiles[x][y].safe == true && current_bombs < bombs {
                tiles = set_tile_as_bomb(tiles, x, y);
                current_bombs += 1;
            }
        }
        }
    }

    tiles
}


//Set tile as bomb and surround bombs of surrounding tiles
fn set_tile_as_bomb (mut tiles: Vec<Vec<Tile>>, x: usize, y: usize) -> Vec<Vec<Tile>> {
    tiles[x][y].safe = false;

    for sur_y in -1..2 {
        if y == 0 && sur_y == -1 {continue};

        for sur_x in -1..2 {
            if x == 0 && sur_x == -1 {continue};

            match tiles[(x as i32 - sur_x)as usize][(y as i32 - sur_y)as usize].get() {
                _ => (),
            }
        }
    }

    tiles
}
