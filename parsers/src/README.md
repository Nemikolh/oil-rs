# Oil Parser architecture

## First pass: `grammar`

The first pass check the oil syntax.
Errors can only be caused by:

* Non-matching tags or unclosed ones.
* Misspelled keywords.
* Path for import syntax.
* Property access.
* Valid identifier.
* Valid style value (number + unit)
* Image and font special import syntax.
* Special property like `background-image` or `font`.
* Template, View and Class definition.

## Second pass: `import`

This pass make sure that all import are resolved and that
they point to valid files. This pass will recursively
calls the first pass on each new file encountered if they
haven't already been encountered.

It also does the dispatching for images and fonts and performs
checks such as file is a valid image or a valid font.

Finally in this pass, each AST is stored in a HashMap where
the key is the path of the file relative to the first one
loaded.

## Third pass: `symbol`

We now check for symbol resolution. Many checks are performed.

### Reachability and typos `res.rs`:

We want to ensure that all symbols are reachable. We can also
error when a symbol hasn't been defined or hasn't been imported
but it exists within an other import.

The harder part of this phase is with template / view. They're the
ones using a lot of symbols.

For this phase, additional symbols are added. Typically,
the one understood by oil such as:

* `button`
* `progress_bar`
* `group`
* `text_input`
* `for` (previously named `repeat`)
* [new] `if`
* [new] `switch`

> Design question:
>     Do we want to allows other symbols to be defined globally?

Finally, this step also applies name shadowing where parameters
hide names.

> Design question:
>     We can limit hiding by parameters only to classes. Would that be okay?


## Fourth pass: `checks`

Check that everything works as expected.

### Parameters checking `parameters.rs`

Once we know that all symbols are reachable we make sure that
where they are used, they have the correct type.

For instance a style symbol can only be used in a `class` property
or as argument to another template but not as a template.

### Style properties.

Checks that properties names and values are valid.

## Fifth pass: `transform`

This pass transform the data into a more efficient format that
can be serialized and reloaded without performing any check.
