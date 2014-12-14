#![allow(non_snake_case)]
use std::fmt;

#[deriving(Show)]
pub struct Creature {
  pub id: int,
  name: String,
  inventory: Vec<Item>,
}

#[deriving(Show)]
pub struct Player {
  pub id: int,
  name: String,
  inventory: Vec<Item>,
}

#[deriving(Show)]
pub struct Item {
  pub id: int,
  pub name: String,
  pub description: Description,
}

type Direction = String;

#[deriving(Show)]
pub struct Exit {
  pub connectingRooms: Vec<(Direction, Room)>,
}

struct SpecialParser(Vec<fn(String) -> (String)>);

impl fmt::Show for SpecialParser {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

type Description = String;

#[deriving(Show)]
pub struct Room {
  pub specialParsers: Vec<SpecialParser>,
  pub exits: Vec<Exit>,
  pub creatures: Vec<Creature>,
  pub description: Description,
  pub items: Vec<Item>,
}

#[deriving(Show)]
pub struct GameWorld {
  pub x: int,
  pub allPlayers: Vec<Player>,
  pub activePlayers: Vec<Player>,
  pub rooms: Vec<Room>,
}
