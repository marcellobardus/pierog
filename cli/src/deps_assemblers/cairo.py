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
