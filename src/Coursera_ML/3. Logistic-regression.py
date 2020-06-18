from sklearn.metrics import roc_auc_score
from pandas import read_csv
import numpy as np

MAX_ITER = 10_000
EPSILON = 10**-5,


def grad_step(X:np.ndarray, y:np.ndarray, w1: float, w2: float, C: float, k: float) -> tuple:
    """Осуществляет один шаг градентного спуска"""

    x1, x2 = X[:, 0], X[:, 1]

    w1_new = w1 + k*np.mean(
        y*x1*(1 - 1 /
              (1 + np.exp(-y*(w1*x1 + w2*x2)))
              )
    ) - k*C*w1

    w2_new = w2 + k*np.mean(
        y*x2*(1 - 1 /
              (1 + np.exp(-y*(w1*x1 + w2*x2)))
              )
    ) - k*C*w2
    return w1_new, w2_new


def predict(X:np.ndarray, w1: float, w2: float) -> list:
    """Классифицирует вектор"""

    predictions = [(1 / (1 + np.exp(-w1*x1 - w2*x2))) for x1, x2 in X]
    return predictions


def log_regression_score(X:np.ndarray, y:np.ndarray, C=0, k=0.1) -> float:
    """Вычисляет AUC-ROC для логистической регрессии с и без регуляризации"""

    w1, w2 = 0.0, 0.0

    # Градентный спуск
    for i in range(MAX_ITER):
        w1_new, w2_new = grad_step(X, y, w1, w2, C, k)
        distance = np.sqrt(np.square(w1_new - w1) + np.square(w2_new - w2))
        w1, w2 = w1_new, w2_new

        # Останавливаемся, если точность достигнута
        if distance < EPSILON:
            print('Остановились на ' + str(i + 1) + ' шаге')
            print('distance = ' + str(distance))
            break

    # Вычисляем вектор предсказаний и AUC-ROC
    prediction = predict(X, w1, w2)

    return roc_auc_score(y, prediction)


dataset = np.genfromtxt('data-logistic.csv', delimiter=',')
y = dataset[:, 0]
X = dataset[:, 1:3]

without_regularization = log_regression_score(X, y, C=0)
with_regularization = log_regression_score(X, y, C=10)
print(f'Without regularization ROC-AUC score {without_regularization:.3f}')
print(f'With regularization ROC-AUC score {with_regularization:.3f}')
