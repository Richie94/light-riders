# starter bot refactored from Riddles Hackman python2 starter bot
import sys

from Bot.game import Game
from Bot.bot import Bot
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--random', type=int, help='makes random moves', default=0, required=False)

    args = parser.parse_args()

    bot = Bot(args.random)
    game = Game()
    game.run(bot)





if __name__ == '__main__':
    main()
