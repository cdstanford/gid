(explore-derivatives (re.inter
    ((_ re.^ 30) (re.++ (re.* re.allchar) (str.to_re "a")))
    (re.* ((_ re.^ 30) (re.++ (re.* re.allchar) (str.to_re "a"))))
))
