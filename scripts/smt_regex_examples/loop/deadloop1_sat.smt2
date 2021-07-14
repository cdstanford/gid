(explore-derivatives (re.union (re.++ (re.++ (re.++ (str.to_re "abcd") (re.* (str.to_re "bcd"))) (str.to_re "cd")) (re.inter (str.to_re "a") (str.to_re "b"))) (str.to_re "abcdefg")))
