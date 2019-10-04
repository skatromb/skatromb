# madlibs.py
import os, re


# TODO: write a function that replaces words (if it needs)
def replaceOccurrences(text: str, wordstoreplace: tuple) -> str:
    return replacedString


# prompt the user for replacing words
wordsToReplace = ('ADJECTIVE', 'NOUN', 'ADVERB', 'VERB')
replaceDict = dict()
for word in wordsToReplace:
    replaceDict[word] = input('Enter an ' + word.lower() + ':\n')

# creating the list of all .txt files in directory
allFiles = os.listdir()
txtRegex = re.compile(r'.*\.txt$')
txtFiles = list()
for filename in allFiles:
    matcher = txtRegex.search(filename)
    if matcher is not None:
        txtFiles.append(matcher.group())

# TODO: read all the .txt files in folder

# TODO: find the occurrences of words
for textFile in allFiles:

    replacedString = replaceOccurrences(textFile, wordsToReplace)
    # and replace them with words from user prompt

    print(replacedString)