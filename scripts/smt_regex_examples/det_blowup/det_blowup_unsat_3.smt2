(explore-derivatives (re.inter
    (re.++ re.all (str.to_re "a") ((_ re.^ 3) re.allchar))
    (re.++ re.all (str.to_re "b") ((_ re.^ 3) re.allchar))
))

