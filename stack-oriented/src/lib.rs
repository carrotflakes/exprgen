pub mod instruction;

pub fn run(mut stack: Vec<i32>, is: &[instruction::Inst]) -> Vec<i32> {
    use instruction::Instruction;
    for inst in is {
        inst.execute(&mut stack);
    }
    stack
}
