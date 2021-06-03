import logging
import os
import subprocess
from csv import DictReader
from xml.etree import ElementTree
from pathlib import Path
from os import remove


CHROME_PARAMS = {'google_chrome_path': '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome',
                 'screenshot_params': '--headless --window-size=1480,1048 --screenshot'}
logger = logging.getLogger(__name__)
logging.getLogger().setLevel(logging.INFO)


def student_list(csv_path: Path) -> dict:
    """Считывает список студентов из csv, кому делать сертификаты"""
    with open(csv_path) as file:
        reader = DictReader(file)
        for row_dic in reader:
            logger.info(row_dic)
            yield row_dic


# Выбиратор нужного темплейта для сертификата
def choose_template(folder: str, student: dict) -> Path:
    """parameters:
        full_name: str
        gender: 'male', 'female'
        homework: True, False"""
    # TODO: Стоит для каждого продукта сделать отдельный маппинг
    lang = 'eng' if student['full_name'].isascii() else None
    gender = None if lang == 'eng' else student['gender']
    homework = 'homework' if student['homework'] == 'да' else None
    filename = '_'.join([elem for elem in [lang, gender, homework] if elem])
    return Path('templates') / f'{folder}' / f'{filename}.svg'

# TODO: Унести в тесты
# for combo in product(('Русский Человек', 'English Person'), ('female', 'male'), (True, False)):
#     params = {'full_name': combo[0], 'gender': combo[1], 'homework': combo[2]}
#     print(f'{params}: {choose_template("architecture", params)}')


def fill_template(product: str, student: dict, output_folder: Path, name_tag_id='NAME_TAG', ):
    """Читаем svg и ищем место для имени по id"""
    template = choose_template(product, student)
    tree = ElementTree.parse(template)

    name_search_pattern = f'.//*[@id="{name_tag_id}"]/NAME_TAG:tspan'
    name_tags = tree.findall(name_search_pattern, {'NAME_TAG': 'http://www.w3.org/2000/svg'})

    if len(name_tags) == 1:
        name_tags[0].text = student['full_name']
    else:
        raise Exception('В шаблоне svg оказалось больше одного места для подстановки ФИ')

    os.makedirs(output_folder, exist_ok=True)
    tree.write(output_folder / f'{student["full_name"]}.svg')


def template2png(filename: str, output_folder: Path, chrome_params: dict,):
    """Делает из `svg` скриншот в `png`"""
    cmd = [f'"{chrome_params["google_chrome_path"]}" {chrome_params["screenshot_params"]}='
           f'"{output_folder}/{filename}.png" '
           f'"{output_folder}/{filename}.svg"']
    logger.info(cmd)
    subprocess.call(cmd, shell=True)
    remove(f'{output_folder}/{filename}.svg')


def generate_certificates(product: str, input_path: Path):
    """Итерируется по списку студентов, модифицирует svg, генерит png"""
    for student in student_list(input_path):
        output_folder = Path("output") / student["course_id"] / student["user_id"]
        fill_template(product, student, output_folder)
        template2png(student['full_name'], output_folder, CHROME_PARAMS)


if __name__ == '__main__':
    generate_certificates('teamlead', Path('input/students.csv'))
