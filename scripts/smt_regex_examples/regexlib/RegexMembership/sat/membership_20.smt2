(explore-derivatives (re.++ (str.to_re "")(re.++ (re.opt (re.++ (re.union (re.range "0" "9")(re.union (re.range "A" "Z")(re.union (re.range "_" "_")(re.union (re.range "a" "z")(re.union (re.range "\xaa" "\xaa")(re.union (re.range "\xb5" "\xb5")(re.union (re.range "\xba" "\xba")(re.union (re.range "\xc0" "\xd6")(re.union (re.range "\xd8" "\xf6") (re.range "\xf8" "\xff")))))))))) (re.range ":" ":")))(re.++ (re.union (re.range "/" "/") (str.to_re (seq.++ "\x5c" (seq.++ "\x5c" ""))))(re.++ (re.* (re.union (re.range "\x00" ".")(re.union (re.range "0" "[")(re.union (re.range "]" "{") (re.range "}" "\xff"))))) (re.union (re.range "/" "/") (re.range "\x5c" "\x5c")))))))
