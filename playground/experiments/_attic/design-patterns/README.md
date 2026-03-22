# Design Patterns

Comprehensive implementation of classic Gang of Four (GoF) design patterns in Python.

## Overview

Design patterns are reusable solutions to common problems in software design. They represent best practices evolved over time by experienced developers.

## Pattern Categories

### Creational Patterns (Object Creation)
Deal with object creation mechanisms, trying to create objects in a manner suitable to the situation.

### Structural Patterns (Object Composition)
Explain how to assemble objects and classes into larger structures while keeping structures flexible and efficient.

### Behavioral Patterns (Object Interaction)
Concerned with algorithms and the assignment of responsibilities between objects.

## Creational Patterns (`creational_patterns.py`)

| Pattern | Purpose | Use When |
|---------|---------|----------|
| **Singleton** | Ensure only one instance exists | Global configuration, database connections |
| **Factory** | Create objects without specifying exact class | Object type determined at runtime |
| **Abstract Factory** | Create families of related objects | Need consistent object families |
| **Builder** | Construct complex objects step by step | Object has many configuration options |
| **Prototype** | Clone objects instead of creating new | Object creation is expensive |
| **Object Pool** | Reuse objects instead of creating new | Objects are expensive to create/destroy |

### Usage Examples

```python
from creational_patterns import *

# Singleton
db = DatabaseConnection()
db.connect("postgresql://localhost/mydb")

# Factory
factory = AnimalFactory()
dog = factory.create_animal("dog")

# Builder
pc = (ComputerBuilder()
      .set_cpu("Intel i9")
      .set_ram("32GB")
      .build())

# Prototype
original = Document("Report", "Content", {"author": "John"})
clone = original.deep_clone()

# Object Pool
pool = ObjectPool(size=10)
obj = pool.acquire()
pool.release(obj)
```

## Structural Patterns (`structural_patterns.py`)

| Pattern | Purpose | Use When |
|---------|---------|----------|
| **Adapter** | Convert one interface to another | Integrating incompatible interfaces |
| **Bridge** | Separate abstraction from implementation | Both may vary independently |
| **Composite** | Compose objects into tree structures | Need to treat individual and composite objects uniformly |
| **Decorator** | Add responsibilities dynamically | Extending functionality without subclassing |
| **Facade** | Simplified interface to complex subsystem | Hide complexity, provide simple API |
| **Flyweight** | Share common state to reduce memory | Many similar objects needed |
| **Proxy** | Placeholder for another object | Control access, lazy loading, caching |

### Usage Examples

```python
from structural_patterns import *

# Adapter
european_socket = EuropeanSocket()
adapter = SocketAdapter(european_socket)

# Bridge
tv = TV()
remote = RemoteControl(tv)
remote.toggle_power()

# Composite
root = Directory("root")
root.add(File("file.txt", 1024))
root.add(Directory("subfolder"))

# Decorator
coffee = SimpleCoffee()
fancy = VanillaDecorator(MilkDecorator(coffee))

# Facade
computer = ComputerFacade()
computer.start()  # Hides complex boot process

# Flyweight
char = CharacterFactory.get_character('A', 'Arial', 12)

# Proxy
image = ImageProxy("photo.jpg")  # Not loaded yet
image.display()  # Loads on first use
```

## Behavioral Patterns (`behavioral_patterns.py`)

| Pattern | Purpose | Use When |
|---------|---------|----------|
| **Strategy** | Define family of interchangeable algorithms | Algorithm varies independently |
| **Observer** | One-to-many dependency notification | State changes need to notify dependents |
| **Command** | Encapsulate request as object | Undo/redo, queuing, logging operations |
| **State** | Alter behavior when state changes | Behavior depends on state |
| **Template Method** | Define algorithm skeleton, defer steps | Common algorithm with varying steps |
| **Iterator** | Sequential access without exposing structure | Traverse collection uniformly |
| **Chain of Responsibility** | Pass request along chain of handlers | Multiple handlers may process request |
| **Mediator** | Encapsulate how objects interact | Reduce coupling between interacting objects |
| **Memento** | Capture and restore object state | Undo/redo, snapshots needed |

### Usage Examples

