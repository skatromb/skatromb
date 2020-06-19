from sklearn.ensemble import RandomForestRegressor
from sklearn.model_selection import cross_val_score, KFold
import pandas as pd


dataset = pd.read_csv('abalone.csv')

# Кодируем пол числовыми значениями
dataset['Sex'] = dataset['Sex'].map({'F': -1, 'M': 1}).fillna(0)
y = dataset['Rings']
del dataset['Rings']
X = dataset

k_fold = KFold(n_splits=5, shuffle=True, random_state=1)

for i in range(1, 51):
    clf = RandomForestRegressor(n_estimators=i, random_state=1, n_jobs=4)
    clf.fit(X, y)
    score = cross_val_score(clf, X, y, scoring='r2', cv=k_fold).mean()
    print(f'{i}: {score:.4f}')
    if score > 0.52:
        break
print('Прибавь ещё +1')
