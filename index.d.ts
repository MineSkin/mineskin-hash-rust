/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export declare function sum(a: number, b: number): number
export declare function encodeImage(buffer: Uint8Array): ImageWithHashes
export declare class ImageWithHashes {
  png: Buffer
  minecraftHash: Buffer
  hash: Buffer
  hex: string
}
