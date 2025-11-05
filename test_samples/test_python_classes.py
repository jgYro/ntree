class Calculator:
    def __init__(self, name):
        self.name = name

    def add(self, a, b):
        return a + b

    def __str__(self):
        return f"Calculator: {self.name}"

class AdvancedCalculator(Calculator):
    def __new__(cls, name):
        return super().__new__(cls)

    def __init__(self, name):
        super().__init__(name)
        self.history = []

    def multiply(self, a, b):
        result = a * b
        self.history.append(f"{a} * {b} = {result}")
        return result

def standalone_function():
    return "I'm not in a class"