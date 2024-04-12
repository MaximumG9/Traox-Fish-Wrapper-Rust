#![allow(non_snake_case)]
use serde::Serialize;

#[derive(Serialize)]
pub struct SimpleRequest<'a> {
    pub loginKey: &'a str,
    pub username: &'a str
}

#[derive(Serialize)]
pub struct GambleRequest<'a> {
    pub username: &'a str,
    pub loginKey: &'a str,
    pub bet: u128
}

#[derive(Serialize)]
pub struct GambleCheckRequest<'a> {
    pub loginKey: &'a str,
    pub username: &'a str,
    pub bet: u128,
    pub check: bool
}

#[derive(Serialize)]
pub struct LoginRequest<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub browserKey: &'a str,
}

#[derive(Serialize)]
pub struct ViewProfileRequest<'a> {
    pub loginKey: &'a str,
    pub username: &'a str,
    pub profile: &'a str
}