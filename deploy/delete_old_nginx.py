
import sys
import os

def main():
    folder_path = sys.argv[1]
    new_file_names = sys.argv[2].split(',')

    try:
        files = os.listdir(folder_path)
        for file_name in files:
            # Check if the file name starts with the first 6  symbols of any path
            # TODO deploy nginx entirely with ansible?
            if file_name not in new_file_names and any(file_name.startswith(new_file_name[:6]) for new_file_name in new_file_names):
                file_path = os.path.join(folder_path, file_name)
                os.remove(file_path)
    except FileNotFoundError:
        print(f"The folder '{folder_path}' does not exist.")

if __name__ == "__main__":
    main()