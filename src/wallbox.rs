use std::convert::TryFrom;

#[derive(Debug)]
pub enum ChargingState {
    StartUp = 0,
    NotReady = 1,
    WaitingForVehicle = 2,
    Charging = 3,
    Error = 4,
    TemporarilyInterrupted = 5,
    Unknown
}

impl Default for ChargingState {
    fn default() -> Self {
        ChargingState::Unknown
    }
}

impl TryFrom<u32> for ChargingState {
    type Error = ();

    fn try_from(value: u32) -> Result<ChargingState, ()> {
        match value {
            0 => Ok(ChargingState::StartUp),
            1 => Ok(ChargingState::NotReady),
            2 => Ok(ChargingState::WaitingForVehicle),
            3 => Ok(ChargingState::Charging),
            4 => Ok(ChargingState::Error),
            5 => Ok(ChargingState::TemporarilyInterrupted),
            _ => Err(())
        }
    }
}

#[derive(Debug)]
pub enum CableState {
    NoCable = 0,
    CableInStation = 1,
    CableLockedInStation = 3,
    CableInStationAndVehicle = 5,
    CableLockedInStationAndVehicle = 7,
    Unknown
}

impl Default for CableState {
    fn default() -> Self {
        CableState::Unknown
    }
}

impl TryFrom<u32> for CableState {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, ()> {
        match value {
            0 => Ok(CableState::NoCable),
            1 => Ok(CableState::CableInStation),
            3 => Ok(CableState::CableLockedInStation),
            5 => Ok(CableState::CableInStationAndVehicle),
            7 => Ok(CableState::CableLockedInStationAndVehicle),
            _ => Err(())
        }
    }
}

#[derive(Debug, Default)]
pub struct Wallbox {
    pub serial_number: u32,
    pub product_type: u32,
    pub firmware_version: u32,

    pub charging_state: ChargingState,
    pub cable_state: CableState,
    pub error_code: u32,
    pub charging_current_phase_1: u32,
    pub charging_current_phase_2: u32,
    pub charging_current_phase_3: u32,
    pub active_power: u32,
    pub total_energy: u32,
    pub voltage_phase_1: u32,
    pub voltage_phase_2: u32,
    pub voltage_phase_3: u32,
    pub power_factor: u32,
    pub max_charging_current: u32,
    pub max_supported_current: u32,
    pub rfid_card: u32,
    pub charged_energy: u32
}
