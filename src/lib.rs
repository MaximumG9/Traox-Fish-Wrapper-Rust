use rand::{distributions::Alphanumeric, rngs::ThreadRng, Rng, distributions::DistIter};
use reqwest::Client;
use responses::SimpleResponse;
use serde::{Serialize, Deserialize};
use std::fmt;
use std::error;
use std::fmt::Debug;
mod requests;
mod responses;

#[derive(Debug, Clone)]
pub struct UnknownError;

impl fmt::Display for UnknownError {
    fn fmt(&self,f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "Unknown error occured on API request (maybe you forgot to keep the account online?)")
    }
}

impl error::Error for UnknownError {}


#[derive(Debug)]
pub enum Error {
    UnknownError(UnknownError), // The server returns an unknown error
    JSONError(serde_json::Error),
    ReqwestError(reqwest::Error)
}

pub struct GambleResult {
    pub slots: [i64; 3],
    pub bet: u128,
    pub winnings: f64
}

impl fmt::Display for GambleResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "slots: [{},{},{}], bet: {}, winnings: {}", self.slots[0],self.slots[1],self.slots[2], self.bet, self.winnings)
    }
}

pub struct FishingSession {
    login_key: String,
    username: String,
    client: Client
}

impl FishingSession {
    pub async fn new<'b>(rng: ThreadRng, username : &'b str, password : &'b str) -> Result<FishingSession, Error> {
        let client = Client::new();

        let key_result = Self::get_login_key(&client,username, password, rng).await;
        if key_result.is_ok()  {
            let session = FishingSession {
                login_key: key_result.unwrap(),
                username: username.to_string(),
                client: client
            };
            Ok(session)
        } else {
            Err(key_result.err().unwrap())
        }
    }

    pub async fn online(&self) -> Result<(), Error> {
        fetch::<requests::SimpleRequest,responses::SimpleResponse>(&self.client, "online",
            &requests::SimpleRequest {
                loginKey: self.login_key.as_str(),
                username: self.username.as_str()
            }).await?;
        Ok(())
    }

    async fn get_login_key(client: &Client, username: &str, password: &str, rng: ThreadRng) -> Result<String, Error> {
        let browser_key = create_random_uuid(rng);
        let result:responses::LoginResponse = fetch(client, "login",
            &requests::LoginRequest {
                username: username,
                password: password,
                browserKey: browser_key.as_str()
            }).await?;
        Ok(result.key.unwrap())
    }

    pub async fn fish(&self) -> Result<f64, Error> {
        let result:responses::FishingResponse = fetch(&self.client, "fish",
            &requests::SimpleRequest {
                loginKey: self.login_key.as_str(),
                username: self.username.as_str()
            }).await?;
        Ok(result.fish.unwrap())
    }

    pub async fn check_key(&self) -> Result<bool,Error> {
        let result:responses::CheckKeyResponse = fetch(&self.client, "checkkey",
            &requests::SimpleRequest {
                loginKey: self.login_key.as_str(),
                username: self.username.as_str(),
            }).await?;
        Ok(result.validKey)
    }

    pub async fn check_gamble(&self, gamble_amount : u128) -> Result<bool,Error> {
        let result:responses::GambleCheckResponse = fetch(&self.client, "gamble",
            &requests::GambleCheckRequest {
                loginKey: self.login_key.as_str(),
                username: self.username.as_str(),
                bet: gamble_amount,
                check: true
            }).await?;
        Ok(result.canAfford)
    }

    pub async fn gamble(&self, gamble_amount : u128) -> Result<GambleResult, Error> {
        let result:responses::GambleResponse = fetch(&self.client, "gamble",
            &requests::GambleRequest {
                loginKey: self.login_key.as_str(),
                username: self.username.as_str(),
                bet: gamble_amount
            }).await?;

        let mut slot_one_value:i64 = 0;
        if !result.slot1.is_string() {
            let res = result.slot1.as_i64();
            slot_one_value = res.unwrap();
        }
        let mut slot_two_value:i64 = 0;
        if !result.slot2.is_string() {
            let res = result.slot2.as_i64();
            slot_two_value = res.unwrap();
        }
        let mut slot_three_value:i64 = 0;
        if !result.slot3.is_string() {
            let res = result.slot3.as_i64();
            slot_three_value = res.unwrap();
        }
        Ok(GambleResult {
            slots: [slot_one_value,slot_two_value,slot_three_value],
            bet: gamble_amount,
            winnings: result.winnings
        })
    }

    pub async fn view_profile(&self, username : &str) -> Result<responses::ViewProfileResponse, Error> {
        let result: responses::ViewProfileResponse = fetch(&self.client, "getProfile",
            &requests::ViewProfileRequest {
                username: self.username.as_str(),
                loginKey: self.login_key.as_str(),
                profile: username
            }).await?;
        return Ok(result);
    }
}

pub async fn create_account(client : &Client, username : &str, password : &str, rng: ThreadRng) -> Result<(), Error> {
    let browser_key = create_random_uuid(rng);
    let _: SimpleResponse = fetch(client,"register", &requests::LoginRequest {
        username: username,
        password: password,
        browserKey: browser_key.as_str()
    }).await?;
    Ok(())
}

