import json

from pybinwalk import _rust


def scan(image_path: str):
    """
    Function to perform a basic binwalk scan.

    `equivalent CLI command`: binwalk image_path

    returns:
            A JSON list of all scan elements
    """
    result = _rust.scan(image_path)
    return json.loads(result)


def extract(image_path: str, output_path: str = None):
    """
    Function to extract data into a specified directory

    `equivalent CLI command`: binwalk -e image_path

    Args:
        `image_path`: Path to the image file
        `output_dir`: The output directory to save the extracted contents too
    """
    if not output_path:
        output_path = "/tmp/binwalk-extraction"

    result = _rust.extract(image_path, output_path)
    return json.loads(result)
