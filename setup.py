from setuptools import setup, find_packages

from pybinwalk.version import version

setup(
    name="pybinwalk",
    version=version,
    author="pUrGe12",
    author_email="achintya.jai@owasp.org",
    url="https://github.com/FauvidoTechnologies/binwalk-python-sdk",
    description="Python bindings for binwalk",
    packages=find_packages(),
    install_requires=[
        "requests",
    ],
    classifiers=[
        "Programming Language :: Python :: 3",
        "Operating System :: OS Independent",
    ],
    python_requires=">=3.8",
)
