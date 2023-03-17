// #![feature(trait_alias)]

enum ErrType<'a> {
    NotFound {
        spected: char,
        found: char,
        input: &'a str,
    },
    Empty,
}
// errtype es para devolver un enum (una intancia)

// Version 2
// fn pchar<'a>(input: &'a str, match_char: char) -> Result<(&'a str, char), ErrType> {
//     if input.is_empty() {
//         Err(ErrType::Empty)
//     // No se puede hacer como en c++ tengo que acceder primero a un iterador con  el cual
//     // usar un metodo para poder recorrer el str
//     } else if input.chars().nth(0).unwrap() == match_char {
//         Ok((&input[1..], match_char))
//     } else {
//         Err(ErrType::NotFound {
//             spected: match_char,
//             found: input.chars().next().unwrap(),
//             input,
//         })
//     }
// }

// trait Parserr = Fn(&str) -> Result<(&str, char), ErrType>;

trait Parser<V>: Fn(&str) -> Result<(&str, V), ErrType> {}

impl<T, V> Parser<V> for T where T: Fn(&str) -> Result<(&str, V), ErrType> {}

fn pcharr(match_char: char) -> impl Parser<char> {
    move |input| {
        if input.is_empty() {
            Err(ErrType::Empty)
        // No se puede hacer como en c++ tengo que acceder primero a un iterador con  el cual
        // usar un metodo para poder recorrer el str
        } else if input.chars().nth(0).unwrap() == match_char {
            Ok((&input[1..], match_char))
        } else {
            Err(ErrType::NotFound {
                spected: match_char,
                found: input.chars().next().unwrap(),
                input,
            })
        }
    }
}

fn andthen<V>(p1: impl Parser<V>, p2: impl Parser<V>) -> impl Parser<Vec<V>> {
    move |input| {
        let (input, valor1) = p1(input)?;
        let (input, valor2) = p2(input)?;
        Ok((input, vec![valor1, valor2]))
    }
}

fn orthen<V>(p1: impl Parser<V>, p2: impl Parser<V>) -> impl Parser<Vec<V>> {
    move |input| {
        if let Ok((input, valor1)) = p1(input) {
            Ok((input, vec![valor1]))
        } else {
            let (input, valor2) = p2(input)?;
            Ok((input, vec![valor2]))
        }
    }
}

fn map<V, K>(p1: impl Parser<V>, f: impl Fn((&str, V)) -> (&str, K)) -> impl Parser<K> {
    move |input| {
        // let (input, valor1) = P1(input)?;
        // let (input, valor2) = f((input, valor1));
        // Ok((input, valor2))
        Ok(f(p1(input)?))
    }
}

fn main() {
    let parsea1 = pcharr('1');
    let parser2 = pcharr('2');
    let parser1and2 = andthen(parsea1, parser2);
    let transtostring = map(parser1and2, |(input, value)| {
        (input, value.into_iter().collect::<String>())
    });
    let parse12 = map(transtostring, |(input, value)| {
        (input, value.parse::<u8>().unwrap())
    });

    match parse12("12") {
        Ok((resultado, caracter)) => {
            println!(
                "Se encontro el caracter {:?}, y quedo {:?} ",
                caracter + 4,
                resultado
            )
        }
        Err(ErrType::Empty) => {
            println!("Esta vacio")
        }
        Err(ErrType::NotFound {
            spected,
            found,
            input,
        }) => {
            println!(
                "Se esperaba el caracter {}, se encontro  {} en el string {} ",
                spected, found, input
            )
        }
    }
}
