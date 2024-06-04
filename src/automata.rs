/* Instrucciones:
Desarrollar Un Autómata que sea capaz de identificar los siguientes tokens:
Palabra reservada (if, main, else, switch, case, default, for, do, while, break, int, String,
double, char, print)
Identificador (Inicia con letra, sin espacios en blanco, sin caracteres especiales, excepto el
guión bajo)
Operador relacional (<, <=, >, >=, ==, !=)
Operador lógico (&&, ||, !)
Operador aritmético (+, -, *, /, %)
Incremento ( ++ )
Decremento ( -- )
Asignación ( = )
Número Entero ( Negativo ó positivo)
Número decimal ( Negativo ó positivo)
Cadena de caracteres (Con el formato “Cualquier carácter” ) 
Comentario ( Con el formato /*   */)
Comentario de Linea ( Con el formato //Cualquier carácter )
Parentesis ( (,) )
Llave  ( {, } )
 
Las entradas para el autómata serán cualquier valor alfanumérico, caracteres especiales,
espacios en blanco, tabuladores y saltos de linea (Enter).

El Sistema deberá de funcionar de la siguiente forma:

La aplicación debe recibir un archivo externo(Con extensión .txt) (done in main.rs, we only get the content (String) from the file)

El archivo contendrá una serie de tokens que deberán de ser identificados por el
sistema,  cada token que contenga el archivo estará separado por un espacio en
blanco, un tabulador o un terminador de  línea.

La aplicación deberá de hacer un recorrido por todo el archivo y deberá identificar los
tokens  contenidos  en  el  mismo.  Si  existen  token  erróneos  el  sistema  deberá
identificarlos.

Una vez que el archivo haya sido analizado, el sistema deberá de imprimir un reporte
de la cantidad y tipo de tokens identificados, así mismo,  la cantidad de tokens
erróneos.

Consideraciones importantes:
Para  identificar  los  tokens  del  archivo,  se  debe  de  realizar  estrictamente  a  través  del
autómata,  es  decir,  el  sistema,  una  vez  que  reciba  el  archivo  externo, deberá  de
recorrerlo caracter por caracter para identificar las coincidencias, cada caracter
representa una entrada para el autómata.
Los únicos tokens que se pueden identificar sin el autómata son las palabras reservadas.
“Si  se  utiliza  una  expresión  regular  para  identificar  los  tokens  se  anula  el
proyecto”
Es indistino el uso de mayúsculas y minúsculas, a excepción de las palabras reservadas
(Todas deben escribirse en minúsculas)
*/

pub struct Automata {
    content: String
}

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
    Brace
}

const RESERVED_WORDS: [&str; 15] = [
    "if", "main", "else", "switch", "case", "default", "for", "do", "while", "break", "int", "String", "double", "char", "print"
];

impl Automata {
    pub fn new(content: String) -> Automata {
        Automata {
            content
        }
    }

