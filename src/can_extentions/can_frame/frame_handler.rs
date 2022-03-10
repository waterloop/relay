use super::super::can_command::CanCommand;
use super::super::fault_reports::{ BmsFaultReport, MotorControllerFaultReport };
use crate::pod_states::PodState;
use super::super::ack_nack::AckNack;
use byteorder::{ LittleEndian, ByteOrder };

/**
 *
 * CAN FRAME HANDLER
 *
 * Functions for decoding a Can Frame
 */
pub trait FrameHandler {
    fn get_command(&self) -> CanCommand;
}

impl FrameHandler for socketcan::CANFrame {
    fn get_command(&self) -> CanCommand {
        let id = self.id();
        let data = self.data();

        match id {
            0x001 => CanCommand::BmsHealthCheck{battery_pack_current: parse_first_float(data), cell_temperature: parse_second_float(data)},
            0x002 => CanCommand::MotorControllerHealthCheck{ igbt_temp: parse_first_float(data), motor_voltage: parse_second_float(data)},
            0x00A => CanCommand::BmsFaultReport(BmsFaultReport::from(data)),
            0x00B => CanCommand::BmsStateChange(get_state_change_ack(data)),
            0x00C => CanCommand::BmsData1{ battery_pack_voltage: parse_first_float(data), state_of_charge: parse_second_float(data)},
            0x00D => CanCommand::BmsData2{ buck_temperature: parse_first_float(data), bms_current: parse_second_float(data)},
            0x00E => CanCommand::BmsData3{ link_cap_voltage: parse_first_float(data)},
            0x014 => CanCommand::MotorControllerFaultReport(MotorControllerFaultReport::from(data)),
            0x015 => CanCommand::MotorControllerStateChange(get_state_change_ack(data)),
            0x016 => CanCommand::MotorControllerData1{ mc_pod_speed: parse_first_float(data), motor_current: parse_second_float(data) },
            0x017 => CanCommand::MotorControllerData2{ battery_current: parse_first_float(data), battery_voltage: parse_second_float(data) },
            0x01F => CanCommand::PodSpeed{ pod_speed: parse_first_float(data)},
            0x020 => CanCommand::PressureHigh(parse_first_float(data)),
            0x021 => CanCommand::PressureLow1(parse_first_float(data)),
            0x022 => CanCommand::PressureLow2(parse_first_float(data)),
            0x030 => CanCommand::Current5V(parse_first_float(data)),
            0x031 => CanCommand::Current12V(parse_first_float(data)),
            0x032 => CanCommand::Current24V(parse_first_float(data)),
            0x040 => CanCommand::Torchic1([Some(parse_first_float(data)), Some(parse_second_float(data))]),
            0x041 => CanCommand::Torchic2([Some(parse_first_float(data)), Some(parse_second_float(data))]),
            id => CanCommand::Unknown(id)
        }
    }
}

/**
 * @func:  get_state_change_data
 * @brief: A State change CAN frame will be a single byte which represents
 * An ack or a Nack of the state change request
 */
fn get_state_change_ack(data: &[u8]) -> AckNack {
    if data.len() == 0 { AckNack::UNKNOWN }
    else { AckNack::from(data[0]) }
}

/**
 * @func parse_two_floats
 * @brief parse frames consisting of 2 4-byte floats
 */
fn parse_first_float(data: &[u8]) -> f32 {
    LittleEndian::read_f32(&[data[0], data[1], data[2], data[3]])
}

/**
 * @func parse_second_float
 * @brief parse second half of frames consisting of 2 4-byte floats
 */
fn parse_second_float(data: &[u8]) -> f32 {
    LittleEndian::read_f32(&[data[4], data[5], data[6], data[7]])
}
