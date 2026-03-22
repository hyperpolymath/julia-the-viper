"""
Structural Design Patterns

Patterns that deal with object composition and relationships.
"""

from abc import ABC, abstractmethod
from typing import List, Dict, Any


# ADAPTER PATTERN

class EuropeanSocket:
    """European electrical socket (220V)."""

    def provide_power(self) -> str:
        return "220V power from European socket"


class AmericanSocket(ABC):
    """American socket interface (110V)."""

    @abstractmethod
    def get_power(self) -> str:
        pass


class SocketAdapter(AmericanSocket):
    """
    Adapter Pattern: Converts interface of a class into another interface.
    Allows incompatible interfaces to work together.
    """

    def __init__(self, european_socket: EuropeanSocket):
        self.european_socket = european_socket

    def get_power(self) -> str:
        power = self.european_socket.provide_power()
        # Convert 220V to 110V
        return power.replace("220V", "110V (adapted from 220V)")


# BRIDGE PATTERN

class Device(ABC):
    """Implementation interface."""

    @abstractmethod
    def turn_on(self):
        pass

    @abstractmethod
    def turn_off(self):
        pass

    @abstractmethod
    def set_channel(self, channel: int):
        pass


class TV(Device):
    def __init__(self):
        self.on = False
        self.channel = 1

    def turn_on(self):
        self.on = True
        print("TV is turned on")

    def turn_off(self):
        self.on = False
        print("TV is turned off")

    def set_channel(self, channel: int):
        self.channel = channel
        print(f"TV channel set to {channel}")


class Radio(Device):
    def __init__(self):
        self.on = False
        self.channel = 1

    def turn_on(self):
        self.on = True
        print("Radio is turned on")

    def turn_off(self):
        self.on = False
        print("Radio is turned off")

    def set_channel(self, channel: int):
        self.channel = channel
        print(f"Radio frequency set to {channel}")


class RemoteControl:
    """
    Bridge Pattern: Separates abstraction from implementation.
    Allows both to vary independently.
    """

    def __init__(self, device: Device):
        self.device = device

    def toggle_power(self):
        if not self.device.on:
            self.device.turn_on()
        else:
            self.device.turn_off()

    def channel_up(self):
        self.device.set_channel(self.device.channel + 1)

    def channel_down(self):
        self.device.set_channel(max(1, self.device.channel - 1))


class AdvancedRemoteControl(RemoteControl):
    """Extended abstraction with additional features."""

    def mute(self):
        print("Device muted")


# COMPOSITE PATTERN

class FileSystemComponent(ABC):
    """Component interface."""

    @abstractmethod
    def get_size(self) -> int:
        pass

    @abstractmethod
    def display(self, indent: int = 0):
        pass


class File(FileSystemComponent):
    """Leaf: Cannot have children."""

    def __init__(self, name: str, size: int):
        self.name = name
        self.size = size

    def get_size(self) -> int:
        return self.size

    def display(self, indent: int = 0):
        print("  " * indent + f"- {self.name} ({self.size} bytes)")


class Directory(FileSystemComponent):
    """
    Composite Pattern: Composes objects into tree structures.
    Treats individual objects and compositions uniformly.
    """

    def __init__(self, name: str):
        self.name = name
        self.children: List[FileSystemComponent] = []

    def add(self, component: FileSystemComponent):
        self.children.append(component)

    def remove(self, component: FileSystemComponent):
        self.children.remove(component)

    def get_size(self) -> int:
        return sum(child.get_size() for child in self.children)

    def display(self, indent: int = 0):
        print("  " * indent + f"+ {self.name}/")
        for child in self.children:
            child.display(indent + 1)


# DECORATOR PATTERN

class Coffee(ABC):
    """Component interface."""

    @abstractmethod
    def get_cost(self) -> float:
        pass

    @abstractmethod
    def get_description(self) -> str:
        pass


class SimpleCoffee(Coffee):
    """Concrete component."""

    def get_cost(self) -> float:
        return 2.0

    def get_description(self) -> str:
        return "Simple coffee"


class CoffeeDecorator(Coffee):
    """
    Decorator Pattern: Adds responsibilities to objects dynamically.
    Alternative to subclassing for extending functionality.
    """

    def __init__(self, coffee: Coffee):
        self._coffee = coffee

    def get_cost(self) -> float:
        return self._coffee.get_cost()

    def get_description(self) -> str:
        return self._coffee.get_description()


class MilkDecorator(CoffeeDecorator):
    def get_cost(self) -> float:
        return self._coffee.get_cost() + 0.5

    def get_description(self) -> str:
        return self._coffee.get_description() + ", milk"


class SugarDecorator(CoffeeDecorator):
    def get_cost(self) -> float:
        return self._coffee.get_cost() + 0.2

    def get_description(self) -> str:
        return self._coffee.get_description() + ", sugar"


class VanillaDecorator(CoffeeDecorator):
    def get_cost(self) -> float:
        return self._coffee.get_cost() + 0.7

    def get_description(self) -> str:
        return self._coffee.get_description() + ", vanilla"


# FACADE PATTERN

class CPU:
    def freeze(self):
        print("CPU: Freezing")

    def jump(self, position: int):
        print(f"CPU: Jumping to {position}")

    def execute(self):
        print("CPU: Executing")


class Memory:
    def load(self, position: int, data: str):
        print(f"Memory: Loading '{data}' at {position}")


