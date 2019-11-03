# fillingGapsInNumbering.py — finds all files in single folder
# with a given prefix (spam001.txt, span002.txt for example)
# and renames it to the numbering with no gaps

import os, re, typing
listStrType = typing.List[str]
DictStrType = typing.
listDictType = typing.List[dict[]]


# TODO: find all the files with a given prefix and return list of this files
def find_files_with_prefix(prefix:str, folder='.') -> listStrType:
    regexp = re.compile(r'(^' + prefix + r')(\d+)(\..{,4}$)')
    print(regexp.search('spam001.txt').groups())
    return list


# TODO: parse the list of files and return a fullname + prefix + numbering + postfix
def filename_parser(filenames:list[str]) -> listDictType: # dict: {'fullname', 'prefix', 'numbering', 'postfix'}
    return


# TODO: order the files list by numbering and reassign a right numbering to the files
#   Return the old-new naming of the files
def files_renamer(parsed_filenames:list[dict]) -> listDictType: # same type as filename_parser
    return



files_with_prefix = find_files_with_prefix(prefix='spam', folder='.')
parsed_filenames = filename_parser(files_with_prefix)
print(files_renamer(parsed_filenames))
