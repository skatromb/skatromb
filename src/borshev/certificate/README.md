## Инструкция

1. В [фигме](https://www.figma.com/file/bh4KdnQy8L8ynnyFMOClgN/Certificate-for-teamlead?node-id=104%3A289)
   нужно задать полю с именем и фамилией название `NAME_TAG`, 
   и сделать версии для мужского и женского имён. 
   
2. Экспортировать эти фреймы в формате `svg`
   с галкой `Include "id" Attribute` и без галки `Outline text`.

3. Поместить файлы в 
   [templates/delegate_muzh.svg](templates/delegate_muzh.svg) и
   [templates/delegate_zhen.svg](templates/delegate_zhen.svg)
   
4. Скачать `csv`-файл с ФИ участников (1 столбец) и полом (2 столбец) в
   [input/FI_gender.csv](input/FI_gender.csv). Пол `{'Мужской', 'Женский'}`.

5. Запустить [certificate.py](certificate.py)
