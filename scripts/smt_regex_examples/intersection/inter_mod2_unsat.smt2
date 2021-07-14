(explore-derivatives
    (re.inter
        (re.* (str.to_re "ab"))
        (re.++ (str.to_re "a") (re.* (str.to_re "ba")))
))

