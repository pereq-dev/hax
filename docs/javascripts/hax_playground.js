const PLAYGROUND_URL = 'https://hax-playground.cryspen.com';

// Fetches the commit hash for latest `main` of hax
async function get_latest_hax_main() {
    let commits = await (await fetch(PLAYGROUND_URL + '/git-refs')).text();
    return commits.match(/(.*);refs\/remotes\/origin\/main;/).pop();
}

// This line should not be edited: it is used in the actions `docs-deploy.yml` and
// `docs-playwright.yml`.
const HAX_PLAYGROUND_FORCED_VERSION = false;

// Call into the API of the hax playground
async function call_playground(result_block, query, text, parent_node) {
    let raw_query = async (API_URL, hax_version, query, files, on_line_received) => {
        let response = await fetch(`${API_URL}/query/${hax_version}/${query}`, {
            method: "POST",
            headers: {
                'Accept': 'application/json',
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(files),
        });

        let decoder = new TextDecoder();
        let leftover = "";
        let reader = response.body.getReader();
        while (true) {
            const { done, value } = await reader.read();
            if (done) break;
            leftover += decoder.decode(value);
            let lines = leftover.split('\n');
            let entire_lines = lines.slice(0, -1);
            leftover = lines.slice(-1)[0];
            for (const line of entire_lines)
                on_line_received(line);
        }
    };
    let ansi_up = new AnsiUp();
    let first = true;
    let logs = document.createElement('div');
    logs.style = 'font-size: 80%; background: #00000010; padding: 3px; white-space:pre-wrap;';
    let hax_version = HAX_PLAYGROUND_FORCED_VERSION || await get_latest_hax_main();
    let lean_backend = query.startsWith('lean');
    raw_query(
        PLAYGROUND_URL,
        hax_version,
        query,
        [['src/lib.rs', text]],
        x => {
            if (first) {
                result_block.style.padding = '0.7em 1.2em';
                result_block.innerText = "";
                result_block.appendChild(logs);
            }
            first = false;
            let json = {};
            try {
                json = JSON.parse(x);
            } catch (_) { }
            if (json.Stderr || json.Stdout) {
                logs.innerHTML += '<div>' + ansi_up.ansi_to_html(json.Stderr || json.Stdout) + "</div>";
            }
            if (json.Done) {
                let out = [];
                for (let file in json.Done.files) {
                    if (file.endsWith('.rs'))
                        continue;
                    let contents = json.Done.files[file];
                    contents = (contents.split(lean_backend ? 'set_option linter.unusedVariables false' : 'open FStar.Mul')[1] || contents).trim();
                    contents = contents.replace(/$/gm, ' ').trim();
                    out.push([file, contents]);
                }
                if (json.Done.success)
                    result_block.innerText = "";
                else
                    result_block.innerHTML += "<br/>";
                let result = document.createElement('pre');
                result.style.whiteSpace = 'pre-wrap';
                if (out.length == 1) {
                    result.textContent = out[0][1];
                } else {
                    result.textContent = out.map(([file, s]) => '(* File: ' + file + ' *) \n' + s).join('\n\n').trim();
                }
                result_block.appendChild(result);
                hljs.highlightBlock(result);
                result_block.innerHTML += `<br/><a style="float:right; font-family: 'Open Sans', sans-serif; font-size: 70%; cursor: pointer; color: gray; text-transform: uppercase; position: relative; top: -10px;" href='${PLAYGROUND_URL}/#${lean_backend ? "lean" : "fstar"}/${hax_version}/${LZString.compressToEncodedURIComponent(text)}'>Open in hax playground ↗</a>`;
                parent_node.classList.remove("state-success", "state-failure");
                parent_node.classList.add("state-" + (json.Done.success ? "success" : "failure"));
                if (json.Done.success && query.includes('+tc')) {
                    result_block.innerHTML += `<div style="float: left; padding: 3px; padding-top: 8px; position: relative; top: 6px;"><span style="color: gray;">Status: </span><span style="color: green">✓ ${lean_backend ? "Lean" : "F*"} successfully typechecked!</span></div>`;
                }
            }
        },
    );
}

function setup_hax_playground() {
    if (document.querySelector('.md-hax-playground'))
        return;
    console.log('setup');
    for (let e of document.querySelectorAll('pre')) {
        let code = e.querySelector("code");
        if (!code)
            continue;
        let lines = [
            ...code.children
        ].map(line => line.innerText.replace(/^\n+/, '').replace(/\n+$/, ''))
            .join("\n").trim().split('\n');
        console.log({ lines });
        let contents = lines.filter(line => !line.startsWith('# ')).join('\n');
        let w = e.parentElement;
        if (!w.classList.contains("playable"))
            continue;
        let backend = w.classList.contains("lean-backend") ? 'lean' : 'fstar';

        code.innerHTML = "<pre></pre>";
        let inner = code.children[0];
        inner.style.backgroundColor = "transparent";
        inner.classList.add("md-hax-playground-pre");

        let editor = new codemirror.EditorView({
            doc: contents,
            extensions: [codemirror.basicSetup, codemirror.rust()],
            parent: inner,
            lineNumbers: false,
        });

        let result_block = document.createElement("pre");
        result_block.classList.add("hax-playground-pre");
        result_block.style.fontFamily = '"Monaco", "Menlo", "Ubuntu Mono", "Consolas", "Source Code Pro", "source-code-pro", monospace';
        result_block.style.fontSize = '0.85em';
        result_block.style.background = '#f3f3f3';
        w.append(result_block);

        let header = lines.filter(line => line.startsWith('# ')).map(line => line.slice(2)).join('\n');
        let getCode = () => header + '\n' + editor.state.doc.toString();


        let button_translate = document.createElement("button");
        button_translate.innerHTML = `<i class="fa-solid fa-play"></i>`;
        button_translate.classList.add('md-icon');
        button_translate.classList.add('md-clipboard');
        button_translate.classList.add('md-hax-playground');
        button_translate.style.right = "2.4em";
        button_translate.onclick = () => {
            call_playground(result_block, backend, getCode(), w);
        };
        e.prepend(button_translate);

        let button_tc = document.createElement("button");
        button_tc.innerHTML = `<i class="fa-solid fa-check"></i>`;
        button_tc.classList.add('md-icon');
        button_tc.classList.add('md-clipboard');
        button_tc.classList.add('md-hax-playground');
        button_tc.style.right = "4.5em";
        button_tc.onclick = () => {
            call_playground(result_block, backend + '+tc', getCode(), w);
        };

        e.prepend(button_tc);
    }
}

window.addEventListener('load', () => {
    setup_hax_playground();
    const observer = new MutationObserver(() => {
        if (document.querySelector('.md-hax-playground'))
            return;
        setTimeout(setup_hax_playground, 200);
    });
    observer.observe(document.querySelector('body'), { childList: true, subtree: true });
});


