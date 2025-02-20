import pickle, os, sys
from queue import Queue

input_queue = Queue()
def inputs(string):           #this allows for multiple inputs at the same time
    if(string != ""): print(string)
    if input_queue.empty():
        inputs = input()
        for word in inputs.split(" "):
            input_queue.put(word)
    return input_queue.get()

def load_args():
    sys.argv.pop(0)
    for word in sys.argv:
        input_queue.put(word)

def update_name():
    name = inputs("")
    new_name = inputs("")
    password = accounts[name]
    del accounts[name]
    accounts.update({new_name : password})
    with open(filename, 'wb') as fpickle:
        pickle.dump(accounts, fpickle)

def update_pass():
    name = inputs("")
    new_password = inputs("")
    accounts[name] = new_password
    with open(filename, 'wb') as fpickle:
        pickle.dump(accounts, fpickle)

def add_account():
    name = inputs("")
    password = inputs("")

    accounts.update({name : password})
    with open(filename, 'wb') as fpickle:
        pickle.dump(accounts, fpickle)

def list_accounts():
    with open(filename, 'rb') as fpickle:
        accounts = pickle.load(fpickle)

    if not accounts == {}:
        for account in accounts:
            print(account + "   " + accounts[account])

def help():
    f = open("help.txt")
    while f:
        print(f.readline())
    f.close()

def get():
    name = inputs("")
    print(accounts[name])

def remove():
    name = inputs("")
    del accounts[name]
    with open(filename, 'wb') as fpickle:
        pickle.dump(accounts, fpickle)

def menu():
    command = inputs("")
    if command == "add":
        add_account()
    elif command == "uname":
        update_name()
    elif command == "upass":
        update_pass()
    elif command == "list":
        list_accounts()
    elif command == "get":
        get()
    elif command == "help":
        pass
    elif command == "del":
        remove()
    elif command == "exit":
        return
    else:
        print(command + ": command not recognized")
        #help()

filename = "accounts_details.pkl"
load_args()

#creates a file if it doesnt exist
if not os.path.isfile(filename): 
    with open(filename, 'wb'): pass

#loads the accounts
try:
    with open(filename, 'rb') as pickled_file: 
        accounts = pickle.load(pickled_file)
except EOFError:
    accounts = {}
menu()
