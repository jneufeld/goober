# goober

My sister posed a game involving emojis. Given a word, represent it with emojis _without_ using the emoji that directly indicates the word. E.g. the solution for "cat" can't include "🐈". Since it's my sister's game, I dubbed it Kaley Encoding.

## Rules

The rules are simple and vague enough to allow for creativity and some amusement. There's no single correct solution for every input; I could use the emojis for ears, eyes, and a tongue to represent a dog (my sister claimed it was a horse).

## Solution

This program takes a single word as input and outputs a solution. If it can't find a solution, it fails.

This solution compares the input to the names of emojis (e.g. "🐕" is `cat` and "🐕" is `dog`), finds the longest sequence of matching characters, and records the emoji-count pairing. The result is a sequence of emojis where the count indicates how many characters from the name can be read. The count is given by the color of the heart: no heart means one letter, a red heart means two, orange means three, etc. How can we remember what the colors mean? [ROY G BIV](https://en.wikipedia.org/wiki/ROYGBIV)—where red means two.

I chose hearts because the world could always use a little more love and kindness when communicating.

## Examples

```
$ goober --reference -i cat
Reference:
🌵 cactus
🦖 t-rex

Result:
🌵❤️🦖
```

The encoding for "cat" is a cactus emoji followed by a red heart and a t-rex. The red heart indicates we use two letters from the preceding emoji while no heart following the t-rex means only use one letter. Using the reference, the result is "ca" from cactus and "t" from t-rex. Cat!

This works for nonsensical inputs (NB some emojis appear wide):

```
$ goober --reference -i asdf
Reference:
🧑‍🚀 astronaut
🗡️ dagger
🏭 factory

Result:
🧑‍🚀❤️🗡️🏭
```

As well as more complex inputs:

```
$ goober --reference -i supercalifragilisticexpialidocious
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

If an emoji can't be found for even a single character, the program will fail:

```
$ goober --reference -i asdfqwerty
panicked at 'No sequence available'
```

## Future Work

Nope.
