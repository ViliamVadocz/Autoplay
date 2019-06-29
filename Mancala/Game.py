#TODO Add comments

# Board creation
start_stones = 4
board = [start_stones]*6 + [0] + [start_stones]*6 + [0]
global turn
turn = 0

def take_turn(pit:int):
    '''Takes a turn in the game Mancala. Expects an integer for the chosen pit to pick stones out of.'''
    # Calculates what the chosen pit is on the board based on the turn.
    board_pit = turn*7 + pit

    # Determines if the pit is valid (on correct side of the board and not empty).
    if pit in range(7) and board[board_pit] != 0:
        print("Picked pit {} with {} stones.".format(board_pit, board[board_pit]))

        # Picks up stones from the pit.
        in_hand = board[board_pit]
        board[board_pit] = 0
        # Moves stones around the board (clockwise).
        for stone in range(in_hand):
            board[(board_pit + 1 + stone) % 14] += 1

        # Last pit is found (needed for special rules)
        last_pit = (board_pit + in_hand) % 14
        # print("The last pit was {}".format(last_pit))

        # Determines if the last stone landed in the Mancala (collection pit) which means an extra turn.
        if last_pit == 6 + turn*7:
            print("-- The last stone landed in the mancala, you get an extra turn!")

        else:
            # Determines if the last stone landed in an empty pit on your own side which means a capture.
            if ( last_pit >= turn*7 and last_pit < 6 + turn*7 ) and  ( board[last_pit+7 % 14] == 0 ):
                print("-- The last stone landed in your own pit opposing an empty one, capture!")
                #TODO Capture

            # Normal turn where no special rules were invoked
            else:
                print("-- Nothing spectacular.")
            
            # Switches turn
            #turn = (turn + 1) % 2

            #TODO Check if no stones left on one side of the board
            #TODO End game and tally points
            #TODO Announce winner

        print_board(board)

    else:
        print("invalid input")

def print_board(board:list):
    '''Prints the board. Expects the boardstate as input.'''
    # Converts the board list into how the board should look so it is recognisable.
    print("Current board state:")
    print(board[:-8:-1])
    print(board[:7])
    print("-"*25)



# DEBUG
print_board(board)

turn = 0
take_turn(2)
take_turn(0)

turn = 1
take_turn(3)