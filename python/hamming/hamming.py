def distance(s1, s2):
    return sum([n1 != n2 and 1 or 0 for n1, n2 in zip(s1, s2)])
