import pandas
from numpy import linspace
from sklearn.datasets import load_boston
from sklearn.model_selection import KFold, cross_val_score
from sklearn.neighbors import KNeighborsRegressor
from sklearn.preprocessing import scale

dataset = load_boston()
scaled_dataset = scale(dataset.data)
target = dataset.target

k_fold = KFold(n_splits=5, shuffle=True, random_state=42)
scores = dict()
p = linspace(start=1, stop=10, num=200)

for i in p:
    regressor = KNeighborsRegressor(n_neighbors=5, weights="distance", p=i)
    scores[i] = cross_val_score(
        estimator=regressor,
        X=scaled_dataset,
        y=target,
        scoring="neg_mean_squared_error",
        cv=k_fold,
        n_jobs=4,
    ).mean()
    print(str(i) + ": " + str(scores[i]))

scores_frame = pandas.Series(scores).sort_values(ascending=False)
print("\n")
print(scores_frame.head(1).round(2))
