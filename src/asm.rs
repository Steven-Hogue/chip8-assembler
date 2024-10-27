use crate::instructions::Opcode;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct ParseOperandError {
    pub message: String,
}
impl ParseOperandError {
    fn new(message: String) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}
impl Error for ParseOperandError {}
impl fmt::Display for ParseOperandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[derive(Clone)]
pub struct Operand {
    pub repr: String,
}
impl Operand {
    fn new(repr: String) -> Operand {
        Operand { repr }
    }

    pub fn parse_numeric_str(value: String) -> Result<u16, ParseOperandError> {
        let parsed = if value.starts_with("0x") || value.starts_with("#") {
            u16::from_str_radix(value.trim_start_matches("0x").trim_start_matches("#"), 16)
        } else if value.starts_with("%") {
            u16::from_str_radix(value.trim_start_matches("%"), 2)
        } else if value.starts_with('\'') && value.ends_with('\'') {
            Ok(value.chars().nth(1).unwrap() as u16)
        } else {
            value.parse::<u16>()
        };

        match parsed {
            Ok(n) => Ok(n),
            Err(_) => Err(ParseOperandError::new(format!("Invalid number: {}", value))),
        }
    }

    pub fn parse_register_str(value: String) -> Result<u16, ParseOperandError> {
        let parsed =
            u16::from_str_radix(value.trim_start_matches("V").trim_start_matches("v"), 16).unwrap();

        if parsed <= 15 {
            Ok(parsed)
        } else {
            Err(ParseOperandError::new(format!(
                "Invalid register number: {}",
                parsed
            )))
        }
    }

    pub fn is_register(&self) -> bool {
        self.repr.starts_with("v") || self.repr.starts_with("V")
    }

    pub fn parse(self) -> Result<u16, ParseOperandError> {
        if self.is_register() {
            Operand::parse_register_str(self.repr)
        } else {
            Operand::parse_numeric_str(self.repr)
        }
    }
}
impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.repr)
    }
}
impl fmt::Debug for Operand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Operand {{repr: '{}'}}", self.repr)
    }
}

pub trait Asm {
    fn get_byte_size(&self) -> usize;
    fn from_line(line: String) -> Self;
}
pub enum AsmEnum {
    Instruction(Instruction),
    Label(Label),
    Define(Define),
    Directive(Directive),
}
impl AsmEnum {
    fn get_byte_size(&self) -> usize {
        match self {
            AsmEnum::Instruction(i) => i.get_byte_size(),
            AsmEnum::Label(l) => l.get_byte_size(),
            AsmEnum::Define(d) => d.get_byte_size(),
            AsmEnum::Directive(d) => d.get_byte_size(),
        }
    }
}
impl fmt::Display for AsmEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AsmEnum::Instruction(i) => write!(f, "{}", i),
            AsmEnum::Label(l) => write!(f, "{}", l),
            AsmEnum::Define(d) => write!(f, "{}", d),
            AsmEnum::Directive(d) => write!(f, "{}", d),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Instruction {
    pub mnemonic: String,
    pub args: Vec<Operand>,
}
impl Instruction {
    fn new(mnemonic: String, args: Vec<String>) -> Instruction {
        Instruction {
            mnemonic,
            args: args.into_iter().map(Operand::new).collect(),
        }
    }
}
impl Asm for Instruction {
    fn get_byte_size(&self) -> usize {
        if !self.mnemonic.chars().next().unwrap().is_alphanumeric() {
            0
        } else {
            2
        }
    }

    fn from_line(line: String) -> Instruction {
        // The mnemonic is the first word separated by whitespace
        // All other args are separated by commas
        let split: Vec<&str> = line.split_whitespace().collect();
        let mnemonic = split[0].to_string();
        let args: Vec<String> = split[1..]
            .join(",")
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        Instruction::new(mnemonic, args)
    }
}
impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Instruction {{mnemonic: '{}', args: [{}], byte_size: {}}}",
            self.mnemonic,
            self.args
                .iter()
                .map(|a| a.to_string())
                .collect::<Vec<String>>()
                .join(", "),
            self.get_byte_size()
        )
    }
}

pub struct Label {
    name: String,
}
impl Label {
    fn new(name: String) -> Label {
        Label { name }
    }
}
impl Asm for Label {
    fn get_byte_size(&self) -> usize {
        0
    }

    fn from_line(line: String) -> Label {
        let name = line.replace(":", "");
        Label::new(name)
    }
}
impl fmt::Display for Label {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Label {{name: '{}'}}", self.name)
    }
}

pub struct Define {
    key: String,
    value: String,
}
impl Define {
    fn new(key: String, value: String) -> Define {
        Define { key, value }
    }
}
impl Asm for Define {
    fn get_byte_size(&self) -> usize {
        0
    }

