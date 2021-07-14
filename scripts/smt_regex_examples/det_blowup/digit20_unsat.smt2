(explore-derivatives (re.inter (re.++ (re.++ re.all (re.range "0" "9")) ((_ re.^ 20) re.allchar)) (re.++ (re.++ re.all (re.union (re.range (_ char #x0) "/") (re.range ":" (_ char #x7F)))) ((_ re.^ 20) re.allchar))))

