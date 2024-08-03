# devlogs

## 2024-08-02 - phil
I am having trouble getting the DSP wired up as a fully-adjustable waveform generator. Not sure what the problem is, and
that's the problem. Luckily, Seth implemented the `piano` module that is a simple tone generator. The issue with the
piano module is that it plays a single tone in stereo. When plotting this in `XY` mode, this ends up being a diagonal
line, where changing pitch changes the length of the line.

 I had an idea yesterday for how to get around my troubles and still get a demo off the ground---use discrete steps,
i.e. a list of pitches, instead of connecting inputs to a fully adjustable signal generator. Left and right channels can
still be independent here, giving some control over the shape. This only allows adjustments in frequency, so maybe there
should be a permanent pi/2 phase shift between them.

Another idea around input mapping is to avoid the keyboard and use the mouse. For example, left click and drag to adjust
one parameter, using the mouse x-axis to adjust one tone and the y-axis motion to adjust the second tone.
