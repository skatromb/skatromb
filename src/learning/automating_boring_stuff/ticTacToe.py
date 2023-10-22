board = {
    "top-L": " ",
    "top-M": " ",
    "top-R": " ",
    "mid-L": " ",
    "mid-M": " ",
    "mid-R": " ",
    "low-L": " ",
    "low-M": " ",
    "low-R": " ",
}


def printBoard(board):
    print(board["top-L"] + "|" + board["top-M"] + "|" + board["top-R"])
    print("-+—+—")
    print(board["mid-L"] + "|" + board["mid-M"] + "|" + board["mid-R"])
    print("-+—+—")
    print(board["low-L"] + "|" + board["low-M"] + "|" + board["low-R"])


turn = "X"
for i in range(9):
    printBoard(board)
    move = None
    while True:
        move = input("Turn for " + turn + ". Move on which space?")
        # Проверка, что имя клетки введено верно
        if move not in board:
            print("Cell name is incorrect")
            continue
        # Проверка на то, что клетка пустая
        if board[move] != " ":
            print("You must choose the new cell")
            continue
        board[move] = turn
        break
    # Меняем ход на другого игрока
    if turn == "X":
        turn = "O"
    else:
        turn = "X"
printBoard(board)
