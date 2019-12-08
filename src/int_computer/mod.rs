type IntProgram = Vec<isize>;

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
    End()
}

#[derive(PartialEq, Debug)]
enum Parameter {
    Immediate(isize),
    Position(usize)
}

pub struct Computer {
    program: IntProgram,
    instr_ptr: usize,
    input_vals: Vec<isize>,
    output_vals: Vec<isize>
}

impl Computer {

    pub fn new(prog: Vec<isize>) -> Self {
        Computer {
            program: prog,
            instr_ptr: 0,
            input_vals: Vec::new(),
            output_vals: Vec::new()
        }
    }

    pub fn new_with_input(prog: Vec<isize>, input_vals: Vec<isize>) -> Self {
        Computer {
            program: prog,
            instr_ptr: 0,
            input_vals,
            output_vals: Vec::new()
        }
    }

    pub fn run(&mut self) {
        loop {
            let instr = self.next_instr();
            if instr == Instruction::End() {
                return;
            } else {
                self.exec_instr(instr);
            }
        }
    }

    fn next_instr(&self) -> Instruction {
        let instr = self.program[self.instr_ptr];

        let opcode = instr % 100;
        let mode_1 = (instr / 100) % 10;
        let mode_2 = (instr / 1000) % 10;
        let mode_3 = (instr / 10000) % 10;

        let get_param = |mode, ptr, prog: Vec<isize>|
            if mode == 1 {
                Parameter::Immediate(prog[ptr])
            } else {
                Parameter::Position(prog[ptr] as usize)
            };

        match opcode {
            1 => Instruction::Add(get_param(mode_1, self.instr_ptr + 1, self.program.clone()),
                                  get_param(mode_2, self.instr_ptr + 2, self.program.clone()),
                                  get_param(mode_3, self.instr_ptr + 3, self.program.clone())),

            2 => Instruction::Mult(get_param(mode_1, self.instr_ptr + 1, self.program.clone()),
                                   get_param(mode_2, self.instr_ptr + 2, self.program.clone()),
                                   get_param(mode_3, self.instr_ptr + 3, self.program.clone())),

            3 => Instruction::Read(get_param(mode_1, self.instr_ptr + 1, self.program.clone())),

            4 => Instruction::Write(get_param(mode_1, self.instr_ptr + 1, self.program.clone())),

            5 => Instruction::JumpIfTrue(get_param(mode_1, self.instr_ptr + 1, self.program.clone()),
                                         get_param(mode_2, self.instr_ptr + 2, self.program.clone())),

            6 => Instruction::JumpIfFalse(get_param(mode_1, self.instr_ptr + 1, self.program.clone()),
                                          get_param(mode_2, self.instr_ptr + 2, self.program.clone())),

            7 => Instruction::LessThan(get_param(mode_1, self.instr_ptr + 1, self.program.clone()),
                                       get_param(mode_2, self.instr_ptr + 2, self.program.clone()),
                                       get_param(mode_3, self.instr_ptr + 3, self.program.clone())),

            8 => Instruction::Equals(get_param(mode_1, self.instr_ptr + 1, self.program.clone()),
                                     get_param(mode_2, self.instr_ptr + 2, self.program.clone()),
                                     get_param(mode_3, self.instr_ptr + 3, self.program.clone())),


            99 => Instruction::End(),

            _ => unimplemented!()
        }
    }

    fn exec_instr(&mut self, instr: Instruction) {
        match instr {
            Instruction::Add(param_1, param_2, param_3) => {
                let value = self.read(param_1) + self.read(param_2);
                self.write(param_3, value);
                self.instr_ptr += 4;
            },
            Instruction::Mult(param_1, param_2, param_3) => {
                let value = self.read(param_1) * self.read(param_2);
                self.write(param_3, value);
                self.instr_ptr += 4;
            },
            Instruction::Read(param_1) => {
                let value = self.input_vals.remove(0);
                self.write(param_1, value);
                self.instr_ptr += 2;
            },
            Instruction::Write(param_1) => {
                self.output_vals.push(self.read(param_1));
                self.instr_ptr += 2;
            },
            Instruction::JumpIfTrue(param_1, param_2) => {
                if self.read(param_1) != 0 {
                    self.instr_ptr = self.read(param_2) as usize;
                } else {
                    self.instr_ptr += 3;
                };
            },
            Instruction::JumpIfFalse(param_1, param_2) => {
                if self.read(param_1) == 0 {
                    self.instr_ptr = self.read(param_2) as usize;
                } else {
                    self.instr_ptr += 3;
                };
            },
            Instruction::LessThan(param_1, param_2, param_3) => {
                if self.read(param_1) < self.read(param_2) {
                    self.write(param_3, 1);
                } else {
                    self.write(param_3, 0);
                }
                self.instr_ptr += 4;
            },
            Instruction::Equals(param_1, param_2, param_3) => {
                if self.read(param_1) == self.read(param_2) {
                    self.write(param_3, 1);
                } else {
                    self.write(param_3, 0);
                }
                self.instr_ptr += 4;
            },
            Instruction::End() => {}
        };
    }

    fn read(&self, param: Parameter) -> isize {
        match param {
            Parameter::Immediate(val) => val,
            Parameter::Position(index) => self.program[index]
        }
    }

    fn write(&mut self, param: Parameter, value: isize) {
        match param {
            Parameter::Immediate(_) => unimplemented!(),
            Parameter::Position(index) => self.program[index] = value
        };
    }

    pub fn get_program(self) -> IntProgram {
        self.program.clone()
    }

    pub fn get_output_vals(self) -> Vec<isize> {
        self.output_vals.clone()
    }
}

#[test]
fn int_computer_tests () {
    let mut computer = Computer::new(vec!(1,9,10,3,2,3,11,0,99,30,40,50));
    computer.run();
    assert_eq!(computer.get_program(), vec!(3500,9,10,70,2,3,11,0,99,30,40,50));

    computer = Computer::new(vec!(1,0,0,0,99));
    computer.run();
    assert_eq!(computer.get_program(), vec!(2,0,0,0,99));

    computer = Computer::new(vec!(2,3,0,3,99));
    computer.run();
    assert_eq!(computer.get_program(), vec!(2,3,0,6,99));

    computer = Computer::new(vec!(1,1,1,4,99,5,6,0,99));
    computer.run();
    assert_eq!(computer.get_program(), vec!(30,1,1,4,2,5,6,0,99));
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