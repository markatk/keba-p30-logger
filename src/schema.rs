table! {
    wallbox_data (id) {
        id -> Int4,
        serial_number -> Int8,
        charging_state -> Nullable<Int8>,
        cable_state -> Nullable<Int8>,
        error_code -> Nullable<Int8>,
        charging_current_phase_1 -> Nullable<Int8>,
        charging_current_phase_2 -> Nullable<Int8>,
        charging_current_phase_3 -> Nullable<Int8>,
        active_power -> Nullable<Int8>,
        total_energy -> Nullable<Int8>,
        voltage_phase_1 -> Nullable<Int8>,
        voltage_phase_2 -> Nullable<Int8>,
        voltage_phase_3 -> Nullable<Int8>,
        power_factor -> Nullable<Int8>,
        max_charging_current -> Nullable<Int8>,
        max_supported_current -> Nullable<Int8>,
        rfid_card -> Nullable<Int8>,
        charged_energy -> Nullable<Int8>,
    }
}
