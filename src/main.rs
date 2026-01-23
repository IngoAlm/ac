use std::env;


fn convert_to_float(zahl1: String, zahl2: String) -> (f64, f64) {
    let z1 = zahl1.parse::<f64>().unwrap();
    // ODER
    let z2: f64 = zahl2.parse().unwrap();
    (z1, z2)
}

fn berechnen(zahl1: f64, zahl2: f64, operator: &str) -> f64 {
    match operator {
        "+" => {
            let ergebnis = zahl1 + zahl2;
            return ergebnis;
        }
        "-" => {
            let ergebnis = zahl1 - zahl2;
            return ergebnis;
        }
        "*" => {
            let ergebnis = zahl1 * zahl1;
            return ergebnis;
        }
        "/" => {
            let ergebnis = zahl1 / zahl2;
            return ergebnis;
        }
        "%" => {
            let ergebnis = zahl1 % zahl2;
            return ergebnis;
        }
        _ => {
            println!("Kein gültiger Operator!");
            return 0.0;
        }
    };
}

fn main() {
    // Pattern mit den Operatoren zur prüfung der Eingabe.
    let pattern = ["+", "-", "*", "/", "%"];

    // Argumente lesen.
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        let eingabe = &args[1];
        for p in &pattern {
            match eingabe.contains(p) {
                true => {
                    let zahlen_vec: Vec<&str> = eingabe.split(p).collect();
                    let (zahl1, zahl2) =
                        convert_to_float(zahlen_vec[0].to_string(), zahlen_vec[1].to_string());
                    let ergebnis = berechnen(zahl1, zahl2, p);
                    println!(
                        "Das Ergebnis von {} {} {} ist {}",
                        zahl1, p, zahl2, ((ergebnis * 100.0).round() / 100.0) // gerundet auf 2 dezimal
                    );
                    break;
                }
                false => {},
            }
        }
    } else if args.len() == 4 {
        let zahl1 = &args[1];
        let operator = &args[2];
        let zahl2 = &args[3];
        println!("Operator: {}", operator);

        for p in &pattern {
            match operator.contains(p) {
                true => {
                    let (z1, z2) = convert_to_float(zahl1.to_string(), zahl2.to_string());
                    let ergebnis = berechnen(z1, z2, p);
                    println!(
                        "Das Ergebnis von {} {} {} ist {}",
                        zahl1, p, zahl2, ((ergebnis * 100.0).round() / 100.0) // gerundet auf 2 Nachkommastellen
                    );
                    break;
                }
                false => {},
            }
        }
    } else {
        println!("Falsche Eingabe!");
        println!("Wie folgt eingeben: ac Zahl1 Operator Zahl2");
        println!("Folgende Operatoren sind gültig: + - * / %");
    }
}