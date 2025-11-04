To run the program, simply open the `release` folder and start `./to_arabic_numerals` from the command line.


You need to have cmake installed I think.
You may need to write 'export CMAKE_POLICY_VERSION_MINIMUM=3.5' without the quotation in your ~/.bashrc file in order for this program to work, if you are running linux.

However, it may also work without it.

How it works?

Simple, first of all `arboard` crate deals with the clipboard.
If the clipboard is empty or there was an error; the program will instead take input from stdin.
this means that you input the text you want to change manually through the command line
after you input a string, it will change only the numbers from it and print it back into the screen.


Note: it will change 123 into ١٢٣
Some call this hindi numbers.
