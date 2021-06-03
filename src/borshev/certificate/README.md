## Инструкция

1. В [фигме](https://www.figma.com/file/bh4KdnQy8L8ynnyFMOClgN/Certificates)
   нужно задать полю с именем и фамилией название `NAME_TAG`, 
   и сделать версии для мужского и женского имён, для английского сертификата,
   и если есть — дополнить вариантами с домашкой
   
2. Экспортировать эти фреймы в формате `svg`
   с галкой `Include "id" Attribute` и без галки `Outline text`.

3. Поместить файлы в отдельную подпапку внутри [templates](templates)
   
4. Скачать `csv`-файл с данными студентов из [metabase](https://edu-dashboard.borshev.com/question/20),
   обогатить информацией о поле студента и положить в [input/students.csv](input).
      - `course_id`
      - `user_id`
      - `full_name`
      - `gender: {'male', 'female'}`,
      - `homework: {'да', ''}` — опциональное.

5. Запустить [certificate.py](certificate.py) под локальным интерпретатором (не в докере)
