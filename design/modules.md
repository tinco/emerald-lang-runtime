Modules
=======

In Emerald we are object oriented, and we share behaviour through modules, which in other languages might be called abstract classes.

A single class or module might include more than one module.

```python

class Animal extends Predator, Carnivore:
    pass
```

When two modules define the same method, the one that is included last will be the one that will be available on `self` directly.

Memory layout
-------------

Based on the code inside of a module, an interface for `self` is derived. For example if a `module` is defined like this:

```python
module Predator:
    def eat(organism):
        self.energy += organism.energy
```

Then in Rust pseudo-code the module and its interface for `self` would look like this:

```rust
interface PredatorSelf {
    energy: i32,
}

struct Predator {
    self: &PredatorSelf,
}

impl Predator {
    fn eat(&self, organism: &PredatorSelf) {
        self.self.energy += organism.energy;
    }
}
```

Then if another class includes the `Predator` module, the `self` interface would be extended with the `Predator` module's interface:

```python
module Organism
    energy: int

# simplest organism that eats other organisms
class Amoeba extends Organism, Predator:
    pass
```

```rust
interface OrganismSelf {
}

struct Organism {
    self: &OrganismSelf,
    energy: i32,
}

struct Amoeba {
    modules: [Organism, Predator],
}

impl Amoeba {
    // how does predator get a reference to the energy field?

    fn eat(&self, organism: &OrganismSelf) {
        self.modules.predator.eat(&self, organism);
    }
}
```
