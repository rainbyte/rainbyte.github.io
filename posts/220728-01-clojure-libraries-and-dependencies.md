---
title: Clojure libraries and dependencies
author: rainbyte
published: 2022-07-28 21:49:00
tags: clojure
language: en
---

The Clojure language provides some keywords to handle libraries in each file:

- `:require` loads a Clojure library so it can be used in the current file or
  in the REPL
- `:use` brings definitions to current namespace via aliases
- `:import` gives access to native Java classes and interfaces

Note: in ClojureScript the native JavaScript code can be usually accessed using
`:require`, but sometimes `:import` is needed (eg. for Google Closure library).

## How to use these keywords

Load `foo.bar` and invoke a function from that library with the
full namespace:

```clojure
(ns user
  (:require
    [foo.bar]
    [foo.baz]))

(foo.bar/a-function)
```

Load a library making an alias to simplify access to a function:

```clojure
(ns user
  (:require
    [foo.bar :as bar]
    [foo.baz :as baz]))

(bar/a-function)
```

Load with `:refer` only the definitions we are interested in:

```clojure
(ns user
  (:require
    [foo.bar :refer [a-function]]))

(a-function)
```

Load with `:refer` only some definitions and also make aliases with `:rename`
for our convenience:

```clojure
(ns user
  (:require
    [foo.bar :refer [a-function]
             :rename [a-function func]]))

(func)
```

The `:use` keyword can be applied with `:only` to indicate which definitions
will be provided:

```clojure
(ns user
  (:use
    [foo.bar :only [a-function]]))

(a-function)
```

The `:use` keyword can also be applied without `:only`, but **beware! It can cause
conflicts**, eg. the following snippet will have a problem if `foo.bar` and
`foo.baz` namespaces provide definitions with the same name:

```clojure
(ns user
  (:use
    [foo.bar]   ;; avoid :use without :only, it can cause conflicts!
    [foo.baz]))

(a-function)
```

Import a native Java class of `java.util` package and invoke the `Date.` method:

```clojure
(ns user
  (:import
    (java.util Date)))

(Date.) ; call a Java method to get the current date
```
