## Инструкция

1. В [фигме](https://www.figma.com/file/bh4KdnQy8L8ynnyFMOClgN/Certificates)
   нужно задать полю с именем и фамилией название `NAME_TAG`, 
   и сделать версии для мужского и женского имён. 
   
2. Экспортировать эти фреймы в формате `svg`
   с галкой `Include "id" Attribute` и без галки `Outline text`.

3. Поместить файлы в отдельную подпапку внутри [templates](templates) согласно [README](templates/README.md)
   
4. Скачать `csv`-файл с ФИ участников и полом участников из [typeform.com](https://typeform.com) в
   [input/students.csv](input). 
      - `full_name: str`
      - `gender: {'male', 'female'}`,
      - `homework: {'да', ''}` — опциональное.

5. Запустить [certificate.py](certificate.py) под локальным интерпретатором (не в докере)
