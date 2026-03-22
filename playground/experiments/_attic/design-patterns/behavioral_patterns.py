"""
Behavioral Design Patterns

Patterns that deal with object interaction and responsibility distribution.
"""

from abc import ABC, abstractmethod
from typing import List, Dict, Any, Optional
from enum import Enum


# STRATEGY PATTERN

class PaymentStrategy(ABC):
    """Strategy interface."""

    @abstractmethod
    def pay(self, amount: float) -> str:
        pass


class CreditCardPayment(PaymentStrategy):
    def __init__(self, card_number: str):
        self.card_number = card_number

    def pay(self, amount: float) -> str:
        return f"Paid ${amount:.2f} using credit card ending in {self.card_number[-4:]}"


class PayPalPayment(PaymentStrategy):
    def __init__(self, email: str):
        self.email = email

    def pay(self, amount: float) -> str:
        return f"Paid ${amount:.2f} using PayPal ({self.email})"


class CryptoPayment(PaymentStrategy):
    def __init__(self, wallet_address: str):
        self.wallet_address = wallet_address

    def pay(self, amount: float) -> str:
        return f"Paid ${amount:.2f} using crypto wallet {self.wallet_address[:10]}..."


class ShoppingCart:
    """
    Strategy Pattern: Defines family of algorithms, encapsulates each one,
    and makes them interchangeable.
    """

    def __init__(self):
        self.items = []
        self.payment_strategy: Optional[PaymentStrategy] = None

    def add_item(self, item: str, price: float):
        self.items.append((item, price))

    def set_payment_strategy(self, strategy: PaymentStrategy):
        self.payment_strategy = strategy

    def checkout(self) -> str:
        total = sum(price for _, price in self.items)

        if not self.payment_strategy:
            return "Please select a payment method"

        return self.payment_strategy.pay(total)


# OBSERVER PATTERN

class Observer(ABC):
    """Observer interface."""

    @abstractmethod
    def update(self, message: str):
        pass


class Subject:
    """Subject being observed."""

    def __init__(self):
        self._observers: List[Observer] = []
        self._state = None

    def attach(self, observer: Observer):
        self._observers.append(observer)

    def detach(self, observer: Observer):
        self._observers.remove(observer)

    def notify(self, message: str):
        for observer in self._observers:
            observer.update(message)

    def set_state(self, state: Any):
        self._state = state
        self.notify(f"State changed to: {state}")


class ConcreteObserver(Observer):
    """
    Observer Pattern: Defines one-to-many dependency.
    When one object changes state, all dependents are notified.
    """

    def __init__(self, name: str):
        self.name = name

    def update(self, message: str):
        print(f"{self.name} received: {message}")


# COMMAND PATTERN

class Command(ABC):
    """Command interface."""

    @abstractmethod
    def execute(self):
        pass

    @abstractmethod
    def undo(self):
        pass


class Light:
    """Receiver."""

    def __init__(self):
        self.is_on = False

    def turn_on(self):
        self.is_on = True
        print("Light is ON")

    def turn_off(self):
        self.is_on = False
        print("Light is OFF")


class LightOnCommand(Command):
    """Concrete command."""

    def __init__(self, light: Light):
        self.light = light

    def execute(self):
        self.light.turn_on()

    def undo(self):
        self.light.turn_off()


class LightOffCommand(Command):
    def __init__(self, light: Light):
        self.light = light

    def execute(self):
        self.light.turn_off()

    def undo(self):
        self.light.turn_on()


class RemoteControl:
    """
    Command Pattern: Encapsulates request as object.
    Allows parameterization, queuing, logging, and undo operations.
    """

    def __init__(self):
        self.history: List[Command] = []

    def execute_command(self, command: Command):
        command.execute()
        self.history.append(command)

    def undo_last(self):
        if self.history:
            command = self.history.pop()
            command.undo()


# STATE PATTERN

class State(ABC):
    """State interface."""

    @abstractmethod
    def handle(self, context: 'DocumentContext'):
        pass


class DraftState(State):
    def handle(self, context: 'DocumentContext'):
        print("Document is in DRAFT state")
        print("Available actions: edit, submit for review")
        context.state = ReviewState()


class ReviewState(State):
    def handle(self, context: 'DocumentContext'):
        print("Document is in REVIEW state")
        print("Available actions: approve, reject")
        context.state = PublishedState()


