JSON.stringify(
    [...document.querySelectorAll('tr')].filter(row => row.firstChild.firstChild.firstChild.innerText !== undefined)
        .map(row => [row.firstChild.firstChild.firstChild.innerText, row.children[1].firstChild.firstChild.innerText])
        .map(row => [
            [...row[0].matchAll(/\\mathsf\{(([^{]|\{.+?\})+?)\}/g)].map(match =>
                match[1].replaceAll('\\_', '_').replaceAll(/\{|\}/g, '')
            ),
            [...row[1].matchAll(/\\mathdef\d+\{([0-9A-F]{2})\}/g)].map(match => match[1]),
        ]).map(row => [row[0].length > 1 ? row[0][0] + '.' + row[0].slice(1).join('') : row[0][0], row[1]]),
)
