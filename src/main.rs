#[allow(non_camel_case_types)] // To match the python opcodes
mod vm;
use vm::VirtualMachine;


fn main() {
    // x = 5
    // print x

    let consts: Vec<i64> = vec!(7, 5);
    let varnames: Vec<&'static str> = vec!("x");
    let instructions = vec!(
        (vm::Opcodes::LOAD_CONST, 1),
        (vm::Opcodes::STORE_FAST, 0),
        (vm::Opcodes::LOAD_FAST, 0),
        (vm::Opcodes::PRINT_ITEM, 0),
        );


    // print 7 + 5

    // let consts: Vec<i64> = vec!(7, 5);
    // let varnames: Vec<&'static str> = vec!();
    // let instructions = vec!(
    //     (vm::Opcodes::LOAD_CONST, 0),
    //     (vm::Opcodes::LOAD_CONST, 1),
    //     (vm::Opcodes::BINARY_ADD, 0),
    //     (vm::Opcodes::PRINT_ITEM, 0),
    //     );

    let mut machine: VirtualMachine = vm::VirtualMachine::new();
    machine.run(instructions, consts, varnames);
}
