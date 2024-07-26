# WEATHER BOT 

## Quick Sidenote:
I already covered the logic behind the weather aspect of the program, so I will
not be covering it here. It'll be under the *Weather* section on my github repos.
This project requires an API key and Guild Id to be set in the environment variables.
You add the a bot to your discord developer portal then, click on the bot url to add
the bot to a server. After that you set the variables in this program to your desired
Guild Id and discord token, you can run the program. Change your User agent in the 
projram to your information as it is required from the weather API. To execute the 
program you use "cargo shuttle run" instead of "cargo run".

## About:
The idea behind the project is to create a discord bot to get weather updates. This
way I don't have to open up another browser just to google weather conditions,
before riding my motorcycle into a rain storm. If you don't ride, it's not a fun time 
I assure you.

### Initial Problems:
- I don't know how to create a discord bot in any fashion
- I am not sure if I can even make one in Rust
- Do I need to learn another API
- Will my weather logic still work in this context
- If it still works in this context will it clash with this code 


### Step 1 - The Research:
When I did some sophisticated research, I just googled it... I found a how to make a
discord bot in rust. Pretty much sums it all up with steps. It's honestly not rocket
science. This knocks out the first two problems immediately, we like that.


### Step 2 - The API
I tried following the steps and they are a little outdated. I can get the hello 
world equivalent up and running but nothing more. There seems to be some refactoring
going on with the Serenity (Rust Discord API) that changes some of the crucial 
functions to add commands or even get it to work outside of the basic hello world.
It just means I have to read the literature on Serenity to learn what needs attention.

After seeing how the examples are done I think I can get it working with some slight
code changes. Starting off with changing the function calls to their correctly named
functions. Then I can add a simple command to the "commands" directory, like the 
example, to just output a string. Add the register command to the main.rs ready function.
Add the command to the interaction_create function. Test if the command shows up on the 
discord bot, it does. Test if the ouput matches the test string, it does. We've made
some progress.


### Step 3 - The Weather Structure
Now the interesting part of the project, adding the weather functionality to the bot.
So far, I have the bot that takes in commands, then outputs strings. And a weather API
that creates a struct, which I can println it's attributes. I have to figure out how
to mesh these things together.

I added the "async" declaration to the run command for the bot as well as the weather struct. 
This way I can just plug in all of the async functions to the command so I don't have to deal 
with any variables going out of scope. No dice, the compiler is asking for something about lifetime 
parameters. Plus now to think about it, I will be calling the weather api EVERY time to function is
called. This also creates the struct, completly unnecessary.

I added the struct to the "ready" function of the main.rs file. Now instead of everything going
out of scope after the function call, the program only has to call the weather API once. I am still 
unable to call the member variables to the struct as it is not in the scope of the function. Too easy,
pass it in as a parameter. Test the new function. *insert catchphrase here*. Finishing up the project
is really just formatting some of the outputs. Nothing noteworthy worth mentioning. Thank you for 
following along.
