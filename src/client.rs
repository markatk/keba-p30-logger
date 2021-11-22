use std::{convert::TryFrom, env};
use log::{info};
use modbus::{Client as ModbusClient, tcp};

use crate::wallbox::{CableState, ChargingState, Wallbox};

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

    pub fn read_wallbox(&mut self) -> modbus::Result<Wallbox> {
        let mut wallbox = Wallbox::default();

        wallbox.charging_state = ChargingState::try_from(self.read_register(1000)?).unwrap();
        wallbox.cable_state = CableState::try_from(self.read_register(1004)?).unwrap();
        wallbox.error_code = self.read_register(1006)?;
        wallbox.charging_current_phase_1 = self.read_register(1008)?;
        wallbox.charging_current_phase_2 = self.read_register(1010)?;
        wallbox.charging_current_phase_3 = self.read_register(1012)?;
        wallbox.serial_number = self.read_register(1014)?;
        wallbox.product_type = self.read_register(1016)?;
        //wallbox.firmware_version = self.read_register(1018)?;
        wallbox.active_power = self.read_register(1020)?;
        wallbox.total_energy = self.read_register(1036)?;
        wallbox.voltage_phase_1 = self.read_register(1040)?;
        wallbox.voltage_phase_2 = self.read_register(1042)?;
        wallbox.voltage_phase_3 = self.read_register(1044)?;
        wallbox.power_factor = self.read_register(1046)?;
        wallbox.max_charging_current = self.read_register(1100)?;
        wallbox.max_supported_current = self.read_register(1110)?;
        wallbox.rfid_card = self.read_register(1500)?;
        wallbox.charged_energy = self.read_register(1502)?;

        Ok(wallbox)
    }

    pub fn update_wallbox(&mut self, wallbox: &mut Wallbox) -> modbus::Result<()> {
        wallbox.charging_state = ChargingState::try_from(self.read_register(1000)?).unwrap();
        wallbox.cable_state = CableState::try_from(self.read_register(1004)?).unwrap();
        wallbox.error_code = self.read_register(1006)?;
        wallbox.charging_current_phase_1 = self.read_register(1008)?;
        wallbox.charging_current_phase_2 = self.read_register(1010)?;
        wallbox.charging_current_phase_3 = self.read_register(1012)?;
        wallbox.active_power = self.read_register(1020)?;
        wallbox.total_energy = self.read_register(1036)?;
        wallbox.voltage_phase_1 = self.read_register(1040)?;
        wallbox.voltage_phase_2 = self.read_register(1042)?;
        wallbox.voltage_phase_3 = self.read_register(1044)?;
        wallbox.power_factor = self.read_register(1046)?;
        wallbox.max_charging_current = self.read_register(1100)?;
        wallbox.max_supported_current = self.read_register(1110)?;
        wallbox.rfid_card = self.read_register(1500)?;
        wallbox.charged_energy = self.read_register(1502)?;

        Ok(())
    }

    fn read_register(&mut self, address: u16) -> modbus::Result<u32> {
        match self.client.read_holding_registers(address, 2) {
            Ok(res) => Ok((res[0] as u32) << 16 | res[1] as u32),
            Err(err) => Err(err)
        }
    }
}

pub fn run() {
    let target_ip = env::var("TARGET_IP").unwrap();

    let mut client = Client::new(&target_ip).unwrap();

    info!("Register 1014: {:?}", client.read_wallbox().unwrap());
}
