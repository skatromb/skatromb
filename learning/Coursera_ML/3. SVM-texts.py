import numpy as np
from sklearn import datasets
from sklearn.feature_extraction.text import TfidfVectorizer
from sklearn.model_selection import GridSearchCV, KFold
from sklearn.svm import SVC

# Загружаем данные
newsgroups = datasets.fetch_20newsgroups(
    subset="all", categories=["alt.atheism", "sci.space"]
)
vectorizer = TfidfVectorizer()

# Векторизируем слова в числа
X = TfidfVectorizer.fit_transform(vectorizer, newsgroups.data)
y = newsgroups.target

grid = {
    "C": np.power(10.0, np.arange(-5, 6))
}  # Задаём константу C, которую хотим оптимизировать
cross_val = KFold(
    n_splits=5, shuffle=True, random_state=241
)  # Настраиваем кросс-валидацию, разбивая на 5 блоков
clf = SVC(kernel="linear", random_state=241)  # Классификатор линейный
gs = GridSearchCV(
    clf, grid, scoring="accuracy", cv=cross_val
)  # Подбор параметров с помощью грида (?)

# Ищем оптимальный C
gs.fit(X, y)
print(gs.best_params_)

# Обучаем с оптимальным C
clf = SVC(kernel="linear", random_state=241, **gs.best_params_)
clf.fit(X, y)
top10 = np.argsort(np.abs(np.asarray(clf.coef_.todense())).reshape(-1))[-10:]

# Смотрим, что за слова у нас получились в топе
feature_mapping = vectorizer.get_feature_names()
top10_words = [feature_mapping[i] for i in top10]
top10_words.sort()

print(top10_words)
