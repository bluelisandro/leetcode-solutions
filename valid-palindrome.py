def palindrome(s: str) -> bool:
    s_alnum = ""
    for char in s:
        if char.isalnum():
            s_alnum += char.lower()
    return s_alnum == s_alnum[::-1]

print(palindrome("race car"))