(explore-derivatives
    (re.inter
        (re.* (str.to_re "aaa"))
        (re.++ (str.to_re "a") (re.* (str.to_re "aaa")))
))

