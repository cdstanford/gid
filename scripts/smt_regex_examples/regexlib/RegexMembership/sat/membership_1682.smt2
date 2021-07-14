(explore-derivatives (re.++ (str.to_re "")(re.++ ((_ re.loop 1 5) (re.union (re.range "!" "!") (re.range "#" "~"))) (str.to_re ""))))
