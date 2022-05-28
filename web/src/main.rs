#![feature(generic_const_exprs)]
use axum::{
    error_handling::HandleErrorLayer,
    extract::{Extension, Path},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde::Deserializer;
use serde::de::{self, SeqAccess};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, RwLock},
    time::Duration,
    fmt,
};
use uuid::Uuid;
use tower::{BoxError, ServiceBuilder};
use wasmthello;
use wasmthello::WasmPlayer;
use wasmthello::Player;
use std::thread;

#[tokio::main]
async fn main() {
    let bot_db = BotDb::default();
    let game_db = GameDb::default();

    let app = Router::new()
        .route("/games", get(index))
        .route("/game/:id", get(game_stats))
        .route("/bots", get(bots))
        .route("/new-bot", post(new_bot))
        // Add middleware to all routes
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|error: BoxError| async move {
                    if error.is::<tower::timeout::error::Elapsed>() {
                        Ok(StatusCode::REQUEST_TIMEOUT)
                    } else {
                        Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Unhandled internal error: {}", error),
                        ))
                    }
                }))
                .timeout(Duration::from_secs(10))
                .layer(Extension(bot_db))
                .layer(Extension(game_db))
                .into_inner(),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct CreateBot {
    name: String,
    creator: String,
    // Wasmtime is fine with the byte array containing either the raw wasm
    // or the byte representation of the text format, so accept either here.
    #[serde(deserialize_with = "deserialize_string_or_byte_array")]
    wasm: Box<[u8]>,
}
#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
struct Bot<'a> {
    name: &'a str,
    creator: &'a str,
    wasm: &'a[u8],
    wins: u32,
    losses: u32,
    ties: u32,
}

async fn new_bot(
    Json(input): Json<CreateBot>,
    Extension(db): Extension<BotDb<'static>>,
    Extension(game_db): Extension<GameDb<'static>>,
) -> impl IntoResponse {
    let mut bot_map = db.write().unwrap(); // RwLock needs to be held the entire time
    if bot_map.get(input.name.as_str()).is_some() {
        Err((StatusCode::BAD_REQUEST, format!("bot with name {} already exists", input.name)))
    } else {
        WasmPlayer::<8>::new(&input.wasm.clone()).map_err(|err|
            (StatusCode::BAD_REQUEST, format!("invalid wasm {}", err.to_string())))?;
        let name = string_to_static_str(input.name);
        let bot = Bot { wins: 0, losses: 0, ties: 0,
            name, creator: string_to_static_str(input.creator),
            wasm: Box::leak(input.wasm)
        };
        let bots: Vec<&'static str> = bot_map.keys().map(|s| *s).collect(); // fetching the existing bot names while lock is still held prevents duplicated battles
        bot_map.insert(name, bot.clone());
        drop(bot_map);
        {
            let db = db.clone();
            let game_db = game_db.clone();
            let bot = bot.clone();
            let bots = bots.clone();
            thread::spawn(move || battle_bots::<8>(db.clone(), game_db.clone(), bot.clone(), bots.clone()));
        }{
            let db = db.clone();
            let game_db = game_db.clone();
            let bot = bot.clone();
            let bots = bots.clone();
        thread::spawn(move || battle_bots::<12>(db.clone(), game_db.clone(), bot.clone(), bots.clone()));
        }{
            let db = db.clone();
            let game_db = game_db.clone();
            let bot = bot.clone();
            let bots = bots.clone();
        thread::spawn(move || battle_bots::<16>(db.clone(), game_db.clone(), bot.clone(), bots.clone()));
        }
        Ok((StatusCode::CREATED, Json(bot)))
    }
}

#[derive(Debug, Serialize)]
struct GameResultSmall<'a> {
    uuid: Uuid,
    white_player: &'a str,
    black_player: &'a str,
    winner: &'a str,
    board_size: usize,
}

#[derive(Debug, Serialize)]
struct GameList<'a> {
    games: Vec<GameResultSmall<'a>>,
}

async fn index(
    Extension(db): Extension<GameDb<'static>>,
) -> impl IntoResponse {
    let list: Vec<GameResultSmall> = db.read().unwrap().values()
        .map(|game| GameResultSmall {
            uuid: game.uuid, white_player: game.white_player.clone(),
            black_player: game.black_player.clone(), winner: game.winner.clone(),
            board_size: game.board_size,
        }).collect();
    (StatusCode::OK, Json(GameList{ games: list }))
}

