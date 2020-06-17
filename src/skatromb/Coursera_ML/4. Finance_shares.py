from sklearn.decomposition import PCA
import numpy as np
import pandas as pd


dataset = pd.read_csv('close_prices.csv').iloc[:, 1:]
y = pd.read_csv('djia_index.csv')['^DJI']

# Обучаем на выборке из 10 компонентов
pca = PCA(n_components=10)
pca.fit(dataset, y)

sum_each = 0
for i, each in enumerate(pca.explained_variance_ratio_):
    sum_each += each
    if sum_each > 0.9:
        print(i + 1)
        break

# Применяем преобразование и берём первую компоненту
first_component = pca.transform(dataset)[:, 0]

# Считаем корреляцию между ней и индексом Доу-Джонса
print(f'{np.min(np.corrcoef(first_component, y)):.2f}')

# Выводим самую увесистую компанию
print(dataset.columns[np.argmax(pca.components_[0])])
