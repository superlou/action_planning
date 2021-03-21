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


def turn_on_faucet(state):
    precondition = (state.pos == 'sink' and not state.faucet_on)
    if not precondition:
        return None

    state = state._replace(faucet_on=True)

    if state.pot_pos == 'sink':
        state = state._replace(pot_filled=True)

    return state._replace(faucet_on=True), 1, 'turn on faucet'


def turn_off_faucet(state):
    precondition = (state.pos == 'sink' and state.faucet_on)
    if not precondition:
        return None

    return state._replace(faucet_on=False), 1, 'turn off faucet'


def turn_on_stove(state):
    precondition = (state.pos == 'stove' and not state.stove_on)
    if not precondition:
        return None

    return state._replace(stove_on=True), 1, 'turn on stove'


def turn_off_stove(state):
    precondition = (state.pos == 'stove' and state.stove_on)
    if not precondition:
        return None

    return state._replace(stove_on=False), 1, 'turn off stove'


def neighbors(state):
    states = []

    states += [move(state, pos) for pos in ['sink', 'counter', 'stove']]
    states += [pick_up(state, 'pot'), put_down(state)]
    states += [turn_on_faucet(state), turn_off_faucet(state)]
    states += [turn_on_stove(state), turn_off_stove(state)]

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
    s_goal = State(pot_pos='stove', stove_on=True, pot_filled=True, holding=None)
    path = a_star(s0, s_goal, neighbors, heuristic)
    print('Path:')

    for step in path:
        print(step[1])
