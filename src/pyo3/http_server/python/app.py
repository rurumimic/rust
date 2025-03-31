class Parameters:
    name: str
    count: int

    def __init__(self, name: str, count: int):
        self.name = name
        self.count = count

    def __str__(self):
        return f"Parameters(name={self.name}, count={self.count})"

def hello(name: str) -> str:
    return f"Hello, {name}!"

