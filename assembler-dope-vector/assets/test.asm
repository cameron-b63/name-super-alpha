# I'm a comment and I get parsed just fine!
    add     $v0, $v0, $v0
    lui     $t0, 63
    add     $t0, $t0, $t0       # Look at me, I'm an inline comment and I work too!
    j       grug

# I sure hope that empty line worked out, but here's a label with a colon in the meantime
grug:
    lui     $v0, 5