# document data generated using this cli tool https://github.com/input-output-hk/catalyst-libs/tree/main/rust/signed_doc

PUT http://localhost:3030/api/draft/document
# cspell: disable
Authorization: Bearer catv1.UJm5ZNT1n7l3_h3c3VXp1R9QAZStRmrxdtYwTrdsxKWIF1hAi3mqbz6dPNiICQCkoXWJs8KCpcaPuE7LE5Iu9su0ZweK_0Qr9KhBNNHrDMCh79-fruK7WyNPYNc6FrjwTPaIAQ
# cspell: enable
Content-Type: application/cbor
hex, 84585fa6012703183270436f6e74656e742d456e636f64696e676262726474797065d82550913c9265f9f944fcb3cf9d9516ae9baf626964d8255001946ea1818a7e0eb6b16169f02ffd4e63766572d8255001946ea1818a7e0eb6b16169f02ffd4ea0581c8b0b807b22616765223a32392c226e616d65223a22416c6578227d0380;
HTTP *
[Asserts]
status >= 201
status <= 204

PUT http://localhost:3030/api/draft/document
# cspell: disable
Authorization: Bearer catv1.UJm5ZNT1n7l3_h3c3VXp1R9QAZStRmrxdtYwTrdsxKWIF1hAi3mqbz6dPNiICQCkoXWJs8KCpcaPuE7LE5Iu9su0ZweK_0Qr9KhBNNHrDMCh79-fruK7WyNPYNc6FrjwTPaIAQ
# cspell: enable
Content-Type: application/cbor
hex, 84585fa6012703183270436f6e74656e742d456e636f64696e676262726474797065d82550913c9265f9f944fcb3cf9d9516ae9baf626964d8255001946ea1818a7e0eb6b16169f02ffd4e63766572d8255001946ea1818a7e0eb6b16169f02ffd4ea0581c8b0b807b22616765223a32392c226e616d65223a22416c6578227d0380;
HTTP 204

GET http://localhost:3030/api/draft/document/01946ea1-818a-7e0e-b6b1-6169f02ffd4e
# cspell: disable
Authorization: Bearer catv1.UJm5ZNT1n7l3_h3c3VXp1R9QAZStRmrxdtYwTrdsxKWIF1hAi3mqbz6dPNiICQCkoXWJs8KCpcaPuE7LE5Iu9su0ZweK_0Qr9KhBNNHrDMCh79-fruK7WyNPYNc6FrjwTPaIAQ
# cspell: enable
HTTP 200

POST http://localhost:3030/api/draft/document/index
# cspell: disable
Authorization: Bearer catv1.UJm5ZNT1n7l3_h3c3VXp1R9QAZStRmrxdtYwTrdsxKWIF1hAi3mqbz6dPNiICQCkoXWJs8KCpcaPuE7LE5Iu9su0ZweK_0Qr9KhBNNHrDMCh79-fruK7WyNPYNc6FrjwTPaIAQ
# cspell: enable
{
    "id": { "eq": "01946ea1-818a-7e0e-b6b1-6169f02ffd4e" }
}
HTTP 200

# try to publish document with the same id, but with the different content
PUT http://localhost:3030/api/draft/document
# cspell: disable
Authorization: Bearer catv1.UJm5ZNT1n7l3_h3c3VXp1R9QAZStRmrxdtYwTrdsxKWIF1hAi3mqbz6dPNiICQCkoXWJs8KCpcaPuE7LE5Iu9su0ZweK_0Qr9KhBNNHrDMCh79-fruK7WyNPYNc6FrjwTPaIAQ
# cspell: enable
Content-Type: application/cbor
hex, 84585fa6012703183270436f6e74656e742d456e636f64696e676262726474797065d82550913c9265f9f944fcb3cf9d9516ae9baf626964d8255001946ea1818a7e0eb6b16169f02ffd4e63766572d8255001946ea1818a7e0eb6b16169f02ffd4ea0581c8b0b807b22616765223a33302c226e616d65223a22416c6578227d0380;
HTTP 422

# publish document with the same id, but with different version
PUT http://localhost:3030/api/draft/document
# cspell: disable
Authorization: Bearer catv1.UJm5ZNT1n7l3_h3c3VXp1R9QAZStRmrxdtYwTrdsxKWIF1hAi3mqbz6dPNiICQCkoXWJs8KCpcaPuE7LE5Iu9su0ZweK_0Qr9KhBNNHrDMCh79-fruK7WyNPYNc6FrjwTPaIAQ
# cspell: enable
Content-Type: application/cbor
hex, 84585fa6012703183270436f6e74656e742d456e636f64696e676262726474797065d82550913c9265f9f944fcb3cf9d9516ae9baf626964d8255001946ea1818a7e0eb6b16169f02ffd4e63766572d825500194d649960076639ebc1613291cf25fa0581c8b0b807b22616765223a32392c226e616d65223a22416c6578227d0380;
HTTP *
[Asserts]
status >= 201
status <= 204

# Test page out of range
POST http://localhost:3030/api/draft/document/index?page=92233720368547759
# cspell: disable
Authorization: Bearer catv1.UJm5ZNT1n7l3_h3c3VXp1R9QAZStRmrxdtYwTrdsxKWIF1hAi3mqbz6dPNiICQCkoXWJs8KCpcaPuE7LE5Iu9su0ZweK_0Qr9KhBNNHrDMCh79-fruK7WyNPYNc6FrjwTPaIAQ
# cspell: enable
{
    "id": { "eq": "01946ea1-818a-7e0e-b6b1-6169f02ffd4e" }
}
HTTP 412