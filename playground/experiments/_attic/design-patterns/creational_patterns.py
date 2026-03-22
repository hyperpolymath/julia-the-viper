"""
Creational Design Patterns

Patterns that deal with object creation mechanisms.
"""

from abc import ABC, abstractmethod
from typing import Dict, Any, Optional
import copy


# SINGLETON PATTERN

class SingletonMeta(type):
    """
    Metaclass for Singleton pattern.
    Thread-safe singleton implementation.
    """
    _instances = {}

    def __call__(cls, *args, **kwargs):
        if cls not in cls._instances:
            cls._instances[cls] = super().__call__(*args, **kwargs)
        return cls._instances[cls]


class DatabaseConnection(metaclass=SingletonMeta):
    """
    Example: Database connection using Singleton.
    Ensures only one connection instance exists.
    """

    def __init__(self):
        self.connection_string = None
        self.connected = False

    def connect(self, connection_string: str):
        if not self.connected:
            self.connection_string = connection_string
            self.connected = True
            print(f"Connected to: {connection_string}")

    def disconnect(self):
        if self.connected:
            self.connected = False
            print("Disconnected")


# FACTORY PATTERN

class Animal(ABC):
    """Abstract product."""

    @abstractmethod
    def speak(self) -> str:
        pass


class Dog(Animal):
    def speak(self) -> str:
        return "Woof!"


class Cat(Animal):
    def speak(self) -> str:
        return "Meow!"


class Bird(Animal):
    def speak(self) -> str:
        return "Tweet!"


class AnimalFactory:
    """
    Factory Pattern: Creates objects without specifying exact class.
    """

    @staticmethod
    def create_animal(animal_type: str) -> Animal:
        animals = {
            'dog': Dog,
            'cat': Cat,
            'bird': Bird
        }

        animal_class = animals.get(animal_type.lower())
        if not animal_class:
            raise ValueError(f"Unknown animal type: {animal_type}")

        return animal_class()


# ABSTRACT FACTORY PATTERN

class Button(ABC):
    @abstractmethod
    def render(self) -> str:
        pass


class Checkbox(ABC):
    @abstractmethod
    def render(self) -> str:
        pass


class WindowsButton(Button):
    def render(self) -> str:
        return "Rendering Windows button"


class WindowsCheckbox(Checkbox):
    def render(self) -> str:
        return "Rendering Windows checkbox"


class MacButton(Button):
    def render(self) -> str:
        return "Rendering Mac button"


class MacCheckbox(Checkbox):
    def render(self) -> str:
        return "Rendering Mac checkbox"


class GUIFactory(ABC):
    """Abstract Factory for creating families of related objects."""

    @abstractmethod
    def create_button(self) -> Button:
        pass

    @abstractmethod
    def create_checkbox(self) -> Checkbox:
        pass


class WindowsFactory(GUIFactory):
    def create_button(self) -> Button:
        return WindowsButton()

    def create_checkbox(self) -> Checkbox:
        return WindowsCheckbox()


class MacFactory(GUIFactory):
    def create_button(self) -> Button:
        return MacButton()

    def create_checkbox(self) -> Checkbox:
        return MacCheckbox()


# BUILDER PATTERN

class Computer:
    """Complex object to be built."""

    def __init__(self):
        self.cpu = None
        self.ram = None
        self.storage = None
        self.gpu = None
        self.os = None

    def __str__(self):
        return (f"Computer(CPU: {self.cpu}, RAM: {self.ram}, "
                f"Storage: {self.storage}, GPU: {self.gpu}, OS: {self.os})")


class ComputerBuilder:
    """
    Builder Pattern: Constructs complex objects step by step.
    Allows different representations using same construction process.
    """

    def __init__(self):
        self.computer = Computer()

    def set_cpu(self, cpu: str) -> 'ComputerBuilder':
        self.computer.cpu = cpu
        return self

    def set_ram(self, ram: str) -> 'ComputerBuilder':
        self.computer.ram = ram
        return self

    def set_storage(self, storage: str) -> 'ComputerBuilder':
        self.computer.storage = storage
        return self

    def set_gpu(self, gpu: str) -> 'ComputerBuilder':
        self.computer.gpu = gpu
        return self

    def set_os(self, os: str) -> 'ComputerBuilder':
        self.computer.os = os
        return self

    def build(self) -> Computer:
        return self.computer


