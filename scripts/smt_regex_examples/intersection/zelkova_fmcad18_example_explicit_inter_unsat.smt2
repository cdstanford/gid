(explore-derivatives (re.inter (re.++ (re.++ (re.++ (re.++ (re.++ (re.++ (str.to_re "ab") re.all) (str.to_re "b")) re.all) (str.to_re "b")) re.all) (str.to_re "b")) (re.union (re.inter (re.++ (re.++ (re.++ (re.++ (re.++ (re.++ (str.to_re "a") re.all) (str.to_re "b")) re.all) (str.to_re "b")) re.all) (str.to_re "b")) re.none) (re.inter (re.comp (re.++ (re.++ (re.++ (re.++ (re.++ (re.++ (str.to_re "a") re.all) (str.to_re "b")) re.all) (str.to_re "b")) re.all) (str.to_re "b"))) re.all))))

