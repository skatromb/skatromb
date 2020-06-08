from sklearn.metrics import roc_auc_score
from pandas import read_csv
import numpy as np

MAX_ITER = 10_000


def sigmoid():

def grad_step(X, y, w, epsilon=10 ** -5, k=0.1, C=0):
    w1, w2 = w
    l = len(w)

    w1_new = w1 + k/l * np.sum(
        
    )
    w2_new = 
    return w1_new, w2_new


dataset = np.genfromtxt('/workspaces/Python/src/skatromb/Coursera_ML/3. data-logistic.csv', delimiter=',')
X = dataset[:, 0]
y = dataset[:, 1:]

for i in range(MAX_ITER):

