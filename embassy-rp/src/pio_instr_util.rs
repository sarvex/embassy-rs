use pio::{InSource, InstructionOperands, JmpCondition, OutDestination, SetDestination};

use crate::pio::{PioInstance, PioStateMachine};

pub fn set_x<PIO: PioInstance, const SM: usize>(sm: &mut PioStateMachine<PIO, SM>, value: u32) {
    const OUT: u16 = InstructionOperands::OUT {
        destination: OutDestination::X,
        bit_count: 32,
    }
    .encode();
    sm.tx().push(value);
    sm.exec_instr(OUT);
}

pub fn get_x<PIO: PioInstance, const SM: usize>(sm: &mut PioStateMachine<PIO, SM>) -> u32 {
    const IN: u16 = InstructionOperands::IN {
        source: InSource::X,
        bit_count: 32,
    }
    .encode();
    sm.exec_instr(IN);
    sm.rx().pull()
}

pub fn set_y<PIO: PioInstance, const SM: usize>(sm: &mut PioStateMachine<PIO, SM>, value: u32) {
    const OUT: u16 = InstructionOperands::OUT {
        destination: OutDestination::Y,
        bit_count: 32,
    }
    .encode();
    sm.tx().push(value);
    sm.exec_instr(OUT);
}

pub fn get_y<PIO: PioInstance, const SM: usize>(sm: &mut PioStateMachine<PIO, SM>) -> u32 {
    const IN: u16 = InstructionOperands::IN {
        source: InSource::Y,
        bit_count: 32,
    }
    .encode();
    sm.exec_instr(IN);

    sm.rx().pull()
}

pub fn set_pindir<PIO: PioInstance, const SM: usize>(sm: &mut PioStateMachine<PIO, SM>, data: u8) {
    let set: u16 = InstructionOperands::SET {
        destination: SetDestination::PINDIRS,
        data,
    }
    .encode();
    sm.exec_instr(set);
}

pub fn set_pin<PIO: PioInstance, const SM: usize>(sm: &mut PioStateMachine<PIO, SM>, data: u8) {
    let set: u16 = InstructionOperands::SET {
        destination: SetDestination::PINS,
        data,
    }
    .encode();
    sm.exec_instr(set);
}

pub fn set_out_pin<PIO: PioInstance, const SM: usize>(sm: &mut PioStateMachine<PIO, SM>, data: u32) {
    const OUT: u16 = InstructionOperands::OUT {
        destination: OutDestination::PINS,
        bit_count: 32,
    }
    .encode();
    sm.tx().push(data);
    sm.exec_instr(OUT);
}
pub fn set_out_pindir<PIO: PioInstance, const SM: usize>(sm: &mut PioStateMachine<PIO, SM>, data: u32) {
    const OUT: u16 = InstructionOperands::OUT {
        destination: OutDestination::PINDIRS,
        bit_count: 32,
    }
    .encode();
    sm.tx().push(data);
    sm.exec_instr(OUT);
}

pub fn exec_jmp<PIO: PioInstance, const SM: usize>(sm: &mut PioStateMachine<PIO, SM>, to_addr: u8) {
    let jmp: u16 = InstructionOperands::JMP {
        address: to_addr,
        condition: JmpCondition::Always,
    }
    .encode();
    sm.exec_instr(jmp);
}
