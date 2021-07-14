(explore-derivatives (re.++ (re.+ (re.union (re.range "A" "Z") (re.range "a" "z")))(re.++ (re.opt (re.range "-" "-")) (re.+ (re.union (re.range "A" "Z") (re.range "a" "z"))))))
