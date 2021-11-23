use std::convert::From;
use crate::wallbox::Wallbox;
use crate::schema::wallbox_data;

#[derive(Insertable)]
#[table_name="wallbox_data"]
pub struct NewWallboxData {
    pub serial_number: i64,
    pub charging_state: Option<i64>,
    pub cable_state: Option<i64>,
    pub error_code: Option<i64>,
    pub charging_current_phase_1: Option<i64>,
    pub charging_current_phase_2: Option<i64>,
    pub charging_current_phase_3: Option<i64>,
    pub active_power: Option<i64>,
    pub total_energy: Option<i64>,
    pub voltage_phase_1: Option<i64>,
    pub voltage_phase_2: Option<i64>,
    pub voltage_phase_3: Option<i64>,
    pub power_factor: Option<i64>,
    pub max_charging_current: Option<i64>,
    pub max_supported_current: Option<i64>,
    pub rfid_card: Option<i64>,
    pub charged_energy: Option<i64>
}

impl From<&Wallbox> for NewWallboxData {
    fn from(item: &Wallbox) -> Self {
        NewWallboxData {
            serial_number: item.serial_number as i64,
            charging_state: Some(item.charging_state as i64),
            cable_state: Some(item.cable_state as i64),
            error_code: Some(item.error_code as i64),
            charging_current_phase_1: Some(item.charging_current_phase_1 as i64),
            charging_current_phase_2: Some(item.charging_current_phase_2 as i64),
            charging_current_phase_3: Some(item.charging_current_phase_3 as i64),
            active_power: Some(item.active_power as i64),
            total_energy: Some(item.total_energy as i64),
            voltage_phase_1: Some(item.voltage_phase_1 as i64),
            voltage_phase_2: Some(item.voltage_phase_2 as i64),
            voltage_phase_3: Some(item.voltage_phase_3 as i64),
            power_factor: Some(item.power_factor as i64),
            max_charging_current: Some(item.max_charging_current as i64),
            max_supported_current: Some(item.max_supported_current as i64),
            rfid_card: Some(item.rfid_card as i64),
            charged_energy: Some(item.charged_energy as i64)
        }
    }
}
