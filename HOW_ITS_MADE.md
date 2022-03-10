# How it's made - a recipe for disaster
So, you want to know how this hot mess works. Me too, to be honest, but I can explain.

## Ingredients
 - One cup `gcc`
 - A sprinkle of `proc_macro`
 - A pinch of `mprotect(1)`
 - Enough `objcopy` flour
 - Steep all in copious amounts of memory-unsafety sauce

## No, seriously.
So, this has two parts to it. One part is the actual executor, the other part is the proc macro that enables this in the first place.

### The executor
The executor is a sequential chain of crimes. This is what it does, roughly in the correct order:
- It writes your source code to `temp_file.c` on the disk.
- It invokes `gcc` on it to compile it. (It does not link it.)
- It uses `objcopy` to obtain a flat binary containing just your code (not an ELF).
- It allocates one megabyte of the finest page-aligned memory.
- It reads the binary produced by `objcopy` into that buffer it allocated earlier.
- It deletes all the files it created, because it has what it needs now.
- It then uses `mprotect(1)` to mark it as readable, writable and executable in memory.
- After all that's done, it jumps.

As you can probably tell, this is possibly the single unsafest thing one can do in Rust.

### The proc macro
The proc macro is what is responsible for making your C code behave. This one's a bit complex.

It uses the Rust tokeniser to tokenise the C code (yes, this works in most cases.), then mostly-accurately reconstructs your original C source code from it, preserving whitespace to provide accurate error messages... ~~or it would if I didn't drop errors silently, anyway~~. This part of the code is mostly based on [Mara Bos](https://m-ou.se/)'s work on [inline-python](https://github.com/fusion-engineering/inline-python). Thanks for your [excellent blog series on how to do this](https://blog.m-ou.se/writing-python-inside-rust-1/), Mara! (FYI, this is also how [inline-lua](https://github.com/ry00001/inline-lua) works).  

During this phase, the reconstructor is on the lookout for `'` tokens (single quote). In Rust, this is normally used to specify lifetimes, so `'identifier` is kosher in the tokeniser's eyes. I use this fact to watch for them, and when I find one, I note it down, and replace it with `((void(*)()){})`.

### What?

Okay, so there's a lot to unpack in `((void(*)()){})`. Let's try to break it down.

The most of the parentheses come from [C's arcane function pointer syntax](https://www.cprogramming.com/tutorial/function-pointers.html). This encompasses everything except for the `{}`s, and is a cast to a function that takes no arguments and returns nothing.

Now, onto the braces. This is by far the most crimey part of this entire project.

### The braces
If you're familiar with Rust, you've probably immediately noticed one thing.

`{}` is how you do a format string in Rust.

At the end of the proc macro, you'd think I'd emit `compile_c("your code as a string literal")`. But I don't. What I actually emit is `compile_c(&format!("your code as a string literal", <magic goes here>))`. This is what the `{}` is for.

I note down what the `something` in `'something` is, and this is where I use it. So, I set up a `Vec` consisting of `TokenStream`s, which is what `quote!` outputs, and I enumerate my list of references. For each one, I emit Rust code that looks like `((something as *const ()) as u64)`.

There's a couple things to note here. First one is `u64`, meaning that it will probably only behave on 64-bit systems. Second one is `something`. Because of when macros run in Rust, I can't simply get the address of `something` then. So, what the macro does is write code that fetches that address at runtime, and format-strings it into the C source for you.

I'm sorry, lmao.

## To wrap things up
This is a horrible idea, do not try any of this at home. This is probably the most unsafe thing you can possibly do in Rust. This was made simply because I wanted to.

But if you've made it here, thank you! Thanks for reading this document, and for checking this repository out in the first place. I hope your day goes really well <3

~ [Rin](https://twitter.com/lostkagamine) <3