use reqwest::Client;
use dotenvy_macro::dotenv;
use once_cell::sync::Lazy;
use crate::structures::{ApiResponse, AuthenticationResponse, GetClients};

static HTTP_CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        .user_agent("ArslandClientLauncher")
        .build()
        .unwrap()
});

const BASE_URL: &str = dotenv!("BACKEND_URL");

#[tauri::command]
#[allow(non_snake_case)]
pub async fn authentication(login: String, password: String) -> Result<ApiResponse<AuthenticationResponse>, String> {
    let res = HTTP_CLIENT
        .post(format!("{}/api/v1/integrations/auth/signin", BASE_URL))
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
#[allow(non_snake_case)]
pub async fn check_token(accessToken: String) -> Result<ApiResponse<AuthenticationResponse>, String> {
    let res = HTTP_CLIENT
        .post(format!("{}/api/v1/integrations/auth/checkToken", BASE_URL))
        .json(&serde_json::json!({
            "accessToken": accessToken
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
#[allow(non_snake_case)]
pub async fn get_clients(accessToken: String) -> Result<ApiResponse<GetClients>, String> {
    let res = HTTP_CLIENT
        .get(format!("{}/api/v1/profiles", BASE_URL))
        .bearer_auth(accessToken)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .error_for_status()
        .map_err(|e| e.to_string())?
        .json::<ApiResponse<GetClients>>()
        .await
        .map_err(|e| e.to_string())?;

    Ok(res)
}
