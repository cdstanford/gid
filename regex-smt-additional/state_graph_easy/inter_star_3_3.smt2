(explore-derivatives (re.inter
    ((_ re.^ 3) (re.++ (re.* re.allchar) (str.to_re "a")))
    (re.* ((_ re.^ 3) (re.++ (re.* re.allchar) (str.to_re "a"))))
))
