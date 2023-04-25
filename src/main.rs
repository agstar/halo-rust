use tracing::info;

use errors::Error;
use settings::SETTINGS;
use std::net::SocketAddr;

mod app;
mod database;
mod errors;
mod extension;
mod logger;
mod models;
mod routes;
mod settings;
mod utils;

// There are a couple approaches to take when implementing E2E tests. This
// approach adds tests on /src/tests, this way tests can reference modules
// inside the src folder. Another approach would be to have the tests in a
// /tests folder on the root of the project, to do this and be able to import
// modules from the src folder, modules need to be exported as a lib.
#[cfg(test)]
mod tests;

#[tokio::main]
async fn main() {
  let app = app::create_app().await;

  let port = SETTINGS.server.port;
  let address = SocketAddr::from(([127, 0, 0, 1], port));

  info!("Server listening on {}", &address);
  axum::Server::bind(&address)
      .serve(app.into_make_service())
      .await
      .expect("Failed to start server");
}
