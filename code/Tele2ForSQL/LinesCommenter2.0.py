"""
def open_files(list(filename)) -> list(file)
def parse_file(file) -> list(code_line)
def parse_line(text) -> code_line

files = open_files(files)

def parse_file(file):
    lines = str.split('\n',file)
    text = []
    text = [text.append(parse_line(line)) for line in lines]
    for line in lines:
        text.append(parse_line(line))
    return  text
"""

import os
import re
from typing import List, TypedDict
from functools import reduce


# Где искать
DIR_NAME = r'C:\Users\ivan.livadnyy\Documents\GitLab\teradata\MDS\Tables'
FILE_NAMES = [
    'DMX_CHARGE.sql',
    'DMX_CHARGE_ARCHIVE.sql',
    # '3. REPLACE PROCEDURE LOAD_DMX_CHARGE_DATE.sql'
]
FILE_PATHS = [os.path.join(DIR_NAME, FILE_NAME) for FILE_NAME in FILE_NAMES]

# Что искать
SUBSTRS_TO_MATCH = [
    # Столбцы
    'SUM_CHARGE_COMPENSATION',
    'SUM_CHARGE_ADDS',
    'SUM_CHARGE_CONTRACT',
    'SUM_CHARGE_SUBS_FEE_ROUM',
    'SUM_CHARGE_RESERVE_BAL',
    'SUM_CHARGE_BAL_CHANGE_TEST_SUBS',
    'SUM_CHARGE_RECIPENT_ADD',
    'SUM_CHARGE_MC',
    'SUM_CHARGE_PACKAGE',
    'SUM_CHARGE_SILENCE_FEE',
    'SUM_CHARGE_PREMIUM_MSISDN',
    'SUM_CHARGE_PROLONG',
    'SUM_CHARGE_RETURN',
    'SUM_CHARGE_DISCOUNT',

    'CNT_SERVICE_COMPENSATIONS',
    'CNT_UNIQUE_SERVICE_COMPENSATION',

    'CNT_SERVICE_ADDS',
    'CNT_UNIQUE_SERVICE_ADDS',

    'CNT_SERVICE_CONTRACT',
    'CNT_UNIQUE_SERVICE_CONTRACT',

    'CNT_SERVICE_SUBS_FEE_ROUM',
    'CNT_UNIQUE_SERVICE_SUBS_FEE_ROUM',

    'CNT_SERVICE_RESERVE_BAL',
    'CNT_UNIQUE_SERVICE_RESERVE_BAL',

    'CNT_SERVICE_BAL_CHANGE_TEST_SUBS',
    'CNT_UNIQUE_SERVICE_BAL_CHANGE_TEST_SUBS',

    'CNT_SERVICE_RECIPENT_ADD',
    'CNT_UNIQUE_SERVICE_RECIPENT_ADD',

    'CNT_SERVICE_MC',
    'CNT_UNIQUE_SERVICE_MC',

    'CNT_SERVICE_PACKAGE',
    'CNT_UNIQUE_SERVICE_PACKAGE',

    'CNT_SERVICE_SILENCE_FEE',
    'CNT_UNIQUE_SERVICE_SILENCE_FEE',

    'CNT_SERVICE_PREMIUM_MSISDN',
    'CNT_UNIQUE_SERVICE_PREMIUM_MSISDN',

    'CNT_SERVICE_PROLONG',
    'CNT_UNIQUE_SERVICE_PROLONG',

    'CNT_SERVICE_RETURN',
    'CNT_UNIQUE_SERVICE_RETURN',
    'CNT_SERVICE_DISCOUNT',
    'CNT_UNIQUE_SERVICE_DISCOUNT',


    # Неймы
    'абонплата_2',
    'баланс_и_остатки_1',
    'баланс_и_остатки_2',
    'возврат',
    'договор',
    'доначисления',
    'компенсация',
    'скидка',
    'списания_1',
    'списания_2',
    'списания_3',
    'списания_4',
    'списания_5',
    'списания_6'
]

# Конфиг
ENCODING = 'UTF-8'
IGNORE_COMMENTED = False

class CodeWithMark(TypedDict):
    code_line: str
    is_marked: bool


def check_and_modify(file_paths: List[str], substrs_to_match: List[str], ignore_commented=False):

    def check_files_exists():
        if reduce(lambda result, file_path: (result is True or not os.path.exists(file_path)), file_paths):
            raise Exception('Указанного пути к файлу не существует')

    def mark_line(code_line: str) -> CodeWithMark:
        # example = {'CREATE TABLE COMMENT_ME': True}
        def match_code_line(substr: str) -> bool:
            regexp = code_line
            if ignore_commented:
                regexp = r'^((?!--).)*' + regexp
            return re.search(regexp, substr) is not None

        is_marked: bool = reduce(lambda result, substr_to_match: result is True or
                                 match_code_line(substr_to_match), substrs_to_match)
        return {'code_line': code_line, 'is_marked': is_marked}

    def modify_or_pass_line(code_line: CodeWithMark) -> str:
        if code_line['is_marked']:
            return '-- ' + code_line['code_line']
        else:
            return code_line['code_line']

    check_files_exists()
    print('ИЗМЕНЯЕМЫЕ СТРОКИ:')

    # Открываем файлы
    files = [open(file_path, mode='r+', encoding=ENCODING) for file_path in file_paths]
    files_code = dict()
    for file in files:
        print('\n\n' + file.name)
        # [{'-- Начало файла': False}, {'CREATE TABLE COMMENT_ME': True}, ...]
        marked_lines = [mark_line(code_line) for code_line in file.readlines()]

        # Показываем пользователю, какие строки собираемся менять
        for i, marked_line in enumerate(marked_lines):
            if marked_line['is_marked']:
                print(str(i) + '.\t' + marked_line['code_line'])

        # {'1.sql': [{'--': False}, {'CREATE TABLE COMMENT_ME': True}, ...], '2.sql': ...}
        files_code[file.name] = marked_lines

    # Если пользователь согласен, прозводим изменения
    if input('Комментим эти строки? (y/yes, n/no)').lower() not in ('y', 'yes'):
        exit()

    # Собственно, меняем строки и закрываем файлы
    for file in files:
        file.seek(0)
        marked_lines = files_code[file.name]
        file.writelines([modify_or_pass_line(line) for line in marked_lines])
        file.close()


check_and_modify(FILE_PATHS, SUBSTRS_TO_MATCH, IGNORE_COMMENTED)
