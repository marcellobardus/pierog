#!/usr/bin/env python3

import argparse
import os
import platform
import base64
import requests
import json

from typing import List

# from src.deps_assemblers.cairo import find_cairo_dependencies


# src.deps_assemblers.cairo
def find_cairo_dependencies(content: str) -> list[str]:
    compilation_dependencies = []

    # Split the content by lines
    lines = content.split("\n")
    # Filter the lines that contain import statements
    for line in lines:
        if line.startswith("from"):
            # Split by blank space
            parts = line.split(" ")
            if not parts[1].startswith("starkware"):
                path = parts[1].replace(".", "/")
                compilation_dependencies.append(path + ".cairo")
    return compilation_dependencies


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


print(files_to_zip)
os.system(f"zip -r upload.zip temp")

with open("upload.zip", "rb") as sources:
    zip_data = base64.b64encode(sources.read()).decode("utf-8")
    base_path = "temp/" + base_path
    target_compilation_path = (
        base_path + args.file.split(".")[0].split("/")[-1] + ".cairo"
    )
    workspace_root_path = base_path

    print("Sending the following files to the server:")
    print(target_compilation_path)
    print(workspace_root_path)
    print(zip_data)

    response = requests.post(
        "http://127.0.0.1:4000/upload",
        params={
            "zip_data": zip_data,
            "target_compilation_path": target_compilation_path,
            "workspace_root_path": workspace_root_path,
        },
    )

    print(response.text)
