#[derive(Debug)]
pub enum Src {
    Const(i32),
    Var(usize),
}

#[derive(Debug)]
pub struct NormalForm {
    src: Src,
    abs: bool,
    negative: bool,
    add: Option<Box<NormalForm>>,
    mul: Option<Box<NormalForm>>,
    div: Option<Box<NormalForm>>,
    rem: Option<Box<NormalForm>>,
    pow: Option<Box<NormalForm>>,
}

impl NormalForm {
    pub fn format(&self) -> String {
        let mut str = match self.src {
            Src::Const(v) => v.to_string(),
            Src::Var(i) => format!("v{}", i),
        };
        if self.abs {
            str = format!("abs({})", str);
        }
        if self.negative {
            str = format!("-{}", str);
        }
        if let Some(add) = &self.add {
            str = format!("({}+({}))", str, add.format());
        }
        if let Some(mul) = &self.mul {
            str = format!("({}*({}))", str, mul.format());
        }
        if let Some(div) = &self.div {
            str = format!("{}/({})", str, div.format());
        }
        if let Some(rem) = &self.rem {
            str = format!("{}%({})", str, rem.format());
        }
        if let Some(pow) = &self.pow {
            str = format!("{}^({})", str, pow.format());
        }
        str
    }

    pub fn compute(&self, vars: &[i32]) -> i32 {
        let mut v = match &self.src {
            Src::Const(v) => *v,
            Src::Var(i) => vars[*i],
        };
        if self.abs {
            v = v.abs();
        }
        if self.negative {
            v = -v;
        }
        if let Some(add) = &self.add {
            v = v.overflowing_add(add.compute(vars)).0;
        }
        if let Some(mul) = &self.mul {
            v = v.overflowing_mul(mul.compute(vars)).0;
        }
        if let Some(div) = &self.div {
            let x = div.compute(vars);
            if x == 0 {
                v = 0;
            } else {
                v /= x;
            }
        }
        if let Some(rem) = &self.rem {
            let x = rem.compute(vars);
            if x == 0 {
                v = 0;
            } else {
                v %= x;
            }
        }
        if let Some(pow) = &self.pow {
            v = v.overflowing_pow(pow.compute(vars) as u32).0;
        }
        v
    }
}

pub fn generate(seed: u128) -> NormalForm {
    let v = (seed % 8) as usize;
    let mut flags = [false; 8];
    flags[(seed as usize >> 4) % 8] = true;
    flags[(seed as usize >> 8) % 8] = true;
    flags[(seed as usize >> 16) % 8] = true;

    NormalForm {
        src: if 10000 < seed || v == 7 { Src::Var(0) } else { Src::Const([0, 1, 2, 3, 5, 8, 16][v]) },
        abs: flags[6],
        negative: flags[7],
        add: if flags[1] {
            Some(Box::new(generate((seed >> 9) / 3 + 1)))
        } else {
            None
        },
        mul: if flags[2] {
            Some(Box::new(generate((seed >> 9) / 5 + 1)))
        } else {
            None
        },
        div: if flags[3] {
            Some(Box::new(generate((seed >> 9) / 7 + 1)))
        } else {
            None
        },
        rem: if flags[4] {
            Some(Box::new(generate((seed >> 9) / 11 + 1)))
        } else {
            None
        },
        pow: if flags[5] {
            Some(Box::new(generate((seed >> 9) / 13 + 1)))
        } else {
            None
        },
    }
}

fn main() {
    let form = NormalForm {
        src: Src::Const(0),
        abs: false,
        negative: false,
        add: None,
        mul: None,
        div: None,
        rem: None,
        pow: None,
    };
    dbg!(form.compute(&[]));

    dbg!(generate(0));
    dbg!(generate(1));
    dbg!(generate(2));
    dbg!(generate(3));
    dbg!(generate(4));
    dbg!(generate(1234567).format());
    
    for i in 0..20 {
        let key= 12345 * i + 654321;
        // println!("{:b}", key);
        let form = generate(key);
        println!("{:?}", &form.format());
        for j in 0..20 {
            let v = form.compute(&[j as i32]);
            print!("{}, ", v);
        }
        println!();
    }
}
