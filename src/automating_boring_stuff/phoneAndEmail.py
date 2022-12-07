#! python3
# phoneAndEmail.py — Finds phone numbers and email addresses on the clipboard.

import re

import pyperclip

AREA_CODE = 1
FIRST_3_DIGITS = 3
LAST_4_DIGITS = 5
EXTENSION = 8
phoneRegex = re.compile(
    r"""(
    (\(?\d{3}\)?)?                      #1! maybe area src
    (\s|-|\.)?                          #2 maybe separator
    (\d{3})                             #3! first 3 digits
    (\s|-|\.)?                          #4 separator
    (\d{4})                             #5! last 4 digits
    (\s*                                #6 separators
    (ext|x|ext\.)\s*                     #7 extension string
    (\d{2,5}))?                         #8! extension itself
)""",
    re.VERBOSE,
)

emailRegex = re.compile(
    r"""(
    [a-zA-Z0-9._%+-]+                   # username
    @                                   # sobaka
    (\w+(.|\-)?)+                       # domain/subdomain
    (\.[a-zA-Z]{2,4})                   # .ru/.com
)""",
    re.VERBOSE,
)

text = str(pyperclip.paste())

# Собираем почты в emails
emails = []
for emailMatchGroups in emailRegex.findall(text):
    emails.append(emailMatchGroups[0])

# Собираем номера телефонов в phones
phones = []
for phoneMatchGroups in phoneRegex.findall(text):
    if len(phoneMatchGroups[AREA_CODE]) > 0:
        areaCode = "(" + phoneMatchGroups[AREA_CODE] + ") "
    else:
        areaCode = ""
    if len(phoneMatchGroups[EXTENSION]) > 0:
        extension = "ext. " + phoneMatchGroups[EXTENSION]
    else:
        extension = ""
    phones.append(
        areaCode
        + "-".join([phoneMatchGroups[FIRST_3_DIGITS], phoneMatchGroups[LAST_4_DIGITS]])
        + extension
    )

emailMatches = emailRegex.findall(text)
phoneMatches = phoneRegex.findall(text)

# Выводим в clipboard телефоны и почты
result = ""
if len(emails) > 0:
    result += "Emails:\n" + "\n".join(emails) + "\n\n"
if len(phones) > 0:
    result += "Phones:\n" + "\n".join(phones)
if len(result) > 0:
    pyperclip.copy(result)
else:
    pyperclip.copy("Nothing matched phone or email")
