(explore-derivatives (re.++ (str.to_re "")(re.++ (re.union (re.range "." "/")(re.union (re.range ":" ":")(re.union (re.range "h" "h")(re.union (re.range "p" "p")(re.union (re.range "t" "t")(re.union (re.range "w" "w") (re.range "|" "|")))))))(re.++ (re.+ (re.union (re.range "\x00" "\x08")(re.union (re.range "\x0e" "\x1f")(re.union (re.range "!" "\x84")(re.union (re.range "\x86" "\x9f") (re.range "\xa1" "\xff")))))) (str.to_re "")))))
