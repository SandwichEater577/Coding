username = input("Enter your username: ")

# Initialize validation flags
valid1 = 0 # Length check
valid2 = 0 # Space check
valid3 = 0 # Digit check

# 1. Check length: Must be 12 characters or less
if len(username) <= 12:
    valid1 = 1
else:
    print("Username is too long (max 12 characters).")

# 2. Check for spaces: .find() returns -1 if the string is NOT found
if username.find(" ") == -1:
    valid2 = 1
else:
    print("Username cannot contain spaces.")

# 3. Check for digits: .isalpha() returns True if the string is ONLY letters
# If it's all letters, then it has no digits!
if username.isalpha():
    valid3 = 1
else:
    print("Username cannot contain digits or special characters.")

# Final check: All conditions must be 1 (True)
if valid1 == 1 and valid2 == 1 and valid3 == 1:
    print(f"Success! '{username}' is a valid username.")
else:
    print("Please try again with a different username.")