# fillingGapsInNumbering.py — finds all files in single folder
# with a given prefix (spam001.txt, span002.txt for example)
# and renames it to the numbering with no gaps

import os, re
from typing import TypedDict, List


class ParsedFilename(TypedDict):
    filename: str
    prefix: str
    numbering: int
    postfix: str


# TODO: find all the files with a given prefix and return list of [(filename + prefix + numbering + postfix)]
def find_and_parse_files_with_prefix(prefix: str, folder='.') -> List[ParsedFilename]:
    # TODO: обходим папку в поисках файлов с префиксом и нумерацией
    #   Формируем лист файлов внутри которого будут наименования и части наименований (лист диктов)
    for root, dirnames, filenames in os.walk(folder):
        for filename in filenames:
            continue

    # files_list = [re.search(r'((^' + prefix + r')(\d+)(\..{,4}$))', filename).groups()]

    # print(regexp.findall('spam001.txt').groups())
    return list


class OldNewFilename(TypedDict):
    new_filename_for_old: str


# TODO: order the files list by numbering and reassign a right numbering to the files
#   Return the old-new naming of the files
def files_renamer(parsed_filenames: List[ParsedFilename]) -> List[OldNewFilename]:
    return


#   TODO: Main program
files_with_prefix = find_and_parse_files_with_prefix(prefix='spam', folder='.')
# print(files_renamer(files_with_prefix))
