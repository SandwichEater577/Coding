"""
Append Element — Python list operations practice.

Demonstrates how to add elements to a list using various methods.
"""


def append_examples():
    """Show different ways to add elements to a Python list."""
    fruits = ["apple", "banana", "cherry"]
    print(f"Original list: {fruits}")

    # append() — add one element to the end
    fruits.append("date")
    print(f"After append('date'): {fruits}")

    # insert() — add at a specific index
    fruits.insert(1, "avocado")
    print(f"After insert(1, 'avocado'): {fruits}")

    # extend() — add multiple elements
    fruits.extend(["elderberry", "fig"])
    print(f"After extend(['elderberry', 'fig']): {fruits}")

    # += operator — same as extend
    fruits += ["grape"]
    print(f"After += ['grape']: {fruits}")

    return fruits


if __name__ == "__main__":
    result = append_examples()
    print(f"\nFinal list ({len(result)} items): {result}")
