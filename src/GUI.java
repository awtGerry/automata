import java.awt.*;
import java.awt.event.ActionEvent;
import java.io.File;
import java.util.*;

import javax.swing.*;

public class GUI {

    // NFA to DFA Variables
    private int n; // number of states
    private int t; // number of transitions
    private String[] path; // path
    private Map<String, Map<String, Set<String>>> nfa = new HashMap<>();
    private Set<String> nfaFinalState = new HashSet<>();
    private java.util.List<String> newStatesList = new ArrayList<>();
    private Map<String, Map<String, String>> dfa = new HashMap<>();
    private String initialState;
    private Set<String> dfaStatesList;
    private Set<String> dfaFinalStates = new HashSet<>();

    // GUI Variables
    private JFrame frame = new JFrame();
    private JTextField field_states = new JTextField();
    private JLabel txt_states = new JLabel();
    private JTextField field_transitions = new JTextField();
    private JLabel txt_trasitions = new JLabel();
    private JLabel txt_path = new JLabel();
    private JTextField field_path = new JTextField();
    private JLabel txt_accepting_states = new JLabel();
    private JTextField field_accepting_states = new JTextField();
    private JButton btn_add_vars = new JButton();
    private JButton btn_add_state = new JButton();
    private JTextField field_state_path = new JTextField();
    private JTextField field_state1 = new JTextField();
    private JTextField field_state2 = new JTextField();
    private JLabel txt_nfa = new JLabel();
    private JScrollPane jScrollPane1 = new JScrollPane();
    private JTextArea text_area_nfa = new JTextArea();
    private JLabel txt_dfa = new JLabel();
    private JScrollPane jScrollPane2 = new JScrollPane();
    private JTextArea text_area_dfa = new JTextArea();
    private JButton btn_convert = new JButton();

    public GUI() { // inicializar gui
        txt_states = custom_txt(txt_states, "Enter number of states:");
        field_states.setPreferredSize(new Dimension(200, 30));
        txt_trasitions = custom_txt(txt_trasitions, "Enter number of transitions:");
        field_transitions.setPreferredSize(new Dimension(200, 30));
        txt_path = custom_txt(txt_path, "Enter path (ej: 0 1):");
        field_path.setPreferredSize(new Dimension(200, 30));
        txt_accepting_states = custom_txt(txt_accepting_states, "Enter accepting states:");
        field_accepting_states.setPreferredSize(new Dimension(200, 30));

        // Add default values
        btn_add_vars = custom_button(btn_add_vars, "Add variables");
        btn_add_vars.addActionListener((ActionEvent e) -> {
            add_vars_pressed();
        });

        // STATES
        field_state_path.setPreferredSize(new Dimension(200, 30));
        field_state_path.setText("Enter state name");
        // remove text when focused
        field_state_path.addFocusListener(new java.awt.event.FocusAdapter() {
            public void focusGained(java.awt.event.FocusEvent evt) {
                field_state_path.setText("");
            }
        });
        field_state1.setPreferredSize(new Dimension(200, 30));
        field_state1.setText("Enter state 1");
        field_state1.addFocusListener(new java.awt.event.FocusAdapter() {
            public void focusGained(java.awt.event.FocusEvent evt) {
                field_state1.setText("");
            }
        });
        field_state2.setPreferredSize(new Dimension(200, 30));
        field_state2.setText("Enter state 2");
        field_state2.addFocusListener(new java.awt.event.FocusAdapter() {
            public void focusGained(java.awt.event.FocusEvent evt) {
                field_state2.setText("");
            }
        });
        btn_add_state = custom_button(btn_add_state, "Add state");
        btn_add_state.addActionListener((ActionEvent e) -> {
            add_state_pressed();
        });

        // NFA
        txt_nfa = custom_txt(txt_nfa, "NFA:");
        text_area_nfa.setEditable(false);
        jScrollPane1.setViewportView(text_area_nfa);
        jScrollPane1.setPreferredSize(new Dimension(500, 300));

        // DFA
        txt_dfa = custom_txt(txt_dfa, "DFA:");
        text_area_dfa.setEditable(false);
        jScrollPane2.setViewportView(text_area_dfa);
        jScrollPane2.setPreferredSize(new Dimension(500, 300));

        // CONVERT
        btn_convert = custom_button(btn_convert, "Convert");
        btn_convert.addActionListener((ActionEvent e) -> {
            convert_pressed();
        });

        // FRAME & GROUP LAYOUT
        frame.setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
        GroupLayout ly = new GroupLayout(frame.getContentPane());
        frame.getContentPane().setLayout(ly);
        frame.setTitle("NFA to DFA");
        frame.setLocationRelativeTo(null);
        frame.setSize(1280, 720);
        frame.setResizable(false);

        gui_groups(ly); // add styles.

        frame.setVisible(true);
        frame.pack();
    }