    fn from_line(line: String) -> Define {
        let split: Vec<&str> = line.split_whitespace().collect();
        let key = split[1].to_string();
        let value = split[2].to_string();
        Define::new(key, value)
    }
}
impl fmt::Display for Define {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Define {{key: '{}', value: '{}'}}", self.key, self.value)
    }
}

pub struct Directive {
    mnemonic: String,
    args: Vec<String>,
}
impl Directive {
    const VALID_DIRECTIVES: [&'static str; 4] = ["db", "dw", "text", "offset"];

    fn new(mnemonic: String, args: Vec<String>) -> Directive {
        Directive { mnemonic, args }
    }
}
impl Asm for Directive {
    fn get_byte_size(&self) -> usize {
        match self.mnemonic.to_lowercase().as_str() {
            "db" => self.args.len(),
            "dw" => self.args.len() * 2,
            "text" => self.args[0].len() + 1,
            "offset" => Operand::parse_numeric_str(self.args[0].clone()).unwrap() as usize,
            _ => 0,
        }
    }

    fn from_line(line: String) -> Directive {
        let split: Vec<&str> = line.split_whitespace().collect();
        let mnemonic = split[0].to_string();
        let remaining = split[1..].join(" ");

        // Get args, grouping things in quotes together
        let mut args: Vec<String> = Vec::new();
        let mut in_quotes = false;
        let mut current_arg = String::new();
        for c in remaining.chars() {
            if c == '\"' {
                in_quotes = !in_quotes;
            } else if (c == ',' || c == ' ') && !current_arg.is_empty() && !in_quotes {
                args.push(current_arg.clone().as_str().trim().to_string());
                current_arg = String::new();
            } else {
                current_arg.push(c);
            }
        }
        args.push(current_arg.clone().as_str().trim().to_string());

        Directive::new(mnemonic, args)
    }
}
impl fmt::Display for Directive {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Directive {{mnemonic: '{}', args: [{}], byte_size: {}}}",
            self.mnemonic,
            self.args.join(", "),
            self.get_byte_size()
        )
    }
}

pub struct Assembly {
    pub instructions: Vec<(AsmEnum, usize)>,
}
impl Assembly {
    fn new(instructions: Vec<AsmEnum>, offset: usize) -> Assembly {
        let instructions = instructions.into_iter().map(|i| (i, 0)).collect();
        let mut new = Assembly { instructions };
        new.update_defines();
        new.update_offsets(offset);
        new
    }

    fn update_offsets(&mut self, offset: usize) {
        let mut byte_offset = 0;
        for (i, off) in self.instructions.iter_mut() {
            let byte_size = i.get_byte_size();
            *off = byte_offset + offset;
            byte_offset += byte_size;
        }
    }

    fn update_labels(&mut self) {
        let mut label_map: HashMap<String, usize> = HashMap::new();
        for (i, off) in self.instructions.iter() {
            if let AsmEnum::Label(l) = i {
                label_map.insert(l.name.clone(), *off);
            }
        }

        for (i, _) in self.instructions.iter_mut() {
            if let AsmEnum::Instruction(inst) = i {
                for arg in inst.args.iter_mut() {
                    if label_map.contains_key(&arg.repr) {
                        *arg = Operand::new(label_map[&arg.repr].to_string());
                    }
                }
            }
        }
    }

    fn update_defines(&mut self) {
        let mut define_map: HashMap<String, String> = HashMap::new();
        for (i, _) in self.instructions.iter() {
            if let AsmEnum::Define(d) = i {
                define_map.insert(d.key.clone(), d.value.clone());
            }
        }

        for (i, _) in self.instructions.iter_mut() {
            match i {
                AsmEnum::Instruction(inst) => {
                    for arg in inst.args.iter_mut() {
                        if define_map.contains_key(&arg.repr) {
                            *arg = Operand::new(define_map[&arg.repr].to_string());
                        }
                    }
                }
                AsmEnum::Directive(dir) => {
                    for arg in dir.args.iter_mut() {
                        if define_map.contains_key(arg) {
                            *arg = define_map[arg].clone();
                        }
                    }
                }
                _ => {}
            }
        }
    }

