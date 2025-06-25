use std::path::PathBuf;
use serde::{Deserialize, Serialize};


// TODO Возможно надо будет создать отдельный пакет и вынести все структуры в отдельный проект
#[derive(Default)]
#[allow(non_snake_case)]
pub struct StorageData {
    pub storage_dir: PathBuf,
    pub accessToken: PathBuf,
}

// TODO надо глянуть какие должны быть строки приватные, а какие публичные
#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct ApiResponse<T> {
    pub status: String,
    pub statusCode: u16,
    pub message: String,
    pub errors: Vec<String>,
    pub data: Option<T>
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct AuthenticationResponse {
    pub textureSkinUrl: String,
    pub textureCloakUrl: String,
    pub textureSkinGuid: String,
    pub textureCloakGuid: String,
    pub fullSkinUrl: Option<String>,
    pub uuid: String,
    pub name: String,
    pub accessToken: String,
    pub expiredDate: String
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct GetClients {
    pub name: String,
    pub displayName: String,
    pub createDate: String,
    pub description: String,
    pub gameVersion: String,
    pub launchVersion: String,
    pub iconBase64: Option<String>,
    pub background: Option<String>,
    pub isEnabled: bool,
    pub priority: u8,
    pub jvmArguments: Option<String>,
    pub gameArguments: Option<String>,
    pub state: u16,
    pub loader: u16,
    pub servers: Option<Vec<Server>>
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Server {
    pub name: String,
    pub version: String,
    pub isOnline: bool,
    pub online: u32,
    pub maxOnline: u32
}