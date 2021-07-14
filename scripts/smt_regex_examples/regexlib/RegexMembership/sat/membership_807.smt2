(explore-derivatives (re.++ (str.to_re "")(re.++ ((_ re.loop 1 1500) (re.range "\x00" "\xff")) (str.to_re ""))))
