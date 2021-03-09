use std::fmt::{Display, Formatter, Result as FMTResult};

const PLAY_OPTION_EVEN: u8 = 1;
const PLAY_OPTION_ODD: u8 = 2;

type GameResult<T> = Result<T, GameError>;

#[derive(Debug, Eq, PartialEq)]
pub enum GameError {
    InvalidOption(u8),
    OutOfRange(u8),
    AlreadyContainAddress,
    PlayOptionAlreadyTaken(HandPlayed),
    MissingPlayerPlay(HandPlayed),
    ParseArgumentError(Option<String>),
}

#[derive(Debug, Eq, PartialEq)]
pub enum HandPlayed {
    Even(u8),
    Odd(u8),
}

#[derive(Debug, Eq, PartialEq)]
pub struct Player<T> {
    pub(crate) address: T,
    hand_played: HandPlayed,
}

pub struct Game<T> {
    even: Option<Player<T>>,
    odd: Option<Player<T>>,
}

#[derive(Eq, PartialEq, Debug)]
pub struct RoundResult<'a, T> {
    pub(crate) winner: &'a Player<T>,
    pub(crate) loser: &'a Player<T>,
}

impl<T> Player<T> {
    pub fn new(play: HandPlayed, address: T) -> Player<T> {
        Player { hand_played: play, address }
    }
}

impl<T: std::cmp::Eq + std::cmp::PartialEq> Game<T> {
    pub fn new() -> Game<T> {
        Game { even: None, odd: None }
    }

    pub fn add_play(&mut self, play: HandPlayed, address: T) -> GameResult<()> {
        if self.contain_address(&address) {
            return Err(GameError::AlreadyContainAddress);
        }

        match play {
            HandPlayed::Even(_) => {
                if self.even.is_none() {
                    return Ok(self.even = Some(Player::new(play, address)));
                }
            }
            HandPlayed::Odd(_) => {
                if self.odd.is_none() {
                    return Ok(self.odd = Some(Player::new(play, address)));
                }
            }
        }

        Err(GameError::PlayOptionAlreadyTaken(play))
    }

    pub fn contain_address(&self, address: &T) -> bool {
        let existing_address = match self {
            Game { even: Some(v), odd: _ } => Some(v),
            Game { even: _, odd: Some(v) } => Some(v),
            _ => None
        };

        return existing_address.is_some() && address.eq(&existing_address.unwrap().address);
    }

    pub fn can_guess(&self) -> bool {
        self.odd.is_some() && self.even.is_some()
    }

    pub fn guess_winner(&self) -> GameResult<RoundResult<T>> {
        let even = match &self.even {
            Some(v) => v,
            None => return Err(GameError::MissingPlayerPlay(HandPlayed::Even(0)))
        };

        let odd = match &self.odd {
            Some(v) => v,
            None => return Err(GameError::MissingPlayerPlay(HandPlayed::Odd(0)))
        };

        if HandPlayed::value_is_even(odd.hand_played.unwrap_value() + even.hand_played.unwrap_value()) {
            return Ok(RoundResult { winner: even, loser: odd });
        }

        Ok(RoundResult { winner: odd, loser: even })
    }

    pub fn reset(&mut self) {
        self.even = None;
        self.odd = None;
    }
}

impl HandPlayed {
    pub fn new(option: &u8, value: u8) -> GameResult<HandPlayed> {
        if value < 1 || value > 5 {
            return Err(GameError::OutOfRange(value));
        }

        match *option {
            PLAY_OPTION_EVEN => Ok(HandPlayed::Even(value)),
            PLAY_OPTION_ODD => Ok(HandPlayed::Odd(value)),
            v => Err(GameError::InvalidOption(v))
        }
    }

    pub fn from_str(option: &str, value: &str) -> GameResult<HandPlayed> {
        let option: Result<u8, std::num::ParseIntError> = option.parse();
        if let Err(e) = option {
            return Err(GameError::ParseArgumentError(Some(e.to_string())));
        }

        let value: Result<u8, std::num::ParseIntError> = value.parse();
        if let Err(e) = value {
            return Err(GameError::ParseArgumentError(Some(e.to_string())));
        }

        Ok(HandPlayed::new(&option.unwrap(), value.unwrap())?)
    }

