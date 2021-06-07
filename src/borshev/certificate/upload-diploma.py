import os
import os.path
import string

import httpx
from tqdm import tqdm


DIPLOMA_FOLDER = 'output'
TOKEN = ''


def detect_diploma_language(filename):
    filename = os.path.splitext(filename)[0].strip()

    return 'EN' if any(letter in filename for letter in string.ascii_letters) else 'RU'


def main():
    for dirpath, _, filenames in tqdm(os.walk(DIPLOMA_FOLDER)):

        dirpath = dirpath.split('/')
        if len(dirpath) != 3:
            continue

        [course_id, student_id] = dirpath[-2:]

        for filename in filenames:
            language = detect_diploma_language(filename)
            with open(f'{DIPLOMA_FOLDER}/{course_id}/{student_id}/{filename}', 'rb') as image:
                response = httpx.post(
                    url='https://education.borshev.com/api/v2/diplomas/',
                    data={
                        'student': student_id,
                        'course': course_id,
                        'language': language,
                    },
                    files={'image': image},
                    headers={
                        f'Authorization': 'Token {TOKEN}',
                    },
                    timeout=20,
                )
                assert response.status_code == 201, [course_id, student_id, filename, language, response.content]


if __name__ == '__main__':
    main()
