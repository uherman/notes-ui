mod models;
mod websocket_handler;

#[macro_use]
extern crate rocket;
extern crate dotenv;

use dotenv::dotenv;
use futures_util::{SinkExt, StreamExt};
use redis::{aio::MultiplexedConnection, AsyncCommands};
use rocket::http::CookieJar;
use std::{borrow::Cow, env};
use websocket_handler::handle_websocket_message;
use ws::frame::{CloseCode, CloseFrame};

/// WebSocket route handler.
///
/// This function handles incoming WebSocket connections and processes messages
/// based on the provided token. If the token is invalid, the connection is closed
/// with an unauthorized status.
///
/// # Arguments
///
/// * `ws` - The WebSocket instance.
/// * `token` - The token used for authentication.
///
/// # Returns
///
/// A WebSocket channel.
#[get("/ws?<username>")]
async fn web_socket(
    ws: ws::WebSocket,
    jar: &CookieJar<'_>,
    username: String,
) -> ws::Channel<'static> {
    let cookie = match jar.get("__Host.__ws") {
        Some(cookie) => cookie.value(),
        None => {
            return ws.channel(move |mut stream| {
                Box::pin(async move {
                    stream
                        .send(ws::Message::Close(Some(CloseFrame {
                            code: CloseCode::Bad(401),
                            reason: Cow::Borrowed("Unauthorized"),
                        })))
                        .await
                        .unwrap();
                    info!("WebSocket closed: No cookie");
                    Ok(())
                })
            });
        }
    };

    let mut db = get_redis_connection().await.unwrap();
    let ws_token: String = match db.hget(format!("user:{}", username), "wsToken").await {
        Ok(token) => token,
        Err(_) => {
            return ws.channel(move |mut stream| {
                Box::pin(async move {
                    stream
                        .send(ws::Message::Close(Some(CloseFrame {
                            code: CloseCode::Bad(401),
                            reason: Cow::Borrowed("Unauthorized"),
                        })))
                        .await
                        .unwrap();
                    info!("WebSocket closed: No token");
                    Ok(())
                })
            });
        }
    };

    if cookie != ws_token {
        return ws.channel(move |mut stream| {
            Box::pin(async move {
                stream
                    .send(ws::Message::Close(Some(CloseFrame {
                        code: CloseCode::Bad(401),
                        reason: Cow::Borrowed("Unauthorized"),
                    })))
                    .await
                    .unwrap();
                info!("WebSocket closed: Unauthorized");
                Ok(())
            })
        });
    }

    ws.channel(move |mut stream| {
        Box::pin(async move {
            while let Some(message) = stream.next().await {
                if let Ok(msg) = message {
                    if msg.is_text() {
                        handle_websocket_message(&mut stream, msg.into_text().unwrap(), db.clone())
                            .await;
                    }
                }
            }
            Ok(())
        })
    })
}

/// Index route handler.
///
/// This function handles requests to the root URL and returns a simple greeting.
///
/// # Returns
///
/// A static string greeting.
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

/// Rocket launch function.
///
/// This function initializes the Rocket framework, loads environment variables,
/// and mounts the routes.
///
/// # Returns
///
/// A Rocket instance.
#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build().mount("/", routes![index, web_socket])
}

/// Fetches an environment variable.
///
/// This function retrieves the value of the specified environment variable.
///
/// # Arguments
///
/// * `key` - The key of the environment variable.
///
/// # Returns
///
/// The value of the environment variable.
fn get_env_var(key: &str) -> String {
    env::var(key).unwrap()
}

/// Establishes a Redis connection.
///
/// This function creates and returns a multiplexed Redis connection.
///
/// # Returns
///
/// A Redis result containing a multiplexed connection.
async fn get_redis_connection() -> redis::RedisResult<MultiplexedConnection> {
    let client = redis::Client::open(get_env_var("REDIS_URL"))?;
    client.get_multiplexed_async_connection().await
}
