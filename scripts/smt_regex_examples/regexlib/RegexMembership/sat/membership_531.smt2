(explore-derivatives (re.++ (str.to_re "")(re.++ (re.opt (str.to_re (seq.++ "A" (seq.++ "T" ""))))(re.++ (re.opt (re.range "U" "U"))(re.++ ((_ re.loop 8 8) (re.range "0" "9")) (str.to_re ""))))))