class HardDrive:
    def read(self, sector: int, size: int) -> str:
        return f"Data from sector {sector} (size: {size})"


class ComputerFacade:
    """
    Facade Pattern: Provides simplified interface to complex subsystem.
    Hides complexity and makes subsystem easier to use.
    """

    def __init__(self):
        self.cpu = CPU()
        self.memory = Memory()
        self.hard_drive = HardDrive()

    def start(self):
        """Simple interface to complex boot process."""
        print("Starting computer...")
        self.cpu.freeze()
        boot_data = self.hard_drive.read(sector=0, size=1024)
        self.memory.load(position=0, data=boot_data)
        self.cpu.jump(position=0)
        self.cpu.execute()
        print("Computer started!")


# FLYWEIGHT PATTERN

class CharacterFlyweight:
    """Flyweight: Shared immutable state."""

    def __init__(self, char: str, font: str, size: int):
        self.char = char
        self.font = font
        self.size = size

    def render(self, position: tuple, color: str):
        """Extrinsic state (position, color) passed in."""
        print(f"Rendering '{self.char}' ({self.font}, {self.size}pt) "
              f"at {position} in {color}")


class CharacterFactory:
    """
    Flyweight Pattern: Shares common state to reduce memory usage.
    Separates intrinsic (shared) from extrinsic (unique) state.
    """

    _flyweights: Dict[tuple, CharacterFlyweight] = {}

    @classmethod
    def get_character(cls, char: str, font: str, size: int) -> CharacterFlyweight:
        key = (char, font, size)

        if key not in cls._flyweights:
            cls._flyweights[key] = CharacterFlyweight(char, font, size)
            print(f"Creating new flyweight for '{char}'")

        return cls._flyweights[key]

    @classmethod
    def get_flyweight_count(cls) -> int:
        return len(cls._flyweights)


# PROXY PATTERN

class Image(ABC):
    """Subject interface."""

    @abstractmethod
    def display(self):
        pass


class RealImage(Image):
    """Real subject: Expensive to create."""

    def __init__(self, filename: str):
        self.filename = filename
        self._load_from_disk()

    def _load_from_disk(self):
        print(f"Loading image from disk: {self.filename}")

    def display(self):
        print(f"Displaying image: {self.filename}")


class ImageProxy(Image):
    """
    Proxy Pattern: Provides placeholder for another object.
    Controls access, adds lazy loading, caching, etc.
    """

    def __init__(self, filename: str):
        self.filename = filename
        self._real_image: RealImage = None

    def display(self):
        """Lazy loading: Only create real image when needed."""
        if self._real_image is None:
            self._real_image = RealImage(self.filename)
        self._real_image.display()


def demo():
    """Demonstrate all structural patterns."""
    print("=" * 60)
    print("STRUCTURAL DESIGN PATTERNS DEMO")
    print("=" * 60)

    # Adapter
    print("\n--- Adapter Pattern ---")
    european = EuropeanSocket()
    adapter = SocketAdapter(european)
    print(adapter.get_power())

    # Bridge
    print("\n--- Bridge Pattern ---")
    tv = TV()
    remote = RemoteControl(tv)
    remote.toggle_power()
    remote.channel_up()

    radio = Radio()
    advanced_remote = AdvancedRemoteControl(radio)
    advanced_remote.toggle_power()
    advanced_remote.mute()

    # Composite
    print("\n--- Composite Pattern ---")
    root = Directory("root")

    docs = Directory("documents")
    docs.add(File("resume.pdf", 2048))
    docs.add(File("letter.doc", 1024))

    pictures = Directory("pictures")
    pictures.add(File("photo1.jpg", 4096))
    pictures.add(File("photo2.jpg", 3072))

    root.add(docs)
    root.add(pictures)
    root.add(File("readme.txt", 512))

    root.display()
    print(f"Total size: {root.get_size()} bytes")

    # Decorator
    print("\n--- Decorator Pattern ---")
    coffee = SimpleCoffee()
    print(f"{coffee.get_description()}: ${coffee.get_cost():.2f}")

    coffee_with_milk = MilkDecorator(coffee)
    print(f"{coffee_with_milk.get_description()}: ${coffee_with_milk.get_cost():.2f}")

    fancy_coffee = VanillaDecorator(SugarDecorator(MilkDecorator(SimpleCoffee())))
    print(f"{fancy_coffee.get_description()}: ${fancy_coffee.get_cost():.2f}")

    # Facade
    print("\n--- Facade Pattern ---")
    computer = ComputerFacade()
    computer.start()

    # Flyweight
    print("\n--- Flyweight Pattern ---")
    text = "HELLO"
    font = "Arial"
    size = 12

    positions = [(x * 10, 0) for x in range(len(text))]
    colors = ["red", "blue", "green", "yellow", "purple"]

    for i, char in enumerate(text):
        flyweight = CharacterFactory.get_character(char, font, size)
        flyweight.render(positions[i], colors[i])

    print(f"Total flyweights created: {CharacterFactory.get_flyweight_count()}")

    # Proxy
    print("\n--- Proxy Pattern ---")
    image1 = ImageProxy("photo1.jpg")
    image2 = ImageProxy("photo2.jpg")

    print("Images created (not loaded yet)")
    print("Now displaying image1:")
    image1.display()
    print("Displaying image1 again (already loaded):")
    image1.display()


if __name__ == '__main__':
    demo()
