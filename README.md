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

We're currently exposing just one endpoint, which is `scan_firmware`. This is equivalent to doing `binwalk.scan(&data)` in the rust counterpart.

```py
import json
from pybinwalk import scan_firmware

def scan(image_path):
	result = scan_firmware(image_path)
	return json.loads(result)
```