    /* BUTTONS */
    private void add_vars_pressed() {
        n = Integer.parseInt(field_states.getText());
        t = Integer.parseInt(field_transitions.getText());
        nfaFinalState = new HashSet<>(Arrays.asList(field_accepting_states.getText().split(" ")));
        path = field_path.getText().split(" ");
        field_states.setEditable(false);
    }

    private void add_state_pressed() {
        String state = field_state_path.getText();
        nfa.put(state, new HashMap<>());
        String[] path0 = field_state1.getText().split(" ");
        String[] path1 = field_state2.getText().split(" ");
        nfa.get(state).put(path[0], new HashSet<>(Arrays.asList(path0)));
        nfa.get(state).put(path[1], new HashSet<>(Arrays.asList(path1)));

        System.out.println(nfa);
        field_state_path.setText("Enter state name");
        field_state1.setText("Enter state 1");
        field_state2.setText("Enter state 2");
    }

private void convert_pressed() {
    for (String state : nfa.keySet()) {
        text_area_nfa.append(state + " : " + nfa.get(state) + "\n");
    }

    newStatesList = new ArrayList<>();
    dfa = new HashMap<>();
    initialState = nfa.keySet().iterator().next();
    dfa.put(initialState, new HashMap<>());

    for (String path : nfa.get(initialState).keySet()) {
        String var = String.join("", nfa.get(initialState).get(path));
        dfa.get(initialState).put(path, var);
        if (!newStatesList.contains(var)) {
            newStatesList.add(var);
        }
    }

    while (!newStatesList.isEmpty()) {
        String newState = newStatesList.remove(0);
        dfa.put(newState, new HashMap<>());
        for (int i = 0; i < newState.length(); i++) {
            String state = newState.substring(i, i + 1);
            if (nfa.containsKey(state)) {
                for (String path : nfa.get(state).keySet()) {
                    Set<String> temp = new HashSet<>();
                    for (int j = 0; j < newState.length(); j++) {
                        temp.addAll(nfa.get(newState.substring(j, j + 1)).get(path));
                    }
                    String s = String.join("", temp);
                    if (!dfa.containsKey(s)) {
                        newStatesList.add(s);
                        dfa.put(s, new HashMap<>()); // Add new state to DFA
                    }
                    dfa.get(newState).put(path, s);
                }
            }
        }
    }

    for (String state : dfa.keySet()) {
        text_area_dfa.append(state + " : " + dfa.get(state) + "\n");
    }

    // Find final states
    dfaStatesList = dfa.keySet();
    for (String x : dfaStatesList) {
        for (int i = 0; i < x.length(); i++) {
            if (nfaFinalState.contains(x.substring(i, i + 1))) {
                dfaFinalStates.add(x);
                break;
            }
        }
    }

    text_area_dfa.append("\nFinal states of the DFA are: " + dfaFinalStates);
}

