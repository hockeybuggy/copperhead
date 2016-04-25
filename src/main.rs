#[allow(non_camel_case_types)] // To match the python opcodes

// 7 + 5

struct Frame {
    data_stack: Vec<i64>,
    block_stack: Vec<i64>,
}

struct VirtualMachine {
    // stack: Vec<Frame>, TODO switch to i64
    stack: Vec<i64>,
}

impl VirtualMachine {
    fn new() -> VirtualMachine {
        VirtualMachine {
            stack: Vec::new(),
        }
    }

    fn run(&mut self, instructions: Vec<(Opcodes, usize)>, data: Vec<i64>) {
        println!("Running instructions: {:?}", instructions);
        for i in &instructions {
            println!("{:?}", i);
            match i.0 {
                Opcodes::LOAD_VALUE => self.load_value(&data, i.1),
                Opcodes::BINARAY_ADD => self.binary_add(),
                Opcodes::PRINT_ITEM => self.print_item(),
            }
        }
    }

    fn load_value(&mut self, data: &Vec<i64>, datum_index: usize) {
        // push off stack
        let datum: i64 = data[datum_index];
        self.stack.push(datum);
    }

    fn binary_add(&mut self) {
        // pop 2 off stack, add them and push result
        let x: Option<i64> = self.stack.pop();
        let y: Option<i64> = self.stack.pop();
        let a = match x {
            Some(ref j) => j,
            None => panic!("Can't pop"),
        };
        let b = match y {
            Some(ref j) => j,
            None => panic!("Can't pop"),
        };
        self.stack.push(a + b);
    }

    fn print_item(&mut self) {
        // pop 1 off stack and print it
        let x: Option<i64> = self.stack.pop();
        let a = match x {
            Some(ref j) => j,
            None => panic!("Can't pop"),
        };
        println!("{:?}", a);
    }
}

// Thing to create new frames
// Create instruction 'class'

#[derive(Debug)]
enum Opcodes {
    LOAD_VALUE,
    BINARAY_ADD,
    PRINT_ITEM,
}


fn main() {
    let data: Vec<i64> = vec!(7, 6);

    let instructions = vec!(
        (Opcodes::LOAD_VALUE, 0),
        (Opcodes::LOAD_VALUE, 1),
        (Opcodes::BINARAY_ADD, 0),
        (Opcodes::PRINT_ITEM, 0),
        );

    let mut machine: VirtualMachine = VirtualMachine::new();
    machine.run(instructions, data);
}

