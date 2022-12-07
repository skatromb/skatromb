import re

text = (
    "Satoshi Nakamoto wants to fuck Alice Nakamoto. RoboCop Nakamoto is "
    "contrary. Mr. Nakamoto doesn"
    "t like satoshi Nakamoto "
    "and Nakamoto which is Satoshi nakamoto grandpa"
)
regex = re.compile(r"([A-Z]\w+ Nakamoto)")
print(regex.findall(text))
