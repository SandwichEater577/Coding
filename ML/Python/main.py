lst = [
    [12, 2, 3, 4],
    [12, 23, 45, 65],
    [12456],
]

# we want to print out the list with the shortest ammount of indexes

print(min(lst, key=len))