tableData = [['apples', 'oranges', 'cherries', 'banana'],
             ['Alice', 'Bob', 'Carol', 'David'],
             ['dogs', 'cats', 'moose', 'goose']]


def printTable(tableData):
    # Узнаем ширину строк по максимальной длине строки в листе
    colWidths = [0] * len(tableData)
    for cols in range(0, len(tableData)):
        colWidths[cols] = len(tableData[cols][0])
        for rows in range(1, len(tableData[cols])):
            maybeMaxLen = len(tableData[cols][rows])
            if maybeMaxLen > colWidths[cols]:
                colWidths[cols] = maybeMaxLen
    # Собственно, выводим табличку
    for rows in range(0, len(tableData[0])):
        for cols in range(0, len(tableData)):
            print(tableData[cols][rows].rjust(colWidths[cols] + 1), end='')
        print()

printTable(tableData)
