# считываем J и S
with open('input.txt') as in_file:
    J = in_file.readline()
    S = in_file.readline()
    in_file.()

# если очередной из S входит в J -> прибавить единицу к счётчику
count = 0
for symbol in S:
    if symbol in J:
        count += 1
# написать в выходной файл
with open('output.txt', 'w') as out_file:
    out_file.write(str(count))
