(explore-derivatives (re.++ (re.++ (str.to_re "") ((_ re.loop 5 5) (re.range "0" "9"))) (re.++ (re.opt (re.union (re.range " " " ") (re.range "-" "-")))(re.++ (re.opt ((_ re.loop 4 4) (re.range "0" "9"))) (str.to_re "")))))
