# Compiling and packaging the final library

After having made all the changes, to make a new release the following procedure is to be adopted

- [x] Update `pybinwalk/version.py` with the latest version number
- [x] Update `pyproject.toml` with the latest version number
- [x] Update `rust/Cargo.toml` with the latest version number

Create the `pyo3` bindings again using (from the root directory):

```sh
make maturin-develop
```

Create the `python + rust wheel` using:

```sh
rm -rf dist
maturin build --release --out dist
```

Then upload it to `pypi` using `twine`.

```sh
twine upload dist/*
```