case $1 in
    1)
        echo "Running automata"
        javac -d bin src/main.java
        java -cp bin main
        ;;
    2)
        echo "Running NFA to DFA"
        javac -d bin src/NFAtoDFA.java
        java -cp bin NFAtoDFA
        ;;
    *)
        echo "Invalid option"
        ;;
esac
