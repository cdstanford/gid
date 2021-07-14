(explore-derivatives
    (re.+ (re.++ re.all (str.to_re "a") ((_ re.^ 3) re.allchar)))
)

