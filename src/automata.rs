use std::collections::HashMap;

pub struct Automata {
    content: String,
}

#[derive(Debug, Hash, PartialEq, Eq)]
enum Token {
    ReservedWord,
    Identifier,
    RelationalOperator,
    LogicalOperator,
    ArithmeticOperator,
    Increment,
    Decrement,
    Assignment,
    Integer,
    Decimal,
    String,
    Comment,
    LineComment,
    Parenthesis,
    Brace,
}

const RESERVED_WORDS: [&str; 15] = [
    "if", "main", "else", "switch", "case", "default", "for", "do", "while", "break", "int", "String", "double", "char", "print",
];

impl Automata {
    pub fn new(content: String) -> Automata {
        Automata { content }
    }

    pub fn analyze(&self) -> String {
        let mut tokens = Vec::new();
        let mut errors = Vec::new();
        let mut token_counts: HashMap<Token, usize> = HashMap::new();
        let mut current_token = String::new();
        let mut current_state = 0;
        let mut current_line = 1;
        let mut current_column = 1;
        let mut flag = false;

        for c in self.content.chars() {
            match current_state {
                0 => {
                    if c.is_alphabetic() {
                        current_token.push(c);
                        current_state = 1;
                    } else if c.is_numeric() {
                        current_token.push(c);
                        current_state = 2;
                    } else if c == '"' { // case for strings
                        current_token.push(c);
                        current_state = 3;
                    } else if c == '/' {
                        current_token.push(c);
                        current_state = 4;
                    } else if c == '(' || c == ')' {
                        current_token.push(c);
                        current_state = 5;
                    } else if c == '{' || c == '}' {
                        current_token.push(c);
                        current_state = 6;
                    } else if c == '+' {
                        current_token.push(c);
                        current_state = 7;
                    } else if c == '-' {
                        current_token.push(c);
                        current_state = 8;
                    } else if c == '*' || c == '%' {
                        current_token.push(c);
                        current_state = 9;
                    } else if c == '=' {
                        current_token.push(c);
                        current_state = 10;
                    } else if c == '<' || c == '>' || c == '!' {
                        current_token.push(c);
                        current_state = 11;
                    } else if c == '&' {
                        current_token.push(c);
                        current_state = 12;
                    } else if c == '|' {
                        current_token.push(c);
                        current_state = 13;
                    } else if c == ' ' || c == '\t' {
                        current_column += 1;
                    } else if c == '\n' {
                        current_line += 1;
                        current_column = 1;
                    } else {
                        errors.push((current_line, current_column, c));
                    }
                }
                1 => {
                    if c.is_alphabetic() || c.is_numeric() || c == '_' {
                        current_token.push(c);
                    } else if c == '.' {
                        errors.push((current_line, current_column, '.'));
                        tokens.push((current_token.clone(), Token::Identifier));
                        current_token.push(c); // Add the dot to the current token
                        current_token.clear();
                        current_state = 0;
                        continue;
                    } else {
                        if RESERVED_WORDS.contains(&current_token.as_str()) {
                            tokens.push((current_token.clone(), Token::ReservedWord));
                            *token_counts.entry(Token::ReservedWord).or_insert(0) += 1;
                        } else {
                            tokens.push((current_token.clone(), Token::Identifier));
                            *token_counts.entry(Token::Identifier).or_insert(0) += 1;
                        }
                        current_token.clear();
                        current_state = 0;
                        continue;
                    }
                }
                2 => { // Integer state
                    if c.is_numeric() {
                        current_token.push(c);
                    } else if c == '.' {
                        current_token.push(c);
                        current_state = 14;
                    } else {
                        tokens.push((current_token.clone(), Token::Integer));
                        *token_counts.entry(Token::Integer).or_insert(0) += 1;
                        current_token.clear();
                        current_state = 0;
                        continue;
                    }
                }
                3 => {
                    if c == '"' {
                        current_token.push(c);
                        tokens.push((current_token.clone(), Token::String));
                        *token_counts.entry(Token::String).or_insert(0) += 1;
                        current_token.clear();
                        current_state = 0;
                    } else {
                        current_token.push(c);
                    }
                }
                4 => {
                    if c == '*' {
                        current_token.push(c);
                        current_state = 15;
                    } else if c == '/' {
                        current_token.push(c);
                        current_state = 16;
                    } else {
                        tokens.push((current_token.clone(), Token::ArithmeticOperator));
                        *token_counts.entry(Token::ArithmeticOperator).or_insert(0) += 1;
                        current_token.clear();
                        current_state = 0;
                        continue;
                    }
                }
                5 => {
                    tokens.push((current_token.clone(), Token::Parenthesis));
                    *token_counts.entry(Token::Parenthesis).or_insert(0) += 1;
                    current_token.clear();
                    current_state = 0;
                    continue;
                }
                6 => {
                    tokens.push((current_token.clone(), Token::Brace));
                    *token_counts.entry(Token::Brace).or_insert(0) += 1;
                    current_token.clear();
                    current_state = 0;
                    continue;
                }
                7 => {
                    if c == '+' {
                        current_token.push(c);
                        tokens.push((current_token.clone(), Token::Increment));
                        *token_counts.entry(Token::Increment).or_insert(0) += 1;
                    } else {
                        tokens.push((current_token.clone(), Token::ArithmeticOperator));
                        *token_counts.entry(Token::ArithmeticOperator).or_insert(0) += 1;
                        current_state = 0;
                        current_token.clear();
                        continue;
                    }
                    current_token.clear();
                    current_state = 0;
                }
                /* 8 => {
                    if c == '-' {
                        current_token.push(c);
                        tokens.push((current_token.clone(), Token::Decrement));
                        *token_counts.entry(Token::Decrement).or_insert(0) += 1;
                    } else if c.is_numeric() {
                        current_token.push(c);
                        current_state = 2;
                    } else {
                        tokens.push((current_token.clone(), Token::ArithmeticOperator));
                        *token_counts.entry(Token::ArithmeticOperator).or_insert(0) += 1;
                        current_state = 0;
                        current_token.clear();
                        continue;
                    }
                    current_token.clear();
                    current_state = 0;

                } */
                8 => {
                    if c == '=' {
                        current_token.push(c);
                        tokens.push((current_token.clone(), Token::RelationalOperator));
                        *token_counts.entry(Token::RelationalOperator).or_insert(0) += 1;
                        current_token.clear();
                        current_state = 0;
                    } else if c == '-' {
                        current_token.push(c);
                        tokens.push((current_token.clone(), Token::Decrement));
                        *token_counts.entry(Token::Decrement).or_insert(0) += 1;
                        current_token.clear();
                        current_state = 0;
                    } else if c.is_numeric() {
                        current_token.push(c);
                        current_state = 2;
                    } else {
                        tokens.push((current_token.clone(), Token::ArithmeticOperator));
                        *token_counts.entry(Token::ArithmeticOperator).or_insert(0) += 1;
                        current_token.clear();
                        current_state = 0;
                        continue;
                    }
                    current_column += 1;
                }
                9 => {
                    tokens.push((current_token.clone(), Token::ArithmeticOperator));
                    *token_counts.entry(Token::ArithmeticOperator).or_insert(0) += 1;
                    current_token.clear();
                    current_state = 0;
                    continue;
                }
                10 => {
                    if c == '=' { // Check if the next character is an equal sign to form the relational operator (e.g ==)
                        current_token.push(c);
                        tokens.push((current_token.clone(), Token::RelationalOperator));
                        *token_counts.entry(Token::RelationalOperator).or_insert(0) += 1;
                    } else {
                        tokens.push((current_token.clone(), Token::Assignment));
                        *token_counts.entry(Token::Assignment).or_insert(0) += 1;
                        current_state = 0;
                        current_token.clear();
                        continue;
                    }
                    current_token.clear();
                    current_state = 0;
                }
                11 => { // Relational state (e.g <, >, <=, >=, !=)
                    if c == '=' {
                        current_token.push(c);
                        tokens.push((current_token.clone(), Token::RelationalOperator));
                        *token_counts.entry(Token::RelationalOperator).or_insert(0) += 1;
                    } else if c == '.' {
                        current_token.push(c);
                        errors.push((current_line, current_column, c));
                        current_state = 0;
                        current_token.clear();
                        continue;
                    } else {
                        tokens.push((current_token.clone(), Token::RelationalOperator));
                        *token_counts.entry(Token::RelationalOperator).or_insert(0) += 1;
                        current_state = 0;
                        current_token.clear();
                        continue;
                    }
                    current_token.clear();
                    current_state = 0;
                }
                12 => {
                    if c == '&' {
                        current_token.push(c);
                        tokens.push((current_token.clone(), Token::LogicalOperator));
                        *token_counts.entry(Token::LogicalOperator).or_insert(0) += 1;
                    } else {
                        errors.push((current_line, current_column, c));
                    }
                    current_token.clear();
                    current_state = 0;
                }
                13 => {
                    if c == '|' {
                        current_token.push(c);
                        tokens.push((current_token.clone(), Token::LogicalOperator));
                        *token_counts.entry(Token::LogicalOperator).or_insert(0) += 1;
                    } else {
                        errors.push((current_line, current_column, c));
                    }
                    current_token.clear();
                    current_state = 0;
                }
                14 => { // Decimal state
                    // Check error (e.g 24..17) that has two dots and there for is not a valid decimal number
                    if c == '.' {
                        current_token.push(c); // Continue in 14 state to avoid more errors (like 17 counting as an integer)
                        errors.push((current_line, current_column, c)); // Add error
                        flag = true; // Set flag to true to avoid adding the token
                        continue;
                    }

                    if c.is_numeric() {
                        current_token.push(c);
                    } else {
                        if !flag { // If there is no error, add the token
                            tokens.push((current_token.clone(), Token::Decimal));
                            *token_counts.entry(Token::Decimal).or_insert(0) += 1;
                        }
                        current_token.clear();
                        current_state = 0;
                        flag = false;
                        continue;
                    }
                }
                15 => {
                    // Check if the comment is closed
                    if c == '*' {
                        current_token.push(c);
                        current_state = 17;
                    } else { // If the comment is not closed, continue adding characters to the comment
                        current_token.push(c);
                    }

                    // if the comment is never closed, add an error
                    if c == '\n' {
                        errors.push((current_line, current_column, c));
                        current_line += 1;
                        current_column = 1;
                        current_state = 0;
                    }
                }
                16 => {
                    if c == '\n' { // If the comment is closed, add the token and reset the current token
                        tokens.push((current_token.clone(), Token::LineComment));
                        *token_counts.entry(Token::LineComment).or_insert(0) += 1;
                        current_token.clear();
                        current_state = 0;
                        current_line += 1;
                        current_column = 1;
                    } else {
                        current_token.push(c);
                    }
                }
                17 => { // Comment state
                    if c == '/' {
                        current_token.push(c);
                        tokens.push((current_token.clone(), Token::Comment));
                        *token_counts.entry(Token::Comment).or_insert(0) += 1;
                        current_token.clear();
                        current_state = 0;
                    } else {
                        current_token.push(c);
                    }

                    // if the comment is never closed, add an error
                    if c == '\n' {
                        errors.push((current_line, current_column, c));
                    }
                }
                _ => {
                    errors.push((current_line, current_column, c));
                }
            }
        }

        let mut result = String::new();
        result.push_str(&format!("Palabras reservadas : {}\n", token_counts.get(&Token::ReservedWord).unwrap_or(&0)));
        result.push_str(&format!("Identificadores : {}\n", token_counts.get(&Token::Identifier).unwrap_or(&0)));
        result.push_str(&format!("Operadores Relacionales : {}\n", token_counts.get(&Token::RelationalOperator).unwrap_or(&0)));
        result.push_str(&format!("Operadores Lógicos : {}\n", token_counts.get(&Token::LogicalOperator).unwrap_or(&0)));
        result.push_str(&format!("Operadores Aritméticos : {}\n", token_counts.get(&Token::ArithmeticOperator).unwrap_or(&0)));
        result.push_str(&format!("Asignaciones : {}\n", token_counts.get(&Token::Assignment).unwrap_or(&0)));
        result.push_str(&format!("Número Enteros : {}\n", token_counts.get(&Token::Integer).unwrap_or(&0)));
        result.push_str(&format!("Números Decimales : {}\n", token_counts.get(&Token::Decimal).unwrap_or(&0)));
        result.push_str(&format!("Incremento : {}\n", token_counts.get(&Token::Increment).unwrap_or(&0)));
        result.push_str(&format!("Decremento : {}\n", token_counts.get(&Token::Decrement).unwrap_or(&0)));
        result.push_str(&format!("Cadena de Caracteres : {}\n", token_counts.get(&Token::String).unwrap_or(&0)));
        result.push_str(&format!("Comentario : {}\n", token_counts.get(&Token::Comment).unwrap_or(&0)));
        result.push_str(&format!("Comentario de Linea : {}\n", token_counts.get(&Token::LineComment).unwrap_or(&0)));
        result.push_str(&format!("Paréntesis : {}\n", token_counts.get(&Token::Parenthesis).unwrap_or(&0)));
        result.push_str(&format!("Llaves : {}\n", token_counts.get(&Token::Brace).unwrap_or(&0)));
        result.push_str(&format!("Errores : {}\n", errors.len()));

        result
    }
}
