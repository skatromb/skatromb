# largeFilesFinder.py finds files more than 1GB and prints the absolute path to them

import os


for root, dirs, files in os.walk('/'):
    for file in files:
        path = os.path.join(root, file)
        try:
            if os.path.getsize(path) > 1000 * 1000 * 1000:
                print(path)
        except FileNotFoundError:
            pass
        except PermissionError:
            pass
