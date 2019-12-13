import os
import re
#file_lines_dict = [str(x) + '.sql'] = ['1\n', '2\n, 3'] * (x + 1) for x in range(3)
# Нужно закомментировать в @sql-файлах @строки
def lines_commenter(filenames: list, substrs_to_comment: list[str]):

    # Для этого нужно получить @файлы[@номера_строк]
    def file_lines_finder() -> dict[str: [list]]: # {filename: [line_number]}
        # @Файлы нужны все, у которых есть искомые @строки
        def

            def line_numbers(filename: str) -> list[int]:
                # Вернём line_number'ы из файлов с вхождениями substrs_to_comment
                def line_contains_substrs(line: str) -> bool:
                    # line содержит любую из substrs_to_comment?
                    return True in [re.search(substr, line) is not None
                                    for substr in substrs_to_comment]

                code_lines = open(filename, encoding='UTF-8').readlines()
                return [line_number for line_number, code_line in enumerate(code_lines)
                                if line_contains_substrs(code_line)]


                return None
        return None


    def comment_lines(filename_line_numbers: dict[]):
        return None
    # Закомментировать их


def lines_commenter(filenames: list[str], substrs_to_comment: list[str]):
    # TODO: Нужно найти в файлах @files, строки, содержащие @words_to_comment и собрать на них указатели
    #   Например, как [имя файла[номер строки]]
    for filename in filenames:
        if not os.path.exists(filename):
            raise Exception('Указанного пути к файлу не существует')
        file = open(filename)
        text_lines = file.readlines()

        # Ищем строки в тексте
        for i, text_line in enumerate(text_lines):
            for substr in substrs_to_comment:
                re.search(pattern=substr, string=text_line)

    # TODO: Показываем эти строки на утверждение закоммента
    print('Сейчас тебе закомменчу следующие строки:\n')
    for filename in filenames:
        print(filename + ':\n')
        for line in lines_to_comment:
            print(line)
    print()

    # TODO: Если пользователь согласен, комментим строки
    if input('Закомментить эти строки? (y/n)').lower() in ('y', 'yes'):


files_list = [
    r'C:\Users\ivan.livadnyy\Documents\GitLab\teradata\SQL\TFS-60980. DMX_CHARGES\1. CREATE TABLES FEE_GROUP, FEE_TYPE.sql',
    r'C:\Users\ivan.livadnyy\Documents\GitLab\teradata\SQL\TFS-60980. DMX_CHARGES\2. DROP AND CREATE TABLES DMX_CHARGE, DMX_CHARGE_ARCHIVE.sql',
    r'C:\Users\ivan.livadnyy\Documents\GitLab\teradata\SQL\TFS-60980. DMX_CHARGES\3. REPLACE PROCEDURE LOAD_DMX_CHARGE_DATE.sql'
]
words_list = [
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
    'CNT_UNIQUE_SERVICE_DISCOUNT'
]
lines_commenter(files_list, words_list)

