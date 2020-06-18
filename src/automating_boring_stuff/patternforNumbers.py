import re


numRegex = re.compile(r'(^|\s\d{1,3}(,\d{3})*\s|$)')
string = '42    1,234    6,368,745    12,34,567   1234'
print(numRegex.findall(string))
