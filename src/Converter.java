/*

Realizar un sistema que reciba como entrada una Tabla de Transición de un Autómata
Finito  No  Determinista y  Genere  La  Tabla  de  Transición a  un  Autómata  Finito
Determinista Equivalente

El sistema debe de solicitar al usuario que ingrese la tabla de transición del AFND y una vez
ingresada, el programa deberá de generar la tabla de transición del  AFD. Es importante
considerar que el usuario debe de ingresar los siguientes datos:
* Cantidad de estados del AFND 
* Cantidad de entradas del AFND (El alfabeto) 
* Cual es el estado inicial
* Cuáles son estados de Aceptación
* Las transiciones para cada estado

Por ejemplo, considera la siguiente tabla de transiciones, el sistema debe solicitar los datos
mencionados

|  | 0 | 1 |
| ->p | p | p,q |
| q | r | - |
| r | s | - |
| s | t | - |
| *t | - | - |

Una vez que el usuario ingrese los datos correspondientes, el sistema deberá de generar y
presentar al usuario la Tabla de Transición del AFD equivalente,

|  | 0 | 1 |
| ->p | p | p,q |
| p,q | p,r | p,q |
| p,r | p,s | p,q |
| p,s | p,t | p,q |
| *p,t | p | p,q |

*/

import java.util.Scanner;
import java.util.concurrent.atomic.AtomicReference;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.HashSet;
import java.util.LinkedList;
import java.util.Queue;

class TransitionTable {
    public HashMap<String, HashMap<String, ArrayList<String>>> table;
    public ArrayList<String> states;
    public ArrayList<String> alphabet;
    public String initialState;
    public ArrayList<String> finalStates;

    public TransitionTable() {
        table = new HashMap<>();
        states = new ArrayList<>();
        alphabet = new ArrayList<>();
        finalStates = new ArrayList<>();
    }

    public void setStates(String[] states) {
        for (String state : states) {
            this.states.add(state);
        }
    }

    public void setAlphabet(int n) {
        for (int i = 0; i < n; i++) {
            alphabet.add(String.valueOf(i));
        }
    }

    public void setInitialState(String initialState) {
        this.initialState = initialState;
    }

    public void setFinalStates(String[] finalStates) {
        for (String state : finalStates) {
            this.finalStates.add(state);
        }
    }

    public void setTransition(String state, String symbol, String[] transitions) {
        HashMap<String, ArrayList<String>> stateTransitions = table.getOrDefault(state, new HashMap<>());
        ArrayList<String> transitionStates = new ArrayList<>();
        for (String transition : transitions) {
            transitionStates.add(transition);
        }
        stateTransitions.put(symbol, transitionStates);
        table.put(state, stateTransitions);
    }

    public void printTable() {
        System.out.print("  | ");
        for (String symbol : alphabet) {
            System.out.print(symbol + " | ");
        }
        System.out.print("\n");
        for (String state : states) {
            System.out.print(state + " | ");
            for (String symbol : alphabet) {
                ArrayList<String> transitions = table.get(state).get(symbol);
                for (String transition : transitions) {
                    System.out.print(transition + " ");
                }
                System.out.print("| ");
            }
            System.out.print("\n");
        }
    }

    public void nfaToDfa() {
      // 1. Generate the initial state of the DFA
      ArrayList<String> initialState = new ArrayList<>();
      initialState.add(this.initialState);

      // 2. Create a queue to process states
      Queue<ArrayList<String>> statesToProcess = new LinkedList<>();
      statesToProcess.add(initialState);

      // 3. Initialize the DFA transition table
      TransitionTable dfaTable = new TransitionTable();

      // 4. Process states until the queue is empty
      while (!statesToProcess.isEmpty()) {
        ArrayList<String> currentNfaState = statesToProcess.poll();

        System.out.println("\nProcessing NFA state: " + String.join(",", currentNfaState));

        // 4.1 Add current state to DFA states
        dfaTable.states.add(String.join(",", currentNfaState));

        // 4.2 Process transitions for each symbol
        for (String symbol : this.alphabet) {
          ArrayList<String> reachableNfaStates = new ArrayList<>();
          System.out.println("Processing symbol: " + symbol);

          // 4.2.1 Find reachable NFA states for the current symbol
          for (String nfaState : currentNfaState) {
            ArrayList<String> transitions = this.table.get(nfaState).get(symbol);
            if (transitions != null) {
              reachableNfaStates.addAll(transitions);
            }
          }

          // 4.2.2 Convert reachable NFA states to unique DFA state
          ArrayList<String> dfaState = new ArrayList<>(new HashSet<>(reachableNfaStates));
          String dfaStateName = String.join(",", dfaState);
          System.out.println("Reachable NFA states: " + String.join(",", reachableNfaStates));

          // 4.2.3 Add transition to DFA table
          dfaTable.setTransition(dfaTable.states.get(dfaTable.states.size() - 1), symbol, new String[]{dfaStateName});

          // 4.2.4 Add new DFA state to processing queue if not already processed
          if (!dfaTable.states.contains(dfaStateName) && !statesToProcess.contains(dfaState)) {
            statesToProcess.add(dfaState);
            System.out.println("Adding state to process: " + dfaStateName);
          }
        }
      }

      // 5. Determine final states of the DFA
      ArrayList<String> dfaFinalStates = new ArrayList<>();
      for (String dfaState : dfaTable.states) {
        for (String finalState : this.finalStates) {
          if (dfaState.contains(finalState)) {
            dfaFinalStates.add(dfaState);
            break;
          }
        }
      }
      dfaTable.setFinalStates(dfaFinalStates.toArray(new String[0]));

      // 6. Print the DFA transition table
      System.out.println("\nDFA Transition Table:");
      dfaTable.printTable();
    }

}

public class Converter {
    public static void main(String[] args) {
        Scanner scanner = new Scanner(System.in);
        TransitionTable transitionTable = new TransitionTable();

        // For now i'll just use the example table
        // System.out.print("Enter the number of states: ");
        // int n = scanner.nextInt();
        String[] states = {"p", "q", "r", "s", "t"};
        transitionTable.setStates(states);

        System.out.print("Enter the number of inputs: ");
        // int m = scanner.nextInt();
        System.out.print("\n");
        int m = 2;
        transitionTable.setAlphabet(m);

        System.out.print("Enter the initial state: ");
        // String initialState = scanner.next();
        System.out.print("\n");
        String initialState = "p";
        transitionTable.setInitialState(initialState);

        System.out.print("Enter the final states: ");
        // String[] finalStates = scanner.next().split(",");
        System.out.print("\n");
        String[] finalStates = {"t"};
        transitionTable.setFinalStates(finalStates);

        transitionTable.setTransition("p", "0", new String[]{"p"});
        transitionTable.setTransition("p", "1", new String[]{"p", "q"});
        transitionTable.setTransition("q", "0", new String[]{"r"});
        transitionTable.setTransition("q", "1", new String[]{"-"});
        transitionTable.setTransition("r", "0", new String[]{"s"});
        transitionTable.setTransition("r", "1", new String[]{"-"});
        transitionTable.setTransition("s", "0", new String[]{"t"});
        transitionTable.setTransition("s", "1", new String[]{"-"});
        transitionTable.setTransition("t", "0", new String[]{"-"});
        transitionTable.setTransition("t", "1", new String[]{"-"});

        transitionTable.printTable();

        // TODO: implement convertion to DFA
        transitionTable.nfaToDfa();

        scanner.close();
    }
}
