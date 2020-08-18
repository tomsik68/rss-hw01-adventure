Rust summer school - assignment 1: Text-based adventure game

# Story file format specification

Story file consists of multiple scene definitions. Every scene definition must have exactly three lines.

On the first line, there is identifier of the scene, which can be any string of [_a-zA-Z0-9-]+ . Scene identifier must be unique within a story.

The second line is the text that is displayed. The text can only span one line. There are no other limits to it.

The third line is definition of transitions to other scenes. It has the format: `transition1,transition2,transition3,transitionN` Where each transition has the following format: `action text:target scene identifier`. Action text must not contain colon, comma or span multiple lines. It must be [_a-zA-Z0-9- ]+ . Action text must be unique in a given scene.

A scene with no transitions is a terminal scene which halts the program. A scene with no transitions always requires an empty line in its place.
