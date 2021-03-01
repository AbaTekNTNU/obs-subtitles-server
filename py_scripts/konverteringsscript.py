import re


def contains_timestamp(line):
    match_obj = re.match(r'\d:\d\d', line)

    if match_obj:
        return True
    else:
        return False


def format_text(text):
    output_lines = []
    for line in text:
        line = line.strip()
        if not line.endswith(':') and not line == "" and not line.startswith('[') and not contains_timestamp(line):
            if ':' in line:
                line = line.split(':')[1]
            line = line.strip()
            output_lines.append(line + '\n')
            print(line)
    return output_lines


def main():
    input_file = open("input.txt", 'r')
    text = input_file.readlines()
    input_file.close()

    output_lines = format_text(text)

    output_file = open("output.txt", "w")
    output_file.writelines(output_lines)
    output_file.close()


main()
