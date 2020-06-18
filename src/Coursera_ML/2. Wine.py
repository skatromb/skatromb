import pandas
from sklearn.model_selection import KFold, cross_val_score
from sklearn.preprocessing import scale
from sklearn.neighbors import KNeighborsClassifier


data = pandas.read_csv('wine.csv')

classes = data['Class']
attributes = data.iloc[:, 1:]
attributes = pandas.DataFrame(data=scale(attributes), columns=attributes.columns)
scores = dict()

k_fold = KFold(n_splits=5, shuffle=True, random_state=42)
for k in range(1, 51):
    classifier = KNeighborsClassifier(n_neighbors=k, n_jobs=4)
    scores[k] = cross_val_score(
        estimator=classifier, X=attributes, y=classes, cv=k_fold, scoring='accuracy', n_jobs=4).mean()
    print(str(k) + ': ' + str(scores[k]))

scores_frame = pandas.Series(scores).sort_values(ascending=False)
print(scores_frame.head(1).round(2))
