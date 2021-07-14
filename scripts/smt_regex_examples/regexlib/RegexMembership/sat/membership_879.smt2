(explore-derivatives (re.++ (str.to_re "")(re.++ (re.range "\x22" "\x22")(re.++ (re.+ (re.union (re.range "\x00" "!") (re.range "#" "\xff")))(re.++ (re.range "\x22" "\x22") (str.to_re ""))))))
