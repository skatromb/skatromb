import subprocess
import os
import xml.etree.ElementTree as ET
from pathlib import Path

CERT_TEMPLATE = {'Мужской': Path('templates/delegate_muzh.svg'),
                 'Женский': Path('templates/delegate_zhen.svg')}
NAME_TAG_ID = 'NAME_TAG'
NAMESPACE = {'NAME_TAG': 'http://www.w3.org/2000/svg'}

GOOGLE_CHROME_PATH = '"/Applications/Google Chrome.app/Contents/MacOS/Google Chrome"'
SCREENSHOT_PARAMS = '--headless --window-size=1480,1048 --screenshot'

OUTPUT_FOLDER = 'output'


def fill_svg(name: str, gender: str) -> ET:
    """Читаем svg и ищем место для имени по id"""
    tree = ET.parse(CERT_TEMPLATE[gender])

    name_search_pattern = f'.//*[@id="{NAME_TAG_ID}"]/NAME_TAG:tspan'
    name_tags = tree.findall(name_search_pattern, NAMESPACE)

    if len(name_tags) == 1:
        name_tags[0].text = name
    else:
        raise BaseException('В шаблоне svg оказалось больше одного места для подстановки ФИ')

    tree.write(f'{Path(OUTPUT_FOLDER) / name}.svg')


def svg2png(name: str):
    """Преобразовывает `svg` в `png`"""
    cmd = [f'{GOOGLE_CHROME_PATH} {SCREENSHOT_PARAMS}='
           f'"{OUTPUT_FOLDER}/{name}.png" '
           f'"{OUTPUT_FOLDER}/{name}.svg"']
    print(cmd[0])
    subprocess.call(cmd, shell=True)

# Обернуть циклом по всем ФИО/полу
# Сохранить


if __name__ == '__main__':
    fill_svg('Марьяна Онысько', 'Женский')
    svg2png('Марьяна Онысько')
