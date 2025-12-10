from z3 import Optimize, Int, If, sat, Sum


def get_input(filename):
    result = []
    with open(filename, "r") as f:
        for line in f:
            s = line.strip()
            if not s:
                continue

            first = s.find(' ')
            last = s.rfind(' ')

            part2 = s[first + 1:last]
            part3 = s[last + 1:]

            part2_list = []
            for tok in part2.split():
                tok_clean = tok.strip("()")
                nums = [int(x) for x in tok_clean.split(',') if x]
                part2_list.append(nums)
            part3_list = [int(x) for x in part3.replace('{', '').replace('}', '').split(',') if x.strip()]

            result.append((part2_list, part3_list))

    return result


def solve_min_button_presses(buttons, target):
    n = len(target)
    b = len(buttons)

    opt = Optimize()

    count = [Int(f"c_{i}") for i in range(b)]
    for c in count:
        opt.add(c >= 0)  # cant press a button negative times

    for i in range(n):
        expr = Sum([If(i in buttons[btn], count[btn], 0) for btn in range(b)])
        opt.add(expr == target[i])

    opt.minimize(Sum(count))

    if opt.check() == sat:
        model = opt.model()
        button_counts = [model.eval(c).as_long() for c in count]
        sequence = []
        for btn, cnt in enumerate(button_counts):
            sequence.extend([btn] * cnt)

        return sequence
    else:
        return None


if __name__ == "__main__":
    input_data = get_input("src/input.txt")

    total = 0
    for i, (buttons, target) in enumerate(input_data):
        seq = solve_min_button_presses(buttons, target)
        total += len(seq)

    print(total)
