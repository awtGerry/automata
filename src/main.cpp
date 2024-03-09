/*
Automata  que  recibe  un  archivo  externo(Con  extensión  .txt), el  cual contendrá
exclusivamente cadenas de texto (Texto plano), el objetivo del sistema es identificar en el
escrito las coincidencias de las siguientes dos palabras: Web y Ebay
*/

/*
La interfaz del sistema debe ser en un entorno visual, es decir, con ventanas, botones, cajas
de texto, etiquetas, tablas, etc.
*/

/*
∑  Representa cualquier carácter , por ejemplo la simbología ∑ -e -w para el estado 1 se
interpreta que si ingresa cualquier caracter , excepto la e y la w se debe transitar al mismo
estado. Si estoy en el estado 1 y se ingresa el caracter w se debe transitar al estado 12. Si
estoy en el estado 1 y se ingresa el carácter e se debe de transitar al estado 15.

Al final, el sistema deberá desplegar de forma visual cuantas coincidencias encontró de la
palabra web y cuantas coincidencias  encontró de la palabra ebay.
*/

/*
Consideraciones importantes:
El conteo de las coincidencias para web y ebay se debe de realizar estrictamente
a través del autómata, es decir, el sistema, una vez que reciba el archivo externo,
deberá de recorrerlo caracter por caracter para identificar las coincidencias, cada
caracter representa una entrada para el autómata.
“Si  se  utiliza  una  expresión  regular  para  identificar  las  palabras  se  anula  el
proyecto”
Para que el sistema cuente una coincidencia, las palabras web y ebay deben de estar sin
espacios  en  blanco entre  cada  caracter,  tampoco  debe  existir  saltos  de  línea  o
tabuladores entre estos, por ejemplo, las siguientes palabras no deben ser contabilizadas
por que tienen espacios en blanco, tabuladores y saltos de línea
we b                             
w    e    b
e
bay

Cada vez que el sistema encuentre la palabra  webay  deberá de contabilizar 1 vez la
palabra web y 1 vez la palabra ebay            webay        webay
Es indistino el uso de mayúsculas y minúsculas, es decir, no importa si la palabra web
aparece en mayúsculas, minúsculas o una combinación de estas, la palabra deberá de ser
contabilizada.
Nota: La implementación del autómata se debe de realizar desde cero, es
decir, el alumno debe de codificar TODA la solución del sistema
*/

/* AUTOMATA */
/*
estado 1 ∑ -e-w
estado 12: (∑ -e-w a estado 1) (w a estado 12) (e a estado 135)
estado 135: (∑ -b-e-w a estado 1) (w a estado 12) (e a estado 15) (b a estado 146)
estado 146: (∑ -a-e-w estado 1) (w a estado 12) (e a estado 15) (a a estado 17)
estado 15: (∑ -b-e-w a estado 1) (e a estado 15) (w a estado 12) (b a estado 16)
estado 16: (∑ -a-e-w a estado 1) (e a estado 15) (w a estado 12) (a a estado 17)
estado 17: (∑ -e-w-y a estado 1) (e a estado 15) (w a estado 12) (y a estado 18)
estado 18: (∑ -e-w a estado 1) (e a estado 15) (w a estado 12)
*/

#include <iostream>
#include <string>

std::string word = "";
int current_index = 0;
bool accept, find = false;

int accept_control();
void start_state();

void print_state(std::string name) // print state
{
    std::cout << word.substr(0, current_index) << " Currently in: " << name << " state" << std::endl;
}

/* EBAY */
void get_ebay_15();
void get_ebay_16();
void get_ebay_17();
void get_ebay_18();

/* WEB */
void get_web_12();
void get_web_135();
void get_web_146();

/* MAIN */
int main(int argc, char *argv[])
{
    word = "webay";
    start_state();
    return 0;
}

// Control
int accept_control() // 0: continue, 1: accept, 2: continue
{
    if (word.length() == 0)
        return -1;
    if (find && current_index == word.length())
        return 2; // continue
    else if (find)
        return 1; // just accept and stop
    if (current_index == word.length())
    {
        if (accept) return 1;
        else return -1;
    }
    else return 0;
}

// DFA
void start_state()
{
    print_state("inicio");
    accept = false;
    int ctrl_result = accept_control();
    if (ctrl_result == 0)
    {
        if (word[current_index] == 'e')
        {
            current_index++;
            get_ebay_15();
        }
        else if (word[current_index] == 'w')
        {
            current_index++;
            get_web_12();
        }
        else
        {
            current_index++;
            start_state();
        }
    }
    else
    {
        std::cout << "Not accepted" << std::endl;
    }
}

