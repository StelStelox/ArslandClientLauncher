use crate::structures::{ApiResponse, AuthenticationResponse, ClientsInfoResponse, GetClients};
use dotenvy_macro::dotenv;
use once_cell::sync::Lazy;
use tauri_plugin_http::reqwest::Client;

static HTTP_CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        .user_agent("ArslandClientLauncher")
        .build()
        .unwrap()
});

#[tauri::command]
pub async fn authentication(login: String, password: String) -> Result<ApiResponse<AuthenticationResponse>, String> {
    let res = HTTP_CLIENT
        .post(format!("{}/api/v1/integrations/auth/signin", dotenv!("BACKEND_URL")))
        .json(&serde_json::json!({
            "login": login,
            "password": password
        }))
        .send()
        .await
        .map_err(|e| e.to_string())?
        .error_for_status()
        .map_err(|e| e.to_string())?
        .json::<ApiResponse<AuthenticationResponse>>()
        .await
        .map_err(|e| e.to_string())?;

    Ok(res)
}

#[tauri::command]
pub async fn check_token(access_token: String) -> Result<ApiResponse<AuthenticationResponse>, String> {
    let res = HTTP_CLIENT
        .post(format!("{}/api/v1/integrations/auth/checkToken", dotenv!("BACKEND_URL")))
        .json(&serde_json::json!({
            "accessToken": access_token
        }))
        .send()
        .await
        .map_err(|e| e.to_string())?
        .error_for_status()
        .map_err(|e| e.to_string())?
        .json::<ApiResponse<AuthenticationResponse>>()
        .await
        .map_err(|e| e.to_string())?;

    Ok(res)
}

#[tauri::command]
pub async fn get_clients(access_token: String) -> Result<ApiResponse<Vec<GetClients>>, String> {
    let res = HTTP_CLIENT
        .get(format!("{}/api/v1/profiles", dotenv!("BACKEND_URL")))
        .bearer_auth(access_token)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .error_for_status()
        .map_err(|e| e.to_string())?
        .json::<ApiResponse<Vec<GetClients>>>()
        .await
        .map_err(|e| e.to_string())?;

    Ok(res)
}

#[tauri::command]
pub async fn get_client_info(access_token: String, client_name: String, os_type: String) -> Result<ApiResponse<ClientsInfoResponse>, String> {
    let res = HTTP_CLIENT
        .post(format!("{}/api/v1/profiles/info", dotenv!("BACKEND_URL")))
        .bearer_auth(access_token)
        .json(&serde_json::json!({
            "profileName": client_name,
            "osType": os_type
        }))
        .send()
        .await
        .map_err(|e| e.to_string())?
        .error_for_status()
        .map_err(|e| e.to_string())?
        .json::<ApiResponse<ClientsInfoResponse>>()
        .await
        .map_err(|e| e.to_string())?;

    Ok(res)
}
