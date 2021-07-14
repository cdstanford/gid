(explore-derivatives (re.++ (re.++ (str.to_re "xyzbc") (re.* (str.to_re "abcde"))) (re.++ (str.to_re "abxxxxxxxxxxxxxxxxxxxxxx") (re.inter
    (re.+ (str.to_re "a"))
    (re.* (str.to_re "b"))
))))
