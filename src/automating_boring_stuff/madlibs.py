# madlibs.py
import os
import re

# prompt the user for replacing words
wordsToReplace = ("ADJECTIVE", "NOUN", "ADVERB", "VERB")
replaceDict = dict()
for word in wordsToReplace:
    replaceDict[word] = input("Enter an " + word.lower() + ":\n")

# creating the list of all .txt files in directory
allFilenames = os.listdir()
txtRegex = re.compile(r".*\.txt$")
txtFilenames = list()
for filename in allFilenames:
    matcher = txtRegex.search(filename)
    if matcher is not None:
        txtFilenames.append(matcher.group())

# read all the .txt files in folder
for txtFilename in txtFilenames:
    print("File " + txtFilename + ":\n")
    # open the content of file and translate it to print
    text = open(txtFilename).read()
    for replacedWord, replaceWith in replaceDict.items():
        text = text.replace(replacedWord, replaceWith)

    print(text + "\n\n")
