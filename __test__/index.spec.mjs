import test from 'ava'

import { sum, encodeImage } from '../index.js'
import * as fs from "fs";

test('sum from native', (t) => {
    t.is(sum(1, 2), 3)
})

// empty hash a67c89456ea2fcd58cdf557c01357632d4f0d28c51dd0d8f79a4c739352f71a7

function testEncode(t, imagePath, expectedHash, trim = false) {
    let buffer = fs.readFileSync(imagePath);

    //let arr = new Uint8Array(buffer);
    let encoded = encodeImage(buffer);

    let hash = Buffer.from(encoded.hash).toString('hex');
    console.log('hash    ', hash);

    let mchash = Buffer.from(encoded.minecraftHash).toString('hex');
    if (trim) {
        mchash = mchash.replace(/^0+/, '');
    }
    console.log('mchash  ', mchash);

    console.log('expected', expectedHash);
    t.is(mchash, expectedHash)
}

testEncode.title = (providedTitle = "encode", a, b) =>
    `${ providedTitle }: ${ b } ${ a }`

for (const params of [
    ['images/a67c89456ea2fcd58cdf557c01357632d4f0d28c51dd0d8f79a4c739352f71a7.png', 'a67c89456ea2fcd58cdf557c01357632d4f0d28c51dd0d8f79a4c739352f71a7'],
    ['images/6a12c0010c6e708422f5ba5121ba4a8adc9c7374c2b96fd0754c84f25e181598.png', '6a12c0010c6e708422f5ba5121ba4a8adc9c7374c2b96fd0754c84f25e181598'],
    ['images/randomImage1.png', '512e1f13b41b6214c7e2597c3a69a06f56564eed5e5d42b3c8f519c079c47806'],
    ['images/randomImage2.png', '57a6d6ddebafe045e9a0501c7c489f6de47e670fc4ef1b0c1b2c2a9a2c8c0cbc'],
    ['images/randomImage3.png', '1b84a8081994cce186eeae4787f95bce663d6711272782ce659d9cc954fec435'],
    ['images/randomImage4.png', 'e19abca2800650b449fff3f12ed3d1bd0f20861d7cdd00c66d62264b52837e2c'],
    ['images/trimmed0.png', '0aa97a60ff386b6075aea693893106358def0cd580e6360599811e08bde38b6f'], // mojang strips the leading 0, this doesn't currently
    ['images/trimmed0.png', 'aa97a60ff386b6075aea693893106358def0cd580e6360599811e08bde38b6f', true]
]) {
    test(testEncode, ...params);
}
