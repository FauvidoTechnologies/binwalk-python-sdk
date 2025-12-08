# Binwalk-python

This is a python SDK for using Binwalk (which is now a Rust project). You can install this library from PyPI.

>[!NOTE]
>This library is not exposing every possible feature of binwalk yet. In due time, it might

## Installation

#### Install via pip

```sh
pip install binwalk-python-sdk
```

#### or build it from this repo

```sh
git clone https://github.com/FauvidoTechnologies/binwalk-python-sdk
cd binwalk-python-sdk
pip install -e .
```

## Quickstart

1. Run a basic scan using the `scan` endpoint:

```py
from pybinwalk import scan

result = basic_scan(image_path)
print(type(result))
# List
```

2. Run an `extraction` on a binary and save the results to a specified directory:

```py
from pybinwalk import extract

in_bin = "/path/to/input/binary"
out_dir = "/path/to/output/DIR"

result = extract(in_bin, out_dir)
```

3. Install dependencies for your OS (currently supports Ubuntu) using the `Manager`:

```py
from pybinwalk import Manager

Manager(stream=True).handle_deps()		# This produces a verbose streaming log as it installs all dependencies
```