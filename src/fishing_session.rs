use rand::{distributions::Alphanumeric, rngs::ThreadRng, Rng, distributions::DistIter};
use reqwest::Client;
use serde::{Serialize, Deserialize};
use std::fmt;
use std::error;
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

#[derive(Debug, Clone)]
pub struct SlotParsingError { slot: String }

impl fmt::Display for SlotParsingError {
    fn fmt(&self,f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "Slot {0} was not parsed correctly (maybe new slot result was added?)",self.slot)
    }
}

impl error::Error for SlotParsingError {}

#[derive(Debug)]
pub enum Error {
    //SlotParsingError(SlotParsingError),
    UnknownError(UnknownError),
    ReqwestError(reqwest::Error)
}
impl fmt::Display for Error {
    fn fmt(&self,f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "Unknown error occured on API request")
    }
}

/* IN PROGRESS
pub struct GambleResult {
    slots: [i32; 3],
    bet: f64,
    winnings: f64
}*/

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

    async fn fetch<T: Serialize + ?Sized, U: for<'a> Deserialize<'a> + responses::Response>(client : &Client, url : &str, body : &T) -> Result<U, Error> {
        let result = client.post("https://traoxfish.us-3.evennode.com/".to_string() + url)
            .json(body)
            .send().await;
        if result.is_ok() {
            let json_result:Result<U, reqwest::Error> = result.ok().unwrap().json().await;
            if json_result.is_ok() {
                let response: U = json_result.ok().unwrap();
                if response.is_ok() {
                    return Ok(response);
                } else {
                    return Err(Error::UnknownError(UnknownError));
                }
            } else {
                return Err(Error::ReqwestError(json_result.err().unwrap()));
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
    /* Finish Implementing
    pub async fn gamble(&self, gamble_amount : f64) -> Result<GambleResult, Error> {
        let result:responses::GamblingResponse = Self::fetch(&self.client, "fish",
            &requests::GambleRequest {
                loginKey: self.login_key.as_str(),
                username: self.username.as_str(),
                bet: gamble_amount
            }).await?;

        let mut slot_one_value = 0;
        if result.slot1 != "jackpot" {
            let res = result.slot1.parse();
            if res.is_ok() {
                slot_one_value = res.ok().unwrap();
            } else {
                return Err(Error::SlotParsingError(SlotParsingError{slot: result.slot1}));
            }
        }
        let mut slot_two_value = 0;
        if result.slot2 != "jackpot" {
            let res = result.slot2.parse();
            if res.is_ok() {
                slot_two_value = res.ok().unwrap();
            } else {
                return Err(Error::SlotParsingError(SlotParsingError{slot: result.slot2}));
            }
        }
        let mut slot_three_value = 0;
        if result.slot3 != "jackpot" {
            let res = result.slot3.parse();
            if res.is_ok() {
                slot_three_value = res.ok().unwrap();
            } else {
                return Err(Error::SlotParsingError(SlotParsingError{slot: result.slot3}));
            }
        }
        Ok(GambleResult {
            slots: [slot_one_value,slot_two_value,slot_three_value],
            bet: gamble_amount,
            winnings: result.winnings
        })
    }*/
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