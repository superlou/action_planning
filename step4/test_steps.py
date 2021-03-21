from collections import namedtuple
from a_star import tuple_diff


def test_namedtuple_distance_simple():
    State = namedtuple('State', 'x y z')
    s1 = State(x=1, y=2, z=3)
    s2 = State(x=1, y=2, z=3)
    assert tuple_diff(s1, s2) == []

    s2 = s2._replace(x=2)
    assert tuple_diff(s1, s2) == [('x', 1, 2)]

    s2 = s2._replace(y=3)
    assert tuple_diff(s1, s2) == [('x', 1, 2), ('y', 2, 3)]

    s2 = s2._replace(z=4)
    assert tuple_diff(s1, s2) == [('x', 1, 2), ('y', 2, 3), ('z', 3, 4)]


def test_namedtuple_distance_with_any():
    State = namedtuple('State', 'x y z')
    s1 = State(x=1, y=2, z=...)
    s2 = State(x=1, y=2, z=3)
    assert tuple_diff(s1, s2) == []

    s2 = s2._replace(x=2)
    assert tuple_diff(s1, s2) == [('x', 1, 2)]

    s2 = s2._replace(y=3)
    assert tuple_diff(s1, s2) == [('x', 1, 2), ('y', 2, 3)]

    s2 = s2._replace(z=4)
    assert tuple_diff(s1, s2) == [('x', 1, 2), ('y', 2, 3)]

    s2 = s2._replace(z=...)
    assert tuple_diff(s1, s2) == [('x', 1, 2), ('y', 2, 3)]
