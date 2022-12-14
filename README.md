# goober

My sister posed a game involving emojis. Given a word, represent it with emojis without using the emoji that directly indicates the word (i.e. the solution for "cat" can't be "π").

I toyed with a few ideas before admitting it isn't as easy as it initially sounds. The rules are simple and vague enough to allow for creativity and some amusement. Since it's my sister's game, I dubbed it Kaley Encoding.

## Rules of the Game

An interesting property of the game is that it doesn't specify a single correct solution for every input. In other words, one could use the emojis for ears, eyes, and a tongue to represent a dog. It would also be valid to use the first character of each emoji's name (i.e. "π" for 'c' and "π" for 'd' because the emojis are a cat and dog, respectively).

There isn't a rule that solutions must be unique or even trivially decodable! This aspect undoubtedly makes the game more fun when not using a computer. (The example above using ears, eyes, and a tongue was apparently a horse, according to my sister.)

## A Heartfelt Solution

This program takes a single word as input and outputs a solution. If it can't find a solution, it fails. But that doesn't mean there _isn't_ a solution. My attempt borrows ideas from Huffman coding and maximum contiguous subsequences. I think. It's been a while since I looked at either topic.

Emojis have a name (e.g. "π" is "cat" and "π" is "dog"). My solution is greedy. It compares the input to the names of emojis, finds the longest sequence of matching characters, and records the emoji-count pairing. The result is a sequence of emojis where its repitition count indicates how many characters from the name can be read. The count is given by the color of the heart: no heart means one letter, a red heart means two, orange means three, etc. How can we remember what the colors mean? [ROY G BIV](https://en.wikipedia.org/wiki/ROYGBIV)βwhere red means two. I chose hearts because the world could always use a little more love and kindness when communicating.

Hopefully examples yield a better explanation:

```
$ goober --reference -i cat
Reference:
π΅ cactus 
π¦ t-rex  

Result:   
π΅β€οΈπ¦  
```

The encoding for "cat" is a cactus emoji followed by a red heart and a t-rex. The red heart indicates we use two letters from the preceding emoji while no heart following the t-rex means only use one letter. Using the reference, the result is "ca" from cactus and "t" from t-rex. Cat!

This works for nonsensical inputs (some emojis appear wide on some systems):

```
$ goober --reference -i asdf
Reference:     
π§βπ astronaut
π‘οΈ dagger      
π­ factory     

Result:        
π§βπβ€οΈπ‘οΈπ­
```

As well as more complex inputs:

```
$ goober --reference -i supercalifragilisticexpialidocious
Reference: 
π¦Έ superhero
π calendar
π§ ice      
πΈ frog    
π§? abacus   
π¦ giraffe 
β libra    
ποΈ stadium   
π§ ice        
π©» x-ray      
βοΈ pick      
π½ alien     
π¦€ dodo       
π¬ cigarette 
π octopus   
βοΈ umbrella  
β sagittarius

Result:
π¦Έπππ§‘π§πΈβ€οΈπ§?π¦β€οΈββ€οΈποΈβ€οΈπ§π§‘π©»βοΈβ€οΈπ½π§‘π¦€β€οΈπ¬β€οΈπβοΈβ
```

But if an emoji can't be found for even a single character, the program will fail:

```
$ goober --reference -i asdfqwerty
panicked at 'No sequence available'
```

## Future Work

It's a game. I don't want to toy with it further.
