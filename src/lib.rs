mod fishing_session;

#[cfg(test)]
mod tests {

    use crate::fishing_session::FishingSession;

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