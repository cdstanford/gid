(explore-derivatives (re.++ (str.to_re "")(re.++ (str.to_re (seq.++ "0" (seq.++ "1" ""))) ((_ re.loop 8 8) (re.range "0" "9")))))
