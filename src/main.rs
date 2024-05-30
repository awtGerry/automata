use std::io::{self, Write};

/* struct Automata {
    states: Vec<String>,
    paths: Vec<String>,
    transitions: Vec<Vec<String>>,
} */

fn states(matrix: &mut Vec<Vec<String>>) {
    println!("Input all states (example: q0 q1 q2):");
    let states = usr_input();
    for (i, estado) in states.iter().enumerate() {
        matrix[i + 1][0] = estado.to_string();
    }
}

fn path(matrix: &mut Vec<Vec<String>>) {
    println!("Input all paths (example: 0 1):");
    let path = usr_input();
    for (i, path) in path.iter().enumerate() {
        matrix[0][i + 1] = path.to_string();
    }
}

fn transitions(matrix: &mut Vec<Vec<String>>) {
    let rows = matrix.len();
    let columns = matrix[0].len();
    for i in 1..=rows {
        for j in 1..=columns {
            let state = &matrix[i][0];
            let path = &matrix[0][j];
            println!("Input transitions for state {} with path {} (example: q0 q1):", state, path);
            let transitions = usr_input();
            matrix[i][j] = transitions.join(" ");
        }
    }
}

fn verify(matrix: &mut Vec<Vec<String>>) {
    let mut rows = Vec::new();
    for i in 1..matrix.len() {
        for j in 1..matrix[0].len() {
            if matrix[i][j].contains(' ') && matrix[i][j].split_whitespace().count() >= 2 {
                let new_s = matrix[i][j].split_whitespace().collect::<Vec<&str>>();
                let mut new_row = vec![" ".to_string(); matrix[0].len()];
                new_row[0] = new_s.join(" ");
                rows.push((i + 1, new_row));
            }
        }
    }
    for (index, (pos, row)) in rows.iter().enumerate() {
        matrix.insert(*pos + index, row.clone());
    }
}

fn print(matrix: &Vec<Vec<String>>) {
    for row in matrix {
        for state in row {
            print!("{}\t", state);
        }
        println!();
    }
}

fn nfa_dfa(matrix: &Vec<Vec<String>>) -> (Vec<Vec<String>>, Vec<Vec<String>>, Vec<String>) {
    let nfa_states = matrix.iter().skip(1).map(|fila| fila[0].clone()).collect::<Vec<String>>();
    let paths = matrix[0].iter().skip(1).map(|elemento| elemento.clone()).collect::<Vec<String>>();

    let mut nfa_transitions = std::collections::HashMap::<(String, String), Vec<String>>::new();
    for i in 1..matrix.len() {
        let states = &matrix[i][0];
        for j in 1..matrix[0].len() {
            let sym = &matrix[0][j];
            let transition = matrix[i][j].split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>();
            if !transition.is_empty() {
                nfa_transitions.insert((states.clone(), sym.clone()), transition);
            }
        }
    }

    let first_state = vec![matrix[1][0].clone()];
    let last_state = matrix.iter().skip(1).filter(|fila| fila[0].contains(' ')).map(|fila| fila[0].clone()).collect::<Vec<String>>();

    let mut dfa_states = Vec::new();
    let mut dfa_transitions = std::collections::HashMap::<Vec<String>, std::collections::HashMap<String, Vec<String>>>::new();
    let mut states_list = vec![first_state.clone()];
    while let Some(state) = states_list.pop() {
        let mut state_transitions = std::collections::HashMap::<String, Vec<String>>::new();
        for sym in &paths {
            let mut reachables = vec![];
            for point in &state {
                if let Some(t) = nfa_transitions.get(&(point.clone(), sym.clone())) {
                    reachables.extend(t.clone());
                }
            }
            let mut dfa = reachables.clone();
            dfa.sort();
            dfa.dedup();
            if !dfa.is_empty() && !dfa_states.contains(&dfa) {
                dfa_states.push(dfa.clone());
                states_list.push(dfa.clone());
            }
            state_transitions.insert(sym.clone(), dfa.clone());
        }
        dfa_transitions.insert(state.clone(), state_transitions);
    }
    let final_states = dfa_states.iter().filter(|estado| last_state.iter().any(|fin| estado.contains(&fin))).cloned().collect::<Vec<Vec<String>>>();

    println!("DFA:");
    for state in &dfa_states {
        println!("{:?}", state);
    }

    println!("\nDFA transitions:");
    for (state, t) in &dfa_transitions {
        for (sym, reachs) in t {
            println!("{:?} -> {:?} -> {:?}", state, sym, reachs);
        }
    }

    println!("\nFinal states:");
    for state in &final_states {
        println!("{:?}", state);
    }

    (dfa_states, final_states, paths)
}

fn usr_input() -> Vec<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().split_whitespace().map(|s| s.to_string()).collect()
}

fn main() {
    println!("Ammount of states: ");
    let filas: usize = usr_input()[0].parse().unwrap();
    println!("Ammount of paths:");
    let columnas: usize = usr_input()[0].parse().unwrap();

    let mut matrix = vec![vec![" ".to_string(); columnas + 1]; filas + 1];

    states(&mut matrix);
    path(&mut matrix);
    transitions(&mut matrix);
    verify(&mut matrix);
    nfa_dfa(&matrix);
}
