(explore-derivatives
    (re.+ (re.++ re.all (str.to_re "a") ((_ re.^ 1000) re.allchar)))
)

