import re

regex = re.compile(r'((Alice|Bob|Carol) (eats|pets|throws) (apples|cats|baseballs)\.)', re.IGNORECASE)
print(regex.findall('Alice eats apples. Bob pets cats. Carol throws baseballs. Alice throws Apples. BOB EATS CATS.'
              ' but not the following'
              'RoboCop eats apples. ALICE THROWS FOOTBALLS. Carol eats 7 cats.'))
