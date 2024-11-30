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

Alternatively we could build the whole thing on .Net, compiling to MSIL. This will give us the advantage of having an
industrial grade GC and JIT, and because it is highly standardized we could always build a Rust implementation of the
runtime afterwards.

There is already a decently complete .Net targeting backend for rustc, so the only challenge remaining would be codegen.

So:

1. Compile entire Rust environment to .Net.
2. Generate MSIL from Emerald
3. Patch it in as needed.

.Net has some nice features for patching in methods and types:

https://learn.microsoft.com/en-us/dotnet/fundamentals/reflection/reflection
https://learn.microsoft.com/en-us/dotnet/csharp/advanced-topics/expression-trees/

#### Strategy

When an Emerald project is loaded, the entire source is loaded and precompiled. During the precompilation step a Rust module
is generated that provides access to all Rust dependencies with any generic methods that are used in the Emerald code specialized
to Emerald object shapes with the required traits.

So the proof of concept we could start without any dynamic behavior, and just generate that Rust module, and some hello world
Emerald function that uses a generic Rust function.

The Emerald object shapes with the required shapes, they just are simple delegators with a list of accessors as needed by the traits.
Their implementations can be regenerated as the objects change. Them being delegators doesn't necessarily hurt performance as
the objects are going to be heap allocated anyway, so either a smart JIT optimizes the whole thing or it doesn't.

## Language

### Syntax

### Semantics

### I/O

### Type System

### Concurrency



