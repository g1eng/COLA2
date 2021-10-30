use std::io::{stdin,BufRead,StdinLock};

struct Cola2 {
    debug: bool,
    serialize: bool,
    ret: u128,
    p: u128,
}

impl Cola2 {

    fn is_on_octet_boundary(&self) -> bool {
        self.ret % 8 == 0 && self.ret != 0
    }

    fn opa(&mut self, p :u128) -> u128 {
        if self.serialize {
            print!("A");
        } else if self.debug {
            print!(" -> {} ", p);
        }
        self.f(p / 2)
    }
    fn opb(&mut self, p :u128) -> u128 {
        if self.serialize {
            print!("B");
        } else if self.debug {
            print!(" -> {} ", p)
        }
        self.f(p * 3 + 1)
    }
    fn f (&mut self, p :u128) -> u128 {
        if p == 0 {
            panic!("undef");
        }
        if p == 1 {
            if self.serialize {
                println!("");
            } else if self.debug {
                println!(" -> 1 [{}]", self.ret);
            } else {
                println!("{} {}", self.p, self.ret);
            }
            return 0
        }
        if self.is_on_octet_boundary() {
            if self.serialize {
                /*if self.ret % 64 == 0 {
                    println!("")
                }*/
                //print!(" ");
                println!("");
            } else if self.debug {
                print!(" \n [{}] ",self.ret);
            }
        }
        self.ret = self.ret + 1;
        match p % 2 {
            0 => self.opa(p),
            1 => self.opb(p),
            _ => 0,
        }
    }
    pub fn run (p :u128, debug :bool, serialize :bool) {
        let mut runner = Cola2{
            debug: debug,
            serialize: serialize,
            ret: 0,
            p: p,
        };
        runner.f(p);

        //pretty
        /*
        if runner.serialize {
            println!("\n\n\n\n\n\n\n\n\n\n\n\n");
        }
        */
    }
}

fn calc_foreach(raw_line: &str) {
    let mut numbers = raw_line.split_whitespace().collect::<Vec<_>>();
    while let Some(n) = numbers.pop() {
        if let Ok(n) = n.parse::<u128>() {
            //serialize
            //Cola2::run(n, false, true);

            //debug
            //Cola2::run(n, true, false);

            //plain
            Cola2::run(n, false, false);
        }
    }
}

fn read_stdin(reader :StdinLock) {
    for line in reader.lines() {
        let line = line.unwrap();
        calc_foreach(&line);
    }
}

fn main() {
    let stdin = stdin();
    let reader = stdin.lock();
    read_stdin(reader);
}
