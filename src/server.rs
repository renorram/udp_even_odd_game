use std::net::{UdpSocket, SocketAddr};
use crate::game::{Game, HandPlayed, GameError};

enum ServerError {
    GameKind(GameError),
    ParsingError(String),
}

pub const DEFAULT_SERVER_ADDRESS: &'static str = "127.0.0.1:34254";

impl ToString for ServerError {
    fn to_string(&self) -> String {
        match self {
            Self::GameKind(e) => format!("{}", e),
            Self::ParsingError(s) => s.to_owned()
        }
    }
}


fn parse_byte_stream(bytes: Vec<u8>) -> Result<HandPlayed, ServerError> {
    match String::from_utf8(bytes) {
        Ok(value) => {
            let options: Vec<&str> = value.split_whitespace().collect();
            if options.len() != 2 {
                return Err(ServerError::ParsingError(format!("You must pass exactly 2 arguments.")));
            }

            match HandPlayed::from_str(options[0], options[1]) {
                Ok(v) => Ok(v),
                Err(e) => Err(ServerError::GameKind(e))
            }
        }
        Err(e) => Err(ServerError::ParsingError(e.to_string()))
    }
}

pub(crate) fn run_server() -> std::io::Result<()> {
    let socket = UdpSocket::bind(DEFAULT_SERVER_ADDRESS)?;
    let mut game: Game<SocketAddr> = Game::new();

    let mut buf = [0; 1024];

    println!("Even and Odd Game!\nServer running and waiting for connections.");

    loop {
        let (bytes_received, source) = socket.recv_from(&mut buf)?;
        let data: Vec<u8> = buf[..bytes_received].to_vec();
        println!("Data received from \"{}\" : \"{:?}\"", source, data);

        match parse_byte_stream(data) {
            Ok(hand_played) => {
                match game.add_play(hand_played, source) {
                    Ok(_) => {
                        if game.can_guess() {
                            if let Ok(result) = game.guess_winner() {
                                socket.send_to("You've win!!\nGame will refresh automatically!".as_bytes(), result.winner.address)?;
                                socket.send_to("You've lost!\nGame will refresh automatically!".as_bytes(), result.loser.address)?;
                                game.reset();
                                println!("Game refresh!");
                            }
                        }
                    }
                    Err(e) => {
                        socket.send_to(&e.to_string().into_bytes(), source)?;
                    }
                }
            }
            Err(e) => {
                socket.send_to(&e.to_string().into_bytes(), source)?;
            }
        };
    }
}
