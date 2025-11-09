def get_user_input():
    while True:
        try:
            return int(input('请输入一个1到100之间的数字：'))
        except ValueError:
            print('请输入一个有效的整数。')

def print_message(message):
    print(message)