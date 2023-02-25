Emerald Programming Language
============================

## WIP

### Ideas

- Syntax like that of python, but with do blocks
- Semantics like that of Ruby
- I/O like in Javascript
- Type system like that of Typescript
- Concurrency like in Rust

### Parser & AST

To implement the Parser I took the RustPython parser and modified it only slightly to support do blocks and the other
small syntax changes.

### Codegen & JIT

I'd like the language to be dynamic but also take advantage of that in long running processes like servers eventually
most possible codepaths have been explored and can be optimized for, trading memory usage for speed.

Specifically for dynamic programming languages there are a lot of ideas coming from TruffleRuby by Chris Seaton et al.
There is also Pypy which seems to perform very well in general. They both claim to have order of magnitude improvements
over eachother so there's probably interesting ideas in both.

There's a bunch of options for implementing a codegen besides writing one from scratch:

- Just directly running on top of YJIT. This seems easy and performant, but the downside is that it pulls in all of the
  Ruby VM which is written in C and it has a lot of baggage. Integrating with Rust and C is going to be very easy though
  probably even integrating with Python should be very easy.
- Directly running on PyPy. It's seemed to be designed for this, but besides that it has the same downsides as YJIT. I
  also have bad experiences with Python codebases so I'm biased towards taking on a big Python codebase.
- Running on top of TruffleRuby, same issues as above but then also with Java. I feel that the TruffleRuby+GraalVM took
  on a much too ambitious project optimizing both Ruby and C at the same time. I would like to set the vision towards a
  future where languages with bad design like C or a lot of historical baggage like Java are not part of the project.

Idea that I'm currently thinking about: Build on top of the cranelift rustc backend.

#### Random ideas

- Work with object shapes: https://chrisseaton.com/truffleruby/rubykaigi21/
- Ideas for cutting down on optimization invalidations:
  - Only allow reopening of classes from within the module they were defined in
  - Or only during loading of modules phase, more like Haskell language extensions (to allow things like ActiveSupport)
- Ruby has a big performance cost with identifier lookups because of too loose rules with regards to namespacing.


#### JIT

For every method definition we keep track of its associated types and the AST of its instructions and an indexed list of 
specialised implementations. Specialisations are paramterized not just by the associated types but by the shape of the
associated types. So whenever a types shape changes, we simply generate new specialisations for all methods that operate
over it. To avoid oscillating shapes we could set a rule that favours growing shapes instead over only modifying them.
Shapes and their associated methods can be garbage collected.

The associated types are the types of the arguments and the return value, but also that of any types and constants that
are referenced inside the method.

We can use an in-memory database like `sled` to cache compiled specialisations. Or maybe just an LRU backed by
`komora/marble`.

What happens to objects when the shape of their type changes? Maybe there's an answer here: https://chrisseaton.com/rubytruffle/pppj14-om/pppj14-om.pdf

### Runtime

There is a GC with a focus on performance written in Rust called RSGC.

## Language

### Syntax

### Semantics

### I/O

### Type System

### Concurrency



