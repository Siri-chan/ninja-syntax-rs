# ninja-syntax
This is a faithful port of `ninja-syntax.py` from the misc/ folder in
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

Like the original work, this code is licensed under the Apache License 2.0

