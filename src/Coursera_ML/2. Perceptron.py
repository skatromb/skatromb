from sklearn.preprocessing import StandardScaler
from sklearn.linear_model import Perceptron
from pandas import read_csv


scaler = StandardScaler()
clf = Perceptron()

# Загружаем данные
test_data, train_data = read_csv('perceptron-test.csv', header=None), read_csv('perceptron-train.csv', header=None)

test_target, train_target = test_data[0], train_data[0]
test_attrs, train_attrs = test_data.iloc[:, 1:], train_data.iloc[:, 1:]

# Учим персептрон
clf.fit(train_attrs, train_target)
# unscaled_predictions = clf.predict(test_attrs)

unscaled_score = clf.score(test_attrs, test_target)
print(unscaled_score)


# Нормируем данные
train_attrs = scaler.fit_transform(train_attrs)
test_attrs = scaler.transform(test_attrs)

# Снова учим персептрон
clf.fit(train_attrs, train_target)
# scaled_predictions = clf.predict(test_attrs)

scaled_score = clf.score(test_attrs, test_target)
print(scaled_score)

print(scaled_score - unscaled_score)
