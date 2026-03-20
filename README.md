from zero to master in rust.


Hi, I'm new to programming. Rust is my first language and I chose elf reader as my first project which I want to expand to more detailed analyses. 
The solution will definitely not be professional, but I will try to optimize it over time to make it as good as possible

Last pull i comment fn set_architecture where i taking e_ident and distinguish elf32 or 64 with if conditions and try make more clear function.
I made parse_ident function for distinguish elf32-64 where i parse full e_ident because i try make this more clear.

Destruct Struct with e_ident and make elf valid or invalid with match pattern. I try do this but i still think about more clear version

Today fight with nvim primary :D, and made little update with elf identification struct and make e_ident constants for later

No time but still try to make code better and try new variant.
