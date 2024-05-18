#!/usr/bin/env python3

import argparse
import os
import platform
import base64

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
    os_system = platform.system()
    if os_system == "Darwin":
        os.system(f"rsync -R {file} temp/")
    elif os_system == "Linux":
        os.system(f"cp --parents {file} temp/")
    else:
        print("Unsupported OS")
        exit(1)

# Remove some garbage files
# for file in os.listdir("temp/"):
#     if not os.path.isdir(f"temp/{file}"):
#         os.system(f"rm temp/{file}")
#     result = subprocess.run(
#         [f"pwd", "cd temp/", f"mv {base_path}* ./"],
#         stdout=subprocess.PIPE,
#         stderr=subprocess.PIPE,
#         text=True,
#     )
#     print(result.stdout)

print(files_to_zip)
os.system(f"zip -r upload.zip temp")

with open("upload.zip", "rb") as sources:
    base64_encoded = base64.b64encode(sources.read())
    explicit_path = base_path + args.file.split(".")[0].split("/")[-1] + ".cairo"
    workspace_root = base_path

    print("Sending the following files to the server:")
    print(explicit_path)
    print(workspace_root)
    print(base64_encoded)
