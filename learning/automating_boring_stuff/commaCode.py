def listToStringWithAnd(words):
    stringWithAnd = words[0]

    # все слова кроме последнего отделяем запятой
    for word in words[1 : (len(words) - 1)]:
        stringWithAnd += ", " + str(word)
    # последнее слово отделяем 'and'
    stringWithAnd += " and " + str(words[len(words) - 1])

    return stringWithAnd


spam = ["apples", "bananas", "tofu", "cats"]
print(listToStringWithAnd(spam))
