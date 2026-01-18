use super::*;

pub mod sprite_sheet;

pub trait Tile {
    fn has_player(&self) -> bool;
    fn has_coin(&self) -> bool;
}

pub type Point = (u8, u8);
pub type NotComplete = Game<IsNotComplete>;
pub type Complete = Game<IsComplete>;

pub struct IsNotComplete;
pub struct IsComplete;
pub struct Game<T> {
    phantom_data: PhantomData<T>,
    grid: HashMap<Point, Box<dyn Tile>>,
    w: u8,
    h: u8,
    coin_x: u8,
    coin_y: u8,
    player_x: u8,
    player_y: u8,
    found: bool,
    sprite_sheet: sprite_sheet::SpriteSheet
}

impl<T> Game<T> {
    pub fn w(&self) -> u8 {
        self.w
    }

    pub fn h(&self) -> u8 {
        self.h
    }

    pub fn coin_x(&self) -> u8 {
        self.coin_x
    }

    pub fn coin_y(&self) -> u8 {
        self.coin_y
    }

    pub fn player_x(&self) -> u8 {
        self.player_x
    }

    pub fn player_y(&self) -> u8 {
        self.player_y
    }

    pub fn found(&self) -> bool {
        self.found
    }

    pub fn render(&self) {
        for y in 0..=self.h() {
            for x in 0..=self.w() {
                let x: u8 = x;
                let y: u8 = y;
                let tile: &Box<_> = self.grid.get(&(x, y)).unwrap_or_else(|| panic!("missing tile at {} {}", x, y));
                if tile.has_player() {
                    print!("{}", self.sprite_sheet.player);
                } else if tile.has_coin() {
                    print!("{}", self.sprite_sheet.coin);
                } else {
                    print!("{}", self.sprite_sheet.empty);
                }
                print!(" ");
            }
            println!();
        }
        println!();
    }
}

impl NotComplete {
    fn move_and_check(mut self: Box<Self>, delta_x: i8, delta_y: i8) -> Result<Box<dyn super::Game<Complete = Complete>>> {
        self.transform(delta_x, delta_y)?;
        if self.found() {
            let Self {
                grid,
                w,
                h,
                coin_x,
                coin_y,
                player_x,
                player_y,
                found,
                sprite_sheet,
                ..
            } = *self;
            let ret: Complete = Complete {
                phantom_data: PhantomData,
                grid,
                w,
                h,
                coin_x,
                coin_y,
                player_x,
                player_y,
                found,
                sprite_sheet,
            };
            let ret: Box<dyn super::Game<Complete = Complete>> = Box::new(ret);
            return Ok(ret);
        }
        Ok(self)
    }

    fn transform(&mut self, delta_x: i8, delta_y: i8) -> Result<()> {
        if self.found() {
            return Ok(())
        }
        if delta_x == 0 && delta_y == 0 {
            return Ok(())
        }
        let w: i8 = self.w().try_into()?;
        let h: i8 = self.h().try_into()?;
        let old_x: i8 = self.player_x().try_into()?;
        let old_y: i8 = self.player_y().try_into()?;
        let mut new_x: i8 = old_x + delta_x;
        let mut new_y: i8 = old_y + delta_y;
        if new_x > w {
            new_x = w;
        }
        if new_x < 0 {
            new_x = 0;
        }
        if new_y > h {
            new_y = h;
        }
        if new_y < 0 {
            new_y = 0;
        }
        let tile: Box<tile::Empty> = Box::default();
        let old_x: u8 = old_x.try_into()?;
        let old_y: u8 = old_y.try_into()?;
        self.grid.insert((old_x, old_y), tile);
        let tile: Box<tile::Player> = Box::default();
        let new_x: u8 = new_x.try_into()?;
        let new_y: u8 = new_y.try_into()?;
        self.player_x = new_x;
        self.player_y = new_y;
        let tile: Box<dyn Tile> = self.grid.insert((new_x, new_y), tile).unwrap_or_else(|| panic!("missing tile at {} {}", new_x, new_y));
        if tile.has_coin() {
            self.found = true;
        }
        self.render();
        Ok(())
    }
}

impl Default for NotComplete {
    fn default() -> Self {
        let mut grid: HashMap<Point, Box<dyn Tile>> = HashMap::default();
        let tile: Box<tile::Empty> = Box::default();
        let w: u8 = 4;
        let h: u8 = 4;
        let mut coin_x: u8 = 0;
        let mut coin_y: u8 = 0;
        let mut player_x: u8 = 0;
        let mut player_y: u8 = 0;
        while coin_x == player_x && coin_y == player_y {
            coin_x = fastrand::u8(0..=w);
            coin_y = fastrand::u8(0..=h);
            player_x = fastrand::u8(0..=w);
            player_y = fastrand::u8(0..=h);
        }
        for x in 0..=w {
            for y in 0..=h {
                let x: u8 = x;
                let y: u8 = y;
                if x == coin_x && y == coin_y {
                    let tile: Box<tile::Coin> = Box::default();
                    grid.insert((x, y), tile);
                } else if x == player_x && y == player_y {
                    let tile: Box<tile::Player> = Box::default();
                    grid.insert((x, y), tile);
                } else {
                    grid.insert((x, y), tile.to_owned());
                }
            }
        }
        let found: bool = false;
        let sprite_sheet: sprite_sheet::SpriteSheet = sprite_sheet::SpriteSheet::default();
        Self {
            phantom_data: PhantomData,
            grid,
            w,
            h,
            coin_x,
            coin_y,
            player_x,
            player_y,
            found,
            sprite_sheet
        }
    }
}

impl super::Game for NotComplete {
    type Complete = Complete;

    fn move_left(self: Box<Self>) -> Result<Box<dyn crate::Game<Complete = Self::Complete>>> {
        self.move_and_check(-1, 0)
    }

    fn move_right(self: Box<Self>) -> Result<Box<dyn crate::Game<Complete = Self::Complete>>> {
        self.move_and_check(1, 0)
    }

    fn move_up(self: Box<Self>) -> Result<Box<dyn crate::Game<Complete = Self::Complete>>> {
        self.move_and_check(0, -1)
    }

    fn move_down(self: Box<Self>) -> Result<Box<dyn crate::Game<Complete = Self::Complete>>> {
        self.move_and_check(0, 1)
    }

    fn complete(self: Box<Self>) -> Option<Game<IsComplete>> {
        None
    }
}

impl super::Game for Complete {
    type Complete = Self;

    fn move_left(self: Box<Self>) -> Result<Box<dyn crate::Game<Complete = Self::Complete>>> {
        Ok(self)
    }

    fn move_right(self: Box<Self>) -> Result<Box<dyn crate::Game<Complete = Self::Complete>>> {
        Ok(self)
    }

    fn move_up(self: Box<Self>) -> Result<Box<dyn crate::Game<Complete = Self::Complete>>> {
        Ok(self)
    }

    fn move_down(self: Box<Self>) -> Result<Box<dyn crate::Game<Complete = Self::Complete>>> {
        Ok(self)
    }

    fn complete(self: Box<Self>) -> Option<Self::Complete> {
        Some(*self)
    }
}