use serde::Serialize;

#[allow(non_snake_case)]
#[derive(Serialize)]
pub struct SimpleRequest<'a> {
    pub loginKey: &'a str,
    pub username: &'a str
}

#[allow(non_snake_case)]
#[derive(Serialize)]
pub struct GambleRequest<'a> {
    pub loginKey: &'a str,
    pub username: &'a str,
    pub bet: f64
}

#[allow(non_snake_case)]
#[derive(Serialize)]
pub struct LoginRequest<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub browserKey: &'a str,
}