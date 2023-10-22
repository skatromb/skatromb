import random
from datetime import datetime

# Задаем коэффициенты цравнения прямой y = kx + c
k = random.uniform(-5, 5)
c = random.uniform(-5, 5)


# Функция
def f(x):
    return k * x + c


# Вывод данных начальной прямой
print(f"Начальная прямая: y = {k}x + {c}")

# Набор точек X:Y
data = {
    1: 2,
    2: 4.2,
    2.5: 5,
    3.8: 7.9,
    4: 9,
    6: 10.2,
    6.6: 13,
    7.2: 15.3,
    8: 17.1,
    8.5: 19.5,
}

# Скорость обучения
edu_rate = 0.0001

repeats = round(10 / edu_rate)

start = datetime.now()

# Тренировка сети
for i in range(repeats):
    # Получить случайную X координату точки
    x = random.choice(list(data.keys()))

    # Получить соответствующую Y координату точки
    true_result = data[x]

    # Получить ответ сети
    neuro_result = f(x)

    # Считаем ошибку сети
    delta = true_result - neuro_result

    # Меняем вес при x в соответствии с дельта-правилом
    k += delta * edu_rate * x

    # Меняем вес при постоянном входе в соответствии с дельта-правилом
    c += delta * edu_rate

end = datetime.now()
time = str(end - start)
# Вывод данных готовой прямой
print(f"Готовая прямая: y = {k}x + {c}")
print(f"Обучение заняло {time}")
