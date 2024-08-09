import random
from datetime import datetime

start = datetime.now()

# Порог функции активации
BIAS = 7
NUM_OF_TRAINS = 10000

# Цифры
NUM = dict()

NUM[0] = "111" "1 1" "1 1" "1 1" "111"

NUM[1] = "  1" "  1" "  1" "  1" "  1"

NUM[2] = "111" "  1" "111" "1  " "111"

NUM[3] = "111" "  1" "111" "  1" "111"

NUM[4] = "1 1" "1 1" "111" "  1" "  1"

NUM[5] = "111" "1  " "111" "  1" "111"

NUM[6] = "111" "1  " "111" "1 1" "111"

NUM[7] = "111" "  1" "  1" "  1" "  1"

NUM[8] = "111" "1 1" "111" "1 1" "111"

NUM[9] = "111" "1 1" "111" "  1" "111"

# Виды цифры 5 (Тестовая выборка)
NUM_5 = dict()

NUM_5[0] = "111" "1  " "111" "   " "111"

NUM_5[1] = "111" "1  " " 1 " "  1" "111"

NUM_5[2] = "111" "1  " " 11" "  1" "111"

NUM_5[3] = "11 " "1  " "111" "  1" "111"

NUM_5[4] = "11 " "1  " "111" "  1" " 11"

NUM_5[5] = "111" "1  " "1 1" "  1" "111"


# Является ли данное число 5
def proceed(number: str) -> int:
    # Рассчитываем взвешенную сумму
    net = 0
    for i in range(len(number)):
        # Обрабатываем каждую точку
        coeff = 1 if number[i] == "1" else 0
        net += coeff * weights[i]

    # Превышен ли порог? (Да - сеть думает, что это 5. Нет - сеть думает, что это другая цифра)
    return net >= BIAS


# Уменьшение значений весов, если сеть ошиблась и выдала 1
def decrease(number: str):
    for i in range(len(number)):
        # Возбужденный ли вход
        if number[i] == "1":
            # Уменьшаем связанный с ним вес на единицу
            weights[i] -= 1


# Увеличение значений весов, если сеть ошиблась и выдала 0
def increase(number):
    for i in range(len(number)):
        # Возбужденный ли вход
        if number[i] == "1":
            # Увеличиваем связанный с ним вес на единицу
            weights[i] += 1


# Инициализация весов сети
weights = [0 for i in range(15)]

# Тренировка сети
for i in range(NUM_OF_TRAINS):
    # Генерируем случайное число от 0 до 9
    number = random.randint(0, 9)

    # Если число не 5
    if number != 5:
        # А сеть выдала True, то снижаем веса
        if proceed(NUM[number]):
            decrease(NUM[number])
    # Если число 5
    else:
        # А сеть выдала False, то повышаем веса
        if not proceed(NUM[5]):
            increase(NUM[5])

# Вывод значений весов
print("Веса")
for row in range(5):
    for column in range(3):
        print(weights[row * column], end="")
    print()

# Прогон по обучающей выборке
for i in range(10):
    print(f"{i} это 5? {proceed(NUM[i])}")
print()

# Прогон по тестовой выборке
for i in range(6):
    print(f"Узнал 5.{i}?  {proceed(NUM_5[i])}")

end = datetime.now()
print(f"Программа выполнялась {str(end - start)}")