async fn fetch<T: Serialize + ?Sized, U: for<'a> Deserialize<'a> + responses::Response + Debug>(client : &Client, url : &str, body : &T) -> Result<U, Error> {
    let result = client.post("https://traoxfish.us-3.evennode.com/".to_string() + url)
        .json(body)
        .send().await;
    if result.is_ok() {
        let text = result.unwrap().text().await.unwrap();
        let json_result:Result<U, serde_json::Error> = serde_json::from_str(text.as_str());
        if json_result.is_ok() {
            let response: U = json_result.ok().unwrap();
            if response.is_ok() {
                return Ok(response);
            } else {
                return Err(Error::UnknownError(UnknownError));
            }
        } else {
            return Err(Error::JSONError(json_result.err().unwrap()));
        }
    } else {
        return Err(Error::ReqwestError(result.err().unwrap()));
    }
}

//Creates a uuid in the form of 
// xxxxxxxx-xxxx-4xxx-xxxx-xxxxxxxx
// where x is a random number or lowercase letter
fn create_random_uuid(rng: ThreadRng) -> String {
    let mut iter: DistIter<Alphanumeric, ThreadRng, u8> = rng.sample_iter(Alphanumeric);

    let mut uuid = String::new();
    uuid += create_random_alphanumeric(&mut iter, 8).as_str();
    uuid += "-";
    uuid += create_random_alphanumeric(&mut iter, 4).as_str();
    uuid += "-4";
    uuid += create_random_alphanumeric(&mut iter, 3).as_str();
    uuid += "-";
    uuid += create_random_alphanumeric(&mut iter, 4).as_str();
    uuid += "-";
    uuid += create_random_alphanumeric(&mut iter, 8).as_str();

    uuid
}

fn create_random_alphanumeric(iter: &mut DistIter<Alphanumeric,ThreadRng, u8>, length: usize) -> String {
    let string : String = iter.take(length).map(char::from).collect();
    string.to_lowercase()
}

#[cfg(test)]
mod tests {

    use rand::{distributions::Alphanumeric, Rng};

    use crate::{create_account, create_random_alphanumeric, FishingSession};

    #[test]
    fn creation() { test_creation(); }
    #[test]
    fn fishing() { test_fishing(); }
    #[test]
    fn check_key() { test_check_key(); }
    #[test]
    fn check_gamble() { test_check_gamble(); }
    #[test]
    fn online() { test_online(); }
    #[test]
    fn gamble() { test_gamble(); }
    #[test]
    fn create_acc() { test_create_account(); }
    #[test]
    fn view_profile() { test_view_profile(); }

    #[tokio::main]
    async fn test_creation() {
        let rng = rand::thread_rng();
        let session_res = FishingSession::new(rng,"CreateTest","Password123").await;

        assert!(session_res.is_ok());        
    }

    #[tokio::main]
    async fn test_fishing() {
        let rng = rand::thread_rng();
        let session_res = FishingSession::new(rng,"FishingTest","Password123").await;

        let session = session_res.unwrap();
        let _  = session.online().await;
        assert!(session.fish().await.is_ok());
    }

    #[tokio::main]
    async fn test_check_key() {
        let rng = rand::thread_rng();
        let session_res = FishingSession::new(rng,"KeyTest","Password123").await;

        let session = session_res.unwrap();
        assert!(session.check_key().await.is_ok());
    }

    #[tokio::main]
    async fn test_check_gamble() {
        let rng = rand::thread_rng();
        let session_res = FishingSession::new(rng,"CheckGambleTest","Password123").await;

        let session = session_res.unwrap();
        let _  = session.online().await;
        assert!(session.check_gamble(1).await.is_ok());
    }

    #[tokio::main]
    async fn test_online() {
        let rng = rand::thread_rng();
        let session_res = FishingSession::new(rng,"OnlineTest","Password123").await;

        let session = session_res.unwrap();
        assert!(session.online().await.is_ok());
    }

    #[tokio::main]
    async fn test_gamble() {
        let rng = rand::thread_rng();
        let session_res = FishingSession::new(rng,"GambleTestUnmute","4U'EM)WMHg%46YH").await;

        let session = session_res.unwrap();
        let _  = session.online().await;
        let res = session.gamble(1).await;
        assert!(res.is_ok());
        let gamble_result = res.unwrap();
        println!("{gamble_result}");
    }

    #[tokio::main]
    async fn test_create_account() {
        let client = reqwest::Client::new();
        let rng = rand::thread_rng();
        let username = create_random_alphanumeric(&mut rand::thread_rng().sample_iter(Alphanumeric), 16);

        let res = create_account(&client, username.as_str(), "Password123", rng).await;
        assert!(res.is_ok());
        println!("{username}")
    }

    #[tokio::main]
    async fn test_view_profile() {
        let rng = rand::thread_rng();
        let session_res = FishingSession::new(rng,"GambleTestUnmute","4U'EM)WMHg%46YH").await;

        let session = session_res.unwrap();
        let _  = session.online().await;
        let res = session.view_profile("MaximumG99").await;
        assert!(res.is_ok());
        let prof = res.unwrap();
        println!("{prof:?}");
    }
}