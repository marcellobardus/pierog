def find_cairo_dependencies(content: str) -> list[str]:
    compilation_dependencies = []

    # Split the content by lines
    lines = content.split("\n")
    # Filter the lines that contain import statements
    for line in lines:
        if line.startswith("from"):
            # Split by blank space
            parts = line.split(" ")
            if parts[1] != "starkware":
                compilation_dependencies.append(parts[1] + ".cairo")
    return compilation_dependencies
