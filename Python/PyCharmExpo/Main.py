import os
import platform

print("Welcome to NHome")
menu_option = 0


def wait_and_confirm():
    # Added logic to handle the empty block and recursion
    choice = input("Would you like to continue? (y): ").lower().strip()
    if choice == "y":
        pass
    else:
        print("Invalid input.")
        wait_and_confirm()


def clear_screen():
    # Detects OS to use the correct clear command
    if platform.system() == "Windows":
        os.system("cls")
    else:
        os.system("clear")


def error_message():
    print("Please enter a valid option")


def exit_message():
    print("Exiting...")
    exit()


def temp_convertor():
    unit = input("Enter the temperature's unit (C/F/K): ").upper().strip()
    try:
        temp = float(input("Enter the temperature: "))
    except ValueError:
        error_message()
        return

    destination_unit = input("Enter the unit u want the temperature to be converted to (C/F/K): ").upper().strip()

    if unit == "C":
        if destination_unit == "F":
            result = round((temp * 9 / 5) + 32, 2)
            print(f"The temperature is {result}°F.")
        elif destination_unit == "K":
            result = round(temp + 273.15, 2)
            print(f"The temperature is {result}K.")
        else:
            error_message()
    elif unit == "F":
        if destination_unit == "C":
            result = round((temp - 32) * 5 / 9, 2)
            print(f"The temperature is {result}°C.")
        elif destination_unit == "K":
            result = round((temp - 32) * 5 / 9 + 273.15, 2)
            print(f"The temperature is {result}K.")
        else:
            error_message()
    elif unit == "K":
        if destination_unit == "C":
            result = round(temp - 273.15, 2)
            print(f"The temperature is {result}°C.")
        elif destination_unit == "F":
            result = round((temp - 273.15) * 9 / 5 + 32, 2)
            print(f"The temperature is {result}°F.")
        else:
            error_message()
    else:
        error_message()

    wait_and_confirm()


def calculator():
    try:
        count_input = input("How many numbers do you want to use? ")
        total_count = int(round(float(count_input)))

        if total_count < 2:
            print("You need at least 2 numbers to perform a calculation.")
            return

        numbers = []
        for i in range(total_count):
            val = input(f"Enter number {i + 1}: ")
            numbers.append(float(val))

        print("\nSupported operations: +, -, *, /")
        operation = input("Choose an operation: ").strip()

        result = numbers[0]
        for next_num in numbers[1:]:
            match operation:
                case "+":
                    result += next_num
                case "-":
                    result -= next_num
                case "*":
                    result *= next_num
                case "/":
                    if next_num == 0:
                        print("Error: Division by zero is not allowed.")
                        return
                    result /= next_num
                case _:
                    print("Invalid operation selected.")
                    return

        print(f"\n--- Result: {result} ---")
        wait_and_confirm()

    except ValueError:
        error_message()


def menu():
    print("\n0 => Exit")
    print("1 => Calculator")
    print("2 => Temperature Convertor")
    chosen_option = input("Choose a program: ")

    match chosen_option:
        case "0":
            exit_message()
        case "1":
            calculator()
        case "2":
            temp_convertor()
        case "" | _:
            error_message()


if __name__ == "__main__":
    while True:
        clear_screen()
        menu()