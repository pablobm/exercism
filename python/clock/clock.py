class Clock:
    def __init__(self, h, m):
        self.h = h
        self.m = 0
        self.add(m)

    def __str__(self):
        return "%02i:%02i" % (self.h, self.m)

    def __eq__(self, other):
        return str(self) == str(other)

    def add(self, m):
        self.m += m
        self.h = self.h + self.m / 60
        self.m = self.m % 60
        self.h = self.h % 24
        return self
