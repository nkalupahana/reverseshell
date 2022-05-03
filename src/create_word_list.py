with open("eff_short_wordlist_1.txt") as f:
    words = [l.split("\t")[1].strip() for l in f.readlines()]
    print('"' + '", "'.join(words) + '"')
    print(len(words))
