use std::io;
struct Tarea {
    descripcion: String,
    completada: bool,
}

fn main() {
    println!("Bienvenido al gestor de tareas");

    let mut tareas: Vec<Tarea> = Vec::new();

    loop {
        println!("ingresa un comando('agregar <descripcion>','salir')");

        let mut entrada = String::new();
        io::stdin()
            .read_line(&mut entrada)
            .expect("Error al leer la entrada");
        let entrada = entrada.trim();

        if entrada == "salir" {
            println!("Saliendo del gestor de tareas");
            break;
        } else if entrada.starts_with("agregar ") {
            let descripcion = entrada[8..].trim();
            if !descripcion.is_empty() {
                tareas.push(Tarea {
                    descripcion: descripcion.to_string(),
                    completada: false,
                });
                println!("Tarea agregada: {}", descripcion);
            } else {
                println!("La descripción de la tarea no puede estar vacía.");
            }
        } else {
            println!("Comando no reconocido. Intenta de nuevo.");
        }

        println!("Tareas pendientes:");
        // usize entero sin signo del tamano de la arquitectura del procesador
        // isize entero con signo del tamano de la arquitectura del procesador
        for (i, tarea) in tareas.iter().enumerate() {
            if !tarea.completada {
                println!("{}. {}", i + 1, tarea.descripcion);
            }
        }
    }
}
