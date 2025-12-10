import dataclasses
from typing import List

import z3


@dataclasses.dataclass
class Switchboard:
    switches: List[List[int]]
    joltages: List[int]

    @staticmethod
    def parse(line: str) -> 'Switchboard':
        sj = line.split("]")[1]
        s, j = sj.split("{")
        switches = [[int(y) for y in x.replace("(", "").replace(")", "").split(",")] for x in s.strip().split(" ")]
        joltages = [int(x) for x in j[:-1].split(",")]
        return Switchboard(switches, joltages)

    def minimum_presses(self):
        how_many_times = [z3.Int(f"s_{i}") for i in range(len(self.switches))]
        solver = z3.Optimize()
        for press in how_many_times:
            solver.add(press >= 0)
        for i, joltage in enumerate(self.joltages):
            res = 0
            for j, switch in enumerate(self.switches):
                if i in switch:
                    res += how_many_times[j]
            solver.add(joltage == res)
        cost = z3.Int('cost')
        solver.add(sum(how_many_times) == cost)
        solver.minimize(cost)
        solver.check()
        return solver.model()[cost].as_long()


def parse(content: str) -> List[Switchboard]:
    return [Switchboard.parse(line) for line in content.split("\n")]


def main():
    content = open("10.txt", "r").read()
    switchboards = parse(content)
    print(sum([switchboard.minimum_presses() for switchboard in switchboards]))


main()
