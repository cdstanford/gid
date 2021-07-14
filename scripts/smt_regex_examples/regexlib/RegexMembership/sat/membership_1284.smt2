(explore-derivatives (re.++ (str.to_re "")(re.++ (re.opt (str.to_re (seq.++ "D" (seq.++ "K" (seq.++ "-" "")))))(re.++ ((_ re.loop 4 4) (re.range "0" "9")) (str.to_re "")))))
