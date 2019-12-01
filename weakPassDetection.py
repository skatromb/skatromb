import re


def password_strength(password:str) -> str:
    weakness_reasons = list()
    if len(password) < 8:
        weakness_reasons.append('must be at least 8 symbols long')
    if password.upper() == password or password.lower() == password:
        weakness_reasons.append('must have both lowercase and uppercase symbols')
    have_1_digit_regex = re.compile(r'\d')
    if have_1_digit_regex.search(password) is None:
        weakness_reasons.append('must have at least 1 digit')
    if len(weakness_reasons) == 0:
        return 'Password is strong enough'
    else:
        return 'Password "' + password + '" is weak because:\nIt ' + '\n'.join(weakness_reasons) + '.'


password = input('Enter the password\n')
print(password_strength(password))
