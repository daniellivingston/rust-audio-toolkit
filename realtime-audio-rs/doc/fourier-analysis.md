
> Unless your audio data consists of pure sinusoidal note sources, a raw FFT magnitude plot will give you all the harmonics or overtones of pitched musical sounds, as well as any fundamental pitch frequency spectrum (if any). So, with most common polyphonic music recordings, pitch detection/estimation, if possible, usually involves a significant amount of post-processing of any FFT results. Not just picking the largest magnitude peaks.
> 
> Either Cepstral processing (or computing the Cepstrum), or the Harmonic Product Spectrum algorithm, look for periodic sequences of overtones or harmonic peaks embedded in FFT spectra, which might indicate a source pitch candidate. (e.g. Look for embedded trains of some number of magnitude peaks at evenly spaced frequency multiples.) But, especially with polyphony and many common chord types, there will likely be some number of false positives that you will have to figure out how to discount or ignore.
>
> Or, instead of using an FFT, you could also try looking at auto-correlation peaks (or other lag-based partial similarity measures), and see if these peaks correspond to the appropriate pitch range and degree of polyphony.
> ([source](https://dsp.stackexchange.com/a/54076))

# References

- Sego, D. Demystifying Fourier analysis. <https://dsego.github.io/demystifying-fourier/>
