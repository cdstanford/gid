(explore-derivatives (re.inter
    ((_ re.^ 30) (re.++ (re.* re.allchar) (str.to_re "a")))
    ((_ re.^ 60) (re.++ (re.* re.allchar) (str.to_re "a")))
    ((_ re.^ 90) (re.++ (re.* re.allchar) (str.to_re "a")))
))
