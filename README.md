# light-riders Bot
My implementation for [riddles.io light riders competition](https://starapple.riddles.io/competitions/light-riders) as my first rust project.

My first approach is the python-version, which works similar, but not as good as the new rust version.
My strategic approach is based on mini-max algorithm and has two parts.
Part 1: My score metric compares the amount of fields I can reach first vs the amount of fields the enemy can reach first. 
I also tried various other metrics, including points for the distance, but they performed a little worse.
Part 2: If the enemy is not reachable any more, my bot tries to keep as many edges as possible to survive long. 

Every iteration the time is stopped and my search depth will be adjusted. So far it is tuned for timebank 10000 and time_per_move 200.

## Build Instructions

Run ```RUSTFLAGS="-Ctarget-cpu=native" cargo build --release``` for getting the optimized program in the target/release folder. 
You can then just execute it in a local program like this [Ai Bot Workspace] (https://github.com/jmerle/ai-bot-workspace).
Just add the path for 'target/release/lightriders-rust' and give it a name. 

## Future Work
Rust Version seems still to be slower or at least not faster than python, which seems a bit odd to me. 
 an internal competition it seemed there is still some flaw in it, so don't expect perfect gameplay.