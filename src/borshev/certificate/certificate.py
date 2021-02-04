import xml.etree.ElementTree as ET
from pathlib import Path

# Читаем svg и ищем по id
CERT_TEMPLATE = {'Мужской': Path('templates/delegate_muzh.svg'),
                 'Женский': Path('templates/delegate_zhen.svg')}
NAME_TAG_ID = 'NAME_TAG'
NAMESPACE = {'NAME_TAG': 'http://www.w3.org/2000/svg'}


def fill_svg(name: str, gender: str) -> ET:
    tree = ET.parse(CERT_TEMPLATE[gender])

    name_search_pattern = f'.//*[@id="{NAME_TAG_ID}"]/NAME_TAG:tspan'
    name_tags = tree.findall(name_search_pattern, NAMESPACE)

    if len(name_tags) == 1:
        name_tags[0].text = name

    tree.write(Path('output') / (name + '.svg'))


# Преобразовать `svg` в `pdf` или `png`
# Обернуть циклом по всем ФИО/полу
# Сохранить

if __name__ == '__main__':
    fill_svg('Ваня Ливадный', 'Мужской')
    fill_svg('Марьяна Онысько', 'Женский')
