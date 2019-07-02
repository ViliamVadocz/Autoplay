class Mancala:

    def __init__(self):
        '''Game creation.'''
        start_stones = 4
        self.board = [start_stones]*6 + [0] + [start_stones]*6 + [0]
        self.player = 0
        self.is_active = True
        self.result = None

    def valid_choice(self, pit: int):
        '''Determines if the pit is valid (on correct side of the board and not empty).'''

        # Calculates what the chosen pit is on the board based on the player.
        board_pit = self.player*7 + pit

        if pit in range(6) and self.board[board_pit] != 0:
            return True
        else:
            return False

    def take_turn(self, pit: int):
        '''Takes a turn in the game Mancala. Expects an integer for the chosen pit to pick stones out of.'''

        # Calculates what the chosen pit is on the board based on the player.
        board_pit = self.player*7 + pit

        # Picks up stones from the pit.
        stones_in_hand = self.board[board_pit]
        self.board[board_pit] = 0

        # Moves stones around the board (anti-clockwise).
        steps = 0
        while stones_in_hand > 0:
            # Increments steps.
            steps += 1
            # Checks that the the pit is not the opponent's mancala (which is skipped when dropping stones).
            if (board_pit + steps) % 14 != (13 + self.player*7) % 14:
                # Drops a stone into the pit.
                self.board[(board_pit + steps) % 14] += 1
                stones_in_hand -= 1

        # Last pit is found (needed for special rules).
        last_pit = (board_pit + steps) % 14

        # Determines if the last stone has not landed in the Mancala (collection pit) which would mean an extra turn.
        if not last_pit == 6 + self.player*7:
            # Determines if the last stone landed in an empty pit on your own side which means a capture. (Exception when opposing pit is empty).
            if (last_pit >= self.player*7) and (last_pit < 6 + self.player*7) and (self.board[last_pit] == 1) and (self.board[12 - last_pit] != 0):
                self.board[6 + self.player*7] += 1 + self.board[12 - last_pit]
                self.board[last_pit] = 0
                self.board[12 - last_pit] = 0

            # Switches player.
            self.player = (self.player + 1) % 2

        # Checks if there are no stones on either side of the board.
        if self.board[:6] == [0]*6 or self.board[7:13] == [0]*6:
            # Ends game.
            self.is_active = False

            # Tallies up stones.
            self.board[6] += sum(self.board[:6])
            self.board[13] += sum(self.board[7:13])

            # Sets result.
            self.result = (self.board[6], self.board[13])