    pub fn to_bytes(&mut self) -> Vec<u8> {
        self.update_labels();

        let mut bytes: Vec<u8> = Vec::new();
        for (i, _) in self.instructions.iter() {
            match i {
                AsmEnum::Instruction(inst) => {
                    let opcode = Opcode::from_instruction(inst.clone());

                    match opcode {
                        Some(opcode) => match opcode.to_bytes() {
                            Ok(b) => {
                                bytes.push((b >> 8) as u8);
                                bytes.push((b & 0xFF) as u8);
                            }
                            Err(e) => panic!("Unable to convert to bytes: {}", e),
                        },
                        None => panic!("Error: Invalid instruction {:?}", inst),
                    }
                }
                AsmEnum::Directive(dir) => match dir.mnemonic.to_lowercase().as_str() {
                    "db" => {
                        for arg in dir.args.iter() {
                            match Operand::parse_numeric_str(arg.clone()) {
                                Ok(n) => bytes.push(n as u8),
                                Err(e) => panic!("Unable to convert to bytes: {}", e),
                            }
                        }
                    }
                    "dw" => {
                        for arg in dir.args.iter() {
                            match Operand::parse_numeric_str(arg.clone()) {
                                Ok(n) => {
                                    bytes.push((n >> 8) as u8);
                                    bytes.push((n & 0xFF) as u8);
                                }
                                Err(e) => panic!("Unable to convert to bytes: {}", e),
                            }
                        }
                    }
                    "text" => {
                        for arg in dir.args.iter() {
                            for c in arg.chars() {
                                bytes.push(c as u8);
                            }
                            bytes.push(0);
                        }
                    }
                    "offset" => match Operand::parse_numeric_str(dir.args[0].clone()) {
                        Ok(n) => {
                            for _ in 0..n {
                                bytes.push(0);
                            }
                        }
                        Err(e) => panic!("Unable to convert to bytes: {}", e),
                    },
                    _ => {}
                },
                _ => {}
            }
        }
        bytes
    }
}
impl fmt::Display for Assembly {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (inst, off) in self.instructions.iter() {
            write!(f, "{:#06x} {}\n", off, inst)?;
        }
        Ok(())
    }
}

fn without_comments(line: String) -> String {
    line.split(';').collect::<Vec<&str>>()[0].to_string()
}

fn extract_label(line: String) -> Option<(String, Option<String>)> {
    match line.find(':') {
        Some(_) => {
            let split: Vec<&str> = line.split(':').collect();
            let label = ":".to_string() + split[0].trim();
            let line = split[1].trim().to_string();
            if split[0].chars().all(|c| c != '\"' && c != '\'') {
                return if line.is_empty() {
                    Some((label, None))
                } else {
                    Some((label, Some(line)))
                };
            }
            None
        }
        None => None,
    }
}

fn format_line(mut line: String) -> Option<String> {
    line = without_comments(line);
    line = line.trim().to_string();

    if line.is_empty() {
        None
    } else {
        Some(line)
    }
}

pub fn generate_full_asm(file_path: &str, offset: usize) -> Assembly {
    let mut full_asm: Vec<AsmEnum> = Vec::new();

    let relative_path =
        file_path.split('/').collect::<Vec<&str>>()[..file_path.split('/').count() - 1].join("/");
    let mut file_queue: Vec<String> = vec![file_path.to_string()];
    let mut all_files: Vec<String> = Vec::new();
    while file_queue.len() > 0 {
        let file_path = file_queue.pop().unwrap();
        // Try to open file, if it fails try to find it in the same directory as the original
        let file = match File::open(&file_path) {
            Ok(f) => f,
            Err(_) => File::open(format!("{}/{}", relative_path, file_path))
                .expect(format!("File not found: {}", file_path).as_str()),
        };

        let mut line_queue = BufReader::new(file)
            .lines()
            .map(|l| l.unwrap())
            .collect::<Vec<String>>()
            .into_iter();
        while let Some(line) = line_queue.next() {
            let mut line = match format_line(line) {
                Some(line) => line,
                None => continue,
            };

            // Parse included files
            let split: Vec<&str> = line.split("include ").collect();
            if split.len() > 1 {
                split[1].replace("\"", "").split_whitespace().for_each(|s| {
                    if !all_files.contains(&s.to_string()) {
                        all_files.push(s.to_string());
                        file_queue.push(s.to_string());
                    }
                });
                continue;
            }

            // Remove labels and put remaining in line_queue
            if let Some((label, rem_line)) = extract_label(line.clone()) {
                full_asm.push(AsmEnum::Label(Label::from_line(label)));
                if let Some(rem_line) = rem_line {
                    // Put rem_line at the front of the line_queue
                    let as_iter = vec![rem_line].into_iter();
                    line_queue = as_iter
                        .chain(line_queue)
                        .collect::<Vec<String>>()
                        .into_iter();
                }
                continue;
            }

            while line.ends_with(',') || line.to_lowercase() == "db" {
                match format_line(line_queue.next().unwrap()) {
                    Some(next_line) => line = line + " " + next_line.as_str(),
                    None => break,
                }
            }

            let first_word = line.split_whitespace().next().unwrap();
            full_asm.push(if first_word == "define" {
                AsmEnum::Define(Define::from_line(line))
            } else if Directive::VALID_DIRECTIVES.contains(&first_word) {
                AsmEnum::Directive(Directive::from_line(line))
            } else {
                AsmEnum::Instruction(Instruction::from_line(line))
            });
        }
    }

    Assembly::new(full_asm, offset)
}
