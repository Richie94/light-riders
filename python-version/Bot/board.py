import sys
from typing import List
import time

from math import floor, ceil

from Bot.player import Player

PLAYER1, PLAYER2, EMPTY, BLOCKED = [0, 1, 2, 3]
S_PLAYER1, S_PLAYER2, S_EMPTY, S_BLOCKED, = ['0', '1', '.', 'x']

CHARTABLE = [(PLAYER1, S_PLAYER1), (PLAYER2, S_PLAYER2), (EMPTY, S_EMPTY), (BLOCKED, S_BLOCKED)]

DIRS = [
    ((-1, 0), "up"),
    ((0, 1), "right"),
    ((1, 0), "down"),
    ((0, -1), "left")
]


class Board:

    def __init__(self, width: int, height: int):
        self.width = width
        self.height = height
        self.cell = [[[EMPTY] for col in range(0, width)] for row in range(0, height)]
        self.desired_depth = 5
        self.next_move = None

        self.score_options = {}

        self.adjacents = {}

    def parse_cell(self, players: List[Player], row: int, col: int, data: str):
        if len(data) != 1:
            return -1
        elif data == ".":
            return EMPTY
        elif data == "0":
            players[0].row = row
            players[0].col = col
            return 0
        elif data == "1":
            players[1].row = row
            players[1].col = col
            return 1
        else:
            return BLOCKED

    def parse(self, players: List[Player], data: str):
        cells = data.split(',')
        col = 0
        row = 0
        for cell in cells:
            if col >= self.width:
                col = 0
                row += 1
            self.cell[row][col] = self.parse_cell(players, row, col, cell)
            col += 1

    def in_bounds(self, row: int, col: int) -> bool:
        return 0 <= row < self.height and 0 <= col < self.width

    def is_legal(self, row: int, col: int) -> bool:
        return (self.in_bounds(row, col)) and (EMPTY == self.cell[row][col])

    def get_adjacent(self, row: int, col: int, use_stored=True) -> List:
        if use_stored and (row, col) in self.adjacents:
            return self.adjacents[(row, col)]
        else:
            result = []
            for (o_row, o_col), _ in DIRS:
                t_row, t_col = o_row + row, o_col + col
                if self.is_legal(t_row, t_col):
                    result.append((t_row, t_col))
            self.adjacents[(row, col)] = result
            return result

    def mini_max(self, player_id: int, players: List[Player], depth: int, alpha: int, beta: int) -> int:
        legal_moves = self.legal_moves(player_id, players)
        if depth == 0 or len(legal_moves) == 0:
            return self.get_score(player_id, players)

        max_value = -1000000

        # order legal moves by near to middle
        # legal_moves.sort(key=lambda x: abs(x[0][0] - self.height) + abs(x[0][1] - self.width))
        # vs sort be near to enemy
        enemy = players[(player_id + 1) % 2]
        me = players[player_id]
        legal_moves.sort(key=lambda x: abs(me.row + x[0][0] - enemy.row) + abs(me.col + x[0][1] - enemy.col))

        while len(legal_moves) > 0:
            move = legal_moves.pop(0)
            self.make_move(players[player_id], player_id, move[0])
            value = -self.mini_max((player_id + 1) % 2, players, depth - 1, -beta, -alpha)
            self.make_move(players[player_id], player_id, move[0], reverse=True)

            if value > max_value:
                max_value = value

                if depth == self.desired_depth:
                    self.next_move = move

            if depth == self.desired_depth:
                self.score_options[move[1]] = value

            alpha = max(alpha, value)
            if alpha >= beta:
                break

        return max_value

    def make_move(self, chosen_player: Player, player_id: int, movement: tuple, reverse=False):
        cells_where_adjacency_changes = []
        for (o_row, o_col), _ in DIRS:
            cells_where_adjacency_changes.append((o_row + chosen_player.row, o_col + chosen_player.col))

        if not reverse:
            self.cell[chosen_player.row][chosen_player.col] = BLOCKED
        else:
            self.cell[chosen_player.row][chosen_player.col] = EMPTY
        # modify player object
        if not reverse:
            chosen_player.row += movement[0]
            chosen_player.col += movement[1]
        else:
            chosen_player.row -= movement[0]
            chosen_player.col -= movement[1]
        # modify field
        self.cell[chosen_player.row][chosen_player.col] = player_id

        for (o_row, o_col), _ in DIRS:
            cells_where_adjacency_changes.append((o_row + chosen_player.row, o_col + chosen_player.col))

        # update all cells where adjacency has changed
        for cell in cells_where_adjacency_changes:
            self.get_adjacent(cell[0], cell[1], use_stored=False)

    def get_amount_of_reachable_points_for_player(self, player_id: int, players: List[Player]) -> int:
        now = time.time()
        current_position = players[player_id]
        reachable_points = set()
        newly_added = self.get_adjacent(current_position.row, current_position.col)

        node_edges = []

        while len(newly_added) > 0:
            last_round_added = set(newly_added)
            newly_added = []

            for point in last_round_added:
                reachable_points.add(point)

            for point in last_round_added:
                adjacent = self.get_adjacent(point[0], point[1])
                node_edges.append(floor(len(adjacent)/2)+1)
                for adjacent_point in adjacent:
                    if adjacent_point not in reachable_points:
                        newly_added.append(adjacent_point)

        sys.stderr.write("Took "+str(time.time()-now)+"\n")
        sys.stderr.flush()

        return sum(node_edges)

    def get_score(self, my_player_id: int, players: List[Player]) -> int:
        my_score = self.get_amount_of_reachable_points_for_player(my_player_id, players)
        enemy_score = self.get_amount_of_reachable_points_for_player((my_player_id + 1) % 2, players)
        return my_score - 2*enemy_score

    def legal_moves(self, my_id: int, players: List[Player]) -> List:
        my_player = players[my_id]
        result = []
        for ((o_row, o_col), order) in DIRS:
            t_row = my_player.row + o_row
            t_col = my_player.col + o_col
            if self.is_legal(t_row, t_col):
                result.append(((o_row, o_col), order))
            else:
                pass
        return result

    def output_cell(self, cell):
        done = False
        for (i, symbol) in CHARTABLE:
            if i in cell:
                if not done:
                    sys.stderr.write(symbol)
                done = True
                break
        if not done:
            sys.stderr.write("!")
            done = True

    def output(self):
        for row in self.cell:
            sys.stderr.write("\n")
            for cell in row:
                self.output_cell(cell)
        sys.stderr.write("\n")
        sys.stderr.flush()
