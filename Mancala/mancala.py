class Mancala:
    def __init__(self):
        '''Game creation.'''
        start_stones = 4
        self.board = [start_stones]*6 + [0] + [start_stones]*6 + [0]
        self.player = 0
        self.is_active = True

    def valid_choice(self, pit:int):
        '''Determines if the pit is valid (on correct side of the board and not empty).'''

        # Calculates what the chosen pit is on the board based on the player.
        board_pit = self.player*7 + pit

        if pit in range(7) and self.board[board_pit] != 0:
            return True

        else:
            # Error messages
            if pit not in range(7):
                print("Invalid Input. Pit out of range.")
            elif self.board[board_pit] == 0:
                print("Chosen pit is empty.")
            else:
                print("Invalid input.")

            return False


    def take_turn(self, pit:int):
        '''Takes a turn in the game Mancala. Expects an integer for the chosen pit to pick stones out of.'''
        print("Player: {}".format(self.player))

        # Calculates what the chosen pit is on the board based on the player.
        board_pit = self.player*7 + pit

        print("Picked pit {} with {} stones.".format(board_pit, self.board[board_pit]))

        # Picks up stones from the pit.
        in_hand = self.board[board_pit]
        self.board[board_pit] = 0
        # Moves stones around the board (clockwise).
        for stone in range(in_hand):
            self.board[(board_pit + 1 + stone) % 14] += 1

        # Last pit is found (needed for special rules).
        last_pit = (board_pit + in_hand) % 14
        ### print("The last pit was {}".format(last_pit))

        # Determines if the last stone landed in the Mancala (collection pit) which means an extra turn.
        if last_pit == 6 + self.player*7:
            print("-> The last stone landed in the mancala, you get an extra turn!")

        else:
            # Determines if the last stone landed in an empty pit on your own side which means a capture.
            if ( last_pit >= self.player*7 and last_pit < 6 + self.player*7 ) and  ( self.board[last_pit] == 1 ):
                print("-> The last stone landed in your own pit opposing an empty one, capture!")
                self.board[6 + self.player*7] += 1 + self.board[12 - last_pit]
                self.board[last_pit] = 0
                self.board[12 - last_pit] = 0

            # Normal turn where no special rules were invoked
            else:
                print("-> Nothing spectacular. Just a regular turn.")
            
            # Switches player
            self.player = (self.player + 1) % 2

        self.print_board(self.board)

        # Checks if there are no stones on either side of the board
        if self.board[:6] == [0]*6 or self.board[7:13] == [0]*6:
            print("-> No stones on one side of the board! Game end!")

            # Ends game
            self.is_active = False

            # Tallies up stones
            self.board[6] += sum(self.board[:6])
            self.board[13] += sum(self.board[7:13])

            # Announces winner
            if self.board[6] > self.board[13]:
                print("Player 1 wins! Score was {} : {}".format(self.board[6], self.board[13]))
            elif self.board[6] < self.board[13]:
                print("Player 2 wins! Score was {} : {}".format(self.board[6], self.board[13]))
            else:
                print("It's a draw! Score was {} : {}".format(self.board[6], self.board[13]))
        

    @staticmethod
    def print_board(boardstate:list):
        '''Prints the board. Expects the boardstate as input.'''
        # Converts the board list into how the board should look so it is recognisable.
        print("-"*25)
        print("Current boardstate:")
        print(boardstate[:-8:-1])
        print("  ", boardstate[:7])
        print("-"*25)



# DEBUG
'''
game = Mancala
print_board(game.board)

while game.is_active:
    print("Player: {}".format(game.player))
    user_input = input()

    if user_input in "exit quit esc end":
        break
    else:
        try:
            int(user_input)
        except:
            print("Invalid input.")
        else: 
            game.take_turn(int(user_input))
'''