mod parser;

fn process_file(current_program_path: &str, source_path: &str) {
    const CALC_SUFFIX: &str = ".calc";
    if !source_path.ends_with(CALC_SUFFIX) {
        eprint!(
            "{}:Invalid argument '{}' : It mustt be with {}",
            current_program_path, source_path, CALC_SUFFIX
        );
        return;
    }

    let source_code = std::fs::read_to_string(&source_path);
    if source_code.is_err() {
        eprintln!(
            "Failed to read from file {}: ({})",
            source_path,
            source_code.unwrap_err()
        );
        return;
    }
    let source_code = source_code.unwrap();
    let parsed_program;
    match parser::parse_program(&source_code) {
        Ok((rest, synatx_tree)) => {
            let trimmed_rest = rest.trim();
            if trimmed_rest.len() > 0 {
                eprintln!(
                    "Invalid remaining code in '{}': {}",
                    source_path, trimmed_rest
                );
                return;
            }
            parsed_program = synatx_tree;
        }
        Err(err) => {
            eprintln!("Invalid code in '{}': {:?}", source_path, err);
            return;
        }
    }

    println!("Parsed program: {:#?}", parsed_program);
}

fn main() {
    // println!("Hello, world!");
    let mut args = std::env::args();
    let current_program_path = args.next().unwrap();
    let source_path = args.next();
    if source_path.is_none() {
        eprintln!("{}: Missing argument <file>.calc", current_program_path);
    } else {
        process_file(&current_program_path, &source_path.unwrap());
    }
}
