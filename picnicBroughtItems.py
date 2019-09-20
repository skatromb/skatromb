allGuests = {'Alice': {'apples': 5, 'pretzels': 12},
             'Bob': {'ham sandwiches': 3, 'apples': 2},
             'Carol': {'cups': 3, 'apple pies': 1}}

def totalBrought(guests, item):
    numBrought = 0
    for v in guests.values():
        numBrought += v.get(item, 0)
    return numBrought

# Собираем список всех вещей
listItems = []
for guestsItems in allGuests.values():
    for item in guestsItems.keys():
        if item not in listItems:
            listItems.append(item)
print(listItems)

# Печатаем список всех вещей
print('Number of things being brought:')
for item in listItems:
    print('- ' + item + ': ' + str(totalBrought(allGuests, item)))