(explore-derivatives (re.inter
    ((_ re.^ 10) (re.++ (re.* re.allchar) (str.to_re "a")))
    (re.* ((_ re.^ 10) (re.++ (re.* re.allchar) (str.to_re "a"))))
))
