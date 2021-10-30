'use strict';

/*
For anchor elements, as known as <a> tags.
 */

document.querySelectorAll('a').forEach(aElement => {
    // Open new tab for links to other site.
    if (aElement.origin !== window.origin) {
        aElement.target = '_blank';
    }
});
