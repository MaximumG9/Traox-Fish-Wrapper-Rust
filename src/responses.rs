#![allow(non_snake_case)]
use serde::Deserialize;

pub trait Response {
    fn is_ok(&self) -> bool;
}

#[derive(Deserialize,Debug)]
pub struct ErrorResponse {
    pub status: String,
    pub error: Option<String>
}
impl Response for ErrorResponse {
    fn is_ok(&self) -> bool {
        false
    }
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
    pub fish: f64,
}
impl Response for FishingResponse {
    fn is_ok(&self) -> bool {
        self.status == "success"
    }
}
#[derive(Deserialize,Debug)]
pub struct LoginResponse {
    pub status: String,
    pub key: String,
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

// {
// "status":"success",
// "fish":2065523,"rareFish":228,
// "veryRareFish":70,"sharks":35,
// "rareSharks":10,"specialFish":35,
// "whales":2,"fishermen":0,
// "chum":0,"fishBuckets":0,
// "fishingBoatFish":0,"fishingBoatCapacity":479120,
// "level":26,"xpRequired":1832,
// "currentLevelXp":718,
// "notifications":{"friendRequests":0,"sendLogs":0,"guildInvites":0},
// "challengeSetting":"everyone",
// "costs":{
//     "rareFishCost":1584154,
//     "veryRareFishCost":3053669,
//     "sharkCost":199316,
//     "rareSharkCost":454994,
//     "specialFishCost":1207269,
//     "specialFishSellCost":1182550,
//     "whaleCost":100000000,
//     "fishermanCost":25000,
//     "chumCost":40000,
//     "fishBucketCost":10000
// },
// "guildInvites":[],
// "guild":"Fishers",
// "isOwnerOfGuild":false,
// "isMod":false,
// "currentJackpot":309400183,
// "minimumBidForJackpot":3094.00183
// }


#[derive(Deserialize,Debug)]
pub struct Notifications {
    pub friendRequests: u64,
    pub sendLogs: u64,
    pub guildInvites: u64
}

#[derive(Deserialize,Debug)]
pub struct Costs {
    pub rareFishCost: u64,
    pub veryRareFishCost: u64,
    pub sharkCost: u64,
    pub rareSharkCost: u64,
    pub specialFishCost: u64,
    pub spedcialFishSellCost: u64,
    pub fishermenCost: u64,
    pub chumCost: u64,
    pub fishBucketCost: u64
}

#[derive(Deserialize,Debug)]
pub struct Invite {
// TODO
}

#[derive(Deserialize,Debug)]
pub struct GetDataResponse {
    pub status: String,
    pub fish: f64,
    pub rareFish: u128,
    pub veryRareFish: u128,
    pub sharks: u128,
    pub rareSharks: u128,
    pub specialFish: u128,
    pub whales: u32,
    pub fishermen: u128,
    pub chum: u128,
    pub fishBuckets: u128,
    pub fishingBoatFish: u128,
    pub fishingBoatCapacity: u128,
    pub level: u64,
    pub xpRequired: String,
    pub currentLevelXp: String,
    pub notifications: Notifications,
    pub challengeSetting: u128,
    pub costs: Costs,
    pub guildInvites: Vec<Invite>,
    pub guild: String,
    pub isOwnerOfGuiid: bool,
    pub isMod: bool,
    pub currentJackpot: f64,
    pub minimumBidForJackpot: f64
}

impl Response for GetDataResponse {
    fn is_ok(&self) -> bool {
        self.status == "success"
    }
}