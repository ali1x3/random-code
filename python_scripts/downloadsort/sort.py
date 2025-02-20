import pickle
import os,shutil

PATH = r"C:\\Users\\User\\Downloads\\" #change this according to which directory you want to sort

def move_file(file_to_sort, file_extension):
    print("Moving " + file_to_sort + "...")
    try:
        shutil.move(PATH + file_to_sort, PATH + file_extension + " files\\" + file_to_sort)
        print("Successful!")

    except FileNotFoundError:
        os.makedirs(PATH + file_extension + " files")
        print("Directory does not exist... Creating directory...")
        print("Moving " + file_to_sort + "...")
        shutil.move(PATH + file_to_sort, PATH + file_extension + " files\\" + file_to_sort)
        print("Successful!")
    except:
        print("something has went wrong...")

all_files = [file for file in os.listdir(PATH) if not file.startswith('.')] #only visible files
files_for_sorting = []

for file in all_files:  #storing all file extensions (including blank which means folder)
    tuple = os.path.splitext(file)
    file_extension = tuple[1].replace('.', '')

    if file_extension != '': #filtering all folders as we dont want to touch them
        move_file(file, file_extension)
    else: 
        print("'" + file + "'" + " is a folder and will not be moved")