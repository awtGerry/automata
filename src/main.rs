use std::io::{self, Write};

fn crear_matriz(filas: usize, columnas: usize) -> Vec<Vec<String>> {
    let mut matriz = vec![vec![" ".to_string(); columnas + 1]; filas + 1];
    matriz
}

fn ingresar_estados(matriz: &mut Vec<Vec<String>>) {
    println!("\nIngresa los estados, separados por espacios:");
    let estados = leer_entrada();
    for (i, estado) in estados.iter().enumerate() {
        matriz[i + 1][0] = estado.to_string();
    }
}

fn ingresar_alfabeto(matriz: &mut Vec<Vec<String>>, columnas: usize) {
    println!("\nIngresa los elementos del alfabeto, separados por espacios:");
    let elementos = leer_entrada();
    for (j, elemento) in elementos.iter().enumerate() {
        matriz[0][j + 1] = elemento.to_string();
    }
}

fn ingresar_transiciones(matriz: &mut Vec<Vec<String>>) {
    let filas = matriz.len() - 1;
    let columnas = matriz[0].len() - 1;
    for i in 0..filas {
        for j in 0..columnas {
            let estado = &matriz[i + 1][0];
            let elemento = &matriz[0][j + 1];
            print!("Ingrese la transición para el estado {} y el elemento {}: ", estado, elemento);
            io::stdout().flush().unwrap();
            let mut transicion = String::new();
            io::stdin().read_line(&mut transicion).unwrap();
            matriz[i + 1][j + 1] = transicion.trim().to_string();
        }
    }
}

fn verificar_transiciones(matriz: &mut Vec<Vec<String>>) {
    let mut nuevas_filas = Vec::new();
    for i in 1..matriz.len() {
        for j in 1..matriz[0].len() {
            if matriz[i][j].contains(' ') && matriz[i][j].split_whitespace().count() >= 2 {
                let elementos_adicionales = matriz[i][j].split_whitespace().collect::<Vec<&str>>();
                let mut nueva_fila = vec![" ".to_string(); matriz[0].len()];
                nueva_fila[0] = elementos_adicionales.join(" ");
                nuevas_filas.push((i + 1, nueva_fila));
            }
        }
    }
    for (index, (pos, fila)) in nuevas_filas.iter().enumerate() {
        matriz.insert(*pos + index, fila.clone());
    }
}

fn imprimir_matriz(matriz: &Vec<Vec<String>>) {
    for fila in matriz {
        for elemento in fila {
            print!("{}\t", elemento);
        }
        println!();
    }
}

fn convertir_afnd_a_afd(matriz: &Vec<Vec<String>>) -> (Vec<Vec<String>>, Vec<Vec<String>>, Vec<String>) {
    let estados_afnd = matriz.iter().skip(1).map(|fila| fila[0].clone()).collect::<Vec<String>>();
    let alfabeto = matriz[0].iter().skip(1).map(|elemento| elemento.clone()).collect::<Vec<String>>();

    let mut transiciones_afnd = std::collections::HashMap::<(String, String), Vec<String>>::new();
    for i in 1..matriz.len() {
        let estado = &matriz[i][0];
        for j in 1..matriz[0].len() {
            let simbolo = &matriz[0][j];
            let transicion = matriz[i][j].split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>();
            if !transicion.is_empty() {
                transiciones_afnd.insert((estado.clone(), simbolo.clone()), transicion);
            }
        }
    }

    let estado_inicial = vec![matriz[1][0].clone()];
    let estados_finales = matriz.iter().skip(1).filter(|fila| fila[0].contains(' ')).map(|fila| fila[0].clone()).collect::<Vec<String>>();

    let mut transiciones_afd = std::collections::HashMap::<Vec<String>, std::collections::HashMap<String, Vec<String>>>::new();
    let mut estados_afd = Vec::new();
    let mut cola_estados = vec![estado_inicial.clone()];
    while let Some(estado_actual) = cola_estados.pop() {
        let mut transiciones_estado = std::collections::HashMap::<String, Vec<String>>::new();
        for simbolo in &alfabeto {
            let mut alcanzables = vec![];
            for estado in &estado_actual {
                if let Some(transicion) = transiciones_afnd.get(&(estado.clone(), simbolo.clone())) {
                    alcanzables.extend(transicion.clone());
                }
            }
            let mut estado_afd = alcanzables.clone();
            estado_afd.sort();
            estado_afd.dedup();
            if !estado_afd.is_empty() && !estados_afd.contains(&estado_afd) {
                estados_afd.push(estado_afd.clone());
                cola_estados.push(estado_afd.clone());
            }
            transiciones_estado.insert(simbolo.clone(), estado_afd.clone());
        }
        transiciones_afd.insert(estado_actual.clone(), transiciones_estado);
    }
    let estados_finales_afd = estados_afd.iter().filter(|estado| estados_finales.iter().any(|fin| estado.contains(&fin))).cloned().collect::<Vec<Vec<String>>>();

    println!("\nEstados del AFD:");
    for estado in &estados_afd {
        println!("{:?}", estado);
    }

    println!("\nTransiciones del AFD:");
    for (estado, transiciones) in &transiciones_afd {
        for (simbolo, alcanzables) in transiciones {
            println!("{:?} -> {:?} -> {:?}", estado, simbolo, alcanzables);
        }
    }

    println!("\nEstados finales del AFD:");
    for estado in &estados_finales_afd {
        println!("{:?}", estado);
    }

    (estados_afd, estados_finales_afd, alfabeto)
}

fn leer_entrada() -> Vec<String> {
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).unwrap();
    entrada.trim().split_whitespace().map(|s| s.to_string()).collect()
}

fn main() {
    println!("Ingresa el número de estados: ");
    let filas: usize = leer_entrada()[0].parse().unwrap();
    println!("Ingresa el tamaño del alfabeto: ");
    let columnas: usize = leer_entrada()[0].parse().unwrap();

    let mut matriz = crear_matriz(filas, columnas);

    ingresar_estados(&mut matriz);
    ingresar_alfabeto(&mut matriz, columnas);
    ingresar_transiciones(&mut matriz);
    verificar_transiciones(&mut matriz);
    convertir_afnd_a_afd(&matriz);
}
