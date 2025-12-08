# manager.py
# The manager class is exposed to the user
# Expected usage:

# ```python3
# from pybinwalk import Manager
# Manager(stream=False).handle_deps()		# Disables streaming the download logs in your terminal
# rest_of_the_code()
# ```

import os
import subprocess
from pathlib import Path


class Manager:
    """
    Dependency manager for ubuntu/debian based systems
    """

    def __init__(self, stream: bool = False):
        self.CWD = Path.cwd()
        BASE_DIR = Path(__file__).resolve().parent
        self.ubuntu_sh = BASE_DIR / "ubuntu.sh"
        self.stream = stream

    def handle_deps(self):
        """
        Runs the ubuntu installation script in a subprocess
        """

        # Ensure that the files have execute permissions
        if os.access(self.ubuntu_sh, os.X_OK):
            pass
        else:
            print(f"{self.ubuntu_sh} is not executable, changing permissions")
            subprocess.run(["chmod", "+x", f"{str(self.ubuntu_sh)}"])

        if not self.stream:
            with subprocess.Popen(
                f"{str(self.ubuntu_sh)}",
                shell=True,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
            ) as proc:  # assuming a default /bin/bash shell
                _, err = proc.communicate()
                err_code = proc.returncode
                if err_code != 0:
                    print(f"Dependency installation finished with a non-zero exit code {err_code}")
                    print(err)
        else:
            subprocess.run(
                f"{str(self.ubuntu_sh)}"
            )  # Just run the script without capturing output