class PublishedState(State):
    def handle(self, context: 'DocumentContext'):
        print("Document is PUBLISHED")
        print("Available actions: archive")


class DocumentContext:
    """
    State Pattern: Alters behavior when internal state changes.
    Object appears to change its class.
    """

    def __init__(self):
        self.state: State = DraftState()

    def request(self):
        self.state.handle(self)


# TEMPLATE METHOD PATTERN

class DataProcessor(ABC):
    """
    Template Method Pattern: Defines skeleton of algorithm.
    Subclasses override specific steps without changing structure.
    """

    def process(self):
        """Template method defining the algorithm structure."""
        self.read_data()
        self.process_data()
        self.validate_data()
        self.save_data()

    @abstractmethod
    def read_data(self):
        pass

    @abstractmethod
    def process_data(self):
        pass

    def validate_data(self):
        """Default implementation (can be overridden)."""
        print("Validating data...")

    @abstractmethod
    def save_data(self):
        pass


class CSVProcessor(DataProcessor):
    def read_data(self):
        print("Reading CSV file...")

    def process_data(self):
        print("Processing CSV data...")

    def save_data(self):
        print("Saving to CSV file...")


class JSONProcessor(DataProcessor):
    def read_data(self):
        print("Reading JSON file...")

    def process_data(self):
        print("Processing JSON data...")

    def save_data(self):
        print("Saving to JSON file...")


# ITERATOR PATTERN

class Iterator(ABC):
    """Iterator interface."""

    @abstractmethod
    def has_next(self) -> bool:
        pass

    @abstractmethod
    def next(self):
        pass


class BookCollection:
    """Aggregate."""

    def __init__(self):
        self.books = []

    def add_book(self, book: str):
        self.books.append(book)

    def create_iterator(self) -> Iterator:
        return BookIterator(self)


class BookIterator(Iterator):
    """
    Iterator Pattern: Provides way to access elements sequentially
    without exposing underlying representation.
    """

    def __init__(self, collection: BookCollection):
        self.collection = collection
        self.index = 0

    def has_next(self) -> bool:
        return self.index < len(self.collection.books)

    def next(self) -> str:
        if self.has_next():
            book = self.collection.books[self.index]
            self.index += 1
            return book
        raise StopIteration


# CHAIN OF RESPONSIBILITY PATTERN

class Handler(ABC):
    """Handler interface."""

    def __init__(self):
        self.next_handler: Optional[Handler] = None

    def set_next(self, handler: 'Handler') -> 'Handler':
        self.next_handler = handler
        return handler

    @abstractmethod
    def handle(self, request: Dict[str, Any]) -> Optional[str]:
        pass


class AuthenticationHandler(Handler):
    """
    Chain of Responsibility: Passes request along chain of handlers.
    Each handler decides to process request or pass it on.
    """

    def handle(self, request: Dict[str, Any]) -> Optional[str]:
        if not request.get('authenticated'):
            return "Authentication failed"

        if self.next_handler:
            return self.next_handler.handle(request)
        return None


class AuthorizationHandler(Handler):
    def handle(self, request: Dict[str, Any]) -> Optional[str]:
        if request.get('role') != 'admin':
            return "Authorization failed: insufficient permissions"

        if self.next_handler:
            return self.next_handler.handle(request)
        return None


class ValidationHandler(Handler):
    def handle(self, request: Dict[str, Any]) -> Optional[str]:
        if not request.get('data'):
            return "Validation failed: no data provided"

        if self.next_handler:
            return self.next_handler.handle(request)
        return "Request processed successfully"


# MEDIATOR PATTERN

class Mediator(ABC):
    """Mediator interface."""

    @abstractmethod
    def notify(self, sender: object, event: str):
        pass


class ChatRoom(Mediator):
    """
    Mediator Pattern: Defines object that encapsulates how objects interact.
    Promotes loose coupling by keeping objects from referring to each other explicitly.
    """

    def __init__(self):
        self.users: List['User'] = []

    def register(self, user: 'User'):
        self.users.append(user)

    def notify(self, sender: 'User', message: str):
        for user in self.users:
            if user != sender:
                user.receive(f"{sender.name}: {message}")


