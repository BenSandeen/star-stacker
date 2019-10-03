# Project Goals
Develop a simple program to allow users to take several photographs taken of the night sky using a _stationary_ camera
and "stack" those photos into a single image, emulating a much longer exposure, in order to make the stars more visible
without producing star trails (stars should appear as points in the final stacked image)

### Important things to consider
* [x] Need some way to load RAW (and probably FITS, TIFF) image files
    * [x] Decided to support BMP and/or TIFF images, since the super popular `image` crate implements BMP
* [ ] Need some way to locate stars
* [ ] Need some way to track stars across multiple images
* [ ] Need way to intelligently crop the edges
* [ ] Need way to either ignore landscape or allow user to tell program what part of a photo is landscape (and should thus
be ignored)
* [ ] Do we want to perform any sort of noise correction/removal or any other processing?