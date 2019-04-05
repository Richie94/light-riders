import random
import sys
import time

from Bot.game import Game


class Bot:

    game: Game

    def __init__(self, random):
        self.game = None
        self.random = random
        self.steps = 0
        self.steps_to_enemy_on_start = 5
        self.start_mode = True

        self.max_depth = 20
        self.min_depth = 5

    def setup(self, game: Game):
        self.game = game

    def do_turn(self):
        now = time.time()
        self.steps += 1
        legal = self.game.field.legal_moves(self.game.my_botid, self.game.players)
        self.game.field.score_options = {}
        random_chosen = None
        if len(legal) == 0:
            self.print_to_err("pass")
            self.game.issue_order_pass()
            return
        else:
            (_, random_chosen) = random.choice(legal)

        enemy = self.game.players[(self.game.my_botid + 1) % 2]
        me = self.game.players[self.game.my_botid]

        if self.random:
            legal.sort(key=lambda x: abs(me.row + x[0][0] - enemy.row) + abs(me.col + x[0][1] - enemy.col))
            self.game.issue_order(legal[0][1])
            self.print_to_err("make random move")
        elif self.steps < self.steps_to_enemy_on_start:
            legal.sort(key=lambda x: abs(me.row + x[0][0] - enemy.row) + abs(me.col + x[0][1] - enemy.col))
            self.game.issue_order(legal[0][1])
            self.print_to_err("go to enemy")
        else:
            self.start_mode = False
            self.game.field.next_move = None
            self.game.field.mini_max(self.game.my_botid, self.game.players, self.game.field.desired_depth, -100000,
                                     100000)

            if self.game.field.next_move is None:
                self.print_to_err("make random move")
                self.game.issue_order(random_chosen)
            else:
                self.print_to_err(
                    "options: " + str([str(i[0]) + ": " + str(i[1]) for i in self.game.field.score_options.items()]))
                self.game.issue_order(self.game.field.next_move[1])

            took_time = time.time() - now
            if took_time < 0.3 and self.game.last_timebank > 5500:
                self.game.field.desired_depth += 1
                if self.game.field.desired_depth > self.max_depth:
                    self.game.field.desired_depth = self.max_depth
                self.print_to_err(
                    "took " + str(took_time) + "-> increase depth (to " + str(self.game.field.desired_depth) + ")")
            else:
                if took_time > 1:
                    self.game.field.desired_depth -= 2

                self.game.field.desired_depth -= 1
                if self.game.field.desired_depth < self.min_depth:
                    self.game.field.desired_depth = self.min_depth
                self.print_to_err(
                    "took " + str(took_time) + "-> decrease depth (to " + str(self.game.field.desired_depth) + ")")

    def print_to_err(self, output):
        print("step " + str(self.steps) + ": " + output, file=sys.stderr)
        sys.stderr.flush()
