import re


def regex_strip(string: str, str_to_remove=' ') -> str:
    regex = re.compile(str_to_remove)
    return regex.sub('', string)


what_to_delete = input('What you want to delete?\n')
print(regex_strip(input('Enter the text in which you want to delete\n')), what_to_delete)