void get_ebay_15()
{
    print_state("15");
    accept = false;
    int ctrl_result = accept_control();
    if (ctrl_result == 0)
    {
        if (word[current_index] == 'b') // go to 16 state
        {
            current_index++;
            get_ebay_16();
        }
        else if (word[current_index] == 'w') // go to 12 state
        {
            current_index++;
            get_web_12();
        }
        else if (word[current_index] == 'e') // stay in 15 state
        {
            current_index++;
            get_ebay_15();
        }
        else // go to 1 state
        {
            current_index++;
            start_state();
        }
    }
    else
    {
        std::cout << "Not accepted" << std::endl;
    }
}

void get_ebay_16()
{
    print_state("16");
    accept = false;
    int ctrl_result = accept_control();
    if (ctrl_result == 0)
    {
        if (word[current_index] == 'a') // go to 17 state
        {
            current_index++;
            get_ebay_17();
        }
        else if (word[current_index] == 'w') // go to 12 state
        {
            current_index++;
            get_web_12();
        }
        else if (word[current_index] == 'e') // stay in 15 state
        {
            current_index++;
            get_ebay_15();
        }
        else // go to 1 state
        {
            current_index++;
            start_state();
        }
    }
    else
    {
        std::cout << "Not accepted" << std::endl;
    }
}

void get_ebay_17()
{
    print_state("17");
    accept = false;
    int ctrl_result = accept_control();
    if (ctrl_result == 0)
    {
        if (word[current_index] == 'y') // go to 18 state
        {
            current_index++;
            get_ebay_18();
        }
        else if (word[current_index] == 'w') // go to 12 state
        {
            current_index++;
            get_web_12();
        }
        else if (word[current_index] == 'e') // stay in 15 state
        {
            current_index++;
            get_ebay_15();
        }
        else // go to 1 state
        {
            current_index++;
            start_state();
        }
    }
    else
    {
        std::cout << "Not accepted" << std::endl;
    }
}

void get_ebay_18()
{
    print_state("18");
    accept = true;
    int ctrl_result = accept_control();
    if (ctrl_result == 0)
    {
        if (word[current_index] == 'w') // go to 12 state
        {
            current_index++;
            get_web_12();
        }
        else if (word[current_index] == 'e') // stay in 15 state
        {
            current_index++;
            get_ebay_15();
        }
        else // go to 1 state
        {
            current_index++;
            start_state();
        }
    }
    else if (ctrl_result == 1)
    {
        find = true;
        std::cout << "Accepted" << std::endl;
    }
    else
    {
        std::cout << "Not accepted" << std::endl;
    }
}

void get_web_12()
{
    print_state("12");
    accept = false;
    int ctrl_result = accept_control();
    if (ctrl_result == 0)
    {
        if (word[current_index] == 'e') // go to 15 state
        {
            current_index++;
            get_ebay_15();
        }
        else if (word[current_index] == 'w') // go to 12 state
        {
            current_index++;
            get_web_12();
        }
        else // go to 1 state
        {
            current_index++;
            start_state();
        }
    }
    else
    {
        std::cout << "Not accepted" << std::endl;
    }
}

void get_web_135()
{
    print_state("135");
    accept = false;
    int ctrl_result = accept_control();
    if (ctrl_result == 0)
    {
        if (word[current_index] == 'b') // go to 146 state
        {
            current_index++;
            get_web_146();
        }
        else if (word[current_index] == 'e') // go to 15 state
        {
            current_index++;
            get_ebay_15();
        }
        else if (word[current_index] == 'w') // go to 12 state
        {
            current_index++;
            get_web_12();
        }
        else // go to 1 state
        {
            current_index++;
            start_state();
        }
    }
    else
    {
        std::cout << "Not accepted" << std::endl;
    }
}

void get_web_146()
{
    print_state("146");
    accept = false;
    int ctrl_result = accept_control();
    if (ctrl_result == 0)
    {
        if (word[current_index] == 'a') // go to 17 state
        {
            current_index++;
            get_ebay_17();
        }
        else if (word[current_index] == 'e') // go to 15 state
        {
            current_index++;
            get_ebay_15();
        }
        else if (word[current_index] == 'w') // go to 12 state
        {
            current_index++;
            get_web_12();
        }
        else // go to 1 state
        {
            current_index++;
            start_state();
        }
    }
    else
    {
        std::cout << "Not accepted" << std::endl;
    }
}
