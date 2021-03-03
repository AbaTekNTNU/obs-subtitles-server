from os import listdir
from os.path import isfile, join
import keyboard
import requests

server_address = ''


def read_server_address():
    global server_address
    f = open("server_address.txt", "r")
    server_address = f.read()
    f.close()


def write_server_address():
    global server_address
    f = open("server_address.txt", "w")
    f.write(server_address)
    f.close()


def get_sub_file_list():
    files = [f for f in listdir('subs') if isfile(join('subs', f))]
    files.sort()
    return files


def print_file_names(all_sub_files):
    i = 1
    for file in all_sub_files:
        print(i, ": ", file)
        i += 1


def read_sub_file(filename):
    f = open("subs/"+filename, "r")
    lines = f.readlines()
    f.close()
    lines = [line.strip() for line in lines]
    return lines


def post_lyric(lyric):
    x = requests.post(server_address, data={'text': lyric})


def clear_terminal():
    print('\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n')


def play_subtitle(filename):
    clear_terminal()
    print('\n\nPlaying"', filename, '"press "space" to go to next lyric, "esc" to stop\n')
    lines = read_sub_file(filename)

    for line in lines:
        print('\n\n\nUp next:', line)
        while True:
            event = keyboard.read_event()
            if event.event_type == keyboard.KEY_UP:
                if event.name == 'space':
                    post_lyric(line)
                    clear_terminal()
                    print(line)
                    break
                if event.name == 'esc':
                    post_lyric('')
                    return


    print('Sketch finished, "space" to clear lyric')
    keyboard.wait('space')
    post_lyric('')


def main():
    global server_address
    read_server_address()

    sub_file_list = get_sub_file_list()
    while True:
        try:
            post_lyric('')
            print('\n\nPosting to server: "'+server_address+'"')
            print('Enter number to choose sketch or enter letter for command\n'
                  '"q" to quit\n'
                  '"s" to change server address\n')
            print_file_names(sub_file_list)
        except:
            print('\n\nCOULD NOT POST TO SERVER! Change to a working one using command "s"')

        command = input("\nSketch/command: ").strip()

        if command == 'q':
            break

        elif command == 's':
            server_address = input('Enter new server address:')
            write_server_address()
            print('Server set to: "'+server_address+'"')

        elif command.isdecimal():
            if int(command) <= len(sub_file_list):
                filename = sub_file_list[int(command)-1]
                play_subtitle(filename)
            else:
                print('Sketch does not exist')

        else:
            print('Command not found!')

main()
