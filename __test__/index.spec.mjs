import test from 'ava'

import { sum, encodeImage } from '../index.js'
import * as fs from "fs";

test('sum from native', (t) => {
    t.is(sum(1, 2), 3)
})

// empty hash a67c89456ea2fcd58cdf557c01357632d4f0d28c51dd0d8f79a4c739352f71a7

test('encode empty classic', t => {
    let buffer = fs.readFileSync('images/a67c89456ea2fcd58cdf557c01357632d4f0d28c51dd0d8f79a4c739352f71a7.png');

    //let arr = new Uint8Array(buffer);
    let encoded = encodeImage(buffer, true);

    let hash = Buffer.from(encoded.hash).toString('hex');
    console.log(hash);

    let mchash = Buffer.from(encoded.minecraftHash).toString('hex');
    console.log(mchash);

    t.is(mchash,'a67c89456ea2fcd58cdf557c01357632d4f0d28c51dd0d8f79a4c739352f71a7')

    console.log(encoded.hex)
})

test('encode random 1 classic', t => {
    let buffer = fs.readFileSync('images/6a12c0010c6e708422f5ba5121ba4a8adc9c7374c2b96fd0754c84f25e181598.png');
    buffer = Buffer.from(buffer.buffer.slice(buffer.byteOffset, buffer.byteOffset + buffer.byteLength))
    //let arr = new Uint8Array(buffer);
    let encoded = encodeImage(buffer, true);

    let hash = Buffer.from(encoded.hash).toString('hex');
    console.log(hash);

    let mchash = Buffer.from(encoded.minecraftHash).toString('hex');
    console.log(mchash);

    t.is(mchash,'6a12c0010c6e708422f5ba5121ba4a8adc9c7374c2b96fd0754c84f25e181598')

    console.log(encoded.hex)
})


test('encode random 2 classic', t => {
    let buffer = fs.readFileSync('images/randomImage1.png');
    let encoded = encodeImage(buffer, true);

    let hash = encoded.hash.toString('hex');
    console.log(hash);

    let mchash = encoded.minecraftHash.toString('hex');
    console.log(mchash);

    t.is(mchash, '512e1f13b41b6214c7e2597c3a69a06f56564eed5e5d42b3c8f519c079c47806');

    console.log(encoded.hex)
})

test('encode random 3 classic', t => {
    let buffer = fs.readFileSync('images/randomImage2.png');
    let encoded = encodeImage(buffer);

    let hash = encoded.hash.toString('hex');
    console.log(hash);

    let mchash = encoded.minecraftHash.toString('hex');
    console.log(mchash);

    t.is(mchash, '57a6d6ddebafe045e9a0501c7c489f6de47e670fc4ef1b0c1b2c2a9a2c8c0cbc');

    console.log(encoded.hex)
})