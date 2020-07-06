# selectiveCopy.py - walks through a folder tree and searches for files
# with a .py file extension and copies them to a new folder

import os
import shutil
import re

NEW_FOLDER = 'New folder'


# Function that receives folder to search, folder to copy and file extension to copy
def extension_copy(folder: str, destination_folder=NEW_FOLDER, extension=None):
    folder = os.path.abspath(folder)
    if not os.path.exists(folder):
        print('The folder "' + folder + '" doesn''t exists')
        exit

    if not os.path.isabs(destination_folder):
        destination_folder == os.path.abspath(destination_folder)

    # Create a folder
    if not os.path.exists(destination_folder):
        os.mkdir(destination_folder)

    # Go through all the sub-folders of folder
    for root, directories, filenames in os.walk(folder):
        for filename in filenames:
            # Match the extension, copy
            if filename.endswith(extension) and not os.path.exists(os.path.join(destination_folder, filename)):
                shutil.copy(os.path.join(root, filename), destination_folder)


extension_copy('.', '/Users/python_skatromb/Documents/Учеба/Python/PycharmProjects/python_skatromb/Python/Python files', '.py')
