# Get root collection

## Request

```
POST /documents/ HTTP/1.1
Accept: */*
Origin: http://10.11.99.1
Referer: http://10.11.99.1/
Accept-Encoding: gzip, deflate
Host: 10.11.99.1
User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/15.6 Safari/605.1.15
Content-Length: 0
Accept-Language: en-GB,en;q=0.9
Connection: keep-alive
```

## Response

```json
[
  {
    "ID": "aa889098-2e7f-45f8-b8de-be4dbb964aa7",
    "modified_client": "2022-08-01T09:35:56.211211Z",
    "Parent": "",
    "type": "DocumentType",
    "vissible_name": "Sport",
    "dummyDocument": false,
    "file_type": "notebook",
    "pageCount": 3,
    "sizeInBytes": "8384324",
  },
  {
    "ID": "1aa828f4-c03c-476b-986c-fa0e9e6b8c85",
    "modified_client": "2021-12-23T09:33:27.902902Z",
    "Parent": "",
    "type": "CollectionType",
    "Version": 0,
    "vissible_name": "Projects"
  },
]
```

# Get collection

## Request

```
POST /documents/1aa828f4-c03c-476b-986c-fa0e9e6b8c85 HTTP/1.1
Accept: */*
Origin: http://10.11.99.1
Referer: http://10.11.99.1/
Accept-Encoding: gzip, deflate
Host: 10.11.99.1
User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/15.6 Safari/605.1.15
Content-Length: 0
Accept-Language: en-GB,en;q=0.9
Connection: keep-alive
```

## Response

```json
[
    {
        "Bookmarked": false,
        "CurrentPage": 39,
        "ID": "e23d40e2-72dd-459d-b3a5-21b101ebb56d",
        "modified_client": "2022-07-29T13:43:59.324324Z",
        "Parent": "1aa828f4-c03c-476b-986c-fa0e9e6b8c85",
        "type": "DocumentType",
        "vissible_name": "ZÃ¼hlke",
        "dummyDocument": false,
        "file_type": "notebook",
        "pageCount": 40,
        "sizeInBytes": "59780714",
    }
]
```

# Download file

## Request

```json

```

## Response

```json

```