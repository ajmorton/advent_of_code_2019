use std::collections::HashMap;

pub type IntValue = isize;
type IntProgram = HashMap<usize, IntValue>;

const MEM_SIZE: usize = 32000;

#[derive(PartialEq)]
enum Instruction {
    Add(Parameter, Parameter, Parameter),
    Mult(Parameter, Parameter, Parameter),
    Read(Parameter),
    Write(Parameter),
    JumpIfTrue(Parameter, Parameter),
    JumpIfFalse(Parameter, Parameter),
    LessThan(Parameter, Parameter, Parameter),
    Equals(Parameter, Parameter, Parameter),
    AdjustRelBase(Parameter),
    End()
}

#[derive(PartialEq, Debug)]
enum Parameter {
    Immediate(IntValue),
    Position(usize),
    Relative(IntValue)
}

#[derive(PartialEq, Debug)]
pub enum Action {
    Continue,
    Read,
    Output(IntValue),
    Halt
}

pub struct Computer {
    memory: IntProgram,
    instr_ptr: usize,
    relative_base: IntValue,
    input_vals: Vec<IntValue>,
    output_vals: Vec<IntValue>
}

impl Computer {

    pub fn new(program: Vec<IntValue>) -> Self {
        // read program into memory
        let mut memory = HashMap::new();
        for (index, val) in program.iter().enumerate() {
            memory.insert(index, *val);
        };

        Computer {
            memory,
            instr_ptr: 0,
            relative_base: 0,
            input_vals: Vec::new(),
            output_vals: Vec::new()
        }
    }

    pub fn new_with_input(program: Vec<IntValue>, input_vals: Vec<IntValue>) -> Self {
        let mut comp = Computer::new(program);
        comp.input_vals = input_vals;
        comp
    }

    pub fn input(&mut self, val: IntValue) {
        self.input_vals.push(val);
    }

    pub fn run(&mut self) -> Action {
        loop {
            let instr = self.next_instr();
            let result = self.exec_instr(instr);

            match result {
                Action::Continue => {},
                _ => return result
            };
        }
    }

    fn next_instr(&self) -> Instruction {
        let instr = self.memory.get(&self.instr_ptr).unwrap();

        let opcode = instr % 100;
        let mode_1 = (instr / 100) % 10;
        let mode_2 = (instr / 1000) % 10;
        let mode_3 = (instr / 10000) % 10;

        let get_param = |mode, ptr: usize, prog: &IntProgram|
            match mode {
                0 => Parameter::Position(*prog.get(&ptr).unwrap_or(&0) as usize),
                1 => Parameter::Immediate(*prog.get(&ptr).unwrap_or(&0)),
                2 => Parameter::Relative(*prog.get(&ptr).unwrap_or(&0)),
                _ => unimplemented!()
            };

        match opcode {
            1 => Instruction::Add(get_param(mode_1, self.instr_ptr + 1, &self.memory),
                                  get_param(mode_2, self.instr_ptr + 2, &self.memory),
                                  get_param(mode_3, self.instr_ptr + 3, &self.memory)),

            2 => Instruction::Mult(get_param(mode_1, self.instr_ptr + 1, &self.memory),
                                   get_param(mode_2, self.instr_ptr + 2, &self.memory),
                                   get_param(mode_3, self.instr_ptr + 3, &self.memory)),

            3 => Instruction::Read(get_param(mode_1, self.instr_ptr + 1, &self.memory)),

            4 => Instruction::Write(get_param(mode_1, self.instr_ptr + 1, &self.memory)),

            5 => Instruction::JumpIfTrue(get_param(mode_1, self.instr_ptr + 1, &self.memory),
                                         get_param(mode_2, self.instr_ptr + 2, &self.memory)),

            6 => Instruction::JumpIfFalse(get_param(mode_1, self.instr_ptr + 1, &self.memory),
                                          get_param(mode_2, self.instr_ptr + 2, &self.memory)),

            7 => Instruction::LessThan(get_param(mode_1, self.instr_ptr + 1, &self.memory),
                                       get_param(mode_2, self.instr_ptr + 2, &self.memory),
                                       get_param(mode_3, self.instr_ptr + 3, &self.memory)),

            8 => Instruction::Equals(get_param(mode_1, self.instr_ptr + 1, &self.memory),
                                     get_param(mode_2, self.instr_ptr + 2, &self.memory),
                                     get_param(mode_3, self.instr_ptr + 3, &self.memory)),

            9 => Instruction::AdjustRelBase(get_param(mode_1, self.instr_ptr + 1, &self.memory)),

            99 => Instruction::End(),

            _ => unimplemented!()
        }
    }

