(explore-derivatives (re.+ (re.++ (re.* (re.union (re.range "\x0a" "\x0a")(re.union (re.range "\x0d" "\x0d") (re.range " " " "))))(re.++ (str.to_re (seq.++ "/" (seq.++ "/" ""))) (re.* (re.union (re.range "\x00" "\x09")(re.union (re.range "\x0b" "\x0c") (re.range "\x0e" "\xff"))))))))
