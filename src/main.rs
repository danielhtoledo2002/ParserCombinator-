enum ParserError<'a> {
    NotEmpty {
        expected: char,
        receive: char,
        input: &'a str,
    },
    Emptyy,
}

fn pchar<'a>(input: &'a str, charac: char) -> Result<&'a str, ParserError> {
    if !input.is_empty() {
        if input.chars().next().unwrap() == charac {
            Ok(&input[1..])
        } else {
            Err(ParserError::NotEmpty {
                expected: charac,
                receive: input.chars().next().unwrap(),
                input,
            })
        }
    } else {
        Err(ParserError::Emptyy)
    }
}

fn main() {
    let x = "oola";
    let s = 'H';

    match pchar(x, s) {
        Ok(input) => println!("El valor resultante es {:?} ", input),
        Err(ParserError::NotEmpty {
            expected,
            receive,
            input,
        }) => println!(
            "El character esperado era {:?} se recibio {:?} y el input fue {:?}",
            expected, receive, input
        ),
        Err(ParserError::Emptyy) => println!("El input es vacio"),
    }
}
