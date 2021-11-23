-- Your SQL goes here

CREATE TABLE wallbox_data (
    id SERIAL PRIMARY KEY,
    serial_number BIGINT NOT NULL,
    charging_state BIGINT,
    cable_state BIGINT,
    error_code BIGINT,
    charging_current_phase_1 BIGINT,
    charging_current_phase_2 BIGINT,
    charging_current_phase_3 BIGINT,
    active_power BIGINT,
    total_energy BIGINT,
    voltage_phase_1 BIGINT,
    voltage_phase_2 BIGINT,
    voltage_phase_3 BIGINT,
    power_factor BIGINT,
    max_charging_current BIGINT,
    max_supported_current BIGINT,
    rfid_card BIGINT,
    charged_energy BIGINT
)
