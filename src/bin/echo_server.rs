use pine::TcpServer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let serv = TcpServer::new("127.0.0.1:9000")?;
    println!("{:#?}", serv);
    serv.start()?;
    Ok(())
}
