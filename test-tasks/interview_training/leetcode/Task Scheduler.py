from typing import List

"""Given a characters array tasks, representing the tasks a CPU needs to do,
where each letter represents a different task.
Tasks could be done in any order. Each task is done in one unit of time.
For each unit of time, the CPU could complete either one task or just be idle.

However, there is a non-negative integer n that represents the cooldown period between two same tasks
(the same letter in the array), that is that there must be at least n units of time between any two same tasks.

Return the least number of units of times that the CPU will take to finish all the given tasks."""

"""Example 1:

Input: tasks = ["A","A","A","B","B","B"], n = 2
Output: 8
Explanation:
A -> B -> idle -> A -> B -> idle -> A -> B
There is at least 2 units of time between any two same tasks.


Example 2:

Input: tasks = ["A","A","A","B","B","B"], n = 0
Output: 6
Explanation: On this case any permutation of size 6 would work since n = 0.
["A","A","A","B","B","B"]
["A","B","A","B","A","B"]
["B","B","B","A","A","A"]
...
And so on.


Example 3:

Input: tasks = ["A","A","A","A","A","A","B","C","D","E","F","G"], n = 2
Output: 16
Explanation:
One possible solution is
A -> B -> C -> A -> D -> E -> A -> F -> G -> A -> idle -> idle -> A -> idle -> idle -> A"""


def count_dict(dic: dict) -> int:
    return sum(value for value in dic.values())


def unspent_letters(initial: dict, spent: dict) -> list:
    # Нужно вернуть отсортированный список от наиболее частых букв к наименее частым
    current_count = {
        letter: initial[letter] - spent[letter]
        for letter in initial
        if initial[letter] - spent[letter] > 0
    }
    return [
        letter[0]
        for letter in reversed(
            sorted(current_count.items(), key=lambda item: item[1])
        )
    ]


def least_interval(tasks: list[str], n: int) -> int:
    schedule = []
    # Разбить все буквы на группы с количеством initial_letters {'A': 6, 'B': 1, ...}
    letters_set = set(tasks)
    initial_letters = {letter: tasks.count(letter) for letter in letters_set}
    spent_letters = {letter: 0 for letter in letters_set}
    # Пока буквы остались, идём по буквам initial_letters с самой большой и делаем проверку
    while unused_letters := unspent_letters(initial_letters, spent_letters):
        for letter in unspent_letters(initial_letters, spent_letters):
            #  Если очередной буквы нет в schedule[-n:], дополняем ей schedul и записываем, что потратили её
            if n == 0 or letter not in schedule[-n:]:
                schedule.append(letter)
                spent_letters[letter] += 1
                break
            #   Если дошли до конца и не нашли подходящей буквы, добавляем idle
            elif letter == unused_letters[-1]:
                schedule.append("idle")

    # После исчерпания букв считаем len(schedule) и отдаём
    print(schedule)
    print(len(schedule))
    return len(schedule)


least_interval(["A", "A", "A", "B", "B", "B"], n=2)
["A", "A", "A", "B", "B", "B", "C", "C", "C", "D", "D", "E"]
["B", "A", "C", "B", "A", "C", "B", "A", "C", "D", "E", "idle", "D"]
