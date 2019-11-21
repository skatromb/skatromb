# fillingGapsInNumbering.py — finds all files in single folder
# with a given prefix (spam001.txt, span002.txt for example)
# and renames it to the numbering with no gaps

import os, re
from typing import TypedDict, List


class ParsedFilenameDict(TypedDict):
    filename: str
    prefix: str
    numbering: int
    postfix: str
    sort

class ParsedFilenameDicts(List):



# TODO: find all the files with a given prefix and return list of [(filename + prefix + numbering + postfix)]
def find_and_parse_files_with_prefix(prefix: str, folder='.') -> List[ParsedFilenameDict]:
    FILENAME, PREFIX, NUMBERING, POSTFIX = 0, 1, 2, 3
    # DONE: обходим папку в поисках файлов с префиксом и нумерацией
    #   Формируем лист файлов внутри которого будут наименования и части наименований (лист диктов)
    parsed_filenames = List[ParsedFilenameDict]
    for root, dirnames, filenames in os.walk(folder):
        for filename in filenames:
            matches = re.search(r'((^' + prefix + r')(\d+)(\..{,4}$))', filename)
            if matches:
                parsed_filenames.append({'filename': matches.group(FILENAME), 'prefix': matches.group(PREFIX),
                                         'numbering': matches.group(NUMBERING), 'postfix': matches.group(POSTFIX)})

    return parsed_filenames


# TODO: order the files list by numbering and reassign a right numbering to the files
#   Return the old-new naming of the files
class OldNewFilename(TypedDict):
    old_filename: str
    new_filename: str


def files_renamer(parsed_filenames: List[ParsedFilenameDict]) -> List[OldNewFilename]:
    # Эта штука сортирует лист диктов по значению дикта 'numbering'
    sorted(parsed_filenames, key=lambda filename: filename['numbering'])
    return


#   TODO: Main program
files_with_prefix = find_and_parse_files_with_prefix(prefix='spam', folder='.')
# print(files_renamer(files_with_prefix))