class User:
    def __init__(self, name: str, chat_room: ChatRoom):
        self.name = name
        self.chat_room = chat_room
        chat_room.register(self)

    def send(self, message: str):
        print(f"{self.name} sends: {message}")
        self.chat_room.notify(self, message)

    def receive(self, message: str):
        print(f"{self.name} receives: {message}")


# MEMENTO PATTERN

class EditorMemento:
    """Memento: Stores state."""

    def __init__(self, content: str):
        self._content = content

    def get_content(self) -> str:
        return self._content


class TextEditor:
    """
    Memento Pattern: Captures and externalizes object's internal state
    without violating encapsulation, allowing restoration later.
    """

    def __init__(self):
        self.content = ""

    def write(self, text: str):
        self.content += text

    def save(self) -> EditorMemento:
        """Create memento."""
        return EditorMemento(self.content)

    def restore(self, memento: EditorMemento):
        """Restore from memento."""
        self.content = memento.get_content()

    def __str__(self):
        return f"Content: {self.content}"


class History:
    """Caretaker: Manages mementos."""

    def __init__(self):
        self.history: List[EditorMemento] = []

    def push(self, memento: EditorMemento):
        self.history.append(memento)

    def pop(self) -> Optional[EditorMemento]:
        return self.history.pop() if self.history else None


def demo():
    """Demonstrate all behavioral patterns."""
    print("=" * 60)
    print("BEHAVIORAL DESIGN PATTERNS DEMO")
    print("=" * 60)

    # Strategy
    print("\n--- Strategy Pattern ---")
    cart = ShoppingCart()
    cart.add_item("Book", 15.99)
    cart.add_item("Pen", 2.50)

    cart.set_payment_strategy(CreditCardPayment("1234567890123456"))
    print(cart.checkout())

    cart.set_payment_strategy(PayPalPayment("user@example.com"))
    print(cart.checkout())

    # Observer
    print("\n--- Observer Pattern ---")
    subject = Subject()
    observer1 = ConcreteObserver("Observer 1")
    observer2 = ConcreteObserver("Observer 2")

    subject.attach(observer1)
    subject.attach(observer2)
    subject.set_state("Active")

    # Command
    print("\n--- Command Pattern ---")
    light = Light()
    remote = RemoteControl()

    remote.execute_command(LightOnCommand(light))
    remote.execute_command(LightOffCommand(light))
    print("Undoing last command:")
    remote.undo_last()

    # State
    print("\n--- State Pattern ---")
    doc = DocumentContext()
    doc.request()
    doc.request()
    doc.request()

    # Template Method
    print("\n--- Template Method Pattern ---")
    print("Processing CSV:")
    csv_processor = CSVProcessor()
    csv_processor.process()

    print("\nProcessing JSON:")
    json_processor = JSONProcessor()
    json_processor.process()

    # Iterator
    print("\n--- Iterator Pattern ---")
    library = BookCollection()
    library.add_book("Design Patterns")
    library.add_book("Clean Code")
    library.add_book("The Pragmatic Programmer")

    iterator = library.create_iterator()
    while iterator.has_next():
        print(f"Book: {iterator.next()}")

    # Chain of Responsibility
    print("\n--- Chain of Responsibility Pattern ---")
    auth = AuthenticationHandler()
    authz = AuthorizationHandler()
    valid = ValidationHandler()

    auth.set_next(authz).set_next(valid)

    request1 = {'authenticated': True, 'role': 'admin', 'data': {'name': 'John'}}
    print(f"Request 1: {auth.handle(request1)}")

    request2 = {'authenticated': True, 'role': 'user', 'data': {'name': 'Jane'}}
    print(f"Request 2: {auth.handle(request2)}")

    # Mediator
    print("\n--- Mediator Pattern ---")
    chat_room = ChatRoom()
    alice = User("Alice", chat_room)
    bob = User("Bob", chat_room)
    charlie = User("Charlie", chat_room)

    alice.send("Hello everyone!")

    # Memento
    print("\n--- Memento Pattern ---")
    editor = TextEditor()
    history = History()

    editor.write("Hello ")
    history.push(editor.save())

    editor.write("World")
    history.push(editor.save())

    editor.write("!!!")
    print(editor)

    print("Undoing...")
    editor.restore(history.pop())
    print(editor)

    print("Undoing again...")
    editor.restore(history.pop())
    print(editor)


if __name__ == '__main__':
    demo()
