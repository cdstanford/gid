(explore-derivatives (re.inter (re.++ (re.++ re.all (re.range "0" "9")) ((_ re.^ 5) re.allchar)) (re.++ (re.++ re.all (re.union (re.range (_ char #x0) "/") (re.range ":" (_ char #xFF)))) ((_ re.^ 5) re.allchar))))

