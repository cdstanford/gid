(explore-derivatives (re.++ (str.to_re "")(re.++ (re.union (re.++ ((_ re.loop 1 3) (re.range "0" "9"))(re.++ (re.range "'" "'")(re.++ (re.* (re.++ ((_ re.loop 3 3) (re.range "0" "9")) (re.range "'" "'")))(re.++ ((_ re.loop 3 3) (re.range "0" "9")) (re.opt (re.++ (re.range "." ".") ((_ re.loop 1 3) (re.range "0" "9")))))))) (re.++ ((_ re.loop 1 3) (re.range "0" "9")) (re.opt (re.++ (re.range "." ".") ((_ re.loop 3 3) (re.range "0" "9")))))) (str.to_re ""))))
