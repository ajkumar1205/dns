use libsql::{params, Builder, Connection, Row};
use simple_dns::rdata::CNAME;
use simple_dns::{rdata::RData, Name, Packet, ResourceRecord, CLASS};
use std::net::SocketAddr;
use std::result::Result;
use std::sync::Arc;
use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket_adr: SocketAddr = "127.0.0.1:53".parse()?;
    let socket = Arc::new(UdpSocket::bind(socket_adr).await?);

    let db = Builder::new_local("./dns.sqlite").build().await.unwrap();
    let conn = Arc::new(db.connect()?);

    loop {
        let mut buf = [0u8; 2048];
        let sock = Arc::clone(&socket);
        let (len, addr) = sock.recv_from(&mut buf).await?;

        let db_conn = Arc::clone(&conn);
        tokio::spawn(async move {
            if let Err(e) = handle_query(sock, &buf, len, addr, db_conn).await {
                panic!("Error: {:?}", e);
            }
        });
    }
}

async fn handle_query(
    socket: Arc<UdpSocket>,
    buf: &[u8],
    len: usize,
    addr: SocketAddr,
    conn: Arc<Connection>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("The address of the request is : {:?}", addr);
    println!("The length of the request is : {:?}", len);

    let packet = Packet::parse(buf)?;
    let mut response = packet.clone().into_reply();

    let val = response.questions[0].qname.to_string();

    let mut val = conn
        .query("SELECT * FROM dns WHERE question = ?1", params![val])
        .await?;

    let row: Row;
    if val.column_count() == 1 {
        row = val.next().await?.unwrap();
        let name = row.get_str(3)?;
        let record = ResourceRecord::new(
            Name::new(name)?,
            CLASS::IN,
            row.get(4)?,
            RData::CNAME(CNAME(Name::new(&name)?)),
        );

        response.answers.push(record);
    } else {
        panic!(
            "\nMore then or less then 1 value was found in the database :: {:?}",
            val.column_count()
        )
    }

    let ans = ResourceRecord::new(
        Name::new("ajaythakur.tech")?,
        CLASS::IN,
        3600,
        RData::CNAME(CNAME(Name::new("ajaythakur.tech")?)),
    );
    response.answers.push(ans);

    println!("Question was for {:?}", packet.questions[0].qname);
    println!("Having class {:?}", packet.questions[0].qclass);
    println!("And the type {:?}", packet.questions[0].qtype);

    println!("{:?}", response);

    socket.send_to(&response.build_bytes_vec()?, addr).await?;

    Ok(())
}
