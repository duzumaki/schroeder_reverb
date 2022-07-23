# schroeder_reverb

A Schroeder reverb implementation in rust.

Disclaimer: I am not adept in digital signal processing or anything. It's just something I found interesting last week and came across papers discussing audio
reverb.

Check out this video if you want some theory context on what this tpye of reverb is: 
https://youtu.be/k1SorBkeqlo

<img width="908" alt="image" src="https://user-images.githubusercontent.com/56260075/180619109-e8577f47-099d-4f56-8e0f-d9953d768b64.png">





How the code works:

1. Reads wav audio file into a "samples" Vector.
2. Runs 4 comb filters in parrallele on the samples in order to add the delay effect that's assosciated with reverb.
(this isn't the best algorithm for reverb as the result is quite metallic compared to modern approaches).
3. Run the combined output of the comb filters through 2 all pass filters in series order to smooth and normalise the samples but also fatten up the impulse response density.


Wav audio was chosen only for 2 reasons.
Wav is uncompressed which means no decoding code needed
The [hound crate](https://crates.io/crates/hound) makes it easy to read and write wav files.

