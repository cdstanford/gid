(explore-derivatives (re.inter
    ((_ re.^ 3) (re.++ (re.* re.allchar) (str.to_re "a")))
    ((_ re.^ 6) (re.++ (re.* re.allchar) (str.to_re "a")))
    ((_ re.^ 9) (re.++ (re.* re.allchar) (str.to_re "a")))
))
