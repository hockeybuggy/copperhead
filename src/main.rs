#[allow(non_camel_case_types)] // To match the python opcodes

// 7 + 5


#[derive(Debug)]
enum Opcodes {
    LOAD_VALUE,
    BINARAY_ADD,
    PRINT_ITEM,
}

fn load_value(stack: &mut Vec<i64>, data: &Vec<i64>, datum_index: usize) {
    // push off stack
    let datum: i64= data[datum_index];
    stack.push(datum);
}

fn binary_add(stack: &mut Vec<i64>) {
    // pop 2 off stack, add them and push result
    let x: Option<i64> = stack.pop();
    let y: Option<i64> = stack.pop();
    let a = match x {
        Some(ref j) => j,
        None => panic!("Can't pop"),
    };
    let b = match y {
        Some(ref j) => j,
        None => panic!("Can't pop"),
    };
    stack.push(a + b);
}

fn print_item(stack: &mut Vec<i64>) {
    // pop 1 off stack and print it
    let x: Option<i64> = stack.pop();
    let a = match x {
        Some(ref j) => j,
        None => panic!("Can't pop"),
    };
    println!("{:?}", a);
}

fn main() {
    let data: Vec<i64> = vec!(7, 5);

    let instructions = vec!(
        (Opcodes::LOAD_VALUE, 0),
        (Opcodes::LOAD_VALUE, 1),
        (Opcodes::BINARAY_ADD, 0),
        (Opcodes::PRINT_ITEM, 0),
        );

    let mut stack: Vec<i64> = Vec::new();

    for i in &instructions {
        println!("{:?}", i);
        match i.0 {
            Opcodes::LOAD_VALUE => load_value(&mut stack, &data, i.1),
            Opcodes::BINARAY_ADD => binary_add(&mut stack),
            Opcodes::PRINT_ITEM => print_item(&mut stack),
        }
    }
}
