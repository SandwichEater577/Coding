import sqlite3

con = sqlite3.connect("bank.db")
con.execute("PRAGMA foreign_keys = ON")
cur = con.cursor()

cur.execute("""CREATE TABLE IF NOT EXISTS users
               (
                   id      INTEGER PRIMARY KEY AUTOINCREMENT,
                   name    VARCHAR(50) NOT NULL,
                   surname VARCHAR(50) NOT NULL,
                   email   VARCHAR(50) NOT NULL UNIQUE,
                   phone   VARCHAR(12) NOT NULL
               )""")

cur.execute("""CREATE TABLE IF NOT EXISTS accounts
               (
                   id      INTEGER PRIMARY KEY AUTOINCREMENT,
                   amount  DECIMAL(20, 2) NOT NULL,
                   user_id INTEGER        NOT NULL,
                   FOREIGN KEY (user_id) REFERENCES users (id)
               )
            """)

cur.execute("""CREATE TABLE IF NOT EXISTS transactions
               (
                   id          INTEGER PRIMARY KEY AUTOINCREMENT,
                   amount      DECIMAL(10, 2) NOT NULL,
                   account_from_id INTEGER        NOT NULL,
                   account_to_id   INTEGER        NOT NULL,
                   date        VARCHAR(20)    NOT NULL,
                   FOREIGN KEY (account_from_id) REFERENCES accounts (id),
                   FOREIGN KEY (account_to_id) REFERENCES accounts (id),
                   CHECK (account_from_id != account_to_id)
               )
            """)

cur.execute("""CREATE TABLE IF NOT EXISTS credentials
               (
                   id       INTEGER PRIMARY KEY AUTOINCREMENT,
                   login    VARCHAR(50)  NOT NULL UNIQUE,
                   password VARCHAR(150) NOT NULL,
                   user_id  INTEGER      NOT NULL,
                   FOREIGN KEY (user_id) REFERENCES users (id)
               )
            """)

program_is_finished = False
should_continue = False

def check_if_user_exists(login, password):
    user = cur.execute(
        """SELECT 1 FROM credentials WHERE login = ? AND password = ?""",
        (login, password),
    ).fetchone()
    return user is not None

def add_user(name, surname, email, phone, login, password):
    with con:
        cur.execute(
            """INSERT INTO users(name, surname, email, phone)
               VALUES (?, ?, ?, ?)""",
            (name, surname, email, phone),
        )
        user_id = cur.lastrowid
        cur.execute(
            """INSERT INTO credentials(login, password, user_id)
               VALUES (?, ?, ?)""",
            (login, password, user_id),
        )

def register_user():
    name = input("Enter your Name: ")
    surname = input("Enter your Surname: ")
    email = input("Enter your E-Mail: ")
    phone = input("Enter your Phone Number: ")
    login = input("Enter your Login: ")
    password = input("Enter your Password: ")
    if not all((name, surname, email, phone, login, password)):
        print("All fields are required.")
        return

    try:
        add_user(name, surname, email, phone, login, password)
    except sqlite3.IntegrityError:
        print("That email or login is already registered.")
    else:
        print("User registered successfully!")
        input("Press Enter to continue...")

def login_user():
    global should_continue
    input_login = input("Enter your Login: ")
    input_password = input("Enter your Password: ")
    if check_if_user_exists(input_login, input_password):
        print("User logged in successfully!")
        should_continue = True
        input("Press Enter to continue...")
    else:
        print("Invalid login or password.")

def exit_program(a=0):
    global should_continue, program_is_finished
    if a == 1:
        print("Logging out...")
        should_continue = False
        program_is_finished = False
    else:
        print("Logging out and exiting program...")
        should_continue = False
        program_is_finished = True

def check_balance():
    # arguments: accounts[id]
    # returns: account[amount] where account[id] = accounts[id]
    ...

while not program_is_finished:
    if not should_continue:
        print("1. Log in")
        print("2. Sign up")
        print("3. Exit Program")
        user_input = input("Enter number for program execution: ")
        match user_input:
            case "1":
                login_user()
            case "2":
                register_user()
            case "3":
                exit_program()
            case _:
                print("Please enter a correct option")
    else:
        print("1. Check balance")
        print("2. Receive transaction")
        print("3. Send transaction")
        print("4. Log out & Exit Program")
        print("5. Log out & Return to main menu")
        user_input = input("Enter number for program execution: ")
        match user_input:
            case "1":
                check_balance()
            case "2":
                # receive_transaction()
                # give instruction to make receiving transactions easier
                # give the user theyre account id to give to the sender
                pass
            case "3":
                # send_transaction()
                # give instruction to make sending transactions easier
                # give the user instructions to ask the receiver for their account id
                # ask the user for the account id of the receiver
                pass
            case "4":
                exit_program()
            case "5":
                exit_program(1)
            case _:
                print("Please enter a correct option")














