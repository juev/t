# t

`t` is a command-line todo list manager for people that want to *finish* tasks,
not organize them.

This is fork [sjl/t](https://github.com/sjl/t) project on c++ language. You need 
only one binary file for work with your task.

**Important**: file format was changed. You cannot use file from sjl/t in this program.  

# Using t

`t` is quick and easy to use.

### Add a Task

To add a task, use `t [task description]`:

    $ t Clean the apartment.
    $ t Write chapter 10 of the novel.
    $ t Buy more beer.
    $

### List Your Tasks

Listing your tasks is even easier -- just use `t`:

    $ t
    9  - Buy more beer.
    30 - Clean the apartment.
    31 - Write chapter 10 of the novel.
    $

`t` will list all of your unfinished tasks and their IDs.

### Finish a Task

After you're done with something, use `t -f ID` to finish it:

    $ t -f 31
    $ t
    9  - Buy more beer.
    30 - Clean the apartment.
    $

### Edit a Task

Sometimes you might want to change the wording of a task.  You can use
`t -e ID [new description]` to do that:

    $ t -e 30 Clean the entire apartment.
    $ t
    9  - Buy more beer.
    30 - Clean the entire apartment.
    $

Yes, nerds, you can use sed-style substitution strings:

    $ t -e 9 /more/a lot more/
    $ t
    9  - Buy a lot more beer.
    30 - Clean the entire apartment.
    $

### Delete the Task List if it's Empty

If you keep your task list in a visible place (like your desktop) you might
want it to be deleted if there are no tasks in it.  To do this automatically
you can use the `--delete-if-empty` option in your alias:

    alias t='t --task-dir ~/Desktop --list todo.txt --delete-if-empty'
