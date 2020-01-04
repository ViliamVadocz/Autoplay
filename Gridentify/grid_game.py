import numpy as np

class Gridentify:

    def __init__(self, board = None):
        self.score = 0
        # Generate new board if not supplied with one.
        self.board = self.new_board(5) if board is None else  board

    @staticmethod
    def new_board(x): return np.random.randint(1, 4, (x**2,), dtype=np.uint16)

    @staticmethod
    def new_num(): return np.random.randint(1, 4, dtype=np.uint16)

    def get_neighboursOf(self):
        neighboursOf = []
        for i, value in enumerate(self.board):
            neighbours = []
            x = i % 5
            y = i // 5
            if x < 4 and self.board[i+1] == value: neighbours.append(i+1)
            if y < 4 and self.board[i+5] == value: neighbours.append(i+5)
            if x > 0 and self.board[i-1] == value: neighbours.append(i-1)
            if y > 0 and self.board[i-5] == value: neighbours.append(i-5)
            neighboursOf.append(neighbours)
        return neighboursOf

    def valid_moves(self):

        # TODO:
        # - for each index, look for all possible moves?
        pass

    def move(self, final: int, used: int):
        """Makes a move in the game.
        
        Arguments:
            final {int} -- index of final position of move.
            used {int} -- 25 bit int showing which indexes were 
                used in the move and need to be reset.
        """
        for i, char in enumerate(bin(used)[2:]):
            if char == '1':
                self.board[final] += self.board[i]
                self.board[i] = self.new_num()

        self.score += self.board[i]

    def show_board(self):
        print('\n')
        for arr in np.split(self.board, 5):
            print(arr)


########
def get_legal_moves():
    moves = []
    def discover_for(move, cell: int):
        for next_cell in neighbours_of[cell]:
            next_move = move + cell
            if (move != next_move and moves.add(next_move)):
                discover_for(next_move, next_cell)

    for i in board:
        value = board[i]
        discover_for([i], i)

    return moves


public HashSet<Move> GetLegalMoves()
    var moves = new HashSet<Move>();
    void DiscoverFor(Move move, int cell)
        foreach (var nextCell in NeighborsOf[cell])
            if (Grid[nextCell] == value)
                var nextMove = move | nextCell;
                if (move != nextMove && moves.Add(nextMove))
                    DiscoverFor(nextMove, nextCell);

    for (int i = 0; i < 25; ++i)
        value = Grid[i];
        DiscoverFor(new Move(i), i);

    return moves;