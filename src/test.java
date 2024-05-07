import java.util.*;

public class NFAtoDFA {

    public static void main(String[] args) {
        Scanner scanner = new Scanner(System.in);
        
        Map<String, Map<String, Set<String>>> nfa = new HashMap<>();
        System.out.print("Enter the number of states: ");
        int n = scanner.nextInt();
        scanner.nextLine(); // Consume the newline
        
        System.out.print("Enter the number of transitions: ");
        int t = scanner.nextInt();
        scanner.nextLine(); // Consume the newline
        
        for (int i = 0; i < n; i++) {
            System.out.print("Enter the state name: ");
            String state = scanner.nextLine();
            nfa.put(state, new HashMap<>());
            for (int j = 0; j < t; j++) {
                System.out.print("Enter the path: ");
                String path = scanner.nextLine();
                System.out.printf("Enter the end state from state %s traveling through path %s: ", state, path);
                Set<String> reachingStates = new HashSet<>(Arrays.asList(scanner.nextLine().split(" ")));
                nfa.get(state).put(path, reachingStates);
            }
        }
        
        
        System.out.println("\nPrinting NFA");
        for (String state : nfa.keySet()) {
            System.out.println(state + " : " + nfa.get(state));
        }
        
        System.out.print("\nAccepting states of NFA: ");
        Set<String> nfaFinalState = new HashSet<>(Arrays.asList(scanner.next().split(" ")));

        
        List<String> newStatesList = new ArrayList<>();
        Map<String, Map<String, String>> dfa = new HashMap<>();
        String initialState = nfa.keySet().iterator().next();
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
                for (String path : nfa.get(state).keySet()) {
                    Set<String> temp = new HashSet<>();
                    for (int j = 0; j < newState.length(); j++) {
                        temp.addAll(nfa.get(newState.substring(j, j + 1)).get(path));
                    }
                    String s = String.join("", temp);
                    if (!dfa.containsKey(s)) {
                        newStatesList.add(s);
                    }
                    dfa.get(newState).put(path, s);
                }
            }
        }
        
        System.out.println("\nPrinting DFA");
        for (String state : dfa.keySet()) {
            System.out.println(state + " : " + dfa.get(state));
        }
        
        Set<String> dfaStatesList = dfa.keySet();
        Set<String> dfaFinalStates = new HashSet<>();
        for (String x : dfaStatesList) {
            for (int i = 0; i < x.length(); i++) {
                if (nfaFinalState.contains(x.substring(i, i + 1))) {
                    dfaFinalStates.add(x);
                    break;
                }
            }
        }
        
        System.out.println("\nFinal states of the DFA are: " + dfaFinalStates);
        
        scanner.close();
    }
}
