(explore-derivatives (re.++ (str.to_re "")(re.++ ((_ re.loop 0 16) (re.union (re.union (re.range "\x00" "\x09") (re.range "\x0b" "\xff")) (re.range "\x0a" "\x0a"))) (str.to_re ""))))
