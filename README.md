# Potentially malicious browser extensions

I’ve been documenting lots of potentially malicious browser extensions in Chrome Web Store [[1]](https://palant.info/2023/05/31/more-malicious-extensions-in-chrome-web-store/) [[2]](https://palant.info/2023/06/05/introducing-pcvark-and-their-malicious-ad-blockers/) [[3]](https://palant.info/2023/06/08/another-cluster-of-potentially-malicious-chrome-extensions/). As the lists of affected extension IDs are getting long and difficult to keep track of, I decided to add them to this repository (see list.txt).

Note: Some extensions listed are no longer available via Chrome Web Store. If they were installed prior to removal however, they often remain installed.

## Using check-extensions utility

Instead of checking extension IDs against the list manually, you can also run the `check-extensions` utility. When run without parameters, it will try locating all browser profiles of Chrome and Chromium-based browsers under your user account. A list of browser profiles to check can also be given explicitly as command line parameters.
