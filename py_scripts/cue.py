from os import listdir
from os.path import isfile, join
import keyboard
import requests

server_address = 'http://localhost:3000'


def get_sub_file_list():
    return [f for f in listdir('subs') if isfile(join('subs', f))]


def print_file_names(all_sub_files):
    i = 0
    for file in all_sub_files:
        print(i,": ",file)
        i += 1


def read_sub_file(filename):
    f = open("subs/"+filename, "r")
    lines = f.readlines()
    f.close()
    lines = [line.strip() for line in lines]
    return lines


def post_lyric(lyric):
    x = requests.post(server_address, data={'text': lyric})
    print(x.text)


def play_subtitle(filename):
    print('Playing', filename, "press 'space' to go to next lyric")
    lines = read_sub_file(filename)

    for line in lines:
        print('Next lyric:',line)
        keyboard.wait('space')
        post_lyric(line)

    print('Sketch finished, "space" to clear lyric')
    keyboard.wait('space')
    post_lyric('')


def main():
    global server_address

    sub_file_list = get_sub_file_list()
    while True:
        try:
            post_lyric('')
            print('\n\nPosting to server: "'+server_address+'"')
            print('Choose sketch or write "q" to quit or "s" to change server address:')
            print_file_names(sub_file_list)
        except:
            print('\n\nCOULD NOT POST TO SERVER! Change to a working one using command "s"')

        command = input("\nEnter command: ")

        if command == 'q':
            break

        elif command == 's':
            server_address = input('Enter new server address:')
            print('Server set to: "'+server_address+'"')

        elif command.isdecimal():
            if int(command) < len(sub_file_list):
                filename = sub_file_list[int(command)]
                play_subtitle(filename)
            else:
                print('Sketch does not exist')

        else:
            print('Command not found!')

main()
