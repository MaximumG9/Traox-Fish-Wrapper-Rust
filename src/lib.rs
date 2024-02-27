mod fishing_session;
use rand::{distributions::Alphanumeric, rngs::ThreadRng, Rng, distributions::DistIter};
use reqwest::Client;
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
    UnknownError(UnknownError),
    JSONError(serde_json::Error),
    ReqwestError(reqwest::Error)
}
impl fmt::Display for Error {
    fn fmt(&self,f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "Unknown error occured on API request")
    }
}

pub struct GambleResult {
    pub slots: [i64; 3],
    pub bet: u128,
    pub winnings: f64
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
                login_key: key_result.ok().unwrap(),
                username: username.to_string(),
                client: client
            };
            Ok(session)
        } else {
            Err(key_result.err().unwrap())
        }
    }

    pub async fn online(&self) -> Result<(), Error> {
        Self::fetch::<requests::SimpleRequest,responses::SimpleResponse>(&self.client, "online",
            &requests::SimpleRequest {
                loginKey: self.login_key.as_str(),
                username: self.username.as_str()
            }).await?;
        Ok(())
    }

    async fn fetch<T: Serialize + ?Sized, U: for<'a> Deserialize<'a> + responses::Response + Debug>(client : &Client, url : &str, body : &T) -> Result<U, Error> {
        let result = client.post("https://traoxfish.us-3.evennode.com/".to_string() + url)
            .json(body)
            .send().await;
        if result.is_ok() {
            let text = result.unwrap().text().await.ok().unwrap();
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

    async fn get_login_key(client: &Client, username: &str, password: &str, rng: ThreadRng) -> Result<String, Error> {
        let browser_key = create_random_uuid(rng);
        let result:responses::LoginResponse = Self::fetch(client, "login",
            &requests::LoginRequest {
                username: username,
                password: password,
                browserKey: browser_key.as_str()
            }).await?;
        Ok(result.key.unwrap())
    }

    pub async fn fish(&self) -> Result<f64, Error> {
        let result:responses::FishingResponse = Self::fetch(&self.client, "fish",
            &requests::SimpleRequest {
                loginKey: self.login_key.as_str(),
                username: self.username.as_str()
            }).await?;
        Ok(result.fish.unwrap())
    }

    pub async fn check_key(&self) -> Result<bool,Error> {
        let result:responses::CheckKeyResponse = Self::fetch(&self.client, "checkkey",
            &requests::SimpleRequest {
                loginKey: self.login_key.as_str(),
                username: self.username.as_str(),
            }).await?;
        Ok(result.validKey)
    }

    pub async fn check_gamble(&self, gamble_amount : u128) -> Result<bool,Error> {
        let result:responses::GambleCheckResponse = Self::fetch(&self.client, "gamble",
            &requests::GambleCheckRequest {
                loginKey: self.login_key.as_str(),
                username: self.username.as_str(),
                bet: gamble_amount,
                check: true
            }).await?;
        Ok(result.canAfford)
    }

    pub async fn gamble(&self, gamble_amount : u128) -> Result<GambleResult, Error> {
        let result:responses::GambleResponse = Self::fetch(&self.client, "gamble",
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
    return string.to_lowercase();
}

#[cfg(test)]
mod tests {

    use crate::FishingSession;

    #[test]
    fn main() {
        test();
    }

    #[tokio::main]
    async fn test() {
        println!("Hello, world!");
        let rng = rand::thread_rng();
        let session_res = FishingSession::new(rng,"Cargovroom","Password123").await;
        if session_res.is_ok() {
            println!("Succeeded to create");
            let session = session_res.ok().unwrap();

            let online_res = session.online().await;

            let key_check_res = session.check_key().await;

            let gamble_res = session.gamble(1).await;

            let fish_res = session.fish().await;

            let gamble_check_res = session.check_gamble(1).await;

            

            if online_res.is_ok() {
                println!("Succeeded to go online");
            } else {
                println!("failed to go online with error:  {0}",online_res.err().unwrap());
            }

            if key_check_res.is_ok() {
                println!("Succeeded to check key: \nvalidKey: {0}",key_check_res.ok().unwrap());
            } else {
                println!("failed to fish with error: {0}",key_check_res.err().unwrap());
            }

            if gamble_res.is_ok() {
                let gamba = gamble_res.ok().unwrap();
                println!("Succeeded to gamble with results: \nbet: {0}\nslots: {1:?}\nwinnings: {2}", gamba.bet,gamba.slots,gamba.winnings);
            } else {
                println!("failed to gamble with error: {0}", gamble_res.err().unwrap());
            }

            if gamble_check_res.is_ok() {
                let gamba = gamble_check_res.ok().unwrap();
                println!("Succeeded to check gamble: \ncanAfford: {0}", gamba);
            } else {
                println!("failed to check gamble with error: {0}",gamble_check_res.err().unwrap());
            }

            if fish_res.is_ok() {
                println!("Succeeded to fish");
            } else {
                println!("failed to fish with error: {0}",fish_res.err().unwrap());
            }

        } else {
            println!("Failed to login with error: {0}", session_res.err().unwrap());
        }
        
    }
}