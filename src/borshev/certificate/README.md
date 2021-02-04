1. В [фигме](https://www.figma.com/file/bh4KdnQy8L8ynnyFMOClgN/Certificate-for-teamlead?node-id=104%3A289)
нужно задать полю с именем и фамилией название `NAME_TAG`, затем экспортировать весь фрейм в `svg`
с галкой `Include "id" Attribute` и без галки `Outline text`.

2. Поместить файл в [templates/delegate.svg](templates/delegate.svg)

3. Поместить цсвшник с ФИ участников (1 столбец) и полом (2 столбец) в
   [templates/FI_gender.csv](templates/FI_gender.csv). Пол `{'Мужской', 'Женский'}`.

4. Запустить [certificate.py](certificate.py)
