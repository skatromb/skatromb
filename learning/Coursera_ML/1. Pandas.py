import pandas

data = pandas.read_csv("titanic.csv", index_col="PassengerId")
print("1: \n" + str(data["Sex"].value_counts()), end="\n" * 3)


print(
    "2: \n"
    "{:.2f}".format(
        float(data["Survived"].sum() / data.shape[0]).__round__(4) * 100
    ),
    end="\n" * 3,
)


print(
    "3: \n"
    "{:.2f}".format(
        float(data["Pclass"].value_counts()[1] / data.shape[0]).__round__(4)
        * 100
    ),
    end="\n" * 3,
)

print(
    "4: \n" "{:.2f}".format(float(data["Age"].mean()).__round__(4)),
    end="\n" * 3,
)

print("{:.2f}".format(float(data["Age"].median()).__round__(4)), end=" ")


print(
    "5: \n"
    "{:.2f}".format(
        float(
            data["SibSp"].corr(other=data["Parch"], method="pearson")
        ).__round__(4)
    ),
    end="\n" * 3,
)


female_names = data["Name"].loc[data["Sex"] == "female"]

parenthesis_name = female_names.str.extract(r"((?<=\()\w+)")
mrs_miss_name = female_names.str.extract(r"((?<=Mrs. )\w+|(?<=Miss. )\w+)")

result_name = parenthesis_name.combine_first(mrs_miss_name)[0].value_counts()
most_popular_female_name = str(result_name.head(1).index[0])

print("6: \n" + most_popular_female_name)

# Survived
# Pclass
# Name
# Sex
# Age
# SibSp
# Parch
# Ticket
# Fare
# Cabin
# Embarked
