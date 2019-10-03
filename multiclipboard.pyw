#! /usr/bin/env python3
# multiclipboard.pyw - Saves and loads pieces of text to the clipboard.
# Usage: multiclipboard.pyw save <keyboard> - Saves clipboard to keyword.
#        multiclipboard.pyw <keyword> - Loads keyword to clipboard.
#        multiclipboard.pyw list - Loads all keywords to clipboard.

import shelve, pyperclip, sys


mcbFile = shelve.open('mcb')
# delete and delete <keyword> modes
if len(sys.argv) == 3 and sys.argv[1].lower() == 'delete':
    del mcbFile[sys.argv[2]]
elif len(sys.argv) == 2 and sys.argv[1].lower() == 'delete':
    # TODO: Нужно удалять из db файла все. Понять, как это сделать
    del mcbFile.
# Save clipboard content.
elif len(sys.argv) == 3 and sys.argv[1].lower() == 'save':
    mcbFile[sys.argv[2]] = pyperclip.paste()
elif len(sys.argv) == 2:
    # List keywords and load content.
    if sys.argv[1].lower() == 'list':
        pyperclip.copy(str(list(mcbFile.keys())))
    elif sys.argv[1] in mcbFile:
        pyperclip.copy(mcbFile[sys.argv[1]])

mcbFile.close()
