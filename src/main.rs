
use auto_traffic_control::v1::{
    atc_service_client::AtcServiceClient,
    game_service_client::GameServiceClient,
    GetVersionRequest,
    StartGameRequest,
};

const ADDRESS: &'static str = "http://localhost:4747";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    check_version().await?;
    start_game().await?;

    Ok(())
}

async fn check_version() -> Result<(), Box<dyn std::error::Error>> {
    let mut atc_service = AtcServiceClient::connect(ADDRESS).await?;

    let response = atc_service.get_version(GetVersionRequest {}).await?;
    let version_field = response.into_inner().version;

    if let Some(version) = version_field {
        let mut version_string = format!("{}.{}.{}", version.major, version.minor, version.patch);

        if !version.pre.is_empty() {
            version_string.push('-');
            version_string.push_str(&version.pre);
        }

        println!("Auto Traffic Control is running version '{version_string}'");
    } else {
        println!("Requesting the version returned an empty response.");
    }

    Ok(())
}

async fn start_game() -> Result<(), Box<dyn std::error::Error>> {
    let mut game_service = GameServiceClient::connect(ADDRESS).await?;
    let _response = game_service.start_game(StartGameRequest {}).await?;

    Ok(())
}