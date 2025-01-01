use serde::{Serialize, Deserialize};
use bip39::{Mnemonic, Language};
use tiny_keccak::{Keccak, Hasher};
use hex;
use rand::{RngCore, rngs::OsRng};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub address: String,
    pub balance: f64,
}

pub fn create_wallet(password_hash: &str) -> Result<(String, String), Box<dyn std::error::Error>> {
    // 1. 生成随机熵值
    let mut entropy = [0u8; 16];
    OsRng.fill_bytes(&mut entropy);
    
    // 2. 从熵值生成助记词
    let mnemonic = Mnemonic::from_entropy_in(Language::English, &entropy)?;
    
    // 3. 从助记词生成种子
    let seed = mnemonic.to_seed("");
    
    // 4. 从种子生成私钥
    let mut hasher = Keccak::v256();
    hasher.update(&seed);
    let mut private_key = [0u8; 32];
    hasher.finalize(&mut private_key);
    
    // 5. 生成 FAIC 格式地址
    let mut hasher = Keccak::v256();
    hasher.update(&private_key);
    let mut hash = [0u8; 32];
    hasher.finalize(&mut hash);
    
    // 使用前20字节作为地址基础，并添加 FAIC 前缀
    let address_base = hex::encode(&hash[..20]);
    let address = format!("FAIC{}", address_base);
    
    Ok((
        address,
        mnemonic.to_string()
    ))
}

pub fn import_wallet_from_mnemonic(mnemonic: &str) -> Result<String, Box<dyn std::error::Error>> {
    // 1. 验证助记词
    let mnemonic = Mnemonic::parse_in(Language::English, mnemonic)?;
    
    // 2. 从助记词生成种子
    let seed = mnemonic.to_seed("");
    
    // 3. 从种子生成私钥
    let mut hasher = Keccak::v256();
    hasher.update(&seed);
    let mut private_key = [0u8; 32];
    hasher.finalize(&mut private_key);
    
    // 4. 生成 FAIC 格式地址
    let mut hasher = Keccak::v256();
    hasher.update(&private_key);
    let mut hash = [0u8; 32];
    hasher.finalize(&mut hash);
    
    let address_base = hex::encode(&hash[..20]);
    let address = format!("FAIC{}", address_base);
    
    Ok(address)
}
