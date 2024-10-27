use crate::asm::{Instruction, Operand, ParseOperandError};

pub struct Opcode {
    base: u16,
    vx: Option<Operand>,
    vy: Option<Operand>,
    nnn: Option<Operand>,
    kk: Option<Operand>,
    n: Option<Operand>,
}
impl Opcode {
    fn new(base: u16) -> Self {
        Self {
            base: base,
            vx: None,
            vy: None,
            nnn: None,
            kk: None,
            n: None,
        }
    }

    fn set_vx(self, value: Operand) -> Self {
        Opcode {
            vx: Some(value),
            ..self
        }
    }
    fn set_vy(self, value: Operand) -> Self {
        Opcode {
            vy: Some(value),
            ..self
        }
    }
    fn set_nnn(self, value: Operand) -> Self {
        Opcode {
            nnn: Some(value),
            ..self
        }
    }
    fn set_kk(self, value: Operand) -> Self {
        Opcode {
            kk: Some(value),
            ..self
        }
    }
    fn set_n(self, value: Operand) -> Self {
        Opcode {
            n: Some(value),
            ..self
        }
    }

    pub fn to_bytes(&self) -> Result<u16, ParseOperandError> {
        let nnn = match &self.nnn {
            Some(value) => Some(value.clone().parse()?),
            None => None,
        };
        let vx = match &self.vx {
            Some(value) => Some(value.clone().parse()?),
            None => None,
        };
        let vy = match &self.vy {
            Some(value) => Some(value.clone().parse()?),
            None => None,
        };
        let kk = match &self.kk {
            Some(value) => Some(value.clone().parse()?),
            None => None,
        };
        let n = match &self.n {
            Some(value) => Some(value.clone().parse()?),
            None => None,
        };

        let bytes: u16 = match (nnn, vx, vy, kk, n) {
            (Some(nnn), None, None, None, None) => self.base | nnn,
            (None, Some(vx), None, None, None) => self.base | (vx as u16) << 0x8,
            (None, Some(vx), Some(vy), None, None) => {
                self.base | (vx as u16) << 0x8 | (vy as u16) << 0x4
            }
            (None, Some(vx), None, Some(kk), None) => self.base | (vx as u16) << 0x8 | (kk as u16),
            (None, Some(vx), Some(vy), None, Some(n)) => {
                self.base | (vx as u16) << 0x8 | (vy as u16) << 0x4 | (n as u16)
            }
            (None, None, None, None, Some(n)) => self.base | (n as u16),
            (None, None, None, None, None) => self.base,
            (_, _, _, _, _) => {
                return Err(ParseOperandError {
                    message: format!("Invalid opcode: {:?}", self),
                })
            }
        };

        Ok(bytes)
    }

