(explore-derivatives (re.++ (re.range "<" "<")(re.++ (re.* (re.union (re.range "\x00" "\x08")(re.union (re.range "\x0e" "\x1f")(re.union (re.range "!" ";")(re.union (re.range "=" "=")(re.union (re.range "?" "\x84")(re.union (re.range "\x86" "\x9f") (re.range "\xa1" "\xff"))))))))(re.++ (re.opt (re.++ (re.union (re.range "\x09" "\x0d")(re.union (re.range " " " ")(re.union (re.range "\x85" "\x85") (re.range "\xa0" "\xa0")))) (re.* (re.union (re.range "\x00" ";")(re.union (re.range "=" "=") (re.range "?" "\xff")))))) (re.range ">" ">")))))
