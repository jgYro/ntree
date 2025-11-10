import os
import requests

class Calculator:
    def __init__(self, name):
        self.name = name
        print(f"Calculator created: {name}")

    def add(self, a, b):
        result = a + b
        self.log_operation("add", a, b, result)
        return result

    def log_operation(self, op, a, b, result):
        print(f"{self.name}: {op}({a}, {b}) = {result}")

def process_data():
    calc = Calculator("main")
    result = calc.add(5, 3)

    # External function calls
    path = os.path.join("data", "file.txt")
    response = requests.get("https://api.example.com")

    # Method calls
    data = response.json()
    return data

def standalone_function(x):
    return x * 2