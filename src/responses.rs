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

#[derive(Deserialize,Debug)]
pub struct ViewProfileResponse {
    pub status: String,
    pub displayName: String,
    pub level: u64,
    pub fish: f64,
    pub rareFish: u128,
    pub veryRareFish: u128,
    pub sharks: u128,
    pub rareSharks: u128,
    pub whales: u32,
    pub specialFish: u128,
    pub allTimeFish: f64,
    pub fishGambled: f64,
    pub profilePicture: String,
    pub joinDate: String,
    pub lastOnlineDate: String,
    pub friendStatus: String,
    pub rank: String,
    pub fishermen: u128,
    pub chum: u128,
    pub fishBuckets: u128,
    pub playtime: String,
    pub friends: u128,
    pub challengeSetting: String,
    pub fishPerClick: u128,
    pub fishPerSecond: u128,
    pub guild: String,
    pub isOwnerOfGuiid: bool
}
impl Response for ViewProfileResponse {
    fn is_ok(&self) -> bool {
        self.status == "success"
    }
}