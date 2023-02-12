use exprgen::instruction::Inst;
use exprgen::instruction::Instruction;
use exprgen::run;

pub fn generate(len: usize, mut stack_size: usize, mut seed: u128) -> Vec<Inst> {
    let mut cs = Vec::new();
    while cs.len() < len {
        if len - cs.len() < stack_size {
            let n = 4;
            let inst = match seed % n {
                0 => Inst::Add,
                1 => Inst::Sub,
                2 => Inst::Mul,
                3 => Inst::Mod,
                4.. => unreachable!(),
            };
            stack_size += inst.arity().1;
            stack_size -= inst.arity().0;
            cs.push(inst);
            seed = seed / n;

            continue;
        }
        // if len == cs.len() + 1 && stack_size == 1 {
        //     let n = 1;
        //     let inst = match seed % n {
        //         0 => Command::Neg,
        //         1.. => unreachable!(),
        //     };
        //     stack_size += inst.arity().1;
        //     stack_size -= inst.arity().0;
        //     cs.push(inst);
        //     seed = seed / n;
        // }

        let mut n = 6;
        if 1 <= stack_size {
            n += 2;
        }
        if 2 <= stack_size {
            n += 5;
        }
        if 3 <= stack_size {
            n += 1;
        }
        let inst = match seed % n {
            i @ (0..=5) => Inst::PushConst([0, 1, 2, 3, 5, 7][i as usize]),
            6 => Inst::Dup,
            7 => Inst::Neg,
            8 => Inst::Add,
            9 => Inst::Sub,
            10 => Inst::Mul,
            11 => Inst::Mod,
            12 => Inst::Swap,
            13 => Inst::Rot,
            14.. => unreachable!(),
        };
        stack_size += inst.arity().1;
        stack_size -= inst.arity().0;
        cs.push(inst);
        seed = seed / n;
    }
    cs
}

fn main() {
    use Inst::*;
    dbg!(run(vec![], &[PushConst(1), PushConst(2), Add, Dup, Mul]));

    println!("{:?}", generate(3, 0, 123450));
    println!("{:?}", generate(4, 0, 1234567));
    println!("{:?}", run(vec![], &generate(4, 0, 1234567)));
    println!("{:?}", generate(5, 0, 12345678));
    println!("{:?}", run(vec![], &generate(5, 0, 12345678)));
    println!("{:?}", generate(6, 0, 123456789));
    println!("{:?}", run(vec![], &generate(6, 0, 123456789)));

    println!();

    for i in 0..10 {
        let code = generate(6, 1, 123456789 + i * 123);
        println!("{:?}", &code);
        for j in 0..20 {
            let stack = run(vec![j as i32], &code);
            let res = stack.last().unwrap();
            print!("{}, ", res);
        }
        println!();
    }
}