    fn exec_instr(&mut self, instr: Instruction) -> Action {
        match instr {
            Instruction::Add(param_1, param_2, param_3) => {
                let value = self.read(param_1) + self.read(param_2);
                self.write(param_3, value);
                self.instr_ptr += 4;
                Action::Continue
            },
            Instruction::Mult(param_1, param_2, param_3) => {
                let value = self.read(param_1) * self.read(param_2);
                self.write(param_3, value);
                self.instr_ptr += 4;
                Action::Continue
            },
            Instruction::Read(param_1) => {
                match self.input_vals.len() {
                    0 => Action::Read,
                    _ => {
                        let value = self.input_vals.remove(0);
                        self.write(param_1, value);
                        self.instr_ptr += 2;
                        Action::Continue
                    }
                }
            },
            Instruction::Write(param_1) => {
                let value = self.read(param_1);
                self.output_vals.push(value);
                self.instr_ptr += 2;
                Action::Output(value)
            },
            Instruction::JumpIfTrue(param_1, param_2) => {
                if self.read(param_1) != 0 {
                    self.instr_ptr = self.read(param_2) as usize;
                } else {
                    self.instr_ptr += 3;
                };
                Action::Continue
            },
            Instruction::JumpIfFalse(param_1, param_2) => {
                if self.read(param_1) == 0 {
                    self.instr_ptr = self.read(param_2) as usize;
                } else {
                    self.instr_ptr += 3;
                };
                Action::Continue
            },
            Instruction::LessThan(param_1, param_2, param_3) => {
                if self.read(param_1) < self.read(param_2) {
                    self.write(param_3, 1);
                } else {
                    self.write(param_3, 0);
                }
                self.instr_ptr += 4;
                Action::Continue
            },
            Instruction::Equals(param_1, param_2, param_3) => {
                if self.read(param_1) == self.read(param_2) {
                    self.write(param_3, 1);
                } else {
                    self.write(param_3, 0);
                }
                self.instr_ptr += 4;
                Action::Continue
            },
            Instruction::AdjustRelBase(param_1) => {
                self.relative_base += self.read(param_1);
                self.instr_ptr += 2;
                Action::Continue
            },
            Instruction::End() => Action::Halt
        }
    }

    fn read(&self, param: Parameter) -> IntValue {
        match param {
            Parameter::Immediate(val) => val,
            Parameter::Position(index) => *self.memory.get(&index).unwrap_or(&0),
            Parameter::Relative(offset) => *self.memory.get(&((self.relative_base + offset) as usize)).unwrap_or(&0)
        }
    }

    fn write(&mut self, param: Parameter, value: IntValue) {
        match param {
            Parameter::Immediate(_) => unimplemented!(),
            Parameter::Position(index) => self.memory.insert(index, value),
            Parameter::Relative(offset) => self.memory.insert((self.relative_base + offset) as usize,value)
        };
    }

    pub fn get_program(self) -> IntProgram {
        self.memory.clone()
    }

    pub fn get_output_vals(self) -> Vec<IntValue> {
        self.output_vals.clone()
    }
}

#[test]
fn int_computer_jump_tests() {
    let program = vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31, 1106,0,36,98,
                       0,0,1002,21,125,20,4,20,1105,1,46,104, 999,1105,1,46,1101,1000,1,20,4,20,
                       1105,1,46,98,99];

    // input less than 8
    let mut computer = Computer::new_with_input(program.clone(), vec![7]);
    computer.run();
    assert_eq!(computer.get_output_vals().pop().unwrap(), 999);

    // input equal to 8
    let mut computer = Computer::new_with_input(program.clone(), vec![8]);
    computer.run();
    assert_eq!(computer.get_output_vals().pop().unwrap(), 1000);

    // input greater than 8
    let mut computer = Computer::new_with_input(program.clone(), vec![9]);
    computer.run();
    assert_eq!(computer.get_output_vals().pop().unwrap(), 1001);

}