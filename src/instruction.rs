pub trait Instruction {
    type Value;

    fn arity(&self) -> (usize, usize);
    fn execute(&self, stack: &mut Vec<Self::Value>);
}

#[derive(Debug, Clone)]
pub enum Inst {
    PushConst(i32),
    Neg,
    Add,
    Sub,
    Mul,
    Mod,
    Dup,
    Swap,
    Rot,
}

impl Instruction for Inst {
    type Value = i32;

    fn arity(&self) -> (usize, usize) {
        match self {
            Inst::PushConst(_) => (0, 1),
            Inst::Neg => (1, 1),
            Inst::Add => (2, 1),
            Inst::Sub => (2, 1),
            Inst::Mul => (2, 1),
            Inst::Mod => (2, 1),
            Inst::Dup => (1, 2),
            Inst::Swap => (2, 2),
            Inst::Rot => (3, 3),
        }
    }

    fn execute(&self, stack: &mut Vec<i32>) {
        match self {
            Inst::PushConst(v) => stack.push(*v),
            Inst::Neg => {
                let a = stack.pop().unwrap();
                stack.push(-a);
            }
            Inst::Add => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a + b);
            }
            Inst::Sub => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a - b);
            }
            Inst::Mul => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a * b);
            }
            Inst::Mod => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                if b == 0 {
                    stack.push(0);
                    return;
                }
                stack.push(a % b);
            }
            Inst::Dup => {
                let a = stack.pop().unwrap();
                stack.extend_from_slice(&[a, a]);
            }
            Inst::Swap => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.extend_from_slice(&[a, b]);
            }
            Inst::Rot => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                let c = stack.pop().unwrap();
                stack.extend_from_slice(&[a, c, b]);
            }
        }
    }
}
