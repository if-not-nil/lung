# client
#   client: you
v0.1 challenge please
client: jebediah

# server
v0.1 status 90: challenge given
session: a2f3j5
nonce: [base64 32 bytes]

# client
v0.1 challenge accepted
session: a2f3j5
client: jebediah
hmac: [base64 HMAC_SHA256(PSK, nonce || "alice")]

# server
v0.1 status 91: challenge completed
session: a2f3j5
ok: true

# messages after this
v0.1 msg
session: a2f3j5
nonce: [base64 12 bytes]
enc: [base64 AEAD(PSK, nonce, plaintext)]