class ComputerDirector:
    """Director: Knows how to build specific configurations."""

    @staticmethod
    def build_gaming_pc(builder: ComputerBuilder) -> Computer:
        return (builder
                .set_cpu("Intel i9")
                .set_ram("32GB DDR5")
                .set_storage("2TB NVMe SSD")
                .set_gpu("RTX 4090")
                .set_os("Windows 11")
                .build())

    @staticmethod
    def build_office_pc(builder: ComputerBuilder) -> Computer:
        return (builder
                .set_cpu("Intel i5")
                .set_ram("16GB DDR4")
                .set_storage("512GB SSD")
                .set_gpu("Integrated")
                .set_os("Windows 11 Pro")
                .build())


# PROTOTYPE PATTERN

class Prototype(ABC):
    """Prototype interface."""

    @abstractmethod
    def clone(self):
        pass


class Document(Prototype):
    """
    Prototype Pattern: Creates new objects by cloning existing ones.
    Useful when object creation is expensive.
    """

    def __init__(self, title: str, content: str, metadata: Dict[str, Any]):
        self.title = title
        self.content = content
        self.metadata = metadata

    def clone(self) -> 'Document':
        """Shallow copy."""
        return copy.copy(self)

    def deep_clone(self) -> 'Document':
        """Deep copy."""
        return copy.deepcopy(self)

    def __str__(self):
        return f"Document(title='{self.title}', content='{self.content[:20]}...', metadata={self.metadata})"


# OBJECT POOL PATTERN

class Reusable:
    """Reusable object."""

    def __init__(self, id: int):
        self.id = id
        self.in_use = False

    def use(self):
        self.in_use = True
        print(f"Object {self.id} is now in use")

    def release(self):
        self.in_use = False
        print(f"Object {self.id} is released")


class ObjectPool:
    """
    Object Pool Pattern: Reuses objects instead of creating new ones.
    Useful for expensive-to-create objects.
    """

    def __init__(self, size: int):
        self.pool = [Reusable(i) for i in range(size)]

    def acquire(self) -> Optional[Reusable]:
        """Get available object from pool."""
        for obj in self.pool:
            if not obj.in_use:
                obj.use()
                return obj
        return None

    def release(self, obj: Reusable):
        """Return object to pool."""
        obj.release()


def demo():
    """Demonstrate all creational patterns."""
    print("=" * 60)
    print("CREATIONAL DESIGN PATTERNS DEMO")
    print("=" * 60)

    # Singleton
    print("\n--- Singleton Pattern ---")
    db1 = DatabaseConnection()
    db2 = DatabaseConnection()
    print(f"db1 is db2: {db1 is db2}")  # True - same instance
    db1.connect("postgresql://localhost/mydb")

    # Factory
    print("\n--- Factory Pattern ---")
    factory = AnimalFactory()
    dog = factory.create_animal("dog")
    cat = factory.create_animal("cat")
    print(f"Dog says: {dog.speak()}")
    print(f"Cat says: {cat.speak()}")

    # Abstract Factory
    print("\n--- Abstract Factory Pattern ---")
    windows_factory = WindowsFactory()
    button = windows_factory.create_button()
    checkbox = windows_factory.create_checkbox()
    print(button.render())
    print(checkbox.render())

    # Builder
    print("\n--- Builder Pattern ---")
    gaming_pc = ComputerDirector.build_gaming_pc(ComputerBuilder())
    office_pc = ComputerDirector.build_office_pc(ComputerBuilder())
    print(f"Gaming PC: {gaming_pc}")
    print(f"Office PC: {office_pc}")

    # Custom build
    custom_pc = (ComputerBuilder()
                 .set_cpu("AMD Ryzen 9")
                 .set_ram("64GB")
                 .set_storage("4TB")
                 .build())
    print(f"Custom PC: {custom_pc}")

    # Prototype
    print("\n--- Prototype Pattern ---")
    original = Document("Report", "Lorem ipsum dolor sit amet...",
                       {"author": "John", "version": 1})
    clone1 = original.clone()
    clone2 = original.deep_clone()

    print(f"Original: {original}")
    print(f"Clone 1: {clone1}")
    print(f"Clone 2: {clone2}")

    # Modify clone
    clone1.title = "Report Copy"
    clone1.metadata["version"] = 2
    print(f"After modification:")
    print(f"Original metadata: {original.metadata}")  # Affected by shallow copy
    print(f"Clone 1 metadata: {clone1.metadata}")

    # Object Pool
    print("\n--- Object Pool Pattern ---")
    pool = ObjectPool(3)
    obj1 = pool.acquire()
    obj2 = pool.acquire()
    obj3 = pool.acquire()
    obj4 = pool.acquire()  # Should be None (pool exhausted)

    print(f"Object 4 acquired: {obj4 is not None}")

    pool.release(obj1)
    obj4 = pool.acquire()  # Should succeed now
    print(f"Object 4 acquired after release: {obj4 is not None}")


if __name__ == '__main__':
    demo()
