import test from 'ava'

import { sum, encodeImage } from '../index.js'
import * as fs from "fs";

test('sum from native', (t) => {
    t.is(sum(1, 2), 3)
})

// empty hash a67c89456ea2fcd58cdf557c01357632d4f0d28c51dd0d8f79a4c739352f71a7

test('encode empty classic', t => {
    let buffer = fs.readFileSync('images/a67c89456ea2fcd58cdf557c01357632d4f0d28c51dd0d8f79a4c739352f71a7.png');
    console.info('buffer.length',buffer.length);
    console.info('buffer.byteLength', buffer.byteLength)
    console.info('buffer',buffer);
    buffer = Buffer.from(buffer.buffer.slice(buffer.byteOffset, buffer.byteOffset + buffer.byteLength))
    console.info('buffer.length',buffer.length);
    console.info('buffer.byteLength', buffer.byteLength)
    console.info('buffer',buffer);


    //let arr = new Uint8Array(buffer);
    let encoded = encodeImage(buffer, true);

    let hash = Buffer.from(encoded.hash).toString('hex');
    console.log(hash);

    let mchash = Buffer.from(encoded.minecraftHash).toString('hex');
    console.log(mchash);

    t.is(mchash,'a67c89456ea2fcd58cdf557c01357632d4f0d28c51dd0d8f79a4c739352f71a7')

    console.log(encoded.hex)
})

// test('encode random 1 classic', t => {
//     let buffer = fs.readFileSync('images/6a12c0010c6e708422f5ba5121ba4a8adc9c7374c2b96fd0754c84f25e181598.png');
//     buffer = Buffer.from(buffer.buffer.slice(buffer.byteOffset, buffer.byteOffset + buffer.byteLength))
//     console.log(buffer.length);
//     console.log(buffer.byteLength);
//     //let arr = new Uint8Array(buffer);
//     let encoded = encodeImage(buffer, true);
//
//     let hash = Buffer.from(encoded.hash).toString('hex');
//     console.log(hash);
//
//     let mchash = Buffer.from(encoded.minecraftHash).toString('hex');
//     console.log(mchash);
//
//     t.is(mchash,'6a12c0010c6e708422f5ba5121ba4a8adc9c7374c2b96fd0754c84f25e181598')
//
//     console.log(encoded.hex)
// })
//
// test('encode random 1 slim', t => {
//     let buffer = fs.readFileSync('images/6a12c0010c6e708422f5ba5121ba4a8adc9c7374c2b96fd0754c84f25e181598.png');
//     buffer = Buffer.from(buffer.buffer.slice(buffer.byteOffset, buffer.byteOffset + buffer.byteLength))
//     console.log(buffer.length);
//     console.log(buffer.byteLength);
//     //let arr = new Uint8Array(buffer);
//     let encoded = encodeImage(buffer, false);
//
//     console.log(Buffer.from('6a12c0010c6e708422f5ba5121ba4a8adc9c7374c2b96fd0754c84f25e181598', 'hex'))
//     console.log(Buffer.from(encoded.minecraftHash))
//     console.log(encoded.minecraftHash)
//
//     let hash = encoded.hash.toString('hex');
//     console.log(hash);
//
//     let mchash = encoded.minecraftHash.toString('hex');
//     console.log(mchash);
//
//     console.log(encoded.hex)
// })