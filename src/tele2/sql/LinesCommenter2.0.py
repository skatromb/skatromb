import os
import re
from typing import List, TypedDict
from functools import reduce


# Где искать
FILE_PATHS = [
    # r'C:\Users\ivan.livadnyy\Documents\GitLab\teradata\MDS\Tables\DMX_CHARGE.sql',
    # r'C:\Users\ivan.livadnyy\Documents\GitLab\teradata\MDS\Tables\DMX_CHARGE_ARCHIVE.sql',
    r'/Users/skatromb/PycharmProjects/Python/src/allMyCats.py'
]

# Что искать
SUBSTRS_TO_MATCH = [
    # Столбцы
    # 'SUM_CHARGE_COMPENSATION',
    # 'SUM_CHARGE_ADDS',
    # 'SUM_CHARGE_CONTRACT',
    # 'SUM_CHARGE_SUBS_FEE_ROUM',
    # 'SUM_CHARGE_RESERVE_BAL',
    # 'SUM_CHARGE_BAL_CHANGE_TEST_SUBS',
    # 'SUM_CHARGE_RECIPENT_ADD',
    # 'SUM_CHARGE_MC',
    # 'SUM_CHARGE_PACKAGE',
    # 'SUM_CHARGE_SILENCE_FEE',
    # 'SUM_CHARGE_PREMIUM_MSISDN',
    # 'SUM_CHARGE_PROLONG',
    # 'SUM_CHARGE_RETURN',
    # 'SUM_CHARGE_DISCOUNT',
    #
    # 'CNT_SERVICE_COMPENSATIONS',
    # 'CNT_UNIQUE_SERVICE_COMPENSATION',
    #
    # 'CNT_SERVICE_ADDS',
    # 'CNT_UNIQUE_SERVICE_ADDS',
    #
    # 'CNT_SERVICE_CONTRACT',
    # 'CNT_UNIQUE_SERVICE_CONTRACT',
    #
    # 'CNT_SERVICE_SUBS_FEE_ROUM',
    # 'CNT_UNIQUE_SERVICE_SUBS_FEE_ROUM',
    #
    # 'CNT_SERVICE_RESERVE_BAL',
    # 'CNT_UNIQUE_SERVICE_RESERVE_BAL',
    #
    # 'CNT_SERVICE_BAL_CHANGE_TEST_SUBS',
    # 'CNT_UNIQUE_SERVICE_BAL_CHANGE_TEST_SUBS',
    #
    # 'CNT_SERVICE_RECIPENT_ADD',
    # 'CNT_UNIQUE_SERVICE_RECIPENT_ADD',
    #
    # 'CNT_SERVICE_MC',
    # 'CNT_UNIQUE_SERVICE_MC',
    #
    # 'CNT_SERVICE_PACKAGE',
    # 'CNT_UNIQUE_SERVICE_PACKAGE',
    #
    # 'CNT_SERVICE_SILENCE_FEE',
    # 'CNT_UNIQUE_SERVICE_SILENCE_FEE',
    #
    # 'CNT_SERVICE_PREMIUM_MSISDN',
    # 'CNT_UNIQUE_SERVICE_PREMIUM_MSISDN',
    #
    # 'CNT_SERVICE_PROLONG',
    # 'CNT_UNIQUE_SERVICE_PROLONG',
    #
    # 'CNT_SERVICE_RETURN',
    # 'CNT_UNIQUE_SERVICE_RETURN',
    # 'CNT_SERVICE_DISCOUNT',
    # 'CNT_UNIQUE_SERVICE_DISCOUNT',


    # Неймы
    # 'абонплата_2',
    # 'баланс_и_остатки_1',
    # 'баланс_и_остатки_2',
    # 'возврат',
    # 'договор',
    # 'доначисления',
    # 'компенсация',
    # 'скидка',
    # 'списания_1',
    # 'списания_2',
    # 'списания_3',
    # 'списания_4',
    # 'списания_5',
    # 'списания_6'"""
    'print'
]

# Конфиг
ENCODING = 'UTF-8'
IGNORE_COMMENTED = False


class CodeWithMark(TypedDict):
    code_line: str
    is_marked: bool


LINES_WILL_BE_DICT = {'commented': 'Комментим', 'deleted': 'Удаляем'}


def check_and_modify(file_paths: List[str], substrs_to_match: List[str],
                     lines_will_be='commented', ignore_commented_lines=True):
    """ lines_will_be: 'commented' or 'deleted' """

    # Проверка на существование всех путей
    def check_files_exists():
        for file_path in file_paths:
            if not os.path.exists(file_path):
                raise Exception('Пути к файлу не существует: "' + file_path + '"')

    def mark_line(code_line: str) -> CodeWithMark:
        """ CodeWithMark() = {'CREATE TABLE COMMENT_ME': True}"""

        # Проверка, матчится ли строка
        def match_code_line(substr: str, ignore_commented=ignore_commented_lines) -> bool:
            regexp = substr
            if ignore_commented:
                regexp = r'^((?!--).)*' + regexp
            return re.search(regexp, code_line) is not None

        # Вернуть строку с меткой, сматчилась ли она
        is_marked: bool = reduce(lambda result, substr_to_match: result is True or
                                 match_code_line(substr_to_match), substrs_to_match, False)
        return {'code_line': code_line, 'is_marked': is_marked}

    # Провести изменение строки: закомментить, или удалить
    def modify_or_pass_line(code_line: CodeWithMark, action=lines_will_be) -> str:
        new_line = code_line['code_line']
        if code_line['is_marked']:
            if action == 'commented':
                new_line = '-- ' + new_line
            elif action == 'deleted':
                new_line = ''
            else:
                raise Exception('lines_will_be должен быть из ' + str(LINES_WILL_BE_DICT.keys()))
        return new_line

    # Тело check_and_modify
    #
    # Проверки
    check_files_exists()

    print('\nИЗМЕНЯЕМЫЕ СТРОКИ:')

    # Открываем файлы
    files = [open(file_path, mode='r+', encoding=ENCODING) for file_path in file_paths]
    files_code = dict()

    for file in files:
        print('\n\n' + file.name + '\n')
        # [{'-- Начало файла': False}, {'CREATE TABLE COMMENT_ME': True}, ...]
        marked_lines = [mark_line(code_line) for code_line in file.readlines()]

        # Показываем пользователю, какие строки собираемся менять
        for i, marked_line in enumerate(marked_lines):
            if marked_line['is_marked']:
                just = len(str(len(marked_lines)))
                print(str(i).rjust(just) + '.\t' + marked_line['code_line'], end='')

        # {'1.sql': [{'--': False}, {'CREATE TABLE COMMENT_ME': True}, ...], '2.sql': ...}
        files_code[file.name] = marked_lines

    # Если пользователь согласен, прозводим изменения
    if input(LINES_WILL_BE_DICT[lines_will_be] + ' эти строки? (y/yes, n/no)\n').lower() not in ('y', 'yes'):
        exit()

    # Собственно, меняем строки и закрываем файлы
    for file in files:
        file.seek(0)
        file.truncate()
        marked_lines = files_code[file.name]
        file.writelines([modify_or_pass_line(line) for line in marked_lines])
        file.close()


check_and_modify(FILE_PATHS, SUBSTRS_TO_MATCH, lines_will_be='deleted', ignore_commented_lines=False)