async fn bots(
    Extension(db): Extension<BotDb<'static>>,
) -> impl IntoResponse {
    let list: Vec<Bot> = db.read().unwrap().values()
        .map(|bot| bot.clone()).collect();
    println!("list: {:?}", list);
    (StatusCode::OK, Json(list))
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct GameResult<'a> {
    uuid: Uuid,
    white_player: &'a str,
    black_player: &'a str,
    winner: &'a str,
    moves: Box<[u8]>,
    board_size: usize,
    misplay: bool,
}

async fn game_stats(
    Path(id): Path<Uuid>,
    Extension(db): Extension<GameDb<'static>>,
) -> Result<(StatusCode, Json<GameResult<'static>>), StatusCode> {
    let result = db.read().unwrap().get(&id)
        .cloned().ok_or(StatusCode::NOT_FOUND)?;
    Ok((StatusCode::OK, Json(result)))
}

fn battle_bots<'a, const N: usize>(db: BotDb<'a>, game_db: GameDb<'a>, contender: Bot<'a>, bot_list: Vec<&'a str>) -> Option<()> where [(); N*N]: Sized {
    let bots = {
        let read = db.read().unwrap();
        bot_list.into_iter().map(|name| (name, read.get(&name).expect("bot removed?").wasm))
            .collect::<Vec<_>>()
    };
    let white_results = bots.iter().map(|params| {
        let name = params.0;
        let mut contender_player = WasmPlayer::<N>::new(&contender.wasm).unwrap();
        let mut opponent_player = WasmPlayer::<N>::new(&params.1).unwrap();
        let game = wasmthello::play_game(&mut contender_player, &mut opponent_player);
        let winner = match game.winner() {
            Some(Player::White) => contender.name,
            Some(Player::Black) => name,
            None => "Tie"
        };
        GameResult {
            uuid: Uuid::new_v4(),
            white_player: contender.name,
            black_player: name,
            winner,
            moves: game.move_list().clone().into_boxed_slice(),
            board_size: N,
            misplay: game.is_misplay(),
        }});
    let black_results = bots.iter().map(|params| {
        let name = params.0;
        let mut contender_player = WasmPlayer::<N>::new(&contender.wasm).unwrap();
        let mut opponent_player = WasmPlayer::<N>::new(&params.1).unwrap();
        let game = wasmthello::play_game(&mut opponent_player, &mut contender_player);
        let winner = match game.winner() {
            Some(Player::White) => name,
            Some(Player::Black) => contender.name,
            None => "Tie"
        };
        GameResult {
            uuid: Uuid::new_v4(),
            white_player: name,
            black_player: contender.name,
            winner,
            moves: game.move_list().clone().into_boxed_slice(),
            board_size: N,
            misplay: game.is_misplay(),
        }});
    let results = white_results.chain(black_results).collect::<Vec<_>>();
    {
        let mut write = game_db.write().unwrap();
        for result in &results {
            write.insert(result.uuid, result.clone());
        }
    } // drop write lock
    {
        let mut write = db.write().unwrap();
        let mut contender = *write.get(contender.name)?; // Update score in case other games have occurred since this bot was created
        for result in results {
            let mut opponent = *if result.white_player == contender.name {
                write.get(&result.black_player)
            } else {
                write.get(&result.white_player)
            }?;

            if result.winner == contender.name {
                contender.wins += 1;
                opponent.losses += 1;
            } else if result.winner == opponent.name {
                contender.losses += 1;
                opponent.wins += 1;
            } else {
                assert!(result.winner == "Tie");
                contender.ties += 1;
                opponent.ties += 1;
            }
            write.insert(opponent.name, opponent);
        }
        write.insert(contender.name, contender);
    } // drop write lock
    Some(())
}

type BotDb<'a> = Arc<RwLock<HashMap<&'a str, Bot<'a>>>>;
type GameDb<'a> = Arc<RwLock<HashMap<Uuid, GameResult<'a>>>>;

struct DeserializeStringOrByteArrayVisitor;

impl<'de> de::Visitor<'de> for DeserializeStringOrByteArrayVisitor {
    type Value = Box<[u8]>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a byte array or a string")
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Box::from(v))
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error>
    {
        let mut vec = Vec::new();
        if let Some(size) = seq.size_hint() {
            vec.resize_with(size, Default::default);
        }
        loop {
            match seq.next_element()? {
                Some(elem) => vec.push(elem),
                None => break
            };
        };
        Ok(Box::from(vec.as_slice()))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Box::from(v.as_bytes()))
    }
}

fn deserialize_string_or_byte_array<'de, D>(deserializer: D) -> Result<Box<[u8]>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(DeserializeStringOrByteArrayVisitor)
}

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}
