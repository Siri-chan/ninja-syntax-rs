# ninja-syntax
This is a faithful port of `ninja_syntax.py` from the misc/ folder in
the source tree for the Ninja build system.
It aims to avoid extending the API wherever possible, but uses a few wrappers 
*(an enum instead of the variable union type, for example)* to make certain 
functionality possible within the constraints of the language.
As rust ahs no support for optional parameters, some functions have a long
number of `Option<T>` arguments to set. 
This is intended, to match the functionality of the python script.
Basically none of the API is documented, although I may bother to fix that with time.
I also haven't run a linter or anything over this code, or run many tests - 
so don't expect much stability.
That said, this does implement all of the public and private API from the 
original file, and that completeness - in my eyes, justifies the 1.0.0 version.
Do also note that this crate provides even less abstraction than the crate 
published under the same name on crates.io - this is mostly just because 
I wanted to be able to write to stdout, and also to more-directly work with some 
old draft python code.
I also decided not to rename this and publish it to crates.io, mostly because I 
kinda reinvented the wheel, and taking attention from others work isn't cool.
If you are working from scratch, check out the other package of the same name.

To depend on this crate, add the following dependency:
```toml
ninja-syntax = { git = "https://github.com/Siri-chan/ninja-syntax-rs.git" }
```

Like the original work, this code is licensed under the Apache License 2.0

