var pendingHljs = null;

// Runs highlighting and changes external links to use target blank.
export function run_hljs() {
    if (pendingHljs === null) {
        // We have to use a timeout to ensure this happens after the dom is processed.
        pendingHljs = setTimeout(__run_hljs, 0);
    }
}

function __run_hljs() {
    pendingHljs = null;
    hljs.highlightAll();
}
