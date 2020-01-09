import numpy as np

class Mancala:

    def __init__(self, start_stones: int=4, pit_num: int=7):
        """Game creation.
        
        Keyword Arguments:
            start_stones {int} -- Number of stones in pits at start. (default: {4})
            pit_num {int} -- Number of pits per side (mancalas included). (default: {7})
        """

        assert start_stones > 0, 'The number of stones has to be greater than 0.'
        assert pit_num > 0, 'The number of pits has to be greater than 0.'

        self.pit_num = pit_num
        # self.max_stones = 2*start_stones*(pit_num-1)
        self.board = self.make_board(start_stones, pit_num)
        self.player = 0


    def make_board(self, start_stones: int, pit_num: int):
        """Creates a Mancala board.
        
        Arguments:
            start_stones {int} -- number of stones that start in each pit.
            pit_num {int} -- number of pits on the board per side (including mancalas).
        
        Returns:
            np.ndarray -- The board: num_pits and one mancala per side.
        """
        board = np.zeros(2*pit_num, dtype=np.int8)
        board[:pit_num-1] = start_stones
        board[pit_num:-1] = start_stones
        return board


    def valid_choice(self, pit: int):
        """Determines whether the chosen pit is a valid choice for the player.
        
        Arguments:
            pit {int} -- The chosen pit.
        
        Returns:
            bool -- True if the pit is a valid move.
        """
        # Calculates what the chosen pit is on the board based on the player.
        board_pit = self.player*self.pit_num + pit

        # Checks that the chosen pit is in the correct range and that it is not empty.
        if (0 <= pit < self.pit_num) and self.board[board_pit] != 0:
            return True
        else:
            return False


    def take_turn(self, pit: int):
        """Takes a turn in the game.
        
        Arguments:
            pit {int} -- The chosen pit to take stones out of.
        """

        assert self.valid_choice(pit), 'The chosen pit is not a valid choice for this player.'
        
        # Calculates what the chosen pit is on the board based on the player.
        board_pit = self.player*self.pit_num + pit
        board_size = 2*self.pit_num

        # Pick up stones.
        stones_in_hand = self.board[board_pit]

        # Calculate board changes.
        board_changes = np.zeros_like(self.board)
        board_changes[board_pit] = -(stones_in_hand+1)

        while stones_in_hand > 0:
            if stones_in_hand + board_pit > board_size:
                # Too many stones; have to loop over.
                board_changes[board_pit:board_size] += 1
                stones_in_hand -= board_size - board_pit
                board_pit = 0

                # Skip enemy mancala
                if self.player == 0:
                    board_changes[-1] -= 1
                    stones_in_hand += 1


            else:
                # Don't have to loop.
                board_changes[board_pit:(board_pit+stones_in_hand+1)] += 1
                stones_in_hand = 0

                # Skip enemy mancala here?
                # Check last pit here?

        # Problems:
        # ~~Does not loop around~~
        # Does not skip enemy mancala
        
        # To detect if last fell in empty, check if last == 1
        

        

if __name__ == "__main__":
    Mancala()