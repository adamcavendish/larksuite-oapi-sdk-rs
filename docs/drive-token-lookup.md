# Drive token lookup

`client.drive_v2().file.query_by_token(token, option)` resolves a token to typed
`obj_token`, `obj_type`, and `is_wiki_token` metadata using
`GET /open-apis/drive/v2/files/query_by_token`. It accepts user or tenant tokens
and requires `drive:drive.metadata:readonly`. The input token is query-encoded.

Check `success()` before reading `data`. `is_wiki_token` describes the input;
`obj_token` and `obj_type` describe the underlying object. Successful resolution
does not imply that the object can be downloaded, or grant download permission.
Choose the appropriate native-document export or file-download API yourself.
The SDK does not fall back to another resolver or perform a download implicitly.
