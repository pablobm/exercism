ALPHABET = "abcdegfhijklmnopqrstuvwxyz"

def is_pangram(phrase):
    unused_chars = set(ALPHABET)

    for char in phrase.lower():
        unused_chars.discard(char)

    return len(unused_chars) == 0
