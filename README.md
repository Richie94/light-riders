# light-riders Bot
My implementation for [riddles.io light riders competition](https://starapple.riddles.io/competitions/light-riders)

My first approach is the python-version, which works similar, but not as good as the new rust version.
My strategic approach is based on mini-max algorithm. 
My score metric compares the amount of field I can reach first vs enemy can reach first.

If the enemy is not reachable any more, my bot tries to keep as many adges as possible to survive long. 

Every iteration the time is stopped and my search depth will be adjusted. So far it is tuned for timebank 10000 and time_per_move 200.

## Build Instructions

Run ```RUSTFLAGS="-Ctarget-cpu=native" cargo build --release``` for getting the optimized program in the target/release folder. You can then just execute it in a local program like this [Ai Bot Workspace] (https://github.com/jmerle/ai-bot-workspace).
