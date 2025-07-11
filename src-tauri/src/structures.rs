use serde::{Deserialize, Serialize};
use std::path::PathBuf;

// TODO Возможно надо будет создать отдельный пакет и вынести все структуры в отдельный проект
#[derive(Default)]
pub struct StorageData {
    pub storage_dir: PathBuf,
    pub access_token: PathBuf,
}

// TODO надо глянуть какие должны быть строки приватные, а какие публичные
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct ApiResponse<T> {
    pub status: String,
    pub statusCode: u16,
    pub message: String,
    pub errors: Vec<String>,
    pub data: Option<T>,
}

#[derive(Serialize, Deserialize, Debug)]
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
    pub expiredDate: String,
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
    pub servers: Option<Vec<Server>>,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Server {
    pub name: String,
    pub version: String,
    pub isOnline: bool,
    pub online: u32,
    pub maxOnline: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct ClientsInfoResponse {
    pub javaPath: String,
    pub profileName: String,
    pub displayName: String,
    pub minecraftVersion: String,
    pub clientVersion: String,
    pub launchVersion: String,
    pub iconBase64: Option<String>,
    pub description: String,
    pub arguments: String,
    pub isEnabled: bool,
    pub priority: u8,
    pub jvmArguments: String,
    pub gameArguments: String,
    pub hasUpdate: bool,
    pub state: u8,
    pub files: Vec<GetFiles>,
    pub whiteListFolders: Option<Vec<GetWhiteListFolders>>,
    pub whiteListFiles: Option<Vec<GetWhiteListFiles>>,
    pub usersWhiteList: Vec<AuthenticationResponse>,
    pub background: String
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct GetFiles {
    pub name: String,
    pub directory: String,
    pub size: u32,
    pub hash: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct GetWhiteListFolders {
    pub path: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct GetWhiteListFiles {
    pub name: String,
    pub directory: String,
    pub size: u32,
    pub hash: String
}