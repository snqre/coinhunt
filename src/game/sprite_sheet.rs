use super::*;

pub struct Tile {
    l: char,
    m: char,
    r: char
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s: String = String::new();
        s.push(self.l.to_owned());
        s.push(self.m.to_owned());
        s.push(self.r.to_owned());
        write!(f, "{}", s)
    }
}

impl From<(char, char, char)> for Tile {
    fn from(value: (char, char, char)) -> Self {
        let l: char = value.0;
        let m: char = value.1;
        let r: char = value.2;
        Self {
            l,
            m,
            r
        }
    }
}

pub struct SpriteSheet {
    pub empty: Tile,
    pub coin: Tile,
    pub player: Tile
}

impl Default for SpriteSheet {
    fn default() -> Self {
        let empty: Tile = ('[', ' ', ']').into();
        let coin: Tile = ('[', 'o', ']').into();
        let player: Tile = ('[', '*', ']').into();
        Self {
            empty,
            coin,
            player
        }
    }
}