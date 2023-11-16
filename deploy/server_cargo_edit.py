import sys

def modify_dependencies(filename):
    try:
        with open(filename, 'r') as file:
            lines = file.readlines()

        for i, line in enumerate(lines):
            if '[dependencies.blog-ui]' in line:
                # Add # in the next line
                lines[i + 1] = '#' + lines[i + 1].lstrip()

                # Remove # at the beginning of the second line
                lines[i + 2] = lines[i + 2].lstrip('#')

        with open(filename, 'w') as file:
            file.writelines(lines)

        print(f"Changes applied to {filename}")

    except FileNotFoundError:
        print(f"File {filename} not found.")

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python script.py <filename>")
        sys.exit(1)

    filename = sys.argv[1]
    modify_dependencies(filename)