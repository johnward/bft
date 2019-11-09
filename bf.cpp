#include <iostream>
#include <vector>
#include <stack>
#include <fstream>

#define STACK_INITIAL_SIZE 300
#define DEAD_CODE_LIMIT 100
typedef unsigned char byte;

enum class State {
    READING,
    WHILE_IGNORING
};

int main(int argc, char** argv) {

    if ( argc < 2 ) {
        std::cerr << "Usage: " << argv[0] << " <file_name>" << std::endl;
        exit(1);
    }

    /** Get the source stream. */
    std::ifstream source(argv[1]);

    if ( ! source ) {
        std::cerr << "Invalid file: " << argv[1] << std::endl;
        exit(1);
    }

    /** Allocate the program stack. */
    size_t stack_size = STACK_INITIAL_SIZE;
    byte* stack = new byte[stack_size];

    /** Ensure it's initialized to zero */
    std::fill(stack, stack + stack_size, 0);

    /** Initial offset: pointing to the beginning of the stack */
    size_t offset = 0;

    /** Store the tokens, to allow looping */
    std::vector<char> tokens;
    size_t current_token_index = 0;

    /** Store the loop entry points */
    std::stack<size_t> entry_points;

    /** Store the states, to allow multiple looping */
    std::stack<State> states;

    /** Start reading */
    states.push(State::READING);

    while ( true ) {
        /** Declare the token */
        char token;

        /** Get the current state */
        State state = states.top();

        /** Realloc if we have not enough space, allocate 2 * stack_size */
        if ( offset == stack_size ) {
            size_t new_size = 2 * stack_size;

            /** Allocate space */
            byte* tmp = new byte[new_size];

            /** Copy old data */
            std::copy(stack, stack + stack_size, tmp);

            /** Set to 0 new data */
            std::fill(tmp + stack_size, tmp + new_size, 0);

            /** Delete old space */
            delete[] stack;

            /** Set the new stack data */
            stack = tmp;

            /** Keep track of the new stack size */
            stack_size = new_size;
        }

        /** If we are reading from `tokens` and reached the end, read next token from the file and push it into `tokens` */
        if ( current_token_index == tokens.size() ) {
            if ( (source >> token) )
                tokens.push_back(token);
            else
                break; /** Exit if the program ended */
        } else {
            token = tokens[current_token_index];
        }

        /** If we are ignoring chars... Just process '[' (add to the state stack) and ']' (remove from the state stack) */
        if ( state == State::WHILE_IGNORING && ! (token == ']' || token == '[') ) {
            current_token_index++;
            continue;
        }

        /** Main processing */
        switch ( token ) {
            case '>':
                offset++;
                break;
            case '<':
                offset--;
                break;
            case '+':
                stack[offset]++;
                break;
            case '-':
                stack[offset]--;
                break;
            /**
             * I know these could be written as
             * std::cout << static_cast<char>(stack[offset]);
             * and
             * std::cin >> static_cast<char>(stack[offset]);
             * but i find this way more readable
             */
            case '.':
                putchar(stack[offset]);
                fflush(stdout);
                break;
            case ',':
                stack[offset] = getchar();
                fflush(stdin);
                break;
            case '[':
                /** Add the current token to the stack, to come back later */
                entry_points.push(current_token_index);

                /** If the condition is false, or we're already ignoring, just ignore */
                if ( state == State::WHILE_IGNORING || ! stack[offset] )
                    states.push(State::WHILE_IGNORING);
                break;
            case ']':
                /** If we're ignoring just remove the last state */
                if ( state == State::WHILE_IGNORING )
                    states.pop();
                /** Else go back to the loop */
                else
                    current_token_index = entry_points.top() - 1;

                /** Remove the last entry_point */
                entry_points.pop();
                break;
            default:
                break; // ignore comments
        }

        /** Go to the next token */
        current_token_index++;

        /** Dead code elimination */
        if ( current_token_index > DEAD_CODE_LIMIT && entry_points.empty() ) {
            tokens.clear();
            current_token_index = 0;
        }
    }

    /** Program terminated, delete the stack data */
    delete[] stack;

    return 0;
}