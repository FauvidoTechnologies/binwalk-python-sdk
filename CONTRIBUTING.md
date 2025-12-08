# Contributing guidelines

All contributions are welcome. Please feel free to report any bugs, patches or suggestions for `pybinwalk`.

## Setting up

### Pull Requests

- Fork the repo
- Make your changes
- Make a PR

### Bugs/Issues

- Create an issue on GitHub

## Nuances

This is a `rust` + `python` project. This means additional care needs to be taken in terms of package development. All changes suggested will go through some thorough testing to ensure that it doesn't break the delicate bindings.

Before committing, you must run pre-commit,

```sh
make pre-commit
```

And, as a preliminary check, you should try building the rust binaries (it won't be added to the repo don't worry)

```sh
make maturin-develop
```

Once both the checks pass, please make a pull-request.

## Features

Go wild!