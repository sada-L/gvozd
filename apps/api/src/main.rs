mod app;

use crate::app::container::AppContainer;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let server = AppContainer::new().await.server;
    tokio::select! {
        result = server => {
            if let Err(e) = result {
                return Err(std::io::Error::new(std::io::ErrorKind::NetworkDown, e));
            }
        }
    }
    Ok(())
}