    pub fn from_instruction(instruction: Instruction) -> Option<Opcode> {
        let mnemonic = instruction.mnemonic;
        let operands = instruction.args;

        let opcode = match mnemonic.to_uppercase().as_str() {
            "CLS" => Opcode::new(0x00E0),
            "RET" => Opcode::new(0x00EE),
            "SYS" => Opcode::new(0x0000).set_nnn(operands[0].clone()),
            "JP" => match operands[0].repr.as_str() {
                "V0" | "v0" => Opcode::new(0xB000).set_nnn(operands[1].clone()),
                _ => Opcode::new(0x1000).set_nnn(operands[0].clone()),
            },
            "CALL" => Opcode::new(0x2000).set_nnn(operands[0].clone()),
            "SE" => match operands[1].is_register() {
                true => Opcode::new(0x5000)
                    .set_vx(operands[0].clone())
                    .set_vy(operands[1].clone()),
                false => Opcode::new(0x3000)
                    .set_vx(operands[0].clone())
                    .set_kk(operands[1].clone()),
            },
            "SCD" => {
                //SCD nibble
                Opcode::new(0x00C0).set_n(operands[0].clone())
            }
            "SCR" => {
                //SCR
                Opcode::new(0x00FB)
            }
            "SCL" => {
                //SCL
                Opcode::new(0x00FC)
            }
            "EXIT" => {
                //EXIT
                Opcode::new(0x00FD)
            }
            "LOW" => {
                //LOW
                Opcode::new(0x00FE)
            }
            "HIGH" => {
                //HIGH
                Opcode::new(0x00FF)
            }
            "DRW" => {
                //DRW Vx, Vy, nibble
                Opcode::new(0xD000)
                    .set_vx(operands[0].clone())
                    .set_vy(operands[1].clone())
                    .set_n(operands[2].clone())
            }
            "LD" => {
                match (
                    operands[0].is_register(),
                    operands[1].is_register(),
                    operands[0].repr.as_str(),
                    operands[1].repr.as_str(),
                    operands.len(),
                ) {
                    (true, true, _, _, 2) => Opcode::new(0x8000)
                        .set_vx(operands[0].clone())
                        .set_vy(operands[1].clone()),
                    (true, _, _, "R", 2) => Opcode::new(0xF085).set_vx(operands[0].clone()),
                    (true, _, _, "DT", 2) => Opcode::new(0xF007).set_vx(operands[0].clone()),
                    (true, _, _, "K", 2) => Opcode::new(0xF00A).set_vx(operands[0].clone()),
                    (true, _, _, "[I]", 2) => Opcode::new(0xF065).set_vx(operands[0].clone()),
                    (true, _, _, _, 2) => Opcode::new(0x6000)
                        .set_vx(operands[0].clone())
                        .set_kk(operands[1].clone()),
                    (false, true, "HF", _, 2) => Opcode::new(0xF030).set_vx(operands[1].clone()),
                    (false, true, "R", _, 2) => Opcode::new(0xF075).set_vx(operands[1].clone()),
                    (false, true, "ST", _, 2) => Opcode::new(0xF018).set_vx(operands[1].clone()),
                    (false, true, "F", _, 2) => Opcode::new(0xF029).set_vx(operands[1].clone()),
                    (false, true, "B", _, 2) => Opcode::new(0xF033).set_vx(operands[1].clone()),
                    (false, true, "[I]", _, 2) => Opcode::new(0xF055).set_vx(operands[1].clone()),
                    (false, false, "I", _, 2) => Opcode::new(0xA000).set_nnn(operands[1].clone()),
                    (false, true, _, _, 2) => Opcode::new(0xF015).set_vx(operands[1].clone()),
                    (true, true, _, _, 3) => match operands[2].repr.as_str() {
                        "I" => Opcode::new(0x5001)
                            .set_vx(operands[0].clone())
                            .set_vy(operands[1].clone()),
                        _ => return None,
                    },
                    (false, true, _, _, 3) => match operands[0].repr.as_str() {
                        "I" => Opcode::new(0x5002)
                            .set_vx(operands[1].clone())
                            .set_vy(operands[2].clone()),
                        _ => return None,
                    },
                    (_, _, _, _, _) => return None,
                }
            }
            "SNE" => match operands[1].is_register() {
                true => Opcode::new(0x9000)
                    .set_vx(operands[0].clone())
                    .set_vy(operands[1].clone()),
                false => Opcode::new(0x4000)
                    .set_vx(operands[0].clone())
                    .set_kk(operands[1].clone()),
            },
            "ADD" => match (operands[0].is_register(), operands[1].is_register()) {
                (true, false) => Opcode::new(0x7000)
                    .set_vx(operands[0].clone())
                    .set_kk(operands[1].clone()),
                (false, true) => Opcode::new(0xF01E).set_vx(operands[1].clone()),
                (_, _) => Opcode::new(0x8004)
                    .set_vx(operands[0].clone())
                    .set_vy(operands[1].clone()),
            },
            "OR" => Opcode::new(0x8001)
                .set_vx(operands[0].clone())
                .set_vy(operands[1].clone()),
            "AND" => Opcode::new(0x8002)
                .set_vx(operands[0].clone())
                .set_vy(operands[1].clone()),
            "XOR" => Opcode::new(0x8003)
                .set_vx(operands[0].clone())
                .set_vy(operands[1].clone()),
            "SUB" => Opcode::new(0x8005)
                .set_vx(operands[0].clone())
                .set_vy(operands[1].clone()),
            "SHR" => {
                if operands.len() == 1 {
                    Opcode::new(0x8006).set_vx(operands[0].clone())
                } else {
                    Opcode::new(0x8006)
                        .set_vx(operands[0].clone())
                        .set_vy(operands[1].clone())
                }
            }
            "SUBN" => Opcode::new(0x8007)
                .set_vx(operands[0].clone())
                .set_vy(operands[1].clone()),
            "SHL" => {
                if operands.len() == 1 {
                    Opcode::new(0x800E).set_vx(operands[0].clone())
                } else {
                    Opcode::new(0x800E)
                        .set_vx(operands[0].clone())
                        .set_vy(operands[1].clone())
                }
            }
            "RND" => Opcode::new(0xC000)
                .set_vx(operands[0].clone())
                .set_kk(operands[1].clone()),
            "SKP" => Opcode::new(0xE09E).set_vx(operands[0].clone()),
            "SKNP" => Opcode::new(0xE0A1).set_vx(operands[0].clone()),
            _ => return None,
        };

        Some(opcode)
    }
}
impl std::fmt::Debug for Opcode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{{ base: {:#06x}, vx: {:?}, vy: {:?}, nnn: {:?}, kk: {:?}, n: {:?} }}",
            self.base, self.vx, self.vy, self.nnn, self.kk, self.n
        )
    }
}
