import numpy as np
import pandas as pd
from sklearn.metrics import (
    accuracy_score,
    f1_score,
    precision_recall_curve,
    precision_score,
    recall_score,
    roc_auc_score,
)

dataset = np.genfromtxt("classification.csv", delimiter=",", skip_header=True)
target, y = dataset.T

# Считаем метрики ошибок классификации
TP = np.sum((target == 1) & (y == 1))
TN = np.sum((target == 0) & (y == 0))
FP = np.sum((target == 0) & (y == 1))
FN = np.sum((target == 1) & (y == 0))
print(TP, FP, FN, TN)

# Считаем метрики классификатора
print(
    f"{accuracy_score(target, y):.2f} {precision_score(target, y):.2f} {recall_score(target, y):.2f} {f1_score(target, y):.2f}"
)

dataset = pd.read_csv("scores.csv")

y_true = dataset["true"]
scores = dataset[["score_logreg", "score_svm", "score_knn", "score_tree"]]

roc_auc_scores = [roc_auc_score(y_true, scores[column]) for column in scores]
print(" ".join(f"{value:.2f}" for value in roc_auc_scores))

# Исследуем точность и полноту классификаторов
print()
print("Анализ точности классификаторов:")
for score_name in scores:
    precision, recall, threshold = precision_recall_curve(y_true, scores[score_name])
    print(score_name, ": ", f"{np.max(precision[np.where(recall > 0.7)]):.2f}")
