def displayInventory(inventory):
    print("Inventory:")
    item_total = 0
    for name, amount in inventory.items():
        print(str(amount) + " " + name)
        item_total += amount
    print("Total number of items: " + str(item_total))


def addToInventory(inventory: dict, addedItems: list):
    for addedItem in addedItems:
        if addedItem in inventory:
            inventory[addedItem] += 1
        else:
            inventory[addedItem] = 1
    return inventory


inventory = {"gold coin": 42, "rope": 1}
dragonLoot = ["gold coin", "dagger", "gold coin", "gold coin", "ruby"]
inv = addToInventory(inventory, dragonLoot)
displayInventory(inventory)
