use serde::Deserialize;

pub trait Response {
    fn is_ok(&self) -> bool;
}

#[derive(Deserialize)]
pub struct SimpleResponse {
    pub status: String
}
impl Response for SimpleResponse {
    fn is_ok(&self) -> bool {
        self.status == "success"
    }
}

#[derive(Deserialize)]
pub struct FishingResponse {
    pub status: String,
    pub fish: Option<f64>,
}
impl Response for FishingResponse {
    fn is_ok(&self) -> bool {
        self.status == "success"
    }
}
#[derive(Deserialize)]
pub struct LoginResponse {
    pub status: String,
    pub key: Option<String>,
}
impl Response for LoginResponse {
    fn is_ok(&self) -> bool {
        self.status == "success"
    }
}

/* TODO
#[derive(Deserialize)]
pub struct GamblingResponse {
    pub status: String,
    pub winnings: f64,
    pub slot1: String, // Has to be string so "jackpot" is valid
    pub slot2: String,
    pub slot3: String
}
impl Response for GamblingResponse {
    fn is_ok(&self) -> bool {
        self.status == "success"
    }
}*/
