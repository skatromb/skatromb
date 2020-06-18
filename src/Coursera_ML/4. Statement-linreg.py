from sklearn.feature_extraction import DictVectorizer
from sklearn.feature_extraction.text import TfidfVectorizer
from sklearn.linear_model import Ridge
from scipy.sparse import hstack
import pandas as pd


class Cols:
    FULL_DESCRIPTION = 'FullDescription'
    LOCATION_NORMALIZED = 'LocationNormalized'
    CONTRACT_TIME = 'ContractTime'
    SALARY_NORMALIZED = 'SalaryNormalized'


# Загружаем исходники текстов
train = pd.read_csv('salary-train.csv')
test = pd.read_csv('salary-test-mini.csv')

# Нормализуем тексты: приводим к нижнему регистру и заменяем всё, что не цифры и буквы на пробелы
train_dataset = train[Cols.FULL_DESCRIPTION].str.lower().replace('[^a-z0-9]', ' ', regex=True)
test_dataset = test[Cols.FULL_DESCRIPTION].str.lower().replace('[^a-z0-9]', ' ', regex=True)

# Считаем частотность слов
vectorizer = TfidfVectorizer(min_df=5)  # , max_df=1_000_000
train_vectorized = vectorizer.fit_transform(train_dataset)
test_vectorized = vectorizer.transform(test_dataset)

# Преобразуем два оставшихся столбца LocationNormalized, ContractTime
train['LocationNormalized'].fillna('nan', inplace=True)
train['ContractTime'].fillna('nan', inplace=True)

enc = DictVectorizer()
X_train_categ = enc.fit_transform(train[['LocationNormalized', 'ContractTime']].to_dict('records'))
X_test_categ = enc.transform(test[['LocationNormalized', 'ContractTime']].to_dict('records'))

# Объединяем все признаки в одну матрицу
X_train = hstack([train_vectorized, X_train_categ])
X_test = hstack([test_vectorized, X_test_categ])

y_train = train[Cols.SALARY_NORMALIZED]
y_test = train[Cols.SALARY_NORMALIZED]

# Обучаем
ridge = Ridge(alpha=1, random_state=241)
ridge.fit(X=X_train, y=y_train)

# Прогнозируем
prediction = ridge.predict(X_test)
for x in prediction:
    print(f'{x:.2f}', end=' ')
