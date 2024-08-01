# HellScope

A bullet hell rhythm game created for
[Bevy Jam #5](https://itch.io/jam/bevy-jam-5) with the theme: **cycles**.

![demo](doc/assets/dynamic.mp4)

## Credits
|Philip Linden | Audio, Concept & Code |
|Shane Celis   | Audio, Shaders & Code |
|John Breen    | Art                   |
|David Breen   | Concept               |

## Gameplay
Core Gameplay:

- [x] Change parameters of two waveforms, displayed as a Lissajous pattern.
- [ ] Control the X phase with A and D, control the Y phase with W and S. (Maybe change this with the mouse position?)
- [ ] Bullets are locked on rails to the Lissajou pattern and traverse the pattern
      for the duration of one period.
- [ ] Left click to fire bullets whose lifetime is tied to the X waveform period,
      and Right click to fire bullets whose lifetime is tied to the Y waveform
      period.
- [ ] Shooting consumes a (rechargeable) resource called _charge_.
- [ ] "Anomalies" are the target for bullets. Anomalies add a resource called _heat_
      which is the player's damage bar. If this bar fills, the game ends.

Progression:

- [ ] Spend charge to fire bullets or buy upgrades.
- [ ] Upgrades increase frequency, amplitude, or charge capacity, or to reduce heat.
- [ ] Allow infinite upgrades, to ridiculous results.

Stretch Goals:

- [ ] The X and Y waveforms map directly to Left and Right audio channels, including
      game sound effects, UI blips and beeps, and music.
- [ ] the longer your lissajous pattern the longer the projectiles stay on the
      screen, but the more complex and erratic their pattern so you have less aim,
      but technically more firepower on screen at once
- [ ] You can get items that make your lissajous path go crazy and it makes the
      bullets go crazy all over the screen.
    - These are like the audio impulse weapons that make a mushroom shape. or a
      world or spring, ect.
    - They allow your projectiles to hit enemies at multiple Hz effectively
      being screen clear/ bomb items
- [ ] Bosses can inflict audio disturbances to your line that can shake projectiles
      off it.
    - Like maybe the bass builds through the boss fight in a cycle, each beat
      perturbing your line a bit more till the projectiles just shake off and
      fly in random directions both harming boss and you.

## Before Release TODO

- [ ] Publish new `bevy_video_glitch`; remove cargo patch.

## Look & Feel
The interface looks like a classic analog oscilloscope. Changing parameters with
upgrades or user inputs directly reflects the turning of dials.

Heavy inspiration from radar displays, analog oscilloscopes, and early 80's
"sci-fi" computer terminals. Vaporwave background music, retro blips and beeps
for UI interactions. Muted color palette of slate gray and pure green, with pure
red warnings. No solid faces, only wireframes of primitive shapes. CRT-effects,
intense bloom, and video glitches during anomalies.

![inspiration 3](doc/assets/lissajous.gif)
[inspiration - lissajous tutorial](https://www.youtube.com/watch?v=t6nGiBzGLD8)

# Signal Processing

This game is inspired by digital signal processing (DSP) for audio sythesizers
and the way analog oscilloscopes visualize two signals as orthogonal axes.

Here, the idea is to play an audio signal as a stereo pair of tones. The right
audio channel plays waveform X while the left audio channel plays waveform Y.
Audio is normally presented as two waveforms showing signal over time. For
this game, we want to show these waveforms as signals plotted together on 
orthogonal axes. This is visualized as a [Lissajous curve](https://academo.org/demos/lissajous-curves/).

![o-scope](doc/assets/dynamic.mp4)

## Lissajous Curves
The Lissajous curve is created by converting each wave into its cartesian form,
and plotting them on orthogonal axes. Recall the unit circle is defined in (x,y)
coordinates:

```
                y
                ↑
           (0,1)|               sin(θ + φ)
            . - + - .          /
         .'     |     '.      /
       '        |       o (x,y)
      '         |      / '  \   
     '          |     /   '  \
     '          |    /    '  cos(θ + φ) 
      '         |  θ/     '
       '        |  /     '
         '.     | / φ .'
-----+----------+---------+--> x
   (-1,0)       |         (1,0)
                |

x = cos(θ + φ) = cos(ωt + φ)
y = sin(θ + φ) = sin(ωt + φ)

where:
θ = ωt = angle (in radians)
ω = angular frequency (rad/s)
t = time (s)
φ = phase shift (in radians)

Point (x,y) rotates counterclockwise as t increases.
Phase shift φ moves the starting point of rotation.
```

The Lissajous curve is formed by setting x and y to be driven by two waveforms.
The amplitudes, frequencies, and phases of waveform X and waveform Y are plotted
using the same relationships that built the unit circle. When the parameters
for each wave become distinct, you get some wild patterns.
```
x = A*cos(ω₁t + φ₁)
y = B*sin(ω₂t + φ₂)

where:
A, B = amplitudes
ω₁, ω₂ = angular frequencies (rad/s)
t = time (s)
φ₁, φ₂ = phase shifts (in radians)

for example, 
    if
        ω₁== ω₂
    and φ₁== φ₂
    
    Then the pattern is an ellipse. 
```
![lissajous](doc/assets/lissajous.gif)

## Wave Generation in FunDSP
We use [FunDSP](https://github.com/SamiPerttu/fundsp) in [Bevy](https://bevyengine.org/)
via [bevy_fundsp](https://github.com/harudagondi/bevy_fundsp).

FunDSP approaches audio synthesis and DSP with node graphs. Every synthesized
waveform signal begins its life as a simple oscillator like a [sine, square,
or sawtooth wave](https://www.perfectcircuit.com/signal/difference-between-waveforms).

![wave examples](https://i.imgur.com/I16C1Bd.png)

These signals are filtered, combined, and altered by a FunDSP's linear operators.

![fundsp operators](https://github.com/SamiPerttu/fundsp/raw/master/operators.png)
