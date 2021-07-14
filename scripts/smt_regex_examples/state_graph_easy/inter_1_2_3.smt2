(explore-derivatives (re.inter
    ((_ re.^ 1) (re.++ (re.* re.allchar) (str.to_re "a")))
    ((_ re.^ 2) (re.++ (re.* re.allchar) (str.to_re "a")))
    ((_ re.^ 3) (re.++ (re.* re.allchar) (str.to_re "a")))
))
