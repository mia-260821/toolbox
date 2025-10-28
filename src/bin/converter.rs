use std::env;
use std::process;

fn convert(num_str: &str, from_radius: u32, to_radius: u32) -> Result<String, String> {
    let num = u64::from_str_radix(num_str, from_radius);
    if num.is_err() {
        return Err(num.err().expect("it should be ParseIntError").to_string());
    }
    let num = num.expect("no error here");
    match to_radius {
        2 => Ok(format!("{:04b}", num)),
        16 => Ok(format!("{:X}", num)),
        10 => Ok(format!("{}", num)),
        8 => Ok(format!("{:o}", num)),
        _ => Err(format!("unknown target form {to_radius}"))
    }
}

fn xor(num1_str: &str, num2_str: &str, radius: u32) -> Result<String, String> {
    let num1 = u64::from_str_radix(num1_str, radius);
    if num1.is_err() {
        return Err(num1.err().expect(format!("{num1_str} should be ParseIntError").as_str()).to_string());
    }
    let num2 = u64::from_str_radix(num2_str, radius);
    if num2.is_err() {
        return Err(num1.err().expect(format!("{num2_str} should be ParseIntError").as_str()).to_string());
    }
    let rslt = num1.unwrap() ^ num2.unwrap();
    match radius {
        16 => Ok(format!("{:X}", rslt)),
        2 => Ok(format!("{:04b}", rslt)),
        _ => Ok(format!("{}", rslt))
    }
}


fn main() {
    // Collect arguments passed from the command line
    let args: Vec<String> = env::args().collect();

    // Check if we have exactly one argument (string)
    if args.len() < 3 {
        eprintln!("Usage: 2to16|2to10|10to16|10to2|16to10|16to2|xor2|xor16 <string>");
        process::exit(1);
    }

    // The string argument
    let operator = &args[1];

    let result  = match operator.as_str() {
        "2to16" => convert(&args[2].as_str(), 2, 16),
        "2to10" => convert(&args[2].as_str(), 2, 10),
        "10to2" => convert(&args[2].as_str(), 10, 2),
        "10t016" => convert(&args[2].as_str(), 10, 16),
        "16to2" => convert(&args[2].as_str(), 16, 2),
        "16to10" => convert(&args[2].as_str(), 16, 10),
        "xor2" => xor(&args[2], &args[3], 2),
        "xor16" => xor(&args[2], &args[3], 16),
        _ => {
            eprintln!("unknown operation");
            process::exit(1);
        }
    };

    match result {
        Ok(s) => {
            println!("{s}");
            process::exit(0);
        }
        Err(e) => {
            eprintln!("Error {e}");
            process::exit(1);
        }
    }
}
