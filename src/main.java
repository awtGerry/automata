import java.awt.event.ActionEvent;
import java.awt.event.ActionListener;
import java.io.BufferedReader;
import java.io.File;
import java.io.FileNotFoundException;
import java.io.FileReader;
import java.io.IOException;
import java.util.Scanner;

// gui
import javax.swing.*;

public class Main {
    // Define the variables
    public static String word = "";
    public int current_index = 0;
    public int ebay_count = 0, web_count = 0;
    public boolean accept, find = false;

    // Constructor
    public Main() {
        gui();
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
            if (word.charAt(current_index) == 'e')
            {
                current_index++;
                get_ebay_15();
            }
            else if (word.charAt(current_index) == 'w')
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
            System.out.println("Not accepted");
        }
    }

    void get_ebay_15()
    {
        print_state("15");
        accept = false;
        int ctrl_result = accept_control();
        if (ctrl_result == 0)
        {
            if (word.charAt(current_index) == 'b') // go to 16 state
            {
                current_index++;
                get_ebay_16();
            }
            else if (word.charAt(current_index) == 'w') // go to 12 state
            {
                current_index++;
                get_web_12();
            }
            else if (word.charAt(current_index) == 'e') // stay in 15 state
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
            System.out.println("Not accepted");
        }
    }

    void get_ebay_16()
    {
        print_state("16");
        accept = false;
        int ctrl_result = accept_control();
        if (ctrl_result == 0)
        {
            if (word.charAt(current_index) == 'a') // go to 17 state
            {
                current_index++;
                get_ebay_17();
            }
            else if (word.charAt(current_index) == 'w') // go to 12 state
            {
                current_index++;
                get_web_12();
            }
            else if (word.charAt(current_index) == 'e') // stay in 15 state
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
            System.out.println("Not accepted");
        }
    }

    void get_ebay_17()
    {
        print_state("17");
        accept = false;
        int ctrl_result = accept_control();
        if (ctrl_result == 0)
        {
            if (word.charAt(current_index) == 'y') // go to 18 state
            {
                current_index++;
                get_ebay_18();
            }
            else if (word.charAt(current_index) == 'w') // go to 12 state
            {
                current_index++;
                get_web_12();
            }
            else if (word.charAt(current_index) == 'e') // stay in 15 state
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
            System.out.println("Not accepted");
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
            if (word.charAt(current_index) == 'w') // go to 12 state
            {
                current_index++;
                get_web_12();
            }
            else if (word.charAt(current_index) == 'e') // stay in 15 state
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
            System.out.println("Accepted");
        }
        else
        {
            System.out.println("Not accepted");
        }
    }

    void get_web_12()
    {
        print_state("12");
        accept = false;
        int ctrl_result = accept_control();
        if (ctrl_result == 0)
        {
            if (word.charAt(current_index) == 'e') // go to 135 state
            {
                current_index++;
                get_web_135();
            }
            else if (word.charAt(current_index) == 'w') // go to 12 state
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
            System.out.println("Not accepted");
        }
    }

    void get_web_135()
    {
        print_state("135");
        accept = false;
        int ctrl_result = accept_control();
        if (ctrl_result == 0)
        {
            if (word.charAt(current_index) == 'b') // go to 146 state
            {
                current_index++;
                get_web_146();
            }
            else if (word.charAt(current_index) == 'e') // go to 15 state
            {
                current_index++;
                get_ebay_15();
            }
            else if (word.charAt(current_index) == 'w') // go to 12 state
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
            System.out.println("Not accepted");
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
            if (word.charAt(current_index) == 'a') // go to 17 state
            {
                current_index++;
                get_ebay_17();
            }
            else if (word.charAt(current_index) == 'e') // go to 15 state
            {
                current_index++;
                get_ebay_15();
            }
            else if (word.charAt(current_index) == 'w') // go to 12 state
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
            System.out.println("Not accepted");
        }
    }

    void print_state(String name) // print state
    {
        System.out.println(word.substring(0, current_index) + " -> " + name);
    }

    int get_ebay_count()
    {
        return ebay_count;
    }

    int get_web_count()
    {
        return web_count;
    }

    void set_word(String w)
    {
        word = w;
    }

    String get_word()
    {
        return word;
    }

    void gui()
    {
        // Create the frame
        JFrame frame = new JFrame("DFA");
        frame.setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
        frame.setSize(800, 600);
        frame.setResizable(false);
        frame.setBackground(java.awt.Color.BLACK);
        frame.setForeground(java.awt.Color.WHITE);

        // Create the panel
        JPanel panel = new JPanel();
        panel.setLayout(null);
        panel.setMaximumSize(frame.getSize());

        // Create the text field
        JTextField text = new JTextField();
        text.setBounds(50, 50, 200, 30);
        text.setBorder(BorderFactory.createLineBorder(java.awt.Color.WHITE));
        panel.add(text);
        text.setText("input:");
        // remove the text when the user clicks on it
        text.addMouseListener(new java.awt.event.MouseAdapter() {
            public void mouseClicked(java.awt.event.MouseEvent e) {
                text.setText("");
            }
        });
        text.addActionListener(new ActionListener() {
            @Override
            public void actionPerformed(ActionEvent e) {
                set_word(text.getText());
            }
        });
        // Or create button to get the text from file using JFileChooser
        JButton file_button = new JButton("Get the text from file");
        file_button.setBounds(300, 50, 200, 30);
        file_button.setBorder(BorderFactory.createLineBorder(java.awt.Color.WHITE));
        file_button.addActionListener(new ActionListener() {
            @Override
            public void actionPerformed(ActionEvent e) {
                JFileChooser file_chooser = new JFileChooser();
                int result = file_chooser.showOpenDialog(null);
                String file_text = "";
                if (result == JFileChooser.APPROVE_OPTION)
                {
                    File file = file_chooser.getSelectedFile();
                    try {
                        Scanner scanner = new Scanner(file);
                        while (scanner.hasNextLine())
                        {
                            file_text += scanner.nextLine();
                        }
                        scanner.close();
                        set_word("");
                    } catch (FileNotFoundException e1) {
                        e1.printStackTrace();
                    }
                }
                set_word(file_text);
            }
        });
        panel.add(file_button);

        // Show the results
        JTextArea results = new JTextArea();
        results.setBounds(50, 150, 700, 300);
        results.setBorder(BorderFactory.createLineBorder(java.awt.Color.WHITE));
        results.setEditable(false);
        // Don't allow the text to go out of the bounds
        results.setLineWrap(true);
        results.setWrapStyleWord(true);
        results.setText("Results:\n");
        panel.add(results);

        // Create the scroll pane
        JScrollPane scroll = new JScrollPane(results);
        scroll.setBounds(50, 150, 700, 300);
        panel.add(scroll);

        // Create the button
        JButton button = new JButton("Check");
        button.setBounds(50, 100, 200, 30);
        button.setBorder(BorderFactory.createLineBorder(java.awt.Color.WHITE));
        button.addActionListener(new ActionListener() {
            @Override
            public void actionPerformed(ActionEvent e) {
                word = word.toLowerCase();
                start_state();
                results.setText(results.getText() + "\n" +
                        "Ebay count: " + get_ebay_count() + "\nWeb count: " + get_web_count() + "\n\n" +
                        "Word: " + get_word() + "\n"
                );
            }
        });
        panel.add(button);

        frame.add(panel);
        frame.setVisible(true);
    }

    public static void main(String[] args) {
        new Main();
    }
}
