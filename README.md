# Kaley Encoding

My sister posed a game involving emojis. Given a word, represent it with emojis without using the emoji that directly indicates the word (i.e. the solution for "cat" can't be "🐈").

I toyed with a few ideas before admitting it isn't as easy as it initially sounds. The rules are simple and vague enough to allow for creativity and some amusement. Since it's my sister's game, I dubbed it Kaley Encoding.

## Rules of the Game

 An interesting property of the game is that it doesn't specify a single correct solution for every input. In other words, one could use the emojis for ears, eyes, and a tongue to represent a dog. It would also be valid to use the first character of each emoji's name (i.e. "🐈" for 'c' and "🐕" for 'd' because the emojis are a cat and dog, respectively).

There isn't a rule that solutions must be unique or even trivially decodable! This aspect undoubtedly makes the game more fun when not using a computer. (The example above using ears, eyes, and a tongue was apparently a horse, according to my sister.)

## A Solution

This program takes a single word as input and outputs a solution. If it can't find a solution, it fails. But that doesn't mean there _isn't_ a solution. My attempt borrows ideas from Huffman coding and maximum contiguous subsequences. I think. It's been a while since I looked at either topic.

Emojis have a name (e.g. "🐕" is "cat" and "🐕" is dog). My solution is greedy. It compares the input to the names of emojis, finds the longest sequence of matching characters, and records the emoji-count pairing. The result is a sequence of emojis where its repitition count indicates how many characters from the name can be read. The count is given by the color of the heart: no heart means one letter, a red heart means two, orange means three, etc. How can we remember what the colors mean? [ROY G BIV](https://en.wikipedia.org/wiki/ROYGBIV)—where red means two.

Hopefully examples yield a better explanation:

```
$ ke --reference -i cat
Reference:
🌵 cactus 
🦖 t-rex  

Result:   
🌵❤️🦖  
```

There are two cactus emojis followed by a t-rex. I interpret this to mean the first two characters of "cactus" followed by the first character of "t-rex": cat.

This works for nonsensical inputs (some emojis are wide, like the astronaut):

```
$ ke --reference -i asdf
Reference:     
🧑‍🚀 astronaut
🗡️ dagger      
🏭 factory     

Result:        
🧑‍🚀❤️🗡️🏭
```

As well as more complex inputs:

```
$ ke --reference -i supercalifragilisticexpialidocious
Reference: 
🦸 superhero
📅 calendar
🧊 ice      
🐸 frog    
🧮 abacus   
🦒 giraffe 
♎ libra    
🏟️ stadium   
🧊 ice        
🩻 x-ray      
⛏️ pick      
👽 alien     
🦤 dodo       
🚬 cigarette 
🐙 octopus   
☂️ umbrella  
♐ sagittarius

Result:
🦸💚📅🧡🧊🐸❤️🧮🦒❤️♎❤️🏟️❤️🧊🧡🩻⛏️❤️👽🧡🦤❤️🚬❤️🐙☂️♐
```

But if an emoji can't be found for even a single character, the program will fail:

```
$ ke --reference -i asdfqwerty
panicked at 'No sequence available'
```

## Future Work

It's a game. I don't want to toy with it further.