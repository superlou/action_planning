#!/usr/bin/python3
def reconstruct_path(came_from, current):
    total_path = [(current, 'goal')]

    while current in came_from:
        current, action = came_from[current]
        total_path.insert(0, (current, action))

    return total_path


INF = float('inf')


def tuple_diff(a, b):
    return [(field, getattr(a, field), getattr(b, field))
            for field in b._fields
            if getattr(a, field) != getattr(b, field) and
            getattr(a, field) != ... and getattr(b, field) != ...]


def a_star(start, goal, neighbors, h):
    open_set = [start]
    came_from = {}

    # Cost of path from start node to n
    g_score = {}
    g_score[start] = 0

    # Estimated cost of path from start node through n to goal
    # This is an estimate of the total path cost
    f_score = {}
    f_score[start] = h(start, goal)

    while len(open_set) > 0:
        open_set.sort(key=lambda id: f_score.get(id, INF))
        current = open_set.pop(0)

        if len(tuple_diff(current, goal)) == 0:
            return reconstruct_path(came_from, current)

        # print()
        # print(current)
        for neighbor, d, action in neighbors(current):
            # print(desc)
            tentative_g_score = g_score.get(current, INF) + d
            if tentative_g_score < g_score.get(neighbor, INF):
                # This path to neighbor is the best one seen so far
                came_from[neighbor] = current, action
                g_score[neighbor] = tentative_g_score
                f_score[neighbor] = g_score[neighbor] + h(neighbor, goal)

                if neighbor not in open_set:
                    open_set.append(neighbor)
