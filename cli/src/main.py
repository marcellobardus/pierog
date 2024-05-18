#!/usr/bin/env python3

import argparse
import os

from typing import List

from src.deps_assemblers.cairo import find_cairo_dependencies

parser = argparse.ArgumentParser(
    prog="Pierog",
    description="Upload source files to the server",
    epilog="Text at the bottom of help",
)
parser.add_argument("-f", "--file")
parser.add_argument("-c", "--compiler", default="cairo-0.13.1")
parser.add_argument("-o", "--output", default="upload.zip")


args = parser.parse_args()

base_path_tokens = args.file.split(".")[0].split("/")[:-1]
base_path = ""
for token in base_path_tokens:
    base_path += token + "/"

# Open the file and analyze its imports
with open(args.file, "r") as file:
    file_extension = args.file.split(".")[-1]
    content = file.read()

    compilation_dependencies: List[str] = []

    if file_extension == "cairo":
        compilation_dependencies = find_cairo_dependencies(content)

# Create directory with the main file and all its deps
files_to_zip = [args.file]
for dep in compilation_dependencies:
    dep_path = base_path + dep
    files_to_zip.append(dep_path)

# Create a directory with the main file and all its dependencies
if os.path.exists("temp/"):
    os.system("rm -rf temp")
os.system("mkdir temp")
for file in files_to_zip:
    os.system(f"cp {file} temp/")
os.system(f"zip -r upload.zip temp")


print(files_to_zip)
