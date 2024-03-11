#include <iostream>
#include <string>
#include <fstream>

std::string word = "";
int current_index = 0;
int ebay_count = 0, web_count = 0;
bool accept, find = false;

int accept_control();
void start_state();

void print_state(std::string name) // print state
{
    std::cout << word.substr(0, current_index) << " Currently in: " << name << " state" << std::endl;
}

int get_ebay_count()
{
    return ebay_count;
}

int get_web_count()
{
    return web_count;
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

int main(int argc, char *argv[])
{
    std::ifstream myfile("file.txt");
    if (myfile.is_open())
    {
        std::string line;
        while (getline(myfile, line))
        {
            word += line;
        }
        myfile.close();
    }
    else
    {
        std::cout << "Unable to open file";
        return 1;
    }

    for (int i = 0; i < word.length(); i++)
    {
        word[i] = tolower(word[i]);
    }
    start_state();
    std::cout << "Ebay count: " << get_ebay_count() << std::endl;
    std::cout << "Web count: " << get_web_count() << std::endl;
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
    ebay_count+=1; // if it gets here, it means that it found the word ebay
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
        if (word[current_index] == 'e') // go to 135 state
        {
            current_index++;
            get_web_135();
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
    web_count+=1; // if it reaches this state, it means that it found the word web
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
