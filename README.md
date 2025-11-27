# Binwalk-python

This is a python SDK for using Binwalk (which is now a Rust project). You can install this library from PyPI.

>[!NOTE]
>This library is not exposing every possible feature of binwalk yet. In due time, it might

## Installation

#### Install via pip

```sh
pip install binwalk-python
```

#### or build it from this repo

```sh
git clone https://github.com/FauvidoTechnologies/binwalk-python-sdk
cd binwalk-python-sdk
pip install -e .
```

## Usage

We're currently exposing just one endpoint, which is `basic_scan`. This is equivalent to doing `binwalk.scan(&data)` in the rust counterpart (or `binwalk filename`).

```py
from pybinwalk import basic_scan

result = basic_scan(image_path)
print(type(result))
# List

```