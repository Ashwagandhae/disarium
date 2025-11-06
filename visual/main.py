import matplotlib.pyplot as plt


def dicerium(num):
    digits = list(map(int, str(num)))
    digits = digits
    return sum(digit ** (i + 1) for i, digit in enumerate(digits))


def transform(n):
    return n * 10 - 1


numbers = [transform(n) for n in range(1, 1000)]


differences = [dicerium(n) - n for n in numbers]


colors = ["green" if diff >= 0 else "red" for diff in differences]


plt.figure(figsize=(15, 6))
plt.bar(numbers, differences, color=colors)
plt.title("Differences Between Numbers and Dicerium Procedure (1 to 1000)")
plt.xlabel("Number")
plt.ylabel("Dicerium - Original")
plt.show()
