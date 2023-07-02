# Swipe Categorization

Rapid manual classification of things.

Often I find myself with lists of things - maybe filenames, images,
dependencies, etc, that I want to split into categories. Images into "keep" or
"delete", filenames into "move here" or "move there", dependencies into
"production dependencies" or "development dependencies". When I have to do this
manually, it often involves going row by row in a spreadsheet, text file, or
worst of all, in a web interface. What I really want is to be presented with
each thing, press one button for one choice and another button for another
choice, and immediately be on to the next decision. Yes, like the
swipe-left/swipe-right feature popular among certain dating apps. But in this
case, it's for arbitrary data. Here are some examples:

## Unruly downloads folder

```sh
ls -d ~/Downloads/* |\
  swipe-categorization \
    --left-option keep \
    --right-option delete \
    --right-file delete.txt
```

I'm asked one by one whether to keep or delete each file. When I'm done, after
reviewing `delete.txt` to be sure, I can use `cat delete.txt | xargs rm` to delete
all the files marked for deletion.

## Work files and personal files got mixed up

```sh
ls -d ~/Documents/* |\
  swipe-categorization \
    --left-option work \
    --right-option personal \
    --left-file work.txt \
    --right-file personal.txt
```

Once I'm done classifying, work.txt and personal.txt each contain the files
according to the classification. I can then use `cat work.txt | xargs -I {} -n1
-d\n echo mv {} ~/Documents/work` to move the work files to the workdir, and the
equivalent command with "personal" substituted for "work" where appropriate to
move the personal files.

## Quickly filtering down a menu

This example uses the short flags for options.

`menu.txt`

```
tuna salad
caesar salad
garden salad
ploughmans sandwich
quiches lorraine
vegan chocolate cake
panna cotta
```

```sh
swipe-categorization -l tasty -r bad -o tasty.txt -i menu.txt
```

Now `tasty.txt` contains all the menu items you chose from `menu.txt`

# Installation

```sh
$ git clone https://github.com/hughrawlinson/swipe-categorization.git
$ cd swipe-categorization
$ cargo install --path .
```

# Coming soon?

A big part of this is the ability to rapidly classify large repositories of
images, which is hard to do without being able to see the image. I am planning a
mode that spawns a window, displays the image, and listens to "left" or "right"
keypresses from the user for classification.
