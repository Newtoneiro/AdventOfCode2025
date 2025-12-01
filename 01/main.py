import os

dirname = os.path.dirname(__file__)
input = os.path.join(dirname, "input.txt")


class Dial:
    def __init__(self):
        self._val = 50
        self.counter = 0

    @property
    def val(self):
        return self._val

    @val.setter
    def val(self, value):
        if value == 0:
            self.counter += 1
        self._val = value

    def rotate(self, action):
        dir, num = action[0], int(action[1:])

        if dir == "R":
            self.val = (self.val + num) % 100
        else:
            self.val = (self.val - num) % 100


class Dial2:
    def __init__(self):
        self.val = 50
        self.counter = 0

    def rotate(self, action):
        dir, num = action[0], int(action[1:])

        if dir == "R":
            for _ in range(num):
                self.val += 1
                if self.val == 100:
                    self.val = 0
                    self.counter += 1
        else:
            for _ in range(num):
                self.val -= 1
                if self.val == 0:
                    self.counter += 1
                if self.val == -1:
                    self.val = 99


def main1():
    dial = Dial()
    with open(input) as f:
        for action in f.readlines():
            action = action.strip()
            dial.rotate(action)
    print(dial.counter)


def main2():
    dial = Dial2()
    with open(input) as f:
        for action in f.readlines():
            action = action.strip()
            dial.rotate(action)
    print(dial.counter)


if __name__ == "__main__":
    main1()
    main2()