    fn analyze(&self) {
        let mut tokens = Vec::new();
        let mut errors = Vec::new();
        let mut current_token = String::new();
        let mut current_state = 0;
        let mut current_line = 1;
        let mut current_column = 1;

        for c in self.content.chars() {
            match current_state {
                0 => {
                    if c.is_alphabetic() {
                        current_token.push(c);
                        current_state = 1;
                    } else if c.is_digit(10) {
                        current_token.push(c);
                        current_state = 2;
                    } else if c == '+' {
                        current_token.push(c);
                        current_state = 3;
                    } else if c == '-' {
                        current_token.push(c);
                        current_state = 4;
                    } else if c == '*' {
                        current_token.push(c);
                        current_state = 5;
                    } else if c == '/' {
                        current_token.push(c);
                        current_state = 6;
                    } else if c == '%' {
                        current_token.push(c);
                        current_state = 7;
                    } else if c == '=' {
                        current_token.push(c);
                        current_state = 8;
                    } else if c == '"' {
                        current_token.push(c);
                        current_state = 9;
                    } else if c == '<' {
                        current_token.push(c);
                        current_state = 10;
                    } else if c == '>' {
                        current_token.push(c);
                        current_state = 11;
                    } else if c == '&' {
                        current_token.push(c);
                        current_state = 12;
                    } else if c == '|' {
                        current_token.push(c);
                        current_state = 13;
                    } else if c == '!' {
                        current_token.push(c);
                        current_state = 14;
                    } else if c == '(' || c == ')' {
                        current_token.push(c);
                        current_state = 15;
                    } else if c == '{' || c == '}' {
                        current_token.push(c);
                        current_state = 16;
                    } else if c == '\n' {
                        current_line += 1;
                        current_column = 1;
                    } else if c == ' ' || c == '\t' {
                        current_column += 1;
                    } else {
                        errors.push((current_line, current_column, c));
                    }

                    current_column += 1;
                }
                1 => { // Identifier
                    if c.is_alphanumeric() || c == '_' {
                        current_token.push(c); // means that the token is still being built
                    } else {
                        tokens.push((current_token, Token::Identifier)); // means that the token is finished
                        current_token = String::new(); // reset the token
                        current_state = 0; // reset the state
                    }
                }
                2 => { // Integer
                    if c.is_digit(10) {
                        current_token.push(c);
                    } else {
                        tokens.push((current_token, Token::Integer));
                        current_token = String::new();
                        current_state = 0;
                    }
                }
                3 => { // Increment or ArithmeticOperator
                    if c == '+' {
                        tokens.push((current_token, Token::Increment));
                        current_token = String::new();
                        current_state = 0;
                    } else {
                        errors.push((current_line, current_column, c)); // means that the token is invalid
                    }
                }
                4 => {
                    if c == '-' {
                        tokens.push((current_token, Token::Decrement));
                        current_token = String::new();
                        current_state = 0;
                    } else {
                        errors.push((current_line, current_column, c));
                    }
                }
                5 => {
                    if c == '*' {
                        tokens.push((current_token, Token::Comment));
                        current_token = String::new();
                        current_state = 0;
                    } else {
                        errors.push((current_line, current_column, c));
                    }
                }
                6 => {
                    if c == '/' {
                        tokens.push((current_token, Token::LineComment));
                        current_token = String::new();
                        current_state = 0;
                    } else {
                        errors.push((current_line, current_column, c));
                    }
                }
                7 => {
                    tokens.push((current_token, Token::ArithmeticOperator));
                    current_token = String::new();
                    current_state = 0;
                }
                8 => {
                    tokens.push((current_token, Token::Assignment));
                    current_token = String::new();
                    current_state = 0;
                }
                9 => {
                    if c == '"' {
                        current_token.push(c);
                        tokens.push((current_token, Token::String));
                        current_token = String::new();
                        current_state = 0;
                    } else {
                        current_token.push(c);
                    }
                }
                10 => {
                    if c == '=' {
                        current_token.push(c);
                        tokens.push((current_token, Token::RelationalOperator));
                        current_token = String::new();
                        current_state = 0;
                    } else {
                        tokens.push((current_token, Token::RelationalOperator));
                        current_token = String::new();
                        current_state = 0;
                    }
                }
                11 => {
                    if c == '=' {
                        current_token.push(c);
                        tokens.push((current_token, Token::RelationalOperator));
                        current_token = String::new();
                        current_state = 0;
                    } else {
                        tokens.push((current_token, Token::RelationalOperator));
                        current_token = String::new();
                        current_state = 0;
                    }
                }
                12 => {
                    if c == '&' {
                        current_token.push(c);
                        tokens.push((current_token, Token::LogicalOperator));
                        current_token = String::new();
                        current_state = 0;
                    } else {
                        errors.push((current_line, current_column, c));
                    }
                }
                13 => {
                    if c == '|' {
                        current_token.push(c);
                        tokens.push((current_token, Token::LogicalOperator));
                        current_token = String::new();
                        current_state = 0;
                    } else {
                        errors.push((current_line, current_column, c));
                    }
                }
                14 => {
                    if c == '=' {
                        current_token.push(c);
                        tokens.push((current_token, Token::RelationalOperator));
                        current_token = String::new();
                        current_state = 0;
                    } else {
                        tokens.push((current_token, Token::LogicalOperator));
                        current_token = String::new();
                        current_state = 0;
                    }
                }
                15 => {
                    tokens.push((current_token, Token::Parenthesis));
                    current_token = String::new();
                    current_state = 0;
                }
                16 => {
                    tokens.push((current_token, Token::Brace));
                    current_token = String::new();
                    current_state = 0;
                }
                _ => {}
            }
        }
    }
}
