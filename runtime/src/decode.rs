use std::{os::raw, sync::mpsc::Sender};

pub enum Instr {
    Arith(ArithInstr)
}

pub enum ArithInstr {
    Add
}

pub fn decode(instructions: Vec<u32>, tx: Sender<Instr>) {
    for raw_instr in instructions {
        let cat_code = (raw_instr & 0b1111) as u8;
        //let op_code = (raw_instr & 0b1110000) as u8;

        let instr = match cat_code {
            0b_1000 => Instr::Arith(decode_arith(raw_instr)),
            _ => unreachable!()
        };

        let _ = tx.send(instr);
    }
}

fn decode_arith(raw_instr: u32) -> ArithInstr {
    let op_code = (raw_instr & 0b1110000) as u8 >> 3;

    todo!()
}