    fn value_is_even(value: u8) -> bool {
        value % 2 == 0
    }

    pub fn unwrap_value(&self) -> u8 {
        match self {
            HandPlayed::Even(v) => v.to_owned(),
            HandPlayed::Odd(v) => v.to_owned(),
        }
    }
}

impl Display for GameError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FMTResult {
        match self {
            Self::InvalidOption(v) => write!(f, "{} is not a valid option. The options are: 1 for Even and 2 for Odd", v),
            Self::OutOfRange(v) => write!(f, "{} is out of range. The number must be from 1 to 5.", v),
            Self::AlreadyContainAddress => write!(f, "The game already contain your play. Wait for other player."),
            Self::PlayOptionAlreadyTaken(v) => write!(f, "You tried to play '{}' but this play option was already taken. Choose the other.", v),
            Self::MissingPlayerPlay(v) => write!(f, "Cannot guess now, Missing the player option: {}.", v),
            Self::ParseArgumentError(v) => {
                if let Some(message) = v {
                    write!(f, "Error parsing arguments. Cause: {}", message)
                } else {
                    write!(f, "Error parsing arguments.")
                }
            }
        }
    }
}

impl Display for HandPlayed {
    fn fmt(&self, f: &mut Formatter<'_>) -> FMTResult {
        match self {
            Self::Even(v) => write!(f, "Even({})", v),
            Self::Odd(v) => write!(f, "Odd({})", v),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::game::*;
    use std::net::{SocketAddr, SocketAddrV4, AddrParseError};
    use std::str::FromStr;

    #[test]
    fn it_validates_the_option() {
        assert_eq!(HandPlayed::new(&3, 1), Err(GameError::InvalidOption(3)));
    }

    #[test]
    fn it_validates_the_value() {
        assert_eq!(HandPlayed::new(&PLAY_OPTION_ODD, 6), Err(GameError::OutOfRange(6)));
    }

    #[test]
    fn it_cannot_guess_with_missing_values() -> Result<(), AddrParseError> {
        let mut game: Game<SocketAddr> = Game::new();
        let address_1 = SocketAddr::V4(SocketAddrV4::from_str("127.0.0.1:888")?);

        assert_eq!(game.can_guess(), false);

        let _ = game.add_play(HandPlayed::Odd(1), address_1).expect("Error adding player 1");

        assert_eq!(game.can_guess(), false);

        Ok(())
    }

    #[test]
    fn it_cannot_register_two_plays_on_same_round() -> Result<(), AddrParseError> {
        let mut game: Game<SocketAddr> = Game::new();
        let address_1 = SocketAddr::V4(SocketAddrV4::from_str("127.0.0.1:888")?);

        let _ = game.add_play(HandPlayed::Odd(1), address_1);

        assert_eq!(game.add_play(HandPlayed::Even(2), address_1), Err(GameError::AlreadyContainAddress));

        Ok(())
    }

    #[test]
    fn it_cannot_take_other_player_option() -> Result<(), AddrParseError> {
        let mut game: Game<SocketAddr> = Game::new();
        let address_1 = SocketAddr::V4(SocketAddrV4::from_str("127.0.0.1:888")?);
        let address_2 = SocketAddr::V4(SocketAddrV4::from_str("127.0.0.1:881")?);

        let _ = game.add_play(HandPlayed::Odd(1), address_1);

        assert_eq!(game.add_play(HandPlayed::Odd(3), address_2), Err(GameError::PlayOptionAlreadyTaken(HandPlayed::Odd(3))));

        Ok(())
    }

    #[test]
    fn it_can_guess_correct_winner() -> Result<(), AddrParseError> {
        let mut game: Game<SocketAddr> = Game::new();
        let address_1 = SocketAddr::V4(SocketAddrV4::from_str("127.0.0.1:888")?);
        let address_2 = SocketAddr::V4(SocketAddrV4::from_str("127.0.0.1:881")?);

        let _ = game.add_play(HandPlayed::Odd(1), address_1);
        let _ = game.add_play(HandPlayed::Even(3), address_2);

        let result = RoundResult {
            winner: &Player::new(HandPlayed::Even(3), address_2),
            loser: &Player::new(HandPlayed::Odd(1), address_1),
        };

        assert_eq!(game.guess_winner().unwrap(), result);

        Ok(())
    }
}