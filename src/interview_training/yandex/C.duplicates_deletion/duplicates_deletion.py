# TODO: считываем файл построчно, пропускаем первую строку
with open('input.txt') as in_file:
    in_file.readline()
    unique = [in_file.readline().strip()]
    for line in in_file:
        element = line.strip()
        # TODO: если очередная цифра не равна последней — добавляем в лист
        if element != unique[-1]:
            unique.append(element)

for element in unique:
    print(element)
