from typing import Dict, List

# 7     # n — количество городов
# 0 0   # 1 город
# 0 2   # 2 город
# 2 2   # 3 город
# 0 -2  # 4 город
# 2 -2  # 5 город
# 2 -1  # 6 город
# 2 1   # 7 город
# 2     # k — расстояние без дозаправки
# 1 3   # откуда (1 город), куда (4 город)

# есть запас хода k и расстояние между городами |x2 - x1| + |y2 - y1|


class Town:
    x: int
    y: int
    connections: list

    def __init__(self, x: str, y: str):
        self.x = int(x)
        self.y = int(y)
        self.connections = []

    def __str__(self):
        return f"x={self.x}, y={self.y}, connections={self.connections}"


# Считать input
def take_input(filename) -> (list[dict], int, int, int):
    """towns = [{Town: []}, ...]"""
    with open(filename) as in_file:
        n = int(in_file.readline().strip())

        towns = []

        # Добавляем новые города в список
        for _ in range(n):
            coordinates = in_file.readline().strip().split(" ")
            town = Town(*coordinates)
            towns.append(town)

        k = int(in_file.readline().strip())

        start_dest = in_file.readline().strip().split(" ")
        start, destination = (int(num) for num in start_dest)

        return towns, k, start - 1, destination - 1


# Соединены ли вершины
def is_connected(town_1: Town, town_2: Town, hops: int) -> bool:
    if abs(town_2.x - town_1.x) + abs(town_2.y - town_1.y) > hops:
        return False
    else:
        return True


# Ищем, между какими городами можно доехать
def connected(towns: list[Town]) -> list:
    # Ищем связи каждого города с другими
    for town in towns:
        # Смотрим на связность с каждым другим городом
        for other_town in towns:
            if town != other_town and is_connected(town, other_town, hops):
                town.connections += [other_town]
    return towns


def do_step(cur_path: list, next_steps: list) -> (list, list):
    # return new_path, new_steps
    ...


# TODO: Найти путь
def find_shortest_path(towns: list[Town], start: int):
    path = [[Town]]
    while True:
        new_paths = []
        # for ...

    # План таков: найти связанные города — те, между которыми чуваку хватит топлива доехать.
    #       1. Получим циклический граф переходов. Ещё у него есть начало и желаемый конец.
    #           {1: [2, 4],
    #            2: [1, 3],
    #            3: [2, 7],
    #            4: [1, 5],
    #            5: [4, 6],
    #            6: [5, 7],
    #            7: [3, 6]}
    #       2. Записываем, где мы сейчас в path == [1]
    #       3. Записываем, куда можно пойти дальше в next_steps == [2, 4]
    #       4. Для каждого из дальнейших шагов записываем
    #           path == [1, 2], next_steps == [1, 3] -> [3]: добавляем те, где ещё не были
    #           path == [1, 4], next_steps == [1, 5] -> [5]: добавляем те, где ещё не были
    #       5. Повторяем, п. 2-4, пока не придём в конец или не закончатся next_steps.
    #           path == [1, 2, 3], next_steps == [7] -> []
    #           path == [1, 4, 5], next_steps == [6] -> []
    #       6. Возвращаем len(path), если пришли


if __name__ == "__main__":
    towns, hops, start_town, dest_town = take_input("input.txt")
    towns = connected(towns)

    print(towns)
