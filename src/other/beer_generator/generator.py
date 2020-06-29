from time import time


start_time = time()

beer_data = "recipeData.csv"
lines = (line for line in open(beer_data, encoding="ISO-8859-1"))
lists = (l.split(",") for l in lines)

# Take the column names out of the generator and store them, leaving only data
columns = next(lists)

# Take these columns and use them to create an informative dictionary
beerdicts = (dict(zip(columns, data)) for data in lists)

beer_counts = {}
for bd in beerdicts:
    if bd["Style"] not in beer_counts:
        beer_counts[bd["Style"]] = 1
    else:
        beer_counts[bd["Style"]] += 1

most_popular = 0
most_popular_type = None
for beer, count in beer_counts.items():
    if count > most_popular:
        most_popular = count
        most_popular_type = beer

print(most_popular_type)


end_time = time()
print(f'{end_time - start_time:.3f}')
