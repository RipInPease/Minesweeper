#[derive(Clone, Debug)]
struct Tile {
    is_opened: bool,
    is_bomb: bool,
    surrounding_bombs: u8,
    flagged: bool,
    surrounding_flags: u8,
}

impl Tile {
    fn default () -> Tile {
        Tile{is_opened: false, is_bomb: false, surrounding_bombs: 0, flagged: false, surrounding_flags: 0}
    }
}


fn main() {
   let tiles = init_board(10, 10, 20);

   print_tiles(&tiles);
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
            if density > random_range(0.0..1.0) && tiles[x][y].is_bomb == false && current_bombs < bombs {
                set_tile_as_bomb(&mut tiles, x, y);
                current_bombs += 1;
            }
        }
        }
    }

    tiles
}


//Set tile as bomb and surrounding_bombs of surrounding tiles
fn set_tile_as_bomb (tiles: &mut Vec<Vec<Tile>>, x: usize, y: usize) {
    //set tile as bombs
    tiles[x][y].is_bomb = true;


    //increase surrounding_bombs of surrounding tiles
    for sur_y in -1..2 {
        let y_index = (y as i32 + sur_y) as usize;

        for sur_x in -1..2 {
            let x_index = (x as i32 + sur_x) as usize;

            match tiles.get(x_index) {
                Some(comlumn) => match comlumn.get(y_index) {
                    Some(_) => tiles[x_index][y_index].surrounding_bombs += 1,
                    _ => (),
                },
                _ => (),
            }
        }
    }

}

//when tile is clicked on
fn open_tile (tiles: &mut Vec<Vec<Tile>>, x: usize, y: usize) {
    //set tile as open if safe, else: die
    if tiles[x][y].is_bomb {
        die();
    } else {
        tiles[x][y].is_bomb = true;
    }
    


    //check if surrounding tile is 0
    for sur_y in -1..2 {
        let y_index = (y as i32 + sur_y) as usize;

        for sur_x in -1..2 {
            let x_index = (x as i32 + sur_x) as usize;

            match tiles.get(x_index) {
                Some(comlumn) => match comlumn.get(y_index) {
                    Some(_) => if tiles[x_index][y_index].surrounding_bombs == 0 {
                        open_tile(tiles, x_index, y_index);
                    },
                    _ => (),
                },
                _ => (),
            }
        }
    }
}


fn print_tiles (tiles: &Vec<Vec<Tile>>) {
    for x in tiles {
        println!("");
        for y in x {
            if y.is_bomb {
                print!("1 ");
            } else {
                print!("0 ");
            }
        }
       }
    
       println!("");
    
       for x in tiles {
        println!("");
        for y in x {
            print!("{} ", y.surrounding_bombs);
        }
       }
    
       println!("");
}

fn die () {
    panic!("You died")
}