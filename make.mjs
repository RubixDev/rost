import fs from 'fs'

let got = JSON.parse(fs.readFileSync('./wasm.json').toString())
for (const [instr, bytes] of got) {
    console.log(`/// ${instr} = ${bytes.map(byte => '0x' + byte).join(' ')}`)
    const constName = instr.toUpperCase().replaceAll('.', '_')
    const constType = bytes.length > 1 ? `[u8; ${bytes.length}]` : 'u8'
    const constVal = bytes.length > 1 ? `[${bytes.map(byte => '0x' + byte).join(', ')}]` : '0x' + bytes[0]
    console.log(
        `pub const ${constName}: ${constType} = ${constVal};`,
    )
    console.log()
}
