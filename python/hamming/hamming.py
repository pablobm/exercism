def distance(s1, s2):
    return sum(1 for n1, n2 in zip(s1, s2) if n1 != n2)