```python
from behavioral_patterns import *

# Strategy
cart = ShoppingCart()
cart.set_payment_strategy(CreditCardPayment("1234"))
cart.checkout()

# Observer
subject = Subject()
subject.attach(ConcreteObserver("Observer1"))
subject.set_state("Active")

# Command
remote = RemoteControl()
remote.execute_command(LightOnCommand(light))
remote.undo_last()

# State
doc = DocumentContext()
doc.request()  # Transitions through states

# Template Method
processor = CSVProcessor()
processor.process()  # Executes template

# Iterator
iterator = collection.create_iterator()
while iterator.has_next():
    item = iterator.next()

# Chain of Responsibility
auth_chain = AuthHandler().set_next(AuthzHandler())
result = auth_chain.handle(request)

# Mediator
chatroom = ChatRoom()
user = User("Alice", chatroom)
user.send("Hello!")

# Memento
editor = TextEditor()
saved = editor.save()
editor.restore(saved)
```

## Running the Demos

Each file includes a comprehensive demo:

```bash
# Creational patterns
python creational_patterns.py

# Structural patterns
python structural_patterns.py

# Behavioral patterns
python behavioral_patterns.py
```

## Pattern Selection Guide

### Creational Patterns

**Use Singleton when:**
- Exactly one instance needed (config, connection pool)
- Global access point required

**Use Factory when:**
- Don't know exact types beforehand
- Centralize object creation logic

**Use Builder when:**
- Object has many optional parameters
- Step-by-step construction needed

**Use Prototype when:**
- Creating new instances is expensive
- Objects are similar to existing ones

### Structural Patterns

**Use Adapter when:**
- Need to use existing class with incompatible interface
- Integrating third-party libraries

**Use Decorator when:**
- Add responsibilities to individual objects
- Extension by subclassing impractical

**Use Facade when:**
- Provide simple interface to complex subsystem
- Layer system dependencies

**Use Proxy when:**
- Control access to object
- Lazy initialization needed
- Remote object access

### Behavioral Patterns

**Use Strategy when:**
- Many related classes differ only in behavior
- Need different variants of algorithm
- Algorithm should be hidden from client

**Use Observer when:**
- Change in one object requires changing others
- Number of dependents unknown or dynamic

**Use Command when:**
- Parameterize objects with operations
- Queue, log, or support undo operations

**Use State when:**
- Object behavior depends on state
- State-specific code is substantial

## Anti-Patterns to Avoid

1. **Overuse**: Don't force patterns where simple solutions work
2. **Wrong Pattern**: Choose the right pattern for the problem
3. **Premature Optimization**: Don't add complexity early
4. **God Object**: Don't create all-knowing objects
5. **Spaghetti Code**: Maintain clear structure

## Benefits of Design Patterns

1. **Proven Solutions**: Battle-tested approaches
2. **Common Vocabulary**: Shared language for developers
3. **Maintainability**: Easier to understand and modify
4. **Scalability**: Flexible architecture
5. **Best Practices**: Codified expert knowledge

## Pattern Relationships

### Complementary Patterns

- **Factory + Singleton**: Create single factory instance
- **Strategy + Factory**: Create strategy objects
- **Composite + Iterator**: Traverse composite structures
- **Command + Memento**: Undo/redo operations
- **Observer + Mediator**: Manage notifications

### Alternative Patterns

- **Adapter vs Facade**: Interface conversion vs simplified interface
- **Strategy vs State**: Interchangeable algorithms vs state-dependent behavior
- **Factory vs Abstract Factory**: Single product vs product families

## Best Practices

1. **Understand the Problem**: Don't use patterns blindly
2. **Start Simple**: Add patterns when needed
3. **Document Usage**: Explain why pattern was chosen
4. **Test Thoroughly**: Patterns add complexity
5. **Refactor Carefully**: Extract patterns from existing code

## Further Reading

### Books
- "Design Patterns" (Gang of Four) - Original patterns book
- "Head First Design Patterns" - Beginner-friendly approach
- "Refactoring to Patterns" - When and how to apply patterns

### Online Resources
- [Refactoring Guru](https://refactoring.guru/design-patterns) - Excellent visual explanations
- [Source Making](https://sourcemaking.com/design_patterns) - Pattern catalog
- [Python Patterns](https://python-patterns.guide/) - Python-specific patterns

## Testing

Test pattern implementations:

```python
def test_singleton():
    instance1 = Singleton()
    instance2 = Singleton()
    assert instance1 is instance2

def test_factory():
    factory = AnimalFactory()
    dog = factory.create_animal("dog")
    assert isinstance(dog, Dog)
```

## Contributing

When adding new patterns:
1. Include clear documentation
2. Provide real-world examples
3. Add demo code
4. Update this README

## License

MIT License
