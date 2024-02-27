#![allow(non_snake_case)]
use serde::Deserialize;

pub trait Response {
    fn is_ok(&self) -> bool;
}

#[derive(Deserialize,Debug)]
pub struct SimpleResponse {
    pub status: String
}
impl Response for SimpleResponse {
    fn is_ok(&self) -> bool {
        self.status == "success"
    }
}

#[derive(Deserialize,Debug)]
pub struct GambleCheckResponse {
    pub status: String,
    pub canAfford: bool
}
impl Response for GambleCheckResponse {
    fn is_ok(&self) -> bool {
        self.status == "success"
    }
}

#[derive(Deserialize,Debug)]
pub struct FishingResponse {
    pub status: String,
    pub fish: Option<f64>,
}
impl Response for FishingResponse {
    fn is_ok(&self) -> bool {
        self.status == "success"
    }
}
#[derive(Deserialize,Debug)]
pub struct LoginResponse {
    pub status: String,
    pub key: Option<String>,
}
impl Response for LoginResponse {
    fn is_ok(&self) -> bool {
        self.status == "success"
    }
}

#[derive(Deserialize,Debug)]
pub struct GambleResponse {
    pub status: String,
    pub slot1: serde_json::Value, // Has to be value so "jackpot" is valid
    pub slot2: serde_json::Value,
    pub slot3: serde_json::Value,
    pub winnings: f64
}
impl Response for GambleResponse {
    fn is_ok(&self) -> bool {
        self.status == "success"
    }
}

#[derive(Deserialize,Debug)]
pub struct CheckKeyResponse {
    pub status: String,
    pub validKey: bool
}
impl Response for CheckKeyResponse {
    fn is_ok(&self) -> bool {
        self.status == "success"
    }
}