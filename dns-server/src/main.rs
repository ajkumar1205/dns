use libsql::{params, Builder, Connection};
use simple_dns::rdata::CNAME;
use simple_dns::{
    rdata::RData, Name, Packet, PacketFlag, Question, ResourceRecord, CLASS, OPCODE, RCODE, TYPE,
};
use std::net::SocketAddr;
use std::result::Result;
use std::sync::{Arc, Mutex};
use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket_adr: SocketAddr = "127.0.0.1:53".parse()?;
    let socket_mutex = Arc::new(UdpSocket::bind(socket_adr).await?);

    let db = Builder::new_local("./dns.sqlite").build().await.unwrap();
    let conn = Arc::new(Mutex::new(db.connect()?));

    loop {
        let mut buf = [0u8; 2048];
        let sock = Arc::clone(&socket_mutex);
        let (len, addr) = sock.recv_from(&mut buf).await?;

        let db_conn = Arc::clone(&conn);
        tokio::spawn(async move {
            if let Err(e) = handle_query(sock, &buf, len, addr, db_conn).await {
                eprintln!("Error: {:?}", e);
            }
        });
    }
}

async fn handle_query(
    socket: Arc<UdpSocket>,
    buf: &[u8],
    len: usize,
    addr: SocketAddr,
    conn: Arc<Mutex<Connection>>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("The address of the request is : {:?}", addr);
    println!("The length of the request is : {:?}", len);

    let packet = Packet::parse(buf)?;
    let mut response = packet.clone().into_reply();
    let ans = ResourceRecord::new(
        Name::new("ajaythakur.tech")?,
        CLASS::IN,
        3600,
        RData::CNAME(CNAME(Name::new("ajaythakur.me")?)),
    );
    response.answers.push(ans);

    println!("{:?}", packet);
    println!("#######################");
    println!("#######################");
    println!("{:?}", response);

    socket.send_to(&response.build_bytes_vec()?, addr).await?;

    Ok(())
}
