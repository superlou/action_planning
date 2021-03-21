#!/usr/bin/python3
from collections import namedtuple
from a_star import a_star


def neighbors(state):
    linkage_map = {
        1: [(state._replace(pos=2), 1.5), (state._replace(pos=3), 1.5)],
        2: [(state._replace(pos=1), 1.5), (state._replace(pos=4), 3.0)],
        3: [(state._replace(pos=1), 1.5), (state._replace(pos=5), 4.5)],
        4: [(state._replace(pos=2), 3.0), (state._replace(pos=5), 2.0), (state._replace(pos=6), 2.5)],
        5: [(state._replace(pos=3), 4.5), (state._replace(pos=4), 2.0), (state._replace(pos=6), 2.5)],
        6: [(state._replace(pos=4), 2.5), (state._replace(pos=5), 2.5)],
    }

    return linkage_map[state.pos]


def position(state):
    position_map = {
        1: (0, 0),
        2: (1, 2),
        3: (2, -1),
        4: (4, 2),
        5: (4, 0),
        6: (6, 1),
    }

    return position_map[state.pos]


State = namedtuple('State', 'pos')


def heuristic(node, goal):
    """ Estimate of cost to reach goal from node """
    return (abs((position(node)[0] - position(goal)[0]))
            + abs(position(node)[1] - position(goal)[1]))


if __name__ == '__main__':
    s0 = State(pos=1)
    s_goal = State(pos=6)
    path = a_star(s0, s_goal, neighbors, heuristic)
    print('Path:', path)
