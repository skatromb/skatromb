from collections import defaultdict


def is_anagram(file) -> int:
    first_count, second_count = defaultdict(int), defaultdict(int)
    # TODO: Считываем обе строки посимвольно в словари {'a': 3, 'b': 5}
    for symbol in in_file.readline():
        first_count[symbol] += 1

    for symbol in in_file.readline():
        second_count[symbol] += 1

    # TODO: итерируемся по любому из словарей, проверяем, что букв столько же
    if len(first_count) != len(second_count):
        return 0
    for symbol, count in first_count.items():
        if count != second_count[symbol]:
            return 0
    return 1


with open('input.txt') as in_file:
    print(is_anagram(in_file))
