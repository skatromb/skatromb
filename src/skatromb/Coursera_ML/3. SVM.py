from pandas import read_csv
from sklearn.svm import SVC

C = 100_000
RANDOM_STATE = 241


dataset = read_csv('svm-data.csv', header=None)
y = dataset[0]
X = dataset.iloc[:, 1:]
clf = SVC(C=C, kernel='linear', random_state=RANDOM_STATE)
clf.fit(X=X, y=y)
print(clf.support_ + 1)