    /* GUI METHODS */
    private JButton custom_button(JButton button, String txt) {
        button.setText(txt);
        button.setPreferredSize(new Dimension(90, 23));
        button.setForeground(new Color(219, 226, 239));
        button.setBackground(new Color(50, 130, 184));
        button.setOpaque(true);
        button.setBorderPainted(false);
        button.setFont(new Font("Calibri", Font.PLAIN, 14));
        button.setCursor(Cursor.getPredefinedCursor(Cursor.HAND_CURSOR));
        button.setPreferredSize(new Dimension(90, 23));

        return button;
    }

    private JLabel custom_txt(JLabel label, String txt) {
        label.setText(txt);
        label.setFont(new Font("Calibri", Font.PLAIN, 14));
        label.setForeground(Color.DARK_GRAY);
        label.setOpaque(true);
        return label;
    }

private void gui_groups(GroupLayout ly) {
    // Horizontal group
    ly.setHorizontalGroup(
        ly.createParallelGroup(GroupLayout.Alignment.LEADING)
            .addGroup(ly.createSequentialGroup()
                .addContainerGap()
                .addGroup(ly.createParallelGroup(javax.swing.GroupLayout.Alignment.LEADING)
                    // Variables
                    .addGroup(ly.createSequentialGroup()
                        .addComponent(txt_states)
                        .addGap(15)
                        .addComponent(field_states)
                        .addGap(15)
                        .addComponent(txt_trasitions)
                        .addGap(15)
                        .addComponent(field_transitions)
                        .addGap(15)
                        .addComponent(txt_path)
                        .addGap(15)
                        .addComponent(field_path)
                        .addGap(15)
                        .addComponent(txt_accepting_states)
                        .addGap(15)
                        .addComponent(field_accepting_states)
                    )
                    .addGroup(ly.createSequentialGroup()
                        .addComponent(btn_add_vars)
                        .addGap(113)
                    )

                    // States
                    .addGroup(ly.createSequentialGroup()
                        .addComponent(field_state_path)
                        .addGap(15)
                        .addComponent(field_state1)
                        .addGap(15)
                        .addComponent(field_state2)
                        .addGap(15)
                        .addComponent(btn_add_state)
                    )

                    // NFA
                    .addGroup(ly.createSequentialGroup()
                        .addComponent(txt_nfa)
                        .addGap(15)
                        .addComponent(jScrollPane1)
                    )

                    // DFA
                    .addGroup(ly.createSequentialGroup()
                        .addComponent(txt_dfa)
                        .addGap(15)
                        .addComponent(jScrollPane2)
                    )

                    // Convert
                    .addComponent(btn_convert)
                )
                .addContainerGap()
            )
    );

    // Vertical group
    ly.setVerticalGroup(
        ly.createSequentialGroup()
            .addContainerGap()
            // Variables
            .addGroup(ly.createParallelGroup(GroupLayout.Alignment.BASELINE)
                .addComponent(txt_states)
                .addComponent(field_states)
                .addComponent(txt_trasitions)
                .addComponent(field_transitions)
                .addComponent(txt_path)
                .addComponent(field_path)
                .addComponent(txt_accepting_states)
                .addComponent(field_accepting_states)
            )
            .addGap(15)
            .addComponent(btn_add_vars)
            .addGap(15)

            // States
            .addGroup(ly.createParallelGroup(GroupLayout.Alignment.LEADING)
                .addComponent(field_state1)
                .addComponent(field_state_path)
                .addComponent(field_state2)
                .addComponent(btn_add_state)
            )
            .addGap(15)

            // NFA
            .addGroup(ly.createParallelGroup(GroupLayout.Alignment.BASELINE)
                .addComponent(txt_nfa)
                .addComponent(jScrollPane1)
            )
            .addGap(15)

            // DFA
            .addGroup(ly.createParallelGroup(GroupLayout.Alignment.BASELINE)
                .addComponent(txt_dfa)
                .addComponent(jScrollPane2)
            )
            .addGap(15)

            // Convert
            .addComponent(btn_convert)
            .addContainerGap()
    );
}

    public static void main(String[] args) {
        new GUI();
    }

}
