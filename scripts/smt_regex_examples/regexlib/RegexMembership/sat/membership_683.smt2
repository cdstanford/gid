(explore-derivatives (re.++ (str.to_re "")(re.++ (re.opt (re.range "{" "{"))(re.++ ((_ re.loop 32 32) (re.union (re.range "0" "9")(re.union (re.range "A" "F") (re.range "a" "f"))))(re.++ (re.opt (re.range "}" "}")) (str.to_re ""))))))
