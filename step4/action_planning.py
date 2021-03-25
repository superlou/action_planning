#!/usr/bin/python3
from collections import namedtuple
from a_star import a_star, tuple_diff


def move(state, to):
    if state.pos == to:
        return None

    state = state._replace(pos=to)

    if state.holding is not None:
        moving_object = {state.holding + '_pos': to}
        state = state._replace(**moving_object)

    return state, 1, 'move to ' + to


def pick_up(state, object):
    precondition = (state.pos == getattr(state, object + '_pos')
                    and state.holding is None)
    if not precondition:
        return None

    return state._replace(holding=object), 1, 'pick up ' + object


def put_down(state):
    precondition = (state.holding is not None)
    if not precondition:
        return None

    return state._replace(holding=None), 1, 'put down ' + state.holding


def turn_on(state, object, required_pos):
    precondition = (state.pos == required_pos
                    and not getattr(state, object + '_on'))
    if not precondition:
        return None

    activated_object = {object + '_on': True}
    return state._replace(**activated_object), 1, 'turn on ' + object


def turn_off(state, object, required_pos):
    precondition = (state.pos == required_pos
                    and getattr(state, object + '_on'))
    if not precondition:
        return None

    deactivated_object = {object + '_on': False}
    return state._replace(**deactivated_object), 1, 'turn off ' + object


def wait(state):
    if state.pot_pos == 'sink' and state.faucet_on:
        return state._replace(pot_filled=True), 1, 'wait'

    return state._replace(), 1, 'wait'


def neighbors(state):
    states = []

    states += [move(state, pos) for pos in ['sink', 'counter', 'stove']]
    states += [pick_up(state, 'pot'), put_down(state)]
    states += [turn_on(state, 'faucet', 'sink'),
               turn_off(state, 'faucet', 'sink'),
               turn_on(state, 'stove', 'stove'),
               turn_off(state, 'stove', 'stove')]
    states += [wait(state)]

    states = [state for state in states if state is not None]

    return states


fields = ('pos', 'pot_pos', 'pot_filled', 'faucet_on', 'stove_on', 'holding')
State = namedtuple('State', fields, defaults=(...,) * len(fields))


def heuristic(node, goal):
    """ Estimate of cost to reach goal from node """
    return len(tuple_diff(node, goal))


if __name__ == '__main__':
    s0 = State(pos='counter', pot_pos='counter', pot_filled=False,
               faucet_on=False, stove_on=False, holding=None)
    s_goal = State(pot_pos='stove', stove_on=True, pot_filled=True, holding=None,
                   faucet_on=False)
    path = a_star(s0, s_goal, neighbors, heuristic)
    print('Path:')

    for step in path:
        print(step[1])
