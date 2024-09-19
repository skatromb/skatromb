with open("input.txt") as in_file:
    n = int(in_file.readline())
    count = 0
    max_count = 0
    for char in in_file:
        if char.strip() == "1":
            count += 1
            print(count)
            continue
        else:
            if count > max_count:
                max_count = count
            count = 0
    if count > max_count:
        max_count = count

print("result:", max_count)
