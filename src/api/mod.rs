use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use ring::{digest, pbkdf2};
use std::num::NonZeroU32;
use crate::wallet;
use crate::wallet::create_wallet as create_wallet_internal;
use crate::error::NodeResult;

#[derive(Serialize)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateWalletRequest {
    password_hash: String,
    salt: String,
    nonce: String,
    timestamp: i64,
}

pub async fn create_wallet(req: web::Json<CreateWalletRequest>) -> impl Responder {
    let now = chrono::Utc::now().timestamp();
    if (now - req.timestamp).abs() > 300 {
        return HttpResponse::BadRequest().json(ApiResponse::<Value> {
            success: false,
            data: None,
            error: Some("Request expired".to_string()),
        });
    }

    match create_wallet_internal(&req.password_hash) {
        Ok((address, mnemonic)) => {
            HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(json!({
                    "address": address,
                    "mnemonic": mnemonic,
                })),
                error: None,
            })
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(ApiResponse::<Value> {
                success: false,
                data: None,
                error: Some(e.to_string()),
            })
        }
    }
}

#[derive(Deserialize)]
pub struct ImportWalletRequest {
    mnemonic: String,
}

pub async fn import_wallet(req: web::Json<ImportWalletRequest>) -> impl Responder {
    match wallet::import_wallet_from_mnemonic(&req.mnemonic) {
        Ok(address) => {
            HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(json!({
                    "address": address,
                })),
                error: None,
            })
        }
        Err(e) => {
            HttpResponse::BadRequest().json(ApiResponse::<Value> {
                success: false,
                data: None,
                error: Some(e.to_string()),
            })
        }
    }
}

#[derive(Deserialize)]
pub struct TransferRequest {
    from: String,
    to: String,
    amount: f64,
}

pub async fn transfer(_req: web::Json<TransferRequest>) -> impl Responder {
    HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(json!({
            "txHash": "0x...",
        })),
        error: None,
    })
}

pub async fn get_balance(address: web::Path<String>) -> impl Responder {
    let balance = 1000.0;
    
    HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(json!({
            "balance": balance,
        })),
        error: None,
    })
}

pub async fn get_transaction_history(_address: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(json!({
            "transactions": [],
        })),
        error: None,
    })
}
