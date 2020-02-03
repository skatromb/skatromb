# Список комментируемого редактируй внизу
import os
import re
from typing import List, Tuple


ENCODING = 'UTF-8'


# генератор files_dict = {str(i) + '.sql': ['1\n', '2\n, 3'] * (i + 1) for i in range(2)}
# Нужно закомментировать в sql-файлах substrs_to_comment
def check_and_comment(filenames: List[str], substrs_to_comment: List[str]):

    def files_existence_check():
        # Проверяем список путей на валидность
        for filename in filenames:
            if not os.path.exists(filename):
                raise Exception('Указанного пути к файлу не существует')
        return True

    def get_files_w_line_numbers_to_comment() -> dict:  # {filename: [line_number]}
        def get_line_numbers(filename: str) -> Tuple[int]:
            def line_contains_substrs(line: str) -> bool:
                # line содержит любую из substrs_to_comment?
                # TODO: Попробовать переписать в map()
                if_line_contains_substr = \
                    True in [re.search('' + substr, line) is not None  # ^((?!--).)* - вернуть
                             for substr in substrs_to_comment]
                return if_line_contains_substr

            # Вернём line_number'ы из файлов с вхождениями substrs_to_comment
            code_lines = open(filename, encoding=ENCODING).readlines()
            return tuple([line_number for line_number, code_line in enumerate(code_lines)
                          if line_contains_substrs(code_line)])

        # Возвращаем файлы с номерами строк, подлежащими закомментированию
        # Пример формата {'1.sql', ['--\n', 'CREATE TABLE...;', ..., 'END;']
        files_with_line_numbers_dict = {filename: get_line_numbers(filename)
                                        for filename in filenames}
        return files_with_line_numbers_dict

    files_existence_check()
    files_w_line_numbers_to_comment = get_files_w_line_numbers_to_comment()

    def show_lines_to_comment():
        # Показываем пользователю, какие строки в каких файлах мы собираемся закомментить
        for filename, line_numbers in files_w_line_numbers_to_comment.items():
            file = open(filename, mode='r', encoding=ENCODING)
            print('\n' + filename + ':\n')
            code_lines = file.readlines()

            for i in line_numbers:
                print(code_lines[i], end='')
            file.close()

    def comment_lines():
        show_lines_to_comment()
        if input('Комментим эти строки? (y/yes, n/no)').lower() in ('y', 'yes'):

            for filename, line_numbers in files_w_line_numbers_to_comment.items():
                file = open(filename, mode='r+', encoding=ENCODING)
                code_lines = list(str())
                for i, line in enumerate(file):
                    if not ((i in line_numbers) or (len(line.strip()) == 0 and (i - 1) in line_numbers)):
                        code_lines.append(line)
                file.close()
                file = open(filename, mode='w', encoding=ENCODING)
                # Перезаписываем файл
                file.writelines(code_lines)
                file.close()
                # list(map(lambda line: '-- ' + line, code_lines)))
            print('Ок, закомментил')
        else:
            print('Ладно, не буду комментить строки')

    comment_lines()


# Где искать
FILE_PATHS = [
    r'C:\Users\ivan.livadnyy\Documents\GitLab\teradata\MDS\Tables\DMX_CHARGE.sql',
    r'C:\Users\ivan.livadnyy\Documents\GitLab\teradata\MDS\Tables\DMX_CHARGE_ARCHIVE.sql',
    r'C:\Users\ivan.livadnyy\Documents\GitLab\teradata\MDS\Stored Procedures\LOAD_DMX_CHARGE_DATE.sql',
    r'C:\Users\ivan.livadnyy\Documents\GitLab\teradata\SQL\TFS-60980. DMX_CHARGES\2. DROP AND CREATE TABLES DMX_CHARGE, DMX_CHARGE_ARCHIVE.sql',
    r'C:\Users\ivan.livadnyy\Documents\GitLab\teradata\SQL\TFS-60980. DMX_CHARGES\3. REPLACE PROCEDURE LOAD_DMX_CHARGE_DATE.sql'

]

# Что искать
SUBSTRS_TO_COMMENT = [
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

check_and_comment(FILE_PATHS, SUBSTRS_TO_COMMENT)
