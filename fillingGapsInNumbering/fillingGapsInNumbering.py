# fillingGapsInNumbering.py — finds all files in single folder
# with a given prefix (spam001.txt, span002.txt for example)
# and renames it to the numbering with no gaps

import os, re
from typing import TypedDict, List


class ParsedFilenameDict(TypedDict):
    filename: str
    prefix: str
    numbering: str
    postfix: str


# Find all the files with a given prefix and return list of [(filename + prefix + numbering + postfix)]
def find_and_parse_files_with_prefix(prefix: str, folder='.') -> List[ParsedFilenameDict]:
    FILENAME, PREFIX, NUMBERING, POSTFIX = 0, 1, 2, 3
    # DONE: обходим папку в поисках файлов с префиксом и нумерацией
    #   Формируем лист файлов внутри которого будут наименования и части наименований (лист диктов)
    parsed_filenames: List[ParsedFilenameDict] = list()
    for root, dirnames, filenames in os.walk(folder):
        for filename in filenames:
            matches = re.search(r'(^' + prefix + r')(\d+)(\..{,4}$)', filename)
            if matches:
                parsed_filenames.append({'filename': matches.group(FILENAME), 'prefix': matches.group(PREFIX),
                                         'numbering': matches.group(NUMBERING), 'postfix': matches.group(POSTFIX)})
    return parsed_filenames


class OldNewFilename(TypedDict):
    old_filename: str
    new_filename: str


def files_renamer(parsed_filenames: List[ParsedFilenameDict]) -> List[OldNewFilename]:
    # Order the files list by numbering
    parsed_filenames.sort(key=lambda filename: filename['numbering'])
    old_new_filenames: List[OldNewFilename] = list()
    longest_num_len = len(
        max(parsed_filenames, key=lambda filename: len(filename['numbering']))['numbering'])

    # Map a right numbering for the files
    for parsed_filename, i in zip(parsed_filenames, range(1, len(parsed_filenames))):
        new_numbering_str = str(i).rjust(longest_num_len, '0')
        new_filename = parsed_filename['prefix'] + new_numbering_str + parsed_filename['postfix']
        old_new_filenames.append({'old_filename': parsed_filename['filename'], 'new_filename': new_filename})
    # TODO: rename itself!
    # TODO: Return the old-new naming of the files
    return old_new_filenames


#  Main program
parsed_filenames = find_and_parse_files_with_prefix(prefix='spam', folder='.')
print(files_renamer(parsed_filenames))