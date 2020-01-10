import os
import re
from typing import Tuple


# Где искать
FILE_PATHS = (
    r'/Users/skatromb/PycharmProjects/Python/code/allMyCats.py',
)

# Что искать
SUBSTRS_TO_MATCH = (
    'print',
)

# Конфиг
ENCODING = 'UTF-8'
IGNORE_COMMENTED = False
WHAT_TO_DO_DICT = {'commented': 'Комментим', 'deleted': 'Удаляем'}


def modify_files(file_paths: Tuple[str], substrs_to_match: Tuple[str],
                 lines_will_be='commented', ignore_commented=True):
    # lines_will_be: 'commented' or 'deleted'

    # Проверка на существование всех путей
    def check_files_exists():
        for file_path in file_paths:
            if not os.path.exists(file_path):
                raise Exception('Пути к файлу не существует: "' + file_path + '"')

    def match(code_line: str) -> bool:
        # example = {code_line: 'CREATE TABLE COMMENT_ME', is_marked: True}
        # Проверить, матчится ли строка
        def match_line_with_each(substr: str) -> bool:
            regexp = substr
            if ignore_commented:
                regexp = r'^((?!--).)*' + regexp
            line_matches_substr = re.search(regexp, code_line) is not None
            return line_matches_substr

        # Вернуть строку с меткой, сматчилась ли она
        line_matches_what_we_search = True in [match_line_with_each(substr) for substr in substrs_to_match]
        return line_matches_what_we_search

    # Провести изменение строки: закомментить, или удалить
    def modify(code_line: str) -> str:
        new_line = code_line
        if lines_will_be == 'commented':
            new_line = '-- ' + new_line
        elif lines_will_be == 'deleted':
            new_line = ''
        else:
            raise Exception('lines_will_be должен быть из ' + str(WHAT_TO_DO_DICT.keys()))
        return new_line

    # Проверки
    check_files_exists()

    print('\nИЗМЕНЯЕМЫЕ СТРОКИ:')

    # Открываем файлы
    files = [open(file_path, mode='r+', encoding=ENCODING) for file_path in file_paths]

    # Просматриваем каждый файл
    files_code = {}
    for file in files:
        # TODO: Засунуть проход по файлам в отдельную функцию
        print('\n\n' + file.name + '\n')

        # Ищем и собираем строки, которые будем заменять
        modified_lines, lines_for_print = [], []

        for line_number, code_line in enumerate(file):
            if match(code_line):
                lines_for_print.append({'number': str(line_number + 1), 'code': code_line})
                modified_lines.append(modify(code_line))
            else:
                modified_lines.append(code_line)
            just = len(str(line_number + 1)) + 1  # Вычисляем отступ для красивого форматирования вывода строк

        # Выводим строки пользователю
        for line_for_print in lines_for_print:
            print(line_for_print['number'].rjust(just) + ': ' + line_for_print['code'], end='')

        files_code[file.name] = modified_lines

    # Если пользователь согласен, прозводим изменения
    if input(WHAT_TO_DO_DICT[lines_will_be] + ' эти строки? (y/yes, n/no)\n').lower() not in ('y', 'yes'):
        exit()
    else:

        # Собственно, меняем строки и закрываем файлы
        for file in files:
            file.seek(0)
            file.truncate()
            file.writelines(files_code[file.name])
            file.close()


modify_files(FILE_PATHS, SUBSTRS_TO_MATCH, lines_will_be='deleted', ignore_commented=False)
