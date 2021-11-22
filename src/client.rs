use std::env;
use log::{info};
use modbus::{Client as ModbusClient, tcp};

struct Client {
    client: modbus::Transport
}

impl Client {
    pub fn new(address: &str) -> modbus::Result<Client> {
        match tcp::Transport::new(address) {
            Ok(c) => Ok(Client {
                client: c
            }),
            Err(err) => Err(err.into())
        }
    }

    pub fn read_register(&mut self, address: u16) -> modbus::Result<u32> {
        match self.client.read_holding_registers(address, 2) {
            Ok(res) => Ok((res[0] as u32) << 16 | res[1] as u32),
            Err(err) => Err(err)
        }
    }
}

pub fn run() {
    let target_ip = env::var("TARGET_IP").unwrap();

    let mut client = Client::new(&target_ip).unwrap();

    info!("Register 1014: {}", client.read_register(1500).unwrap());
}
