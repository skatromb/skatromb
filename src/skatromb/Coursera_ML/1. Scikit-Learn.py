import pandas
from sklearn.tree import DecisionTreeClassifier


# Импортируем данные и оставляем то, что нужно
data = pandas.read_csv('titanic.csv')[['Pclass', 'Fare', 'Age', 'Sex', 'Survived']].dropna(axis=0)
dataset = data[['Pclass', 'Fare', 'Age', 'Sex']]

target = data['Survived']

# В столбце пол заменяем строки на числа
sex_dict = {'male': 1, 'female': 0}
dataset['Sex'].replace(to_replace=sex_dict, inplace=True)

# Обучаем дерево
clf = DecisionTreeClassifier(random_state=241)
clf.fit(dataset, target)
