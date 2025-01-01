use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use crate::wallet;
use crate::wallet::create_wallet as create_wallet_internal;
use log::{info, error};

#[derive(Serialize)]
struct ApiResponse {
    success: bool,
    data: Option<Value>,
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
        return HttpResponse::BadRequest().json(ApiResponse {
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
                error: None
            })
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(ApiResponse {
                success: false,
                data: None,
                error: Some(e.to_string())
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
                error: None
            })
        }
        Err(e) => {
            HttpResponse::BadRequest().json(ApiResponse {
                success: false,
                data: None,
                error: Some(e.to_string())
            })
        }
    }
}

#[derive(Deserialize)]
pub struct TransferRequest {
    from_address: String,
    to_address: String,
    amount: f64,
}

pub async fn transfer(req: web::Json<TransferRequest>) -> impl Responder {
    info!("Transfer request received: from={}, to={}, amount={}", 
        req.from_address, req.to_address, req.amount);
    
    // 验证地址格式
    if !req.from_address.starts_with("FAIC") || !req.to_address.starts_with("FAIC") {
        return HttpResponse::BadRequest().json(ApiResponse {
            success: false,
            data: None,
            error: Some("Invalid address format".to_string())
        });
    }

    // 验证金额
    if req.amount <= 0.0 {
        return HttpResponse::BadRequest().json(ApiResponse {
            success: false,
            data: None,
            error: Some("Invalid amount".to_string())
        });
    }

    // 执行转账
    match wallet::transfer(&req.from_address, &req.to_address, req.amount).await {
        Ok(tx_hash) => {
            info!("Transfer successful: hash={}", tx_hash);
            HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(json!({
                    "tx_hash": tx_hash
                })),
                error: None
            })
        },
        Err(e) => {
            error!("Transfer failed: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse {
                success: false,
                data: None,
                error: Some(e.to_string())
            })
        }
    }
}

pub async fn get_balance(address: web::Path<String>) -> impl Responder {
    let balance = wallet::get_balance(&address);
    
    HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(json!({
            "balance": balance
        })),
        error: None
    })
}

pub async fn get_transaction_history(_address: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(json!({
            "transactions": [],
        })),
        error: None
    })
}
