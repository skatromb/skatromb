def collatz(number):
    if (number % 2) == 0:
        number = number // 2
    else:
        number = number * 3 + 1
    print(number)
    return number


inputNumber = None
while not inputNumber:
    try:
        inputNumber = int(input("Введите число: "))
    except ValueError:
        print("Вы ввели не число. Постарайтесь всё же ввести число.")
while inputNumber != 1:
    inputNumber = collatz(inputNumber)
