import time
from testbed.testbed import Person


class PyPerson:
    def __init__(self, age: int, name: str):
        self.age = age
        self.name = name

    def __call__(self, greet):
        return "{}! My name is {}. I'm {}.".format(greet, self.name, self.age)


start = time.time()
person = Person(age=10, name="Taro")
tmp = person("Hello")
print(tmp)
print(time.time() - start)


start = time.time()
person = PyPerson(age=10, name="Taro")
tmp = person("Hello")
print(tmp)
print(time.time() - start)