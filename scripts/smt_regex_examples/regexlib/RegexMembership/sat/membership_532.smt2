(explore-derivatives (re.++ (str.to_re "")(re.++ (re.union (str.to_re (seq.++ "f" (seq.++ "t" (seq.++ "p" "")))) (re.++ (str.to_re (seq.++ "h" (seq.++ "t" (seq.++ "t" (seq.++ "p" ""))))) (re.opt (re.range "s" "s"))))(re.++ (str.to_re (seq.++ ":" (seq.++ "/" (seq.++ "/" ""))))(re.++ (re.opt (re.++ (re.+ (re.union (re.range "\x00" "9") (re.range ";" "\xff")))(re.++ (re.range ":" ":")(re.++ (re.* (re.union (re.range "\x00" "?") (re.range "A" "\xff"))) (re.range "@" "@")))))(re.++ (re.* (re.++ (re.union (re.range "0" "9")(re.union (re.range "A" "Z") (re.range "a" "z")))(re.++ (re.* (re.union (re.range "-" "-")(re.union (re.range "0" "9")(re.union (re.range "A" "Z")(re.union (re.range "_" "_") (re.range "a" "z")))))) (re.range "." "."))))(re.++ (re.++ (re.union (re.range "0" "9")(re.union (re.range "A" "Z") (re.range "a" "z"))) (re.* (re.union (re.range "-" "-")(re.union (re.range "0" "9")(re.union (re.range "A" "Z")(re.union (re.range "_" "_") (re.range "a" "z")))))))(re.++ (re.opt (re.++ (re.range ":" ":") (re.+ (re.range "0" "9"))))(re.++ (re.opt (re.range "/" "/"))(re.++ (re.opt (re.++ (re.* (re.union (re.union (re.range "-" ":")(re.union (re.range "A" "[")(re.union (re.range "]" "]")(re.union (re.range "_" "_")(re.union (re.range "a" "z") (re.range "~" "~")))))) (re.++ (re.range "%" "%") ((_ re.loop 2 2) (re.union (re.range "0" "9")(re.union (re.range "A" "F") (re.range "a" "f")))))))(re.++ (re.range "?" "?") (re.* (re.++ (re.* (re.union (re.union (re.range "," "/")(re.union (re.range ":" ":")(re.union (re.range "=" "=")(re.union (re.range "[" "[")(re.union (re.range "]" "]")(re.union (re.range "_" "_") (re.range "~" "~")))))))(re.union (str.to_re "")(re.union (re.union (re.range "0" "9")(re.union (re.range "A" "Z")(re.union (re.range "a" "{") (re.range "}" "}")))) (re.++ (re.range "%" "%") ((_ re.loop 2 2) (re.union (re.range "0" "9")(re.union (re.range "A" "F") (re.range "a" "f"))))))))) (re.opt (re.range "&" "&")))))))(re.++ (re.opt (re.++ (re.range "#" "#") (re.* (re.union (re.union (re.range "-" ".")(re.union (re.range "0" "9")(re.union (re.range "A" "Z")(re.union (re.range "_" "_") (re.range "a" "z"))))) (re.++ (re.range "%" "%") ((_ re.loop 2 2) (re.union (re.range "0" "9")(re.union (re.range "A" "F") (re.range "a" "f"))))))))) (str.to_re ""))))))))))))
