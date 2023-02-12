# Meta-collage

Takes any video and outputs a video where every frame of said video is composed of smaller images of frames of said video
As I made this program soely to make a steamed hams video there are several notable flaws

- The .cache folder needs to be manually deleted if you change the input frames (The point of this folder was to make it so that)
- The collagify function cant be called twice on different files as the cache is static.

To actually run this.
1. Put the images frames in the input folder (can be produced with ffmpeg)
2. `cargo run -r -- <frames_dir:path> <collage_dir:path> <output_dir:path> <block_size:int>` in the root folder
3. Wait (depending on how many frames you have this can take hours)
4. Take the frames that are outputted and recompile them into a video (this can also be done with ffmpeg)
