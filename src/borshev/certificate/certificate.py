import subprocess
from csv import reader
from xml.etree import ElementTree
from pathlib import Path
from typing import Iterator, Iterable

CERT_TEMPLATE = {'Мужской': Path('templates/delegate_muzh.svg'),
                 'Женский': Path('templates/delegate_zhen.svg')}
NAME_TAG_ID = 'NAME_TAG'
NAMESPACE = {'NAME_TAG': 'http://www.w3.org/2000/svg'}

GOOGLE_CHROME_PATH = '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome'
SCREENSHOT_PARAMS = '--headless --window-size=1480,1048 --screenshot'

INPUT_NAMES_PATH = Path('input/FI_gender.csv')
OUTPUT_FOLDER = 'output'


class Student:
    name: str
    gender: str

    def __init__(self, name: str, gender: str):
        self.name = name
        self.gender = gender


def make_list(csv_path: Path) -> Iterator[Student]:
    """Считывает список студентов из csv, кому делать сертификаты"""
    with open(csv_path) as file:
        csv_reader = reader(file)
        next(csv_reader)  # Пропускаем заголовок
        for row in csv_reader:
            yield Student(row[0], row[1])


def fill_svg(name: str, gender: str):
    """Читаем svg и ищем место для имени по id"""
    tree = ElementTree.parse(CERT_TEMPLATE[gender])

    name_search_pattern = f'.//*[@id="{NAME_TAG_ID}"]/NAME_TAG:tspan'
    name_tags = tree.findall(name_search_pattern, NAMESPACE)

    if len(name_tags) == 1:
        name_tags[0].text = name
    else:
        raise BaseException('В шаблоне svg оказалось больше одного места для подстановки ФИ')

    tree.write(f'{Path(OUTPUT_FOLDER) / name}.svg')


def svg2png(name: str):
    """Преобразовывает `svg` в `png`"""
    cmd = [f'"{GOOGLE_CHROME_PATH}" {SCREENSHOT_PARAMS}='
           f'"{OUTPUT_FOLDER}/{name}.png" '
           f'"{OUTPUT_FOLDER}/{name}.svg"']
    subprocess.call(cmd, shell=True)


def generate_certificates():
    """Забирает список студентов, модифицирует svg, генерит png"""
    for student in make_list(INPUT_NAMES_PATH):
        fill_svg(student.name, student.gender)
        svg2png(student.name)


if __name__ == '__main__':
    generate_certificates()
