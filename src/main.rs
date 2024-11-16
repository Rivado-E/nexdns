use std::net::UdpSocket;

#[derive(Debug)]
struct DnsHeader {
    id: u16,
    flags: u16,
    questions: u16,
    answers: u16,
    authority: u16,
    additional: u16,
}

impl DnsHeader {
    fn from_bytes(bytes: &[u8]) -> Option<DnsHeader> {
        if bytes.len() < 12 {
            return None;
        }
    }
}

fn main() -> std::io::Result<()> {
    {
        let socket = UdpSocket::bind("127.0.0.1:6969")?;

        while true {
            let mut buf = [0; 512]; // gets cut off after 12
            let (size, src) = socket.recv_from(&mut buf)?;
            let header = DnsHeader::from_bytes(&buf);
            println!("{:?}", header);
        }
    }
    Ok(())
}
