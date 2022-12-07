#! python3
# renameDates.pu - Renames filenames with American MM-DD-YYYY date format
# to European DD-MM-YYYY.

import os
import re
import shutil

# Create a regex that matches files with the American date format.
datePattern = re.compile(
    r"""^(.*?)# 1 - all text before the date
(([01])?\d)-                       # 2, 3 - one or two digits for the month
(([0123])?\d)-                     # 4, 5 - one or two digits for the day
((19|20)\d\d)                      # 6, 7 - four digits for the year
(.*?)$                             # 8 - all text after the date
""",
    re.VERBOSE,
)

# Loop over the files in the working directory.
for amerFilename in os.listdir(".."):
    mo = datePattern.search(amerFilename)

    # Skip files without a date.
    if mo is None:
        continue

    # Get the different parts of the filename.
    beforePart = mo.group(1)
    monthPart = mo.group(2)
    dayPart = mo.group(4)
    yearPart = mo.group(6)
    afterPart = mo.group(8)

    # Form the European-style filename.
    euroFilename = beforePart + dayPart + "-" + monthPart + "-" + yearPart + afterPart

    # Get the full, absolute file paths.
    absWorkingDir = os.path.abspath("..")
    amerFilename = os.path.join(absWorkingDir, amerFilename)
    euroFilename = os.path.join(absWorkingDir, euroFilename)

    # Rename the files.
    print(f'Renaming "{amerFilename}" to "{euroFilename}"...')
    shutil.move(amerFilename, euroFilename)  # uncomment after test
