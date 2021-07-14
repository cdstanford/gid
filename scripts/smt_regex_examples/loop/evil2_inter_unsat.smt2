(explore-derivatives (re.inter
        (re.++ (re.++ (re.+ (re.++ (re.+ (re.range (_ char #x1) (_ char #x7F))) re.allchar)) (str.to_re (_ char #x0))) (re.+ (re.range (_ char #x1) (_ char #x7F))))
    (re.* (re.union (str.to_re "a") (str.to_re "b")))
))
