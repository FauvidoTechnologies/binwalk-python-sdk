import json

from binwalk_python_sdk._rust import basic_scan


def basic_scan_py(image_path: str):
    """
    Function to perform a basic binwalk scan.

    `equivalent CLI command`: binwalk image_path

    returns:
            A JSON list of all scan elements
    """
    result = basic_scan(image_path)
    return json.loads(result)


def main() -> None:
    print("Binwalk Python SDK - Use basic_scan_py() to scan files")
