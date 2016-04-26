#[allow(non_camel_case_types)] // To match the python opcodes
mod vm;
use vm::VirtualMachine;

// 7 + 5

fn main() {
    let data: Vec<i64> = vec!(7, 5);

    let instructions = vec!(
        (vm::Opcodes::LOAD_VALUE, 0),
        (vm::Opcodes::LOAD_VALUE, 1),
        (vm::Opcodes::BINARAY_ADD, 0),
        (vm::Opcodes::PRINT_ITEM, 0),
        );

    let mut machine: VirtualMachine = vm::VirtualMachine::new();
    machine.run(instructions, data);
}
