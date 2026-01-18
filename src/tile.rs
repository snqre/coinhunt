use super::*;

pub type Empty = Tile<IsEmpty>;
pub type Coin = Tile<IsCoin>;
pub type Player = Tile<IsPlayer>;

#[derive(Clone)]
pub struct IsEmpty;

#[derive(Clone)]
pub struct IsCoin;

#[derive(Clone)]
pub struct IsPlayer;

#[derive(Clone)]
pub struct Tile<T> {
    phantom_data: PhantomData<T>
}

impl<T> Default for Tile<T> {
    fn default() -> Self {
        Self {
            phantom_data: PhantomData
        }
    }
}

impl game::Tile for Empty {
    fn has_player(&self) -> bool {
        false
    }

    fn has_coin(&self) -> bool {
        false
    }
}

impl game::Tile for Coin {
    fn has_player(&self) -> bool {
        false
    }

    fn has_coin(&self) -> bool {
        true
    }
}

impl game::Tile for Player {
    fn has_player(&self) -> bool {
        true
    }

    fn has_coin(&self) -> bool {
        false
    }
}