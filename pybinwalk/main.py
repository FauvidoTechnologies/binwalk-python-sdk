import json

import rust


# The basic scan type
def basic_scan(image_path: str):
    """
    Function to perform a basic binwalk scan.

    `equivalent CLI command`: binwalk image_path

    returns:
            A JSON list of all scan elements
    """
    result = rust.basic_scan(image_path)
    return json.loads(result)
