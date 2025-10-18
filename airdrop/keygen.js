"use strict";
var __spreadArray = (this && this.__spreadArray) || function (to, from, pack) {
    if (pack || arguments.length === 2) for (var i = 0, l = from.length, ar; i < l; i++) {
        if (ar || !(i in from)) {
            if (!ar) ar = Array.prototype.slice.call(from, 0, i);
            ar[i] = from[i];
        }
    }
    return to.concat(ar || Array.prototype.slice.call(from));
};
Object.defineProperty(exports, "__esModule", { value: true });
var kit_1 = require("@solana/kit");
var keypair = await crypto.subtle.generateKey({ name: "Ed25519" }, true, ["sign", "verify"]);
var privateKeyJwk = await crypto.subtle.exportKey('jwk', keypair.privateKey);
var privateKeyBase64 = privateKeyJwk.d;
if (!privateKeyBase64)
    throw new Error('Failed to get private key bytes');
var privateKeyBytes = new Uint8Array(Buffer.from(privateKeyBase64, 'base64'));
var publicKeyBytes = new Uint8Array(await crypto.subtle.exportKey('raw', keypair.publicKey));
var keypairBytes = new Uint8Array(__spreadArray(__spreadArray([], privateKeyBytes, true), publicKeyBytes, true));
var signer = await (0, kit_1.createKeyPairSignerFromBytes)(keypairBytes);
console.log("You have generated a new Solana wallet: ".concat(signer.address));
