#[allow(non_camel_case_types)] // To match the python opcodes

struct Frame {
    data_stack: Vec<i64>,
    block_stack: Vec<i64>,
}


// Thing to create new frames
// Create instruction 'class'

#[derive(Debug)]
pub enum Opcodes {
    LOAD_CONST,
    STORE_FAST,
    LOAD_FAST,
    BINARY_ADD,
    PRINT_ITEM,
}

pub struct VirtualMachine {
    // stack: Vec<Frame>, TODO switch to i64
    stack: Vec<i64>,
}

impl VirtualMachine {
    pub fn new() -> VirtualMachine {
        VirtualMachine {
            stack: Vec::new(),
        }
    }

    pub fn run(&mut self,
               instructions: Vec<(Opcodes, usize)>,
               consts: Vec<i64>,
               varnames: Vec<&'static str>
              ) {
        println!("Running instructions: {:?}", instructions);
        for i in &instructions {
            println!("{:?}", i);
            match i.0 {
                Opcodes::LOAD_CONST => self.load_const(&consts, i.1),
                Opcodes::BINARY_ADD => self.binary_add(),
                Opcodes::PRINT_ITEM => self.print_item(),
                Opcodes::STORE_FAST => self.store_fast(&varnames, i.1),
                Opcodes::LOAD_FAST => self.load_fast(&varnames, i.1),
            }
        }
    }

    pub fn push(&mut self, datum: i64) {
        self.stack.push(datum);
    }

    pub fn pop(&mut self) -> i64 {
        let opt: Option<i64> = self.stack.pop();
        let a: i64 = match opt {
            Some(ref result) => *result,
            None => panic!("Can't pop"),
        };
        return a;
    }

    pub fn load_const(&mut self, consts: &Vec<i64>, datum_index: usize) {
        // push a constant onto the stack
        let datum: i64 = consts[datum_index];
        self.push(datum);
    }

    pub fn binary_add(&mut self) {
        // pop 2 off stack, add them and push result
        let a: i64 = self.pop();
        let b: i64 = self.pop();
        self.stack.push(a + b);
    }

    pub fn print_item(&mut self) {
        // pop 1 off stack and print it
        let a: i64 = self.pop();
        println!("{:?}", a);
    }

    pub fn store_fast(&mut self, varnames: &Vec<&'static str>, datum_index: usize) {
        // pop 1 off stack and stash it
        let varname: &'static str = varnames[datum_index];
        // TODO put it in the stack frame
        println!("Storing {:?}", varname);
    }

    pub fn load_fast(&mut self, varnames: &Vec<&'static str>, datum_index: usize) {
        // take variable from stash and push it on the stack
        let varname: &'static str = varnames[datum_index];
        let datum: i64 = 1; // TODO get from stack frame
        self.stack.push(datum);
        println!("Loading {:?}", varname);
    }
}
