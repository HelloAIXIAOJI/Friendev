import random
from utils import get_user_input, print_message

def start_game():
    number_to_guess = random.randint(1, 100)
    attempts = 0
    print_message('欢迎来到猜数字游戏！')
    while True:
        guess = get_user_input()
        attempts += 1
        if guess < number_to_guess:
            print_message('太小了！再试一次。')
        elif guess > number_to_guess:
            print_message('太大了！再试一次。')
        else:
            print_message(f'恭喜你，猜对了！你总共猜了 {attempts} 次。')
            break