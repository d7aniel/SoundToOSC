use rosc::{encoder, OscMessage, OscPacket, OscType};
use std::net::UdpSocket;
use std::sync::Arc;

pub struct OscSender {
    socket: Arc<UdpSocket>,
}

impl OscSender {
    pub fn new() -> Result<Self, String> {
        // Bind to port 0 (ephemeral client port assigned by OS).
        // This ensures port 9000 is NEVER blocked, allowing Andragon to listen freely!
        let socket = UdpSocket::bind("0.0.0.0:0")
            .map_err(|e| format!("Failed to create UDP socket: {}", e))?;
        
        socket
            .set_nonblocking(true)
            .map_err(|e| format!("Failed to set non-blocking socket: {}", e))?;

        Ok(Self {
            socket: Arc::new(socket),
        })
    }

    pub fn send_message(&self, target_ip: &str, target_port: u16, address: &str, values: &[f32]) -> Result<(), String> {
        let args = values.iter().map(|&v| OscType::Float(v)).collect();
        let msg = OscMessage {
            addr: address.to_string(),
            args,
        };
        let packet = OscPacket::Message(msg);
        let bytes = encoder::encode(&packet)
            .map_err(|e| format!("OSC encoding error: {:?}", e))?;

        let target_addr = format!("{}:{}", target_ip, target_port);
        self.socket
            .send_to(&bytes, &target_addr)
            .map_err(|e| format!("Failed to send OSC packet to {}: {}", target_addr, e))?;

        Ok(())
    }
}
