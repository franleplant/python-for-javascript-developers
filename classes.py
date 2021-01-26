
class Animal:
    def talk(self):
        print("me talking: ...")

class Cat(Animal):
    class_attribute = "version 1"

    def __init__(self, name):
        super().__init__()
        self.name = name

    def talk(self):
        print(f"meow, im {self.name}")


cat = Cat("mr kitty")
cat.talk()
