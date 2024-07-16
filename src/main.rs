use std::fs::File;
use std::io::*;

/// Takes an input and tries to parse it to int32, otherwise, it will keep asking the user to
/// enter a valid input.
fn read_and_convert() -> u128 {
    let mut user_input: String = Default::default();
    loop {
        println!("Ingrese un numero de lineas");
        match stdin().read_line(&mut user_input) {
            Ok(input) => input,
            Err(error) => {
                eprintln!("Error en la lectura de la entrada del usuario: {}", error);
                continue
            }
        };
        match user_input.trim().parse::<u128>() {
            Ok(num) => return num,
            Err(error) => {
                eprintln!("Error de parseo: {}", error);
                user_input.clear();
                continue
            }
        };
    }
}

fn main() -> Result<()> {
    use std::time::Instant;
    // el archivo a ser creado, en caso de error se detiene la ejecución del programa y se notifica del error
    let f: File = match File::create("multiplosde2.py") {
        Ok(file) => file,
        Err(error) => panic!("Error creando el archivo: {}", error)
    };
    // buffer para la escritura de los datos al archivo
    let mut writer: BufWriter<File> = BufWriter::new(f);
    // limite de numeros a emplear
    let limit = read_and_convert();

    let timer = Instant::now();
    // aca arranca la logica del programa
    writer.write(b"ans = int(input('Ingrese un numero para saber si es par o impar: '))\nif ans == 1:\n\tprint('El numero 1 es impar')\n").expect("Error escribiendo linea");
    for n in 2..=limit {
        writer.write(format!("elif ans == {}:\n", n).as_bytes()).expect("Error escribiendo linea");
        let es_par = if n % 2 == 0 {"es par"} else {"es impar"};
        writer.write(format!("\tprint('El numero {} {}')\n", n, es_par).as_bytes()).expect("Error escribiendo linea");
    }
    writer.write(b"else:\n\tprint('Todavia no sabemos si el resto de numeros son pares o impares, vas a tener que esperar')").expect("Error escribiendo linea");

    // esto hay que ejecutarlo en un ide para saber el tiempo de respuesta, ya probé con esperar a un input pero es un quilombo porque no se guarda el archivo
    let finaltime = timer.elapsed();
    println!("Se finalizó el proceso en un tiempo de {:?}", finaltime);
    Ok(())
}