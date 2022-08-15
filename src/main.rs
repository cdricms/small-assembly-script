use smasm::{registers::{ARegister, BRegister, MainRegister, EqualityRegister}, buffers::Stdout, parser::Parser, datastructs::stack::Stack};

fn main() {

    let mut a_reg = ARegister::new();    
    let mut b_reg = BRegister::new();
    let mut eq_reg = EqualityRegister::new();
    let mut main_reg = MainRegister::new();

    let mut stdout = Stdout::new();

    let mut parser = Parser::new();
    parser.parse("./main.smasm".to_string());

    let mut current_line = 0 as usize;
    let mut indices = Stack::<usize>::new_empty(20);

    while let Some(mut line) = parser.commands.pop() {
        let line_len = line.length;
        while let Some(command) = line.pop() {
            current_line += 1;
            let c = command.as_str();
            let first_com = line_len - 1 == line.length;
            match c {
                "STA" | "STB" if first_com => {
                    if let Some(mut value) = line.pop() {
                        let mut radix = 10u32;
                        if value.len() > 2 {
                            let (rep, v) = value.split_at(2);
                            if rep == "0b" || rep == "0x" {
                                radix = if rep == "0b" {2} else {16};
                                value = v.to_string();
                            } 
                        } 

                        if let Ok(int) = u8::from_str_radix(&value, radix)  {
                            if c == "STA" {
                                a_reg.store(int);
                            } else if c == "STB" {
                                b_reg.store(int);
                            }
                        }
                    }

                },
                "RTA" if first_com => {
                    main_reg.store(a_reg.retrieve());
                },
                "RTB" if first_com => {
                    main_reg.store(b_reg.retrieve());
                },
                "ADD" | "MUL" | "SUB" if first_com => {
                    let m = main_reg.retrieve();
                    let value = if let Some(x) =  line.pop() {
                        let mut radix = 10u32;
                        let mut res = 0u8;
                        if x == "RTA" {
                           res = a_reg.retrieve(); 
                        } else if x == "RTB" {
                            res =  b_reg.retrieve();
                        } else {
                            let mut value = String::new();
                            if x.len() > 2 {
                                let (rep, v) = x.split_at(2);
                                if rep == "0b" || rep == "0x" {
                                    radix = if rep == "0b" {2} else {16};
                                    value = v.to_string();
                                } 
                            } 
                            if let Ok(int) = u8::from_str_radix(&value, radix) {
                                res = int;
                            }

                        }
                        res

                    } else {
                        0u8
                    };

                    let store = line.pop().expect("A register such as A (STA) is expected.");

                    let res = match c {
                        "ADD" => m + value,
                        "MUL" => m * value,
                        "SUB" => m - value,
                        _ => 0u8
                    };

                    if store == "STA" {
                        a_reg.store(res);
                    } else if store == "STB" {
                        b_reg.store(res);
                    }

                }, 
                "SDO" if first_com => {
                    stdout.buffer.push(main_reg.retrieve());
                },
                "OUT" if first_com => {
                    stdout.output();
                },
                "EQ" | "NE" | "GT" | "LT"  if first_com => {
                    let m = main_reg.retrieve();

                    let value = if let Some(x) =  line.pop() {
                        let mut radix = 10u32;
                        let mut res = 0u8;
                        if x == "RTA" {
                           res = a_reg.retrieve(); 
                        } else if x == "RTB" {
                            res =  b_reg.retrieve();
                        } else {
                            let mut value = String::new();
                            if x.len() > 2 {
                                let (rep, v) = x.split_at(2);
                                if rep == "0b" || rep == "0x" {
                                    radix = if rep == "0b" {2} else {16};
                                    value = v.to_string();
                                } 
                            } 
                            if let Ok(int) = u8::from_str_radix(&value, radix) {
                                res = int;
                            }

                        }
                        res

                    } else {
                        0u8
                    };

                    match c {
                        "EQ" => eq_reg.store(if value == m { 1u8 } else { 0u8 }),
                        "NE" => eq_reg.store(if value != m { 1u8 } else { 0u8 }),
                        "LT" => eq_reg.store(if m < value { 1u8 } else { 0u8 }),
                        "GT" => eq_reg.store(if m > value { 1u8 } else { 0u8 }),
                        _ => {}
                    }
                },
                "JMP" if first_com => {
                   todo!() 
                }
                _ => {}
            }
        }
    }



}