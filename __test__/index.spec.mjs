import test from 'ava'

import { sum, encodeImage } from '../index.js'
import * as fs from "node:fs";

test('sum from native', (t) => {
    t.is(sum(1, 2), 3)
})

test('encode random 1', t => {
    let buffer = fs.readFileSync('images/6a12c0010c6e708422f5ba5121ba4a8adc9c7374c2b96fd0754c84f25e181598.png');
    let arr = new Uint8Array(buffer);
    let encoded = encodeImage(arr);

    let hash = Buffer.from(encoded.hash).toString('hex');
    console.log(hash);

    let mchash = Buffer.from(encoded.minecraftHash).toString('hex');
    console.log(mchash);

    console.log(encoded.hex)
})