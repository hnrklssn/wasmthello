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
#[derive(Debug, Deserialize, Serialize, Clone)]
struct Bot {
    name: String,
    creator: String,
    wasm: Box<[u8]>,
    wins: u32,
    losses: u32,
    ties: u32,
}

async fn new_bot(
    Json(input): Json<CreateBot>,
    Extension(db): Extension<BotDb>,
) -> impl IntoResponse {
    if db.read().unwrap().get(&input.name).is_some() {
        Err((StatusCode::BAD_REQUEST, format!("bot with name {} already exists", input.name)))
    } else {
        WasmPlayer::<8>::new(&input.wasm.clone()).map_err(|err|
            (StatusCode::BAD_REQUEST, format!("invalid wasm {}", err.to_string())))?;
        let bot = Bot { wins: 0, losses: 0, ties: 0,
            name: input.name.clone(), creator: input.creator.clone(),
            wasm: input.wasm.clone()
        };
        db.write().unwrap().insert(input.name.clone(), bot.clone());
        Ok((StatusCode::CREATED, Json(bot)))
    }
}

#[derive(Debug, Serialize)]
struct GameResultSmall {
    uuid: Uuid,
    white_player: String,
    black_player: String,
    winner: String,
}

#[derive(Debug, Serialize)]
struct GameList {
    games: Vec<GameResultSmall>,
}

async fn index(
    Extension(db): Extension<GameDb>,
) -> impl IntoResponse {
    let list: Vec<GameResultSmall> = db.read().unwrap().values()
        .map(|game| GameResultSmall {
            uuid: game.uuid, white_player: game.white_player.clone(),
            black_player: game.black_player.clone(), winner: game.winner.clone(),
        }).collect();
    (StatusCode::OK, Json(GameList{ games: list }))
}

async fn bots(
    Extension(db): Extension<BotDb>,
) -> impl IntoResponse {
    let list: Vec<Bot> = db.read().unwrap().values()
        .map(|bot| bot.clone()).collect();
    println!("list: {:?}", list);
    (StatusCode::OK, Json(list))
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct GameResult {
    uuid: Uuid,
    white_player: String,
    black_player: String,
    winner: String,
    moves: Box<[u8]>,
}

async fn game_stats(
    Path(id): Path<Uuid>,
    Extension(db): Extension<GameDb>,
) -> Result<(StatusCode, Json<GameResult>), StatusCode> {
    let result = db.read().unwrap().get(&id)
        .cloned().ok_or(StatusCode::NOT_FOUND)?;
    Ok((StatusCode::OK, Json(result)))
}

type BotDb = Arc<RwLock<HashMap<String, Bot>>>;
type GameDb = Arc<RwLock<HashMap<Uuid, GameResult>>>;

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
