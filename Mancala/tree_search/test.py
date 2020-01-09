import numpy as np

pit_num = 7
board_size = pit_num * 2
board = np.zeros(board_size)

player = 1
board_pit = 5
stones_in_hand = 11

# Calculate board changes.
board_changes = np.zeros_like(board)
board_changes[board_pit] = -(stones_in_hand+1)

while stones_in_hand > 0:
    if stones_in_hand + board_pit > board_size:
        # Too many stones; have to loop over.
        board_changes[board_pit:board_size] += 1
        stones_in_hand -= board_size - board_pit

        # Skip enemy mancala.
        if player == 0:
            board_changes[-1] -= 1
            stones_in_hand += 1

        elif board_pit < pit_num and player == 1:
            board_changes[pit_num-1] -= 1
            stones_in_hand += 1

        board_pit = 0

    else:
        # Don't have to loop.
        board_changes[board_pit:(board_pit+stones_in_hand+1)] += 1

        # Skip enemy mancala.
        if player == 1 and stones_in_hand > pit_num:
            board_changes[pit_num-1] -= 1
            board_changes[pit_num] += 1

        stones_in_hand = 0

        # Check last pit here?

print(board_changes)