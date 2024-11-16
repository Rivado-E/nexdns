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

        Some(DnsHeader {
            id: u16::from_be_bytes([bytes[0], bytes[1]]),
            flags: u16::from_be_bytes([bytes[2], bytes[3]]),
            questions: u16::from_be_bytes([bytes[4], bytes[5]]),
            answers: u16::from_be_bytes([bytes[6], bytes[7]]),
            authority: u16::from_be_bytes([bytes[8], bytes[9]]),
            additional: u16::from_be_bytes([bytes[10], bytes[11]]),
        })
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
