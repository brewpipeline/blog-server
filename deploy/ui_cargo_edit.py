import sys

def modify_dependencies(filename, dependencies_value, features_value):
    try:
        with open(filename, 'r') as file:
            lines = file.readlines()

        replace_value(lines, '[dependencies]', dependencies_value)
        replace_value(lines, '[features]', features_value)

        with open(filename, 'w') as file:
            file.writelines(lines)

        print(f"Changes applied to {filename}")

    except FileNotFoundError:
        print(f"File {filename} not found.")

# TODO comparison by 10 synbols, reconsider after deploy pipeline more clear
def replace_value(lines, search_block, new_value):
    in_block = False

    for i, line in enumerate(lines):
        if search_block in line:
            in_block = True

        if in_block and line.strip().startswith(new_value[:10]):
            lines[i] = f"{new_value}\n"
            break

if __name__ == "__main__":
    if len(sys.argv) != 4:
        print("Usage: python script.py <filename> <dependencies_value> <features_value>")
        sys.exit(1)

    filename = sys.argv[1]
    dependencies_value = sys.argv[2]
    features_value = sys.argv[3]

    modify_dependencies(filename, dependencies_value, features_value)