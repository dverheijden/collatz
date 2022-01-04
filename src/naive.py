from datetime import datetime
from functools import lru_cache

from tqdm import trange


@lru_cache(maxsize=None)
def get_next(number: int) -> int:
    if number <= 1:
        return 1
    if number % 2 == 0:
        return number // 2
    return 3 * number + 1


def get_path_length(number: int) -> int:
    path_length = 0
    while number != 1:
        number = get_next(number)
        path_length += 1
    return path_length


def main() -> None:
    max_path_length = 0
    number_which_produces_longest_path = 0
    now = datetime.now()
    for number in trange(1000000):
        path_length = get_path_length(number)
        if path_length > max_path_length:
            max_path_length = path_length
            number_which_produces_longest_path = number
    print(
        f"Largest path: {max_path_length}\n"
        f"Number which produced it: {number_which_produces_longest_path}\n"
        f"Time elapsed: {(datetime.now() - now).seconds}s"
    )


if __name__ == "__main__":
    main()
