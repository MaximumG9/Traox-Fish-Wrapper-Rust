mod fishing_session;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let rng = rand::thread_rng();
    let session_res = fishing_session::FishingSession::new(rng,"Username","Password").await;
    if session_res.is_ok() {
        println!("Succeeded to create");
        let session = session_res.ok().unwrap();
        let res = session.online().await;
        if res.is_ok() {
            println!("Succeeded to go online");
        } else {
            println!("failed to go online with error:  {0}",res.err().unwrap());
        }

        let res = session.fish().await;
        if res.is_ok() {
            println!("Succeeded to fish");
        } else {
            println!("failed to fish with error: {0}",res.err().unwrap());
        }
    } else {
        println!("Failed to create with error: {0}", session_res.err().unwrap());
    }
    
}

