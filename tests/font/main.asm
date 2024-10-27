; This will only run if you have implemented the custom instructions LD I, Vx, Vy and LD Vx, Vy, I

include "utils.asm"
include "font.asm"

start:
    HIGH

    LD I, abc
    CALL write_text_i
    
    LD I, abc2
    LD V2, 10
    CALL write_text_i

    LD I, abc3
    LD V2, 20
    CALL write_text_i

    LD I, abc4
    LD V2, 30
    CALL write_text_i

    LD I, abc5
    LD V1, 45
    LD V2, 25
    CALL write_text_i

loop:
    JP loop


abc:  text "ABCDEFGHIJKLMNOP"
abc2: text "QRSTUVWXYZ012345"
abc3: text "6789!@#$%^&*()-="
abc4: text "_+[]\;:'?.<"
abc5: text "DONE!"