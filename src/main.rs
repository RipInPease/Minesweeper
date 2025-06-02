use std::io::{Write, stdout};
use core::time::Duration;
use crossterm::{self, QueueableCommand};
use crossterm::event::{self, read, poll, Event, KeyCode, KeyModifiers};
use crossterm::terminal::{self, Clear, ClearType};
use crossterm::cursor::{self, MoveTo};
use crossterm::style::{self, Print, StyledContent, Stylize};






#[derive(Clone, Debug)]
struct Tile {
    opened: bool,
    is_bomb: bool,
    surrounding_bombs: u8,
    flagged: bool,
    surrounding_flags: u8,
}

impl Tile {
    fn default () -> Tile {
        Tile{opened: false, is_bomb: false, surrounding_bombs: 0, flagged: false, surrounding_flags: 0}
    }
}


fn main() {
    let mut stdout = stdout();
    terminal::enable_raw_mode().unwrap();
    stdout.queue(cursor::Hide).unwrap()
          .queue(Clear(ClearType::All)).unwrap()
          .flush().unwrap();

  

    let mut tiles = init_board(10, 6, 40);
    let mut cursor_position: (u16, u16) = (12, 12);
    
    loop {
        while poll(Duration::ZERO).unwrap() {
            match read().unwrap() {
                Event::Key(key_event) => match key_event.code {
                    //Up/Down/Left/Right/Open/Chording and quitting
                    KeyCode::Up => if cursor_position.1 > 0 {cursor_position.1 -= 1},
                    KeyCode::Down => if cursor_position.1 < 20 {cursor_position.1 += 1},
                    KeyCode::Left => if cursor_position.0 > 0 {cursor_position.0 -= 1},
                    KeyCode::Right => if cursor_position.0 < 20 {cursor_position.0 += 1},
                    KeyCode::Enter => (),
                    KeyCode::Char('c') => if key_event.modifiers.contains(KeyModifiers::CONTROL) {die("You quit")} //CTRL+C to quit,
                    _ => (),
                    },
                _ => (),
            }
        
        }

        draw_page(&tiles, &cursor_position);
    }
}


//Given x, y and bombs: Create a tilemap
fn init_board (width: usize, height: usize, bombs: usize) -> Vec<Vec<Tile>>{
    use Babylib::Vec2d;
    use rand::random_range;

    let mut used_bombs = bombs;

    if bombs > width * height {
        used_bombs = (width * height)-1;

        println!{"You chose more bombs than can fit in the grid, bombs set to 1 less than grid size"}
    }
    let density = used_bombs as f32 / ((width as f32) * (height as f32));
    let mut tiles = Vec2d::new::<Tile>(width.into(), height.into(),Tile::default());

    let mut current_bombs = 0;
    while current_bombs < used_bombs {
        for x in 0..width {
        for y in 0..height {
            if density > random_range(0.0..1.0) && tiles[x][y].is_bomb == false && current_bombs < used_bombs {
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

//When tile is clicked on, set as opened. If opened neighbor is 0, set neighbor as opened. RECURSION, BABY!!
fn open_tile (tiles: &mut Vec<Vec<Tile>>, x: usize, y: usize) {
    //set tile as open if safe, else: die
    if tiles[x][y].is_bomb {
        die("You exploded");
    } else if !tiles[x][y].flagged {
        tiles[x][y].opened = true;
    
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
}

//Flag tile, increment surrounding_flags of adjacent tiles
fn toggle_flag (tiles: &mut Vec<Vec<Tile>>, x: usize, y: usize) {
    //Toggle flagged state
    tiles[x][y].flagged = !tiles[x][y].flagged;

    //If you made tile to flagged, increment surrounding_flags of surrounding tiles
    if tiles[x][y].flagged {
        for sur_y in -1..2 {
            let y_index = (y as i32 + sur_y) as usize;

            for sur_x in -1..2 {
                let x_index = (x as i32 + sur_x) as usize;

                match tiles.get(x_index) {
                    Some(comlumn) => match comlumn.get(y_index) {
                        Some(_) => tiles[x_index][y_index].surrounding_flags += 1,
                        _ => (),
                    },
                    _ => (),
                }
            }
        }
    }

    //If you made tile to not flagged, decrement surrounding_flags of surrounding tiles
    if !tiles[x][y].flagged {
        //increase surrounding_flags of surrounding tiles
        for sur_y in -1..2 {
            let y_index = (y as i32 + sur_y) as usize;

            for sur_x in -1..2 {
                let x_index = (x as i32 + sur_x) as usize;

                match tiles.get(x_index) {
                    Some(comlumn) => match comlumn.get(y_index) {
                        Some(_) => tiles[x_index][y_index].surrounding_flags -= 1,
                        _ => (),
                    },
                    _ => (),
                }
            }
        }
    }
}

//Open all adjacent tiles
fn chord(tiles: &mut Vec<Vec<Tile>>, x: usize, y: usize) {
    //First check if you have correct number of surrounding_flag and tile is opened
    if tiles[x][y].surrounding_flags == tiles[x][y].surrounding_bombs && tiles[x][y].opened {

        for sur_y in -1..2 {
            let y_index = (y as i32 + sur_y) as usize;

            for sur_x in -1..2 {
                let x_index = (x as i32 + sur_x) as usize;

                match tiles.get(x_index) {
                    Some(comlumn) => match comlumn.get(y_index) {
                        Some(_) => open_tile(tiles, x_index, y_index),
                        _ => (),
                    },
                    _ => (),
                }
            }
        }
    }

}

//print tiles to terminal
fn print_tiles (tiles: &Vec<Vec<Tile>>) {
    let width = tiles.len();
    let height = tiles[0].len();

    //print if tile is bomb or safe
    for y_index in 0..height {
      for x_index in 0..width {
          if tiles[x_index][y_index].is_bomb {
            print!("1 ");
          } else {
            print!("0 ");
          }
      }
      println!("");
    }

    println!("");

    //print surrounding bombs
    for y_index in 0..height {
        for x_index in 0..width {
          print!("{} ", tiles[x_index][y_index].surrounding_bombs)
          }    
     println!("");
    }
}

//Draw page to terminal
fn draw_page(tiles: &Vec<Vec<Tile>>, cursor_position: &(u16, u16)) {
    let mut stdout = stdout();

    let width = tiles.len() as u16;
    let height = tiles[0].len() as u16;


    for y in 0..height {
        for x in 0..width {
            stdout
            .queue(MoveTo(x, y)).unwrap()
            .queue(Print("-".white().on_dark_grey())).unwrap();
        }
    }

    stdout
            .queue(MoveTo(cursor_position.0, cursor_position.1)).unwrap()
            .queue(Print("-".blue().on_dark_grey())).unwrap();

    stdout.flush().unwrap();
}

//Try again, loser
fn die (message: &str) {
    //Disable raw mode, show cursor, crash program
    let mut stdout = stdout();
    terminal::disable_raw_mode().unwrap();
    stdout.queue(Clear(ClearType::All)).unwrap()
          .queue(cursor::Show).unwrap();
    stdout.flush().unwrap();

    panic!("{}", message)
}
