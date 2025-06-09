use std::io;

fn count(last_number: i32) {
    if last_number.is_positive() {
        for num in 1..=last_number {
            println!("{}", num);
        }
    }
    if last_number.is_negative() {
        for num in last_number..=0 {
            println!("{}", num);
        }
    }
}

fn parse_in(prompt_in: String) -> i32 {
    let prompt_in: i32 = prompt_in
        .trim()
        .parse()
        .expect("Falha ao converter string para i32");

    return prompt_in;
}

fn get_in() -> String {
    let mut prompt_in = String::new();

    io::stdin()
        .read_line(&mut prompt_in)
        .expect("Falha ao ler entrada");

    return prompt_in;
}

fn main() {
    println!("Deseja contar até que número?");
    let prompt = get_in();
    let parsed_in = parse_in(prompt);

    println!("=====================");
    count(parsed_in);
